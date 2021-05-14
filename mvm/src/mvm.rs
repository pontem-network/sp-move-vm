use alloc::borrow::ToOwned;
use alloc::vec::Vec;

use anyhow::Error;

use move_core_types::account_address::AccountAddress;
use move_core_types::gas_schedule::CostTable;
use move_core_types::gas_schedule::InternalGasUnits;
use move_core_types::gas_schedule::{AbstractMemorySize, GasAlgebra, GasUnits};
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{ModuleId, StructTag, TypeTag, CORE_CODE_ADDRESS};
use move_core_types::vm_status::{AbortLocation, StatusCode, VMStatus};
use move_vm_runtime::data_cache::RemoteCache;
use move_vm_runtime::logging::NoContextLog;
use move_vm_runtime::move_vm::MoveVM;
use move_vm_runtime::session::Session;
use move_vm_types::gas_schedule::CostStrategy;
use move_vm_types::natives::balance::{BalanceOperation, NativeBalance, WalletId};
use vm::errors::{Location, VMError, VMResult};

use crate::data::AccessKey;
use crate::data::{
    BalanceAccess, Bank, EventHandler, ExecutionContext, Oracle, State, StateSession, Storage,
    WriteEffects,
};
use crate::types::{Gas, ModuleTx, PublishPackageTx, ScriptTx, VmResult};
use crate::vm_config::loader::load_vm_config;
use crate::Vm;
use hashbrown::HashMap;
use move_core_types::effects::{ChangeSet, Event};

/// MoveVM.
pub struct Mvm<S, E, O, B>
where
    S: Storage,
    E: EventHandler,
    O: Oracle,
    B: BalanceAccess,
{
    vm: MoveVM,
    cost_table: CostTable,
    state: State<S, O>,
    event_handler: E,
    bank: Bank<B>,
}

impl<S, E, O, B> Mvm<S, E, O, B>
where
    S: Storage,
    E: EventHandler,
    O: Oracle,
    B: BalanceAccess,
{
    /// Creates a new move vm with given store and event handler.
    pub fn new(
        store: S,
        event_handler: E,
        oracle: O,
        balance: B,
    ) -> Result<Mvm<S, E, O, B>, Error> {
        let config = load_vm_config(&store)?;

        Ok(Mvm {
            vm: MoveVM::new(),
            cost_table: config.gas_schedule,
            state: State::new(store, oracle),
            event_handler,
            bank: Bank::new(balance),
        })
    }

    /// Stores write set into storage and handle events.
    fn handle_tx_effects(
        &self,
        tx_effects: (ChangeSet, Vec<Event>, HashMap<WalletId, BalanceOperation>),
    ) -> Result<(), VMError> {
        let (change_set, events, wallet_ops) = tx_effects;

        for (addr, acc) in change_set.accounts {
            for (ident, val) in acc.modules {
                let key = AccessKey::from(&ModuleId::new(addr, ident));
                match val {
                    None => {
                        self.state.delete(key);
                    }
                    Some(blob) => {
                        self.state.insert(key, blob);
                    }
                }
            }
            for (tag, val) in acc.resources {
                let key = AccessKey::from((&addr, &tag));
                match val {
                    None => {
                        self.state.delete(key);
                    }
                    Some(blob) => {
                        self.state.insert(key, blob);
                    }
                }
            }
        }

        for (address, ty_tag, msg, caller) in events {
            self.event_handler.on_event(address, ty_tag, msg, caller);
        }

        for (id, op) in wallet_ops.into_iter() {
            match op {
                BalanceOperation::Deposit(amount) => self.bank.deposit(&id, amount)?,
                BalanceOperation::Withdraw(amount) => self.bank.withdraw(&id, amount)?,
            }
        }

        Ok(())
    }

    /// Handle vm result and return transaction status code.
    fn handle_vm_result(
        &self,
        sender: AccountAddress,
        cost_strategy: CostStrategy,
        gas_meta: Gas,
        result: Result<(ChangeSet, Vec<Event>, HashMap<WalletId, BalanceOperation>), VMError>,
        dry_run: bool,
    ) -> VmResult {
        let gas_used = GasUnits::new(gas_meta.max_gas_amount)
            .sub(cost_strategy.remaining_gas())
            .get();

        if dry_run {
            return match result {
                Ok(_) => VmResult::new(StatusCode::EXECUTED, None, gas_used),
                Err(err) => VmResult::new(err.major_status(), err.sub_status(), gas_used),
            };
        }

        match result.and_then(|e| self.handle_tx_effects(e)) {
            Ok(_) => VmResult::new(StatusCode::EXECUTED, None, gas_used),
            Err(err) => {
                let status = err.major_status();
                let sub_status = err.sub_status();
                if let Err(err) = self.emit_vm_status_event(sender, err.into_vm_status()) {
                    VmResult::new(status, sub_status, gas_used);
                    log::warn!("Failed to emit vm status event:{:?}", err);
                }

                VmResult::new(status, sub_status, gas_used)
            }
        }
    }

    fn emit_vm_status_event(&self, sender: AccountAddress, status: VMStatus) -> Result<(), Error> {
        let tag = TypeTag::Struct(StructTag {
            address: CORE_CODE_ADDRESS,
            module: Identifier::new("VMStatus").unwrap(),
            name: Identifier::new("VMStatus").unwrap(),
            type_params: vec![],
        });

        let module = match &status {
            VMStatus::Executed | VMStatus::Error(_) => None,
            VMStatus::MoveAbort(loc, _)
            | VMStatus::ExecutionFailure {
                status_code: _,
                location: loc,
                function: _,
                code_offset: _,
            } => match loc {
                AbortLocation::Module(module) => Some(module.to_owned()),
                AbortLocation::Script => None,
            },
        };
        let msg = bcs::to_bytes(&status)
            .map_err(|err| Error::msg(format!("Failed to generate event message: {:?}", err)))?;

        self.event_handler.on_event(sender, tag, msg, module);
        Ok(())
    }

    fn _publish_module<R, NB: NativeBalance>(
        &self,
        session: &mut Session<'_, '_, R, NB>,
        module: Vec<u8>,
        sender: AccountAddress,
        cost_strategy: &mut CostStrategy,
    ) -> VMResult<()>
    where
        R: RemoteCache,
    {
        cost_strategy.charge_intrinsic_gas(AbstractMemorySize::new(module.len() as u64))?;

        let result = session.publish_module(module, sender, cost_strategy, &NoContextLog::new());
        Self::charge_global_write_gas_usage(cost_strategy, session, &sender)?;
        result
    }

    fn charge_global_write_gas_usage<R, NB: NativeBalance>(
        cost_strategy: &mut CostStrategy,
        session: &mut Session<'_, '_, R, NB>,
        sender: &AccountAddress,
    ) -> VMResult<()>
    where
        R: RemoteCache,
    {
        let total_cost = session.num_mutated_accounts(sender)
            * cost_strategy
                .cost_table()
                .gas_constants
                .global_memory_per_byte_write_cost
                .mul(
                    cost_strategy
                        .cost_table()
                        .gas_constants
                        .default_account_size,
                )
                .get();
        cost_strategy
            .deduct_gas(InternalGasUnits::new(total_cost))
            .map_err(|p_err| p_err.finish(Location::Undefined))
    }
}

impl<S, E, O, B> Vm for Mvm<S, E, O, B>
where
    S: Storage,
    E: EventHandler,
    O: Oracle,
    B: BalanceAccess,
{
    fn publish_module(&self, gas: Gas, module: ModuleTx, dry_run: bool) -> VmResult {
        let (module, sender) = module.into_inner();
        let mut cost_strategy =
            CostStrategy::transaction(&self.cost_table, GasUnits::new(gas.max_gas_amount()));
        let mut session = self.vm.new_session(&self.state, &self.bank);

        let result = self
            ._publish_module(&mut session, module, sender, &mut cost_strategy)
            .and_then(|_| session.finish());

        self.handle_vm_result(sender, cost_strategy, gas, result, dry_run)
    }

    fn publish_module_package(
        &self,
        gas: Gas,
        package: PublishPackageTx,
        dry_run: bool,
    ) -> VmResult {
        let (modules, sender) = package.into_inner();
        let mut cost_strategy =
            CostStrategy::transaction(&self.cost_table, GasUnits::new(gas.max_gas_amount()));

        // We need to create a new vm to publish module packages.
        // Because during batch publishing, the cache mutates.
        // This is not the correct behavior for the dry_run case or for rolling back a transaction.
        let vm = MoveVM::new();
        let mut session = vm.new_session(&self.state, &self.bank);

        for module in modules {
            if let Err(err) = self._publish_module(&mut session, module, sender, &mut cost_strategy)
            {
                return self.handle_vm_result(sender, cost_strategy, gas, Err(err), dry_run);
            }
        }
        self.handle_vm_result(sender, cost_strategy, gas, session.finish(), dry_run)
    }

    fn execute_script(
        &self,
        gas: Gas,
        context: ExecutionContext,
        tx: ScriptTx,
        dry_run: bool,
    ) -> VmResult {
        let state_session = StateSession::new(&self.state, context);
        let mut session = self.vm.new_session(&state_session, &self.bank);

        let (script, args, type_args, senders) = tx.into_inner();
        let sender = senders.get(0).cloned().unwrap_or(AccountAddress::ZERO);

        let mut cost_strategy =
            CostStrategy::transaction(&self.cost_table, GasUnits::new(gas.max_gas_amount()));

        let result = session
            .execute_script(
                script,
                type_args,
                args,
                senders,
                &mut cost_strategy,
                &NoContextLog::new(),
            )
            .and_then(|_| {
                Self::charge_global_write_gas_usage(&mut cost_strategy, &mut session, &sender)
            });

        self.handle_vm_result(
            sender,
            cost_strategy,
            gas,
            result.and_then(|_| session.finish()),
            dry_run,
        )
    }

    fn clear(&self) {
        self.vm.clear();
    }
}

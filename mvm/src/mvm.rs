use anyhow::Error;

use alloc::borrow::ToOwned;
use move_core_types::account_address::AccountAddress;
use move_core_types::gas_schedule::CostTable;
use move_core_types::gas_schedule::{AbstractMemorySize, GasAlgebra, GasUnits};
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{StructTag, TypeTag, CORE_CODE_ADDRESS, NONE_ADDRESS};
use move_core_types::vm_status::{AbortLocation, StatusCode, VMStatus};
use move_vm_runtime::data_cache::TransactionEffects;
use move_vm_runtime::logging::NoContextLog;
use move_vm_runtime::move_vm::MoveVM;
use move_vm_types::gas_schedule::CostStrategy;
use move_vm_types::natives::balance::BalanceOperation;
use vm::errors::{Location, PartialVMError, VMError};
use vm::CompiledModule;

use crate::data::AccessKey;
use crate::data::{
    BalanceAccess, Bank, EventHandler, ExecutionContext, Oracle, State, StateSession, Storage,
    WriteEffects,
};
use crate::types::{Gas, ModuleTx, ScriptTx, VmResult};
use crate::vm_config::loader::load_vm_config;
use crate::Vm;

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
    fn handle_tx_effects(&self, tx_effects: TransactionEffects) -> Result<(), VMError> {
        for (addr, vals) in tx_effects.resources {
            for (struct_tag, val_opt) in vals {
                let ak = AccessKey::from((&addr, &struct_tag));
                match val_opt {
                    None => {
                        self.state.delete(ak);
                    }
                    Some((ty_layout, val)) => {
                        let blob = val.simple_serialize(&ty_layout).ok_or_else(|| {
                            PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                                .finish(Location::Undefined)
                        })?;
                        self.state.insert(ak, blob);
                    }
                };
            }
        }

        for (module_id, blob) in tx_effects.modules {
            self.state.insert(AccessKey::from(&module_id), blob);
        }

        for (address, ty_tag, ty_layout, val, caller) in tx_effects.events {
            let msg = val.simple_serialize(&ty_layout).ok_or_else(|| {
                PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                    .finish(Location::Undefined)
            })?;
            self.event_handler.on_event(address, ty_tag, msg, caller);
        }

        for (id, op) in tx_effects.wallet_ops.into_iter() {
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
        result: Result<TransactionEffects, VMError>,
    ) -> VmResult {
        let gas_used = GasUnits::new(gas_meta.max_gas_amount)
            .sub(cost_strategy.remaining_gas())
            .get();

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
}

impl<S, E, O, B> Vm for Mvm<S, E, O, B>
where
    S: Storage,
    E: EventHandler,
    O: Oracle,
    B: BalanceAccess,
{
    fn publish_module(&self, gas: Gas, module: ModuleTx) -> VmResult {
        let (module, sender) = module.into_inner();

        let mut cost_strategy =
            CostStrategy::transaction(&self.cost_table, GasUnits::new(gas.max_gas_amount()));

        let result = cost_strategy
            .charge_intrinsic_gas(AbstractMemorySize::new(module.len() as u64))
            .and_then(|_| {
                CompiledModule::deserialize(&module)
                    .map_err(|e| e.finish(Location::Undefined))
                    .and_then(|compiled_module| {
                        let module_id = compiled_module.self_id();
                        if sender != *module_id.address() {
                            return Err(PartialVMError::new(
                                StatusCode::MODULE_ADDRESS_DOES_NOT_MATCH_SENDER,
                            )
                            .finish(Location::Module(module_id)));
                        }

                        cost_strategy
                            .charge_intrinsic_gas(AbstractMemorySize::new(module.len() as u64))?;

                        let mut session = self.vm.new_session(&self.state, &self.bank);
                        session
                            .publish_module(
                                module.to_vec(),
                                sender,
                                &mut cost_strategy,
                                &NoContextLog::new(),
                            )
                            .and_then(|_| session.finish())
                    })
            });
        self.handle_vm_result(sender, cost_strategy, gas, result)
    }

    fn execute_script(&self, gas: Gas, context: ExecutionContext, tx: ScriptTx) -> VmResult {
        let state_session = StateSession::new(&self.state, context);
        let mut session = self.vm.new_session(&state_session, &self.bank);

        let (script, args, type_args, senders) = tx.into_inner();
        let sender = senders.get(0).cloned().unwrap_or(NONE_ADDRESS);

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
            .and_then(|_| session.finish());

        self.handle_vm_result(sender, cost_strategy, gas, result)
    }

    fn clear(&self) {
        self.vm.clear();
    }
}

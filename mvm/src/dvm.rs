use crate::access_path::AccessPath;
use crate::data::{EventHandler, State, Storage, WriteEffects};
use crate::types::{Gas, ModuleTx, ScriptTx, VmResult};
use crate::{gas_schedule, Vm};
use move_core_types::gas_schedule::CostTable;
use move_core_types::gas_schedule::{AbstractMemorySize, GasAlgebra, GasUnits};
use move_core_types::vm_status::StatusCode;
use move_vm_runtime::data_cache::TransactionEffects;
use move_vm_runtime::logging::NoContextLog;
use move_vm_runtime::move_vm::MoveVM;
use move_vm_types::gas_schedule::CostStrategy;
use vm::errors::{Location, PartialVMError};
use vm::CompiledModule;

/// MoveVM.
pub struct Mvm<S, E>
where
    S: Storage,
    E: EventHandler,
{
    vm: MoveVM,
    cost_table: CostTable,
    state: State<S>,
    event_handler: E,
}

impl<S, E> Mvm<S, E>
where
    S: Storage,
    E: EventHandler,
{
    /// Creates a new move vm with given store and event handler.
    pub fn new(store: S, event_handler: E) -> Mvm<S, E> {
        Mvm {
            vm: MoveVM::new(),
            cost_table: gas_schedule::cost_table(),
            state: State::new(store),
            event_handler,
        }
    }

    /// Stores write set into storage and handle events.
    fn handle_tx_effects(&self, tx_effects: TransactionEffects) -> VmResult {
        for (addr, vals) in tx_effects.resources {
            for (struct_tag, val_opt) in vals {
                let ap = AccessPath::new(addr, struct_tag.access_vector());
                match val_opt {
                    None => {
                        self.state.delete(ap);
                    }
                    Some((ty_layout, val)) => {
                        let blob = val.simple_serialize(&ty_layout).ok_or_else(|| {
                            PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                                .finish(Location::Undefined)
                        })?;
                        self.state.insert(ap, blob);
                    }
                };
            }
        }

        for (module_id, blob) in tx_effects.modules {
            self.state.insert(
                AccessPath::new(*module_id.address(), module_id.access_vector()),
                blob,
            );
        }

        for (guid, seq_num, ty_tag, ty_layout, val) in tx_effects.events {
            let msg = val.simple_serialize(&ty_layout).ok_or_else(|| {
                PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                    .finish(Location::Undefined)
            })?;
            self.event_handler.on_event(guid, seq_num, ty_tag, msg);
        }

        Ok(())
    }
}

impl<S, E> Vm for Mvm<S, E>
where
    S: Storage,
    E: EventHandler,
{
    fn publish_module(&self, gas: Gas, module: ModuleTx) -> VmResult {
        let (module, sender) = module.into_inner();

        let mut cost_strategy =
            CostStrategy::transaction(&self.cost_table, GasUnits::new(gas.max_gas_amount()));

        cost_strategy.charge_intrinsic_gas(AbstractMemorySize::new(module.len() as u64))?;

        let tx_effects = CompiledModule::deserialize(&module)
            .map_err(|e| e.finish(Location::Undefined))
            .and_then(|compiled_module| {
                let module_id = compiled_module.self_id();
                if sender != *module_id.address() {
                    return Err(PartialVMError::new(
                        StatusCode::MODULE_ADDRESS_DOES_NOT_MATCH_SENDER,
                    )
                    .finish(Location::Module(module_id)));
                }

                cost_strategy.charge_intrinsic_gas(AbstractMemorySize::new(module.len() as u64))?;

                let mut session = self.vm.new_session(&self.state);
                session
                    .publish_module(
                        module.to_vec(),
                        sender,
                        &mut cost_strategy,
                        &NoContextLog::new(),
                    )
                    .and_then(|_| session.finish())
            })?;

        self.handle_tx_effects(tx_effects)
    }

    fn execute_script(&self, gas: Gas, tx: ScriptTx) -> VmResult {
        let mut session = self.vm.new_session(&self.state);

        let (script, args, type_args, senders) = tx.into_inner();
        let mut cost_strategy =
            CostStrategy::transaction(&self.cost_table, GasUnits::new(gas.max_gas_amount()));

        let tx_effects = session
            .execute_script(
                script,
                type_args,
                args,
                senders,
                &mut cost_strategy,
                &NoContextLog::new(),
            )
            .and_then(|_| session.finish())?;

        self.handle_tx_effects(tx_effects)
    }

    fn clear(&mut self) {
        self.vm = MoveVM::new();
    }
}

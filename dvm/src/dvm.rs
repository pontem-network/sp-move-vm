use crate::data::{State, Storage};
use crate::types::{Gas, ModuleTx, ScriptTx, VmResult};
use crate::{gas_schedule, Vm};
use move_core_types::gas_schedule::CostTable;
use move_vm_runtime::move_vm::MoveVM;
use move_vm_types::gas_schedule::CostStrategy;
use move_core_types::gas_schedule::{GasUnits, AbstractMemorySize, GasAlgebra};
use vm::CompiledModule;
use vm::errors::{Location, PartialVMError};
use move_core_types::vm_status::StatusCode;
use move_core_types::language_storage::CORE_CODE_ADDRESS;
use move_vm_runtime::logging::NoContextLog;

pub struct Dvm<S>
    where
        S: Storage,
{
    vm: MoveVM,
    cost_table: CostTable,
    state: State<S>,
}

impl<S> Dvm<S>
    where
        S: Storage,
{
    pub fn new(store: S) -> Dvm<S> {
        Dvm {
            vm: MoveVM::new(),
            cost_table: gas_schedule::cost_table(),
            state: State::new(store),
        }
    }
}

impl<S> Vm for Dvm<S>
    where
        S: Storage,
{
    fn publish_module(&self, gas: Gas, module: ModuleTx) -> VmResult {
        let (module, sender) = module.into_inner();

        let mut cost_strategy =
            CostStrategy::transaction(&self.cost_table, GasUnits::new(gas.max_gas_amount()));

        cost_strategy
            .charge_intrinsic_gas(AbstractMemorySize::new(module.len() as u64))?;

        let res = CompiledModule::deserialize(&module)
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

        // todo save effects.
        Ok(())
    }

    fn execute_script(&self, gas: Gas, tx: ScriptTx) -> VmResult {
        let mut session = self.vm.new_session(&self.state);

        let (script, args, type_args, senders) = tx.into_inner();
        let mut cost_strategy =
            CostStrategy::transaction(&self.cost_table, GasUnits::new(gas.max_gas_amount()));

        let res = session
            .execute_script(
                script,
                type_args,
                args,
                senders,
                &mut cost_strategy,
                &NoContextLog::new(),
            )
            .and_then(|_| session.finish())?;

        // todo save effects.
        Ok(())
    }

    fn clear(&mut self) {
        self.vm = MoveVM::new();
    }
}

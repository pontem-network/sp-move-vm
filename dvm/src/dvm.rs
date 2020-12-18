use crate::data::{State, Storage};
use crate::types::{Gas, ModuleTx, ScriptTx, VmResult};
use crate::{gas_schedule, Vm};
use move_core_types::gas_schedule::CostTable;
use move_vm_runtime::move_vm::MoveVM;

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

        Ok(())
    }

    fn execute_script(&self, gas: Gas, tx: ScriptTx) -> VmResult {

        Ok(())
    }

    fn clear(&mut self) {
        self.vm = MoveVM::new();
    }
}

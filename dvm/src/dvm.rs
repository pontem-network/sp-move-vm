use crate::types::{Gas, ModuleTx, ScriptTx, VmResult};
use crate::{gas_schedule, Vm};
use move_core_types::gas_schedule::CostTable;
use move_vm_runtime::move_vm::MoveVM;
use sp_io::Storage;

pub struct Dvm<S>
where
    S: Storage,
{
    vm: MoveVM,
    cost_table: CostTable,
    store: S,
}

impl<S> Dvm<S>
where
    S: Storage,
{
    pub fn new(store: S) -> Dvm<S> {
        Dvm {
            vm: MoveVM::new(),
            cost_table: gas_schedule::cost_table(),
            store,
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

    fn clear(&mut self) {}
}

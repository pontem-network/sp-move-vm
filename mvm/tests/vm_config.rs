extern crate alloc;
mod common;

use common::StorageMock;
use mvm::gas_schedule::cost_table;
use mvm::vm_config::loader::{load_vm_config, store_vm_config};
use mvm::vm_config::VMConfig;

#[test]
fn load_store_test() {
    let mut cost_table = cost_table();
    cost_table.instruction_table.remove(0);

    let vm_config = VMConfig {
        gas_schedule: cost_table,
    };
    let mock = StorageMock::new();
    store_vm_config(&mock, &vm_config);

    let loaded_vm_config = load_vm_config(&mock).unwrap();

    assert_eq!(vm_config, loaded_vm_config);
}

#[test]
fn load_from_empty_store_test() {
    let loaded_vm_config = load_vm_config(&StorageMock::new()).unwrap();
    assert_eq!(VMConfig::default(), loaded_vm_config);
}

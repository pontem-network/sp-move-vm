#![allow(dead_code)]

use diem_types::account_config::CORE_CODE_ADDRESS;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::ModuleId;
use move_vm_runtime::data_cache::RemoteCache;
use mvm::genesis::{init_storage, GenesisConfig};
use mvm::io::state::State;
use mvm::mvm::Mvm;

use crate::common::mock::{BankMock, EventHandlerMock, StorageMock};

pub mod assets;
pub mod mock;

pub fn vm() -> (
    Mvm<StorageMock, EventHandlerMock, BankMock>,
    StorageMock,
    EventHandlerMock,
    BankMock,
) {
    let store = StorageMock::new();
    let event = EventHandlerMock::default();
    let bank = BankMock::default();
    let genesis_config = GenesisConfig::default();
    let cost_table = genesis_config.cost_table.clone();

    init_storage(store.clone(), genesis_config).unwrap();

    let vm = Mvm::new(store.clone(), event.clone(), bank.clone(), cost_table).unwrap();
    (vm, store, event, bank)
}

pub fn contains_core_module(state: &State<StorageMock>, name: &str) {
    if state
        .get_module(&ModuleId::new(
            CORE_CODE_ADDRESS,
            Identifier::new(name).unwrap(),
        ))
        .unwrap()
        .is_none()
    {
        panic!("Module {} not found", name);
    }
}

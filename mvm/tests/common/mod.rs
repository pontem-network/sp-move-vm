#![allow(dead_code)]

use crate::common::mock::{BankMock, EventHandlerMock, OracleMock, StorageMock};
use mvm::mvm::Mvm;

pub mod assets;
pub mod mock;

pub fn vm() -> (
    Mvm<StorageMock, EventHandlerMock, OracleMock, BankMock>,
    StorageMock,
    EventHandlerMock,
    OracleMock,
    BankMock,
) {
    let store = StorageMock::new();
    let event = EventHandlerMock::default();
    let oracle = OracleMock::default();
    let bank = BankMock::default();
    let vm = Mvm::new(store.clone(), event.clone(), oracle.clone(), bank.clone()).unwrap();
    (vm, store, event, oracle, bank)
}

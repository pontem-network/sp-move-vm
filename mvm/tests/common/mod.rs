#![allow(dead_code)]

use crate::common::mock::{BankMock, EventHandlerMock, StorageMock};
use mvm::mvm::Mvm;

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
    let vm = Mvm::new(store.clone(), event.clone(), bank.clone()).unwrap();
    (vm, store, event, bank)
}

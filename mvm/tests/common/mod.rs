#![allow(dead_code)]

use crate::common::mock::{EventHandlerMock, OracleMock, StorageMock};
use mvm::mvm::Mvm;

pub mod assets;
pub mod mock;

pub fn vm() -> (
    Mvm<StorageMock, EventHandlerMock, OracleMock>,
    StorageMock,
    EventHandlerMock,
    OracleMock,
) {
    let store = StorageMock::new();
    let event = EventHandlerMock::default();
    let oracle = OracleMock::default();
    let vm = Mvm::new(store.clone(), event.clone(), oracle.clone()).unwrap();
    (vm, store, event, oracle)
}

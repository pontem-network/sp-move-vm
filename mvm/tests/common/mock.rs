use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use move_core_types::language_storage::{ModuleId, TypeTag};
use move_core_types::vm_status::StatusCode;
use mvm::data::{EventHandler, ExecutionContext, Oracle, Storage};
use mvm::mvm::Mvm;
use mvm::types::{ModuleTx, ScriptTx};
use mvm::Vm;

use crate::common::assets::gas;
use move_core_types::account_address::AccountAddress;

#[derive(Clone)]
pub struct StorageMock {
    pub data: Rc<RefCell<HashMap<Vec<u8>, Vec<u8>>>>,
}

impl StorageMock {
    pub fn new() -> StorageMock {
        StorageMock {
            data: Rc::new(RefCell::new(Default::default())),
        }
    }
}

impl Default for StorageMock {
    fn default() -> Self {
        StorageMock::new()
    }
}

impl Storage for StorageMock {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let data = self.data.borrow();
        data.get(key).map(|blob| blob.to_owned())
    }

    fn insert(&self, key: &[u8], value: &[u8]) {
        let mut data = self.data.borrow_mut();
        data.insert(key.to_owned(), value.to_owned());
    }

    fn remove(&self, key: &[u8]) {
        let mut data = self.data.borrow_mut();
        data.remove(key);
    }
}

#[derive(Clone, Default)]
pub struct EventHandlerMock {
    pub data: Rc<RefCell<Vec<(AccountAddress, TypeTag, Vec<u8>, Option<ModuleId>)>>>,
}

impl EventHandler for EventHandlerMock {
    fn on_event(
        &self,
        address: AccountAddress,
        ty_tag: TypeTag,
        message: Vec<u8>,
        caller: Option<ModuleId>,
    ) {
        let mut data = self.data.borrow_mut();
        data.push((address, ty_tag, message, caller));
    }
}

#[derive(Clone, Default)]
pub struct OracleMock {
    price_map: Rc<RefCell<HashMap<String, u128>>>,
}

#[allow(dead_code)]
impl OracleMock {
    pub fn set_price(&self, ticker: &str, price: u128) {
        self.price_map.borrow_mut().insert(ticker.to_owned(), price);
    }

    pub fn remove_price(&self, ticker: &str) {
        self.price_map.borrow_mut().remove(ticker);
    }
}

impl Oracle for OracleMock {
    fn get_price(&self, ticker: &str) -> Option<u128> {
        self.price_map.borrow().get(ticker).cloned()
    }
}

pub trait Utils {
    fn pub_mod(&self, module: ModuleTx);
    fn exec(&self, script: ScriptTx) {
        self.exec_with_context(ExecutionContext::new(100, 100), script)
    }
    fn exec_with_context(&self, context: ExecutionContext, script: ScriptTx);
}

impl<S, E, O> Utils for Mvm<S, E, O>
where
    S: Storage,
    E: EventHandler,
    O: Oracle,
{
    fn pub_mod(&self, module: ModuleTx) {
        assert_eq!(
            StatusCode::EXECUTED,
            self.publish_module(gas(), module).status_code
        );
    }

    fn exec_with_context(&self, context: ExecutionContext, script: ScriptTx) {
        assert_eq!(
            StatusCode::EXECUTED,
            self.execute_script(gas(), context, script,).status_code
        );
    }
}

pub fn addr(address: &str) -> AccountAddress {
    AccountAddress::from_hex_literal(address).unwrap()
}

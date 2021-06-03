use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::{ModuleId, TypeTag, CORE_CODE_ADDRESS};
use move_core_types::vm_status::StatusCode;
use mvm::data::{ExecutionContext};
use mvm::mvm::Mvm;
use mvm::types::{ModuleTx, ScriptTx, ModulePackage};
use mvm::Vm;

use crate::common::assets::{gas, stdlib_package};
use mvm::io::traits::{EventHandler, Balance, BalanceAccess, Storage};
use move_core_types::effects::Event;

#[derive(Clone, Debug)]
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
    pub data: Rc<RefCell<Vec<Event>>>,
}

impl EventHandlerMock {
    pub fn pop(&self) -> Option<Event> {
        self.data.borrow_mut().pop()
    }
}

impl EventHandler for EventHandlerMock {
    fn on_event(
        &self,
        guid: Vec<u8>,
        seq_num: u64,
        ty_tag: TypeTag,
        message: Vec<u8>,
    ) {
        let mut data = self.data.borrow_mut();
        data.push((guid, seq_num, ty_tag, message));
    }
}

#[derive(Clone, Debug, Default)]
pub struct BankMock {
    balances: Rc<RefCell<HashMap<AccountAddress, HashMap<String, Balance>>>>,
}

impl BankMock {
    pub fn set_balance(&self, address: &AccountAddress, ticker: &str, amount: Balance) {
        let mut acc_map = self.balances.borrow_mut();
        let acc = acc_map.entry(*address).or_insert_with(HashMap::new);
        *acc.entry(ticker.to_owned()).or_insert(amount) = amount;
    }
}

impl BalanceAccess for BankMock {
    fn get_balance(&self, address: &AccountAddress, ticker: &str) -> Option<Balance> {
        self.balances
            .borrow()
            .get(address)
            .and_then(|acc| acc.get(ticker).cloned())
    }

    fn deposit(&self, address: &AccountAddress, ticker: &str, amount: Balance) {
        let mut acc_map = self.balances.borrow_mut();
        let acc = acc_map.entry(*address).or_insert_with(HashMap::new);
        let val = acc.entry(ticker.to_owned()).or_insert(0);
        if *val < amount {
            panic!(
                "Not enough currency in the account [{}::{}] You need {} units in stock {}",
                address, ticker, amount, val
            );
        }
        *val -= amount;
    }

    fn withdraw(&self, address: &AccountAddress, ticker: &str, amount: Balance) {
        let mut acc_map = self.balances.borrow_mut();
        let acc = acc_map.entry(*address).or_insert_with(HashMap::new);
        let val = acc.entry(ticker.to_owned()).or_insert(0);
        *val += amount;
    }
}

pub trait Utils {
    fn pub_stdlib(&self);
    fn pub_mod(&self, module: ModuleTx);
    fn exec(&self, script: ScriptTx) {
        self.exec_with_context(ExecutionContext::new(100, 100), script)
    }
    fn exec_with_context(&self, context: ExecutionContext, script: ScriptTx);
}

impl<S, E, B> Utils for Mvm<S, E, B>
where
    S: Storage,
    E: EventHandler,
    B: BalanceAccess,
{
    fn pub_stdlib(&self) {
        let pac = stdlib_package().into_tx(CORE_CODE_ADDRESS);
        let res = self.publish_module_package(gas(), pac, false);
        if res.status_code != StatusCode::EXECUTED {
            panic!("Transaction failed: {:?}", res);
        }
    }

    fn pub_mod(&self, module: ModuleTx) {
        let res = self.publish_module(gas(), module, false);
        if res.status_code != StatusCode::EXECUTED {
            panic!("Transaction failed: {:?}", res);
        }
    }

    fn exec_with_context(&self, context: ExecutionContext, script: ScriptTx) {
        let res = self.execute_script(gas(), context, script, false);
        if res.status_code != StatusCode::EXECUTED {
            panic!("Transaction failed: {:?}", res);
        }
    }
}

pub fn addr(address: &str) -> AccountAddress {
    AccountAddress::from_hex_literal(address).unwrap()
}

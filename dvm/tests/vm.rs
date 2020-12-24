#[macro_use]
extern crate alloc;

use serde::Deserialize;

use alloc::rc::Rc;
use core::cell::RefCell;
use dvm::data::{State, Storage};
use dvm::dvm::Dvm;
use dvm::types::{Gas, ModuleTx, ScriptTx};
use dvm::Vm;
use hashbrown::HashMap;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{ModuleId, StructTag, CORE_CODE_ADDRESS};
use move_vm_runtime::data_cache::RemoteCache;
use move_vm_types::values::Value;

#[derive(Clone)]
pub struct Mock {
    pub data: Rc<RefCell<HashMap<Vec<u8>, Vec<u8>>>>,
}

impl Mock {
    pub fn new() -> Mock {
        Mock {
            data: Rc::new(RefCell::new(Default::default())),
        }
    }
}

impl Storage for Mock {
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

fn gas() -> Gas {
    Gas::new(10_000, 1).unwrap()
}

fn store_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("assets/target/modules/0_Store.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

fn script(args: u64) -> ScriptTx {
    ScriptTx::new(
        include_bytes!("assets/target/scripts/0_main.mv").to_vec(),
        vec![Value::u64(args)],
        vec![],
        vec![CORE_CODE_ADDRESS],
    )
    .unwrap()
}

#[derive(Deserialize)]
struct StoreU64 {
    pub val: u64,
}

#[test]
fn test_public_module() {
    let mock = Mock::new();
    let vm = Dvm::new(mock.clone());
    let state = State::new(mock.clone());
    vm.publish_module(gas(), store_module()).unwrap();
    let store_module_id = ModuleId::new(CORE_CODE_ADDRESS, Identifier::new("Store").unwrap());
    assert_eq!(
        state.get_module(&store_module_id).unwrap().unwrap(),
        include_bytes!("assets/target/modules/0_Store.mv").to_vec()
    );
}

#[test]
fn test_execute_script() {
    let test_value = 13;
    let mock = Mock::new();
    let vm = Dvm::new(mock.clone());
    let state = State::new(mock.clone());
    vm.publish_module(gas(), store_module()).unwrap();
    vm.execute_script(gas(), script(test_value)).unwrap();

    let tag = StructTag {
        address: CORE_CODE_ADDRESS,
        module: Identifier::new("Store").unwrap(),
        name: Identifier::new("U64").unwrap(),
        type_params: vec![],
    };
    let blob = state
        .get_resource(&CORE_CODE_ADDRESS, &tag)
        .unwrap()
        .unwrap();
    let store: StoreU64 = lcs::from_bytes(&blob).unwrap();
    assert_eq!(test_value, store.val);
}

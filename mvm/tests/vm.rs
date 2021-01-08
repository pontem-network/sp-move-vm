#[macro_use]
extern crate alloc;

use serde::Deserialize;

use alloc::rc::Rc;
use core::cell::RefCell;
use hashbrown::HashMap;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{ModuleId, StructTag, TypeTag, CORE_CODE_ADDRESS};
use move_core_types::vm_status::StatusCode;
use move_vm_runtime::data_cache::RemoteCache;
use mvm::data::{EventHandler, State, Storage};
use mvm::mvm::Mvm;
use mvm::types::{Gas, ModuleTx, ScriptArg, ScriptTx};
use mvm::Vm;

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
    pub data: Rc<RefCell<Vec<(Vec<u8>, u64, TypeTag, Vec<u8>)>>>,
}

impl EventHandler for EventHandlerMock {
    fn on_event(&self, guid: Vec<u8>, seq_num: u64, ty_tag: TypeTag, message: Vec<u8>) {
        let mut data = self.data.borrow_mut();
        data.push((guid, seq_num, ty_tag, message));
    }
}

fn gas() -> Gas {
    Gas::new(10_000, 1).unwrap()
}

fn store_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("assets/target/modules/1_Store.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

fn event_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("assets/target/modules/0_Event.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

fn vector_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("assets/target/modules/2_Vector.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

fn store_script(args: u64) -> ScriptTx {
    ScriptTx::new(
        include_bytes!("assets/target/scripts/1_store_u64.mv").to_vec(),
        vec![ScriptArg::U64(args)],
        vec![],
        vec![CORE_CODE_ADDRESS],
    )
    .unwrap()
}

fn emit_event_script(args: u64) -> ScriptTx {
    ScriptTx::new(
        include_bytes!("assets/target/scripts/0_emit_event.mv").to_vec(),
        vec![ScriptArg::U64(args)],
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
    let mock = StorageMock::new();
    let vm = Mvm::new(mock.clone(), EventHandlerMock::default());
    let state = State::new(mock);
    assert_eq!(
        StatusCode::EXECUTED,
        vm.publish_module(gas(), store_module()).status_code
    );
    let store_module_id = ModuleId::new(CORE_CODE_ADDRESS, Identifier::new("Store").unwrap());
    assert_eq!(
        state.get_module(&store_module_id).unwrap().unwrap(),
        include_bytes!("assets/target/modules/1_Store.mv").to_vec()
    );
}

#[test]
fn test_execute_script() {
    let test_value = 13;
    let mock = StorageMock::new();
    let event_handler = EventHandlerMock::default();
    let vm = Mvm::new(mock.clone(), event_handler);
    let state = State::new(mock);
    assert_eq!(
        StatusCode::EXECUTED,
        vm.publish_module(gas(), store_module()).status_code
    );
    assert_eq!(
        StatusCode::EXECUTED,
        vm.execute_script(gas(), store_script(test_value))
            .status_code
    );

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
    let store: StoreU64 = bcs::from_bytes(&blob).unwrap();
    assert_eq!(test_value, store.val);
}

#[test]
fn test_store_event() {
    let test_value = 13;
    let mock = StorageMock::new();
    let event_handler = EventHandlerMock::default();
    let vm = Mvm::new(mock, event_handler.clone());
    assert_eq!(
        StatusCode::EXECUTED,
        vm.publish_module(gas(), vector_module()).status_code
    );
    assert_eq!(
        StatusCode::EXECUTED,
        vm.publish_module(gas(), event_module()).status_code
    );
    assert_eq!(
        StatusCode::EXECUTED,
        vm.execute_script(gas(), emit_event_script(test_value))
            .status_code
    );

    let (guid, seq, tag, msg) = event_handler.data.borrow_mut().remove(0);
    assert_eq!(guid, b"GUID".to_vec());
    assert_eq!(seq, 1);
    assert_eq!(test_value, bcs::from_bytes::<StoreU64>(&msg).unwrap().val);
    assert_eq!(
        TypeTag::Struct(StructTag {
            address: CORE_CODE_ADDRESS,
            module: Identifier::new("Event").unwrap(),
            name: Identifier::new("U64").unwrap(),
            type_params: vec![],
        }),
        tag
    );
}

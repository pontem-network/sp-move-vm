#[macro_use]
extern crate alloc;

use common::mock::Utils;
use common::{assets::*, mock::*, vm};
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{ModuleId, StructTag, TypeTag, CORE_CODE_ADDRESS};
use move_vm_runtime::data_cache::RemoteCache;
use mvm::data::{ExecutionContext, State};

mod common;

#[test]
fn test_public_module() {
    let (vm, store, _, oracle) = vm();
    let state = State::new(store, oracle);

    vm.pub_mod(store_module());

    let store_module_id = ModuleId::new(CORE_CODE_ADDRESS, Identifier::new("Store").unwrap());

    assert_eq!(
        &state.get_module(&store_module_id).unwrap().unwrap(),
        store_module().code()
    );
}

#[test]
fn test_execute_script() {
    let test_value = 13;

    let (vm, store, _, oracle) = vm();
    let state = State::new(store, oracle);

    vm.pub_mod(store_module());

    vm.exec(store_u64_script(addr("0x1"), test_value));

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

    let (vm, _, event, _) = vm();

    vm.pub_mod(event_module());
    vm.pub_mod(event_proxy_module());

    vm.exec(emit_event_script(addr("0x1"), test_value));

    let (address, tag, msg, caller) = event.data.borrow_mut().remove(0);
    assert_eq!(address, addr("0x1"));
    assert_eq!(test_value, bcs::from_bytes::<StoreU64>(&msg).unwrap().val);
    assert_eq!(
        caller.unwrap(),
        ModuleId::new(addr("0x1"), Identifier::new("EventProxy").unwrap())
    );
    assert_eq!(
        TypeTag::Struct(StructTag {
            address: CORE_CODE_ADDRESS,
            module: Identifier::new("EventProxy").unwrap(),
            name: Identifier::new("U64").unwrap(),
            type_params: vec![],
        }),
        tag
    );

    let (address, tag, msg, caller) = event.data.borrow_mut().remove(0);
    assert_eq!(address, addr("0x1"));
    assert_eq!(test_value, bcs::from_bytes::<StoreU64>(&msg).unwrap().val);
    assert_eq!(caller, None);
    assert_eq!(
        TypeTag::Struct(StructTag {
            address: CORE_CODE_ADDRESS,
            module: Identifier::new("EventProxy").unwrap(),
            name: Identifier::new("U64").unwrap(),
            type_params: vec![],
        }),
        tag
    );
}

#[test]
fn test_load_system_resources() {
    let (vm, store, _, oracle) = vm();
    let state = State::new(store, oracle);

    vm.pub_mod(store_module());
    vm.pub_mod(time_module());
    vm.pub_mod(block_module());

    let block = 1000;
    let timestamp = 10;

    vm.exec_with_context(
        ExecutionContext::new(timestamp, block),
        store_sys_resources_script(addr("0x1"), addr("0x2")),
    );

    let tag = StructTag {
        address: CORE_CODE_ADDRESS,
        module: Identifier::new("Store").unwrap(),
        name: Identifier::new("U64").unwrap(),
        type_params: vec![],
    };
    let blob = state.get_resource(&addr("0x1"), &tag).unwrap().unwrap();
    let store: StoreU64 = bcs::from_bytes(&blob).unwrap();
    assert_eq!(store.val, block);

    let blob = state.get_resource(&addr("0x2"), &tag).unwrap().unwrap();
    let store: StoreU64 = bcs::from_bytes(&blob).unwrap();
    assert_eq!(store.val, timestamp);
}

#[test]
fn test_oracle() {
    let (vm, store, _, oracle) = vm();
    let state = State::new(store, oracle.clone());

    vm.pub_mod(store_module());
    vm.pub_mod(coins_module());
    vm.pub_mod(pont_module());

    let eth_btc = 13;
    let btc_pont = 234646734213;
    oracle.set_price("ETH_BTC", eth_btc);
    oracle.set_price("BTC_PONT", btc_pont);

    vm.exec(get_price_script(addr("0x1"), addr("0x2")));

    let tag = StructTag {
        address: CORE_CODE_ADDRESS,
        module: Identifier::new("Store").unwrap(),
        name: Identifier::new("U128").unwrap(),
        type_params: vec![],
    };
    let blob = state.get_resource(&addr("0x1"), &tag).unwrap().unwrap();
    let store: StoreU128 = bcs::from_bytes(&blob).unwrap();
    assert_eq!(store.val, eth_btc);

    let blob = state.get_resource(&addr("0x2"), &tag).unwrap().unwrap();
    let store: StoreU128 = bcs::from_bytes(&blob).unwrap();
    assert_eq!(store.val, btc_pont);
}

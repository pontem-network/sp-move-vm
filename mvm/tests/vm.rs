#[macro_use]
extern crate alloc;

use common::mock::Utils;
use common::{assets::*, mock::*, vm};
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{ModuleId, StructTag, TypeTag, CORE_CODE_ADDRESS};
use move_core_types::vm_status::StatusCode;
use move_vm_runtime::data_cache::RemoteCache;
use mvm::data::{ExecutionContext, State};
use mvm::types::Gas;
use mvm::Vm;
use mvm::io::traits::BalanceAccess;

mod common;

#[test]
fn test_public_module() {
    let (vm, store, _, _) = vm();
    let state = State::new(store);

    vm.pub_mod(store_module());

    let store_module_id = ModuleId::new(CORE_CODE_ADDRESS, Identifier::new("Store").unwrap());

    assert_eq!(
        &state.get_module(&store_module_id).unwrap().unwrap(),
        store_module().code()
    );
}

#[test]
fn test_public_module_without_gas() {
    let (vm, _, _, _) = vm();
    let gas = Gas::new(1, 1).unwrap();
    let res = vm.publish_module(gas, store_module(), false);
    assert_eq!(res.status_code, StatusCode::OUT_OF_GAS);
}

#[test]
fn test_execute_script() {
    let test_value = 13;

    let (vm, store, _, _) = vm();
    let state = State::new(store);

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
    vm.pub_stdlib();
    vm.pub_mod(event_proxy_module());

    vm.exec(emit_event_script(addr("0x1"), test_value));

    let (guid, seq, tag, msg) = event.data.borrow_mut().remove(0);
    assert_eq!(guid, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    assert_eq!(seq, 0);
    assert_eq!(test_value, bcs::from_bytes::<StoreU64>(&msg).unwrap().val);
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
    let (vm, store, _, _) = vm();
    let state = State::new(store);

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

// #[test]
// fn test_error_event() {
//     let (vm, _, events, _) = vm();
//     vm.pub_mod(abort_module());
//     let sender = AccountAddress::random();
//     vm.execute_script(
//         gas(),
//         ExecutionContext::new(0, 0),
//         error_script(sender),
//         false,
//     );
//     let event = events.pop().unwrap();
//     assert_eq!(sender, event.0);
//     assert_eq!(
//         Some(ModuleId::new(
//             CORE_CODE_ADDRESS,
//             Identifier::new("Abort").unwrap()
//         )),
//         event.3
//     );
// }

#[test]
fn test_publish_pac() {
    let (vm, state, _, _) = vm();
    let state = State::new(state);

    let pac = stdlib_package().into_tx(CORE_CODE_ADDRESS);

    let res = vm.publish_module_package(gas(), pac, false);
    if res.status_code != StatusCode::EXECUTED {
        panic!("Transaction failed: {:?}", res);
    }

    fn contains_module(state: &State<StorageMock>, name: &str) {
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

    contains_module(&state, "Block");
    contains_module(&state, "Coins");
    contains_module(&state, "PONT");
    contains_module(&state, "Time");
    contains_module(&state, "Event");
    contains_module(&state, "Pontem");
}

#[test]
fn test_invalid_pac() {
    let (vm, _, _, _) = vm();
    let pac = invalid_package().into_tx(CORE_CODE_ADDRESS);
    let res = vm.publish_module_package(gas(), pac, false);
    assert_eq!(res.status_code, StatusCode::LINKER_ERROR);
}

// #[test]
// fn test_balance() {
//     let (vm, _, _,  bank) = vm();
//     vm.pub_mod(coins_module());
//     vm.pub_mod(pont_module());
//     vm.pub_mod(event_module());
//     vm.pub_mod(pontem_module());
//
//     let addr_1 = AccountAddress::random();
//     let addr_2 = AccountAddress::random();
//     let init_usdt = 1024;
//     let init_pont = 64;
//     let init_btc = 13;
//
//     bank.set_balance(&addr_1, "USDT", init_usdt);
//     bank.set_balance(&addr_1, "PONT", init_pont);
//     bank.set_balance(&addr_1, "BTC", init_btc);
//
//     vm.exec(test_balance_script(
//         addr_1, addr_2, init_usdt, init_pont, init_btc,
//     ));
//
//     assert_eq!(bank.get_balance(&addr_1, "USDT"), Some(512));
//     assert_eq!(bank.get_balance(&addr_1, "PONT"), Some(61));
//     assert_eq!(bank.get_balance(&addr_1, "BTC"), Some(13));
//
//     assert_eq!(bank.get_balance(&addr_2, "USDT"), Some(512));
//     assert_eq!(bank.get_balance(&addr_2, "PONT"), Some(3));
//     assert_eq!(bank.get_balance(&addr_2, "BTC"), None);
// }
//
// #[test]
// fn test_transfer() {
//     let (vm, store, _, bank) = vm();
//     let state = State::new(store);
//
//     vm.pub_mod(coins_module());
//     vm.pub_mod(pont_module());
//     vm.pub_mod(event_module());
//     vm.pub_mod(pontem_module());
//
//     vm.exec(reg_coin_script(
//         TypeTag::Struct {
//             0: StructTag {
//                 address: CORE_CODE_ADDRESS,
//                 module: Identifier::new("PONT").unwrap(),
//                 name: Identifier::new("T").unwrap(),
//                 type_params: vec![],
//             },
//         },
//         "PONT",
//         2,
//     ));
//
//     let alice = AccountAddress::random();
//     let bob = AccountAddress::random();
//
//     let alice_balance = 100;
//     bank.set_balance(&alice, "PONT", alice_balance);
//
//     let send_to_bob = 4;
//
//     vm.exec(test_transfer_script(alice, bob, send_to_bob));
//
//     assert_eq!(
//         bank.get_balance(&alice, "PONT"),
//         Some(alice_balance - send_to_bob)
//     );
//
//     let bob_account = state
//         .get_resource(
//             &bob,
//             &StructTag {
//                 address: CORE_CODE_ADDRESS,
//                 module: Identifier::new("Pontem").unwrap(),
//                 name: Identifier::new("T").unwrap(),
//                 type_params: vec![TypeTag::Struct(StructTag {
//                     address: CORE_CODE_ADDRESS,
//                     module: Identifier::new("PONT").unwrap(),
//                     name: Identifier::new("T").unwrap(),
//                     type_params: vec![],
//                 })],
//             },
//         )
//         .unwrap()
//         .unwrap();
//
//     let bob_account: u128 = bcs::from_bytes(&bob_account).unwrap();
//
//     assert_eq!(bob_account, send_to_bob);
// }

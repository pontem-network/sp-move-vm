// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

pub mod account;
pub mod bcs;
pub mod event;
pub mod hash;
pub mod reflect;
pub mod signature;
pub mod signer;
pub mod u256;
pub mod vector;

#[cfg(feature = "testing")]
pub mod unit_test;

#[cfg(feature = "testing")]
pub mod debug;

use move_core_types::{account_address::AccountAddress, identifier::Identifier};
use move_vm_runtime::native_functions::{NativeFunction, NativeFunctionTable};

pub fn all_natives(move_std_addr: AccountAddress) -> NativeFunctionTable {
    const NATIVES: &[(&str, &str, NativeFunction)] = &[
        ("BCS", "to_bytes", bcs::native_to_bytes),
        ("Event", "write_to_event_store", event::write_to_event_store),
        ("Hash", "sha2_256", hash::native_sha2_256),
        ("Hash", "sha3_256", hash::native_sha3_256),
        ("Signer", "borrow_address", signer::native_borrow_address),
        ("Vector", "length", vector::native_length),
        ("Vector", "empty", vector::native_empty),
        ("Vector", "borrow", vector::native_borrow),
        ("Vector", "borrow_mut", vector::native_borrow),
        ("Vector", "push_back", vector::native_push_back),
        ("Vector", "pop_back", vector::native_pop),
        ("Vector", "destroy_empty", vector::native_destroy_empty),
        ("Vector", "swap", vector::native_swap),
        #[cfg(feature = "testing")]
        ("Debug", "print", debug::native_print),
        #[cfg(feature = "testing")]
        (
            "Debug",
            "print_stack_trace",
            debug::native_print_stack_trace,
        ),
        #[cfg(feature = "testing")]
        (
            "UnitTest",
            "create_signers_for_testing",
            unit_test::native_create_signers_for_testing,
        ),
        ("U256", "from_u8", u256::from_u8),
        ("U256", "from_u64", u256::from_u64),
        ("U256", "from_u128", u256::from_u128),
        ("U256", "as_u8", u256::as_u8),
        ("U256", "as_u64", u256::as_u64),
        ("U256", "as_u128", u256::as_u128),
        ("U256", "add", u256::add),
        ("U256", "sub", u256::sub),
        ("U256", "mul", u256::mul),
        ("U256", "div", u256::div),
        (
            "DiemAccount",
            "create_signer",
            account::native_create_signer,
        ),
        (
            "DiemAccount",
            "destroy_signer",
            account::native_destroy_signer,
        ),
        (
            "Signature",
            "ed25519_validate_pubkey",
            signature::native_ed25519_publickey_validation,
        ),
        (
            "Signature",
            "ed25519_verify",
            signature::native_ed25519_signature_verification,
        ),
        ("Reflect", "type_of", reflect::type_of),
    ];
    NATIVES
        .iter()
        .cloned()
        .map(|(module_name, func_name, func)| {
            (
                move_std_addr,
                Identifier::new(module_name).unwrap(),
                Identifier::new(func_name).unwrap(),
                func,
            )
        })
        .collect()
}

// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::VecDeque;
use alloc::vec::Vec;
use move_binary_format::errors::PartialVMResult;
use move_core_types::account_address::AccountAddress;
use move_vm_types::{
    gas_schedule::NativeCostIndex,
    loaded_data::runtime_types::Type,
    natives::function::{native_gas, NativeContext, NativeResult},
    values::Value,
};
use smallvec::smallvec;

pub fn native_create_signer(
    context: &mut impl NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let address = pop_arg!(arguments, AccountAddress);
    let cost = native_gas(context.cost_table(), NativeCostIndex::CREATE_SIGNER, 0);
    Ok(NativeResult::ok(cost, smallvec![Value::signer(address)]))
}

/// NOTE: this function will be deprecated after the Diem v3 release, but must
/// remain for replaying old transactions
pub fn native_destroy_signer(
    context: &mut impl NativeContext,
    ty_args: Vec<Type>,
    arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let cost = native_gas(context.cost_table(), NativeCostIndex::DESTROY_SIGNER, 0);
    Ok(NativeResult::ok(cost, smallvec![]))
}

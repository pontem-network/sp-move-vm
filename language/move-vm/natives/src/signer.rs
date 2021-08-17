// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::VecDeque;
use alloc::vec::Vec;
use move_binary_format::errors::PartialVMResult;
use move_vm_types::{
    gas_schedule::NativeCostIndex,
    loaded_data::runtime_types::Type,
    natives::function::{native_gas, NativeContext, NativeResult},
    values::{values_impl::SignerRef, Value},
};
use smallvec::smallvec;

pub fn native_borrow_address(
    context: &impl NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(_ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let signer_reference = pop_arg!(arguments, SignerRef);
    let cost = native_gas(context.cost_table(), NativeCostIndex::SIGNER_BORROW, 1);

    Ok(NativeResult::ok(
        cost,
        smallvec![signer_reference.borrow_signer()?],
    ))
}

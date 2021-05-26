// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::types::account_address;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use move_core_types::gas_schedule::GasAlgebra;
use move_vm_types::values::SignerRef;
use move_vm_types::{
    gas_schedule::NativeCostIndex,
    loaded_data::runtime_types::Type,
    natives::function::{native_gas, NativeContext, NativeResult},
    values::Value,
};
use smallvec::smallvec;
use vm::errors::PartialVMResult;

pub fn native_emit_event(
    context: &mut impl NativeContext,
    mut ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.len() == 1);
    debug_assert!(arguments.len() == 2);

    let ty = ty_args.pop().unwrap();
    let msg = arguments.pop_back().unwrap();
    let address = account_address(&pop_arg!(arguments, SignerRef).borrow_signer()?.0)?;

    let cost = native_gas(
        context.cost_table(),
        NativeCostIndex::EMIT_EVENT,
        msg.size().get() as usize,
    );

    if !context.save_event(address, ty, msg, context.caller().cloned())? {
        return Ok(NativeResult::err(cost, 0));
    }

    Ok(NativeResult::ok(cost, smallvec![]))
}

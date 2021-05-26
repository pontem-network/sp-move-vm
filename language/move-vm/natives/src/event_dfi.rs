// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use alloc::borrow::ToOwned;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use move_core_types::account_address::AccountAddress;
use move_core_types::gas_schedule::GasAlgebra;
use move_core_types::vm_status::StatusCode;
use move_vm_types::values::{Container, ContainerRef, SignerRef, ValueImpl};
use move_vm_types::{
    gas_schedule::NativeCostIndex,
    loaded_data::runtime_types::Type,
    natives::function::{native_gas, NativeContext, NativeResult},
    values::Value,
};
use smallvec::smallvec;
use vm::errors::{PartialVMError, PartialVMResult};

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

    let save_res = context.save_event(address, ty, msg, context.caller().cloned())?;

    if !save_res {
        return Ok(NativeResult::err(cost, 0));
    }

    Ok(NativeResult::ok(cost, smallvec![]))
}

fn account_address(value: &ValueImpl) -> PartialVMResult<AccountAddress> {
    fn find_address(container: &Container) -> PartialVMResult<AccountAddress> {
        match container {
            Container::Locals(values) | Container::Struct(values) => {
                let values = values.borrow();
                if values.len() != 1 {
                    Err(PartialVMError::new(StatusCode::INTERNAL_TYPE_ERROR)
                        .with_message("Invalid signer value.".to_owned()))
                } else {
                    account_address(&values[0])
                }
            }
            Container::VecAddress(_)
            | Container::VecU8(_)
            | Container::VecU64(_)
            | Container::Vec(_)
            | Container::VecU128(_)
            | Container::VecBool(_) => Err(PartialVMError::new(StatusCode::INTERNAL_TYPE_ERROR)
                .with_message("Invalid signer value.".to_owned())),
        }
    }

    match value {
        ValueImpl::U8(_)
        | ValueImpl::U64(_)
        | ValueImpl::U128(_)
        | ValueImpl::Bool(_)
        | ValueImpl::Invalid => Err(PartialVMError::new(StatusCode::INTERNAL_TYPE_ERROR)
            .with_message("Invalid signer value.".to_owned())),
        ValueImpl::Address(address) => Ok(*address),
        ValueImpl::Container(container) => find_address(container),
        ValueImpl::ContainerRef(container_ref) => match container_ref {
            ContainerRef::Local(container) => find_address(container),
            ContainerRef::Global {
                status: _,
                container,
            } => find_address(container),
        },
        ValueImpl::IndexedRef(index_ref) => match &index_ref.container_ref {
            ContainerRef::Local(container) => find_address(container),
            ContainerRef::Global {
                status: _,
                container,
            } => find_address(container),
        },
    }
}

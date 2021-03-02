// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use move_core_types::gas_schedule::GasAlgebra;
use move_vm_types::{
    gas_schedule::NativeCostIndex,
    loaded_data::runtime_types::Type,
    natives::function::{native_gas, NativeContext, NativeResult},
    values::Value,
};

use alloc::borrow::ToOwned;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::convert::TryInto;
use move_core_types::account_address::AccountAddress;
use move_core_types::vm_status::StatusCode;
use move_vm_types::values::{Container, ContainerRef, SignerRef, ValueImpl};
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

    let save_res = context.save_event(
        EventKey::new_from_address(&address, 0).to_vec(),
        0,
        ty,
        msg,
        context.caller().cloned(),
    )?;

    if !save_res {
        return Ok(NativeResult::err(cost, 0));
    }

    Ok(NativeResult::ok(cost, vec![]))
}

fn account_address(value: &ValueImpl) -> PartialVMResult<AccountAddress> {
    fn find_address(container: &Container) -> PartialVMResult<AccountAddress> {
        match container {
            Container::Locals(values)
            | Container::VecR(values)
            | Container::VecC(values)
            | Container::StructR(values)
            | Container::StructC(values) => {
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

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(any(test, feature = "fuzzing"), derive(Arbitrary))]
pub struct EventKey([u8; EventKey::LENGTH]);

impl EventKey {
    /// Construct a new EventKey from a byte array slice.
    pub fn new(key: [u8; Self::LENGTH]) -> Self {
        EventKey(key)
    }

    /// The number of bytes in an EventKey.
    pub const LENGTH: usize = AccountAddress::LENGTH + 8;

    /// Get the byte representation of the event key.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Convert event key into a byte array.
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    /// Get the account address part in this event key
    pub fn get_creator_address(&self) -> AccountAddress {
        let mut arr_bytes = [0u8; AccountAddress::LENGTH];
        arr_bytes.copy_from_slice(&self.0[EventKey::LENGTH - AccountAddress::LENGTH..]);

        AccountAddress::new(arr_bytes)
    }

    /// If this is the `ith` EventKey` created by `get_creator_address()`, return `i`
    pub fn get_creation_number(&self) -> u64 {
        u64::from_le_bytes(self.0[0..8].try_into().unwrap())
    }

    /// Create a unique handle by using an AccountAddress and a counter.
    pub fn new_from_address(addr: &AccountAddress, salt: u64) -> Self {
        let mut output_bytes = [0; Self::LENGTH];
        let (lhs, rhs) = output_bytes.split_at_mut(8);
        lhs.copy_from_slice(&salt.to_le_bytes());
        rhs.copy_from_slice(addr.as_ref());
        EventKey(output_bytes)
    }
}

// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use move_core_types::gas_schedule::GasAlgebra;
use move_vm_types::{
    gas_schedule::NativeCostIndex,
    loaded_data::runtime_types::Type,
    natives::function::{native_gas, NativeContext, NativeResult},
    values::Value,
};

use crate::types::account_address;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::convert::TryInto;
use move_core_types::account_address::AccountAddress;
use move_vm_types::values::SignerRef;
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

    let save_res = context.save_event(address, ty, msg, context.caller().cloned())?;

    if !save_res {
        return Ok(NativeResult::err(cost, 0));
    }

    Ok(NativeResult::ok(cost, vec![]))
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use alloc::borrow::ToOwned;
use alloc::collections::VecDeque;
use alloc::vec::Vec;

use crate::types::account_address;
use crate::types::balance::{create_balance, destroy_balance};
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::TypeTag;
use move_core_types::vm_status::StatusCode;
use move_vm_types::natives::balance::{BalanceOperation, WalletId};
use move_vm_types::natives::function::PartialVMError;
use move_vm_types::values::SignerRef;
use move_vm_types::values::ValueImpl;
use move_vm_types::{
    gas_schedule::NativeCostIndex,
    loaded_data::runtime_types::Type,
    natives::function::{native_gas, NativeContext, NativeResult},
    values::Value,
};
use smallvec::smallvec;
use vm::errors::PartialVMResult;

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

/// deposit_from_native<Token>(address: &signer, amount: u128): Pontem::T<Token>;
pub fn native_deposit(
    context: &mut impl NativeContext,
    mut ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.len() == 1);
    debug_assert!(arguments.len() == 2);

    let amount = pop_arg!(arguments, u128);
    let address = account_address(&pop_arg!(arguments, SignerRef).borrow_signer()?.0)?;

    let wallet_id = wallet_id(context, address, ty_args.pop().unwrap())?;

    if let Some(balance) = context.get_balance(&wallet_id) {
        if balance >= amount {
            context.save_balance_operation(wallet_id, BalanceOperation::Deposit(amount));
            let cost = native_gas(context.cost_table(), NativeCostIndex::DEPOSIT, 0);
            Ok(NativeResult::ok(cost, smallvec![create_balance(amount)]))
        } else {
            Err(
                PartialVMError::new(StatusCode::ABORTED).with_message(format!(
                    "Not enough coins to deposit.({:?}), {:?}",
                    wallet_id, amount
                )),
            )
        }
    } else {
        Err(PartialVMError::new(StatusCode::RESOURCE_DOES_NOT_EXIST)
            .with_message(format!("Balance({:?}) not found.", wallet_id)))
    }
}

/// withdraw_to_native<Token>(address: &signer, balance: Pontem::T<Token>);
pub fn native_withdraw(
    context: &mut impl NativeContext,
    mut ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.len() == 1);
    debug_assert!(arguments.len() == 2);
    let balance = destroy_balance(arguments.pop_back().unwrap().0)?;
    let address = account_address(&pop_arg!(arguments, SignerRef).borrow_signer()?.0)?;

    let wallet_id = wallet_id(context, address, ty_args.pop().unwrap())?;

    context.save_balance_operation(wallet_id, BalanceOperation::Withdraw(balance));

    let cost = native_gas(context.cost_table(), NativeCostIndex::WITHDRAW, 0);
    Ok(NativeResult::ok(cost, smallvec![]))
}

/// get_native_balance<Token>(address: &signer): u128;
pub fn get_balance(
    context: &mut impl NativeContext,
    mut ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.len() == 1);
    debug_assert!(arguments.len() == 1);

    let address = account_address(&pop_arg!(arguments, SignerRef).borrow_signer()?.0)?;
    let wallet_id = wallet_id(context, address, ty_args.pop().unwrap())?;

    if let Some(balance) = context.get_balance(&wallet_id) {
        let cost = native_gas(context.cost_table(), NativeCostIndex::GET_BALANCE, 0);
        Ok(NativeResult::ok(
            cost,
            smallvec![Value(ValueImpl::U128(balance))],
        ))
    } else {
        Err(PartialVMError::new(StatusCode::RESOURCE_DOES_NOT_EXIST)
            .with_message(format!("Balance({:?}) not found.", wallet_id)))
    }
}

fn wallet_id(
    ctx: &impl NativeContext,
    address: AccountAddress,
    tp: Type,
) -> PartialVMResult<WalletId> {
    match ctx.type_to_type_tag(&tp)? {
        TypeTag::Struct(tag) => Ok(WalletId::new(address, tag)),
        _ => Err(PartialVMError::new(StatusCode::CALL_TYPE_MISMATCH_ERROR)
            .with_message("Invalid type parameter. Structure is expected.".to_owned())),
    }
}

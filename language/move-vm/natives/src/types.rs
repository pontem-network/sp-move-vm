use alloc::borrow::ToOwned;
use move_core_types::account_address::AccountAddress;
use move_core_types::vm_status::StatusCode;
use move_vm_types::natives::function::PartialVMError;
use move_vm_types::values::{Container, ContainerRef, ValueImpl};
use vm::errors::PartialVMResult;

pub fn account_address(value: &ValueImpl) -> PartialVMResult<AccountAddress> {
    fn find_address(container: &Container) -> PartialVMResult<AccountAddress> {
        match container {
            Container::Locals(values) | Container::Vec(values) | Container::Struct(values) => {
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

pub mod balance {
    use core::cell::RefCell;

    use alloc::rc::Rc;
    use move_core_types::vm_status::StatusCode;
    use move_vm_types::natives::balance::Balance;
    use move_vm_types::natives::function::PartialVMError;
    use move_vm_types::values::Container::Struct;
    use move_vm_types::values::{Value, ValueImpl};
    use vm::errors::PartialVMResult;

    pub fn create_balance(amount: Balance) -> Value {
        Value(ValueImpl::Container(Struct(Rc::new(RefCell::new(vec![
            ValueImpl::U128(amount),
        ])))))
    }

    pub fn destroy_balance(val: ValueImpl) -> PartialVMResult<Balance> {
        match val {
            ValueImpl::Container(Struct(val)) => {
                let fields = val.borrow();
                if fields.len() == 1 {
                    match fields[0] {
                        ValueImpl::U128(balance) => Ok(balance),
                        _ => Err(PartialVMError::new(
                            StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR,
                        )),
                    }
                } else {
                    Err(PartialVMError::new(
                        StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR,
                    ))
                }
            }
            _ => Err(PartialVMError::new(
                StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR,
            )),
        }
    }
}

use alloc::vec::Vec;
use crate::io::traits::{BalanceAccess, Storage};
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::{ModuleId, StructTag, CORE_CODE_ADDRESS};
use move_vm_runtime::data_cache::RemoteCache;
use vm::errors::{PartialVMResult, VMResult};
use crate::io::key::AccessKey;

pub trait WriteEffects {
    fn delete(&self, path: AccessKey);
    fn insert(&self, path: AccessKey, blob: Vec<u8>);
}

pub struct State<S> {
    store: S,
}

impl<S> State<S>
where
    S: Storage,
{
    pub fn new(store: S) -> State<S> {
        State { store }
    }
}

impl<S> RemoteCache for State<S>
where
    S: Storage,
{
    fn get_module(&self, module_id: &ModuleId) -> VMResult<Option<Vec<u8>>> {
        Ok(self.store.get(AccessKey::from(module_id).as_ref()))
    }

    fn get_resource(
        &self,
        address: &AccountAddress,
        tag: &StructTag,
    ) -> PartialVMResult<Option<Vec<u8>>> {
        // if address == &CORE_CODE_ADDRESS {
        //     if let Some(ticker) = self.oracle.get_ticker(tag) {
        //         return Ok(self.oracle.get_price(&ticker));
        //     }
        // }

        Ok(self.store.get(AccessKey::from((address, tag)).as_ref()))
    }
}

impl<S> WriteEffects for State<S>
where
    S: Storage,
{
    fn delete(&self, key: AccessKey) {
        self.store.remove(key.as_ref());
    }

    fn insert(&self, key: AccessKey, blob: Vec<u8>) {
        self.store.insert(key.as_ref(), &blob);
    }
}

const PONT: &str = "PONT";
const COINS: &str = "Coins";

pub struct StateSession<'r, R: RemoteCache> {
    remote: &'r R,
    context: ExecutionContext,
}

impl<R> StateSession<'_, R>
where
    R: RemoteCache,
{
    pub fn new(remote: &R, context: ExecutionContext) -> StateSession<'_, R> {
        StateSession { remote, context }
    }
}

impl<R> RemoteCache for StateSession<'_, R>
where
    R: RemoteCache,
{
    fn get_module(&self, module_id: &ModuleId) -> VMResult<Option<Vec<u8>>> {
        self.remote.get_module(module_id)
    }

    fn get_resource(
        &self,
        address: &AccountAddress,
        tag: &StructTag,
    ) -> PartialVMResult<Option<Vec<u8>>> {
        if address == &CORE_CODE_ADDRESS && tag.address == CORE_CODE_ADDRESS {
            if tag.module.as_str() == "Block" && tag.name.as_str() == "BlockMetadata" {
                return Ok(Some(self.context.block_height.to_le_bytes().to_vec()));
            } else if tag.module.as_str() == "Time" && tag.name.as_str() == "CurrentTimestamp" {
                return Ok(Some(self.context.timestamp.to_le_bytes().to_vec()));
            }
        }
        self.remote.get_resource(address, tag)
    }
}

#[derive(Debug)]
pub struct ExecutionContext {
    pub timestamp: u64,
    pub block_height: u64,
}

impl ExecutionContext {
    pub fn new(timestamp: u64, block_height: u64) -> ExecutionContext {
        ExecutionContext {
            timestamp,
            block_height,
        }
    }
}

pub struct Bank<B: BalanceAccess> {
    access: B,
}

impl<B: BalanceAccess> Bank<B> {
    pub fn new(access: B) -> Bank<B> {
        Bank { access }
    }
}
//
//     pub fn deposit(&self, wallet_id: &WalletId, amount: Balance) -> Result<(), VMError> {
//         if let Some(ticker) = ticker(wallet_id) {
//             self.access.deposit(&wallet_id.address, ticker, amount);
//             Ok(())
//         } else {
//             Err(PartialVMError::new(StatusCode::INTERNAL_TYPE_ERROR).finish(Location::Undefined))
//         }
//     }
//
//     pub fn withdraw(&self, wallet_id: &WalletId, amount: Balance) -> Result<(), VMError> {
//         if let Some(ticker) = ticker(wallet_id) {
//             self.access.withdraw(&wallet_id.address, ticker, amount);
//             Ok(())
//         } else {
//             Err(PartialVMError::new(StatusCode::INTERNAL_TYPE_ERROR).finish(Location::Undefined))
//         }
//     }
// }
//
// impl<B: BalanceAccess> NativeBalance for &Bank<B> {
//     fn get_balance(&self, wallet_id: &WalletId) -> Option<Balance> {
//         if let Some(ticker) = ticker(wallet_id) {
//             self.access.get_balance(&wallet_id.address, ticker)
//         } else {
//             None
//         }
//     }
// }
//
// fn ticker(wallet_id: &WalletId) -> Option<&str> {
//     if wallet_id.tag.address == CORE_CODE_ADDRESS {
//         match wallet_id.tag.module.as_str() {
//             PONT => Some(PONT),
//             COINS => Some(wallet_id.tag.name.as_str()),
//             _ => None,
//         }
//     } else {
//         None
//     }
// }

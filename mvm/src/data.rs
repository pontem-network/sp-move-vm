use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;

use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::{
    ModuleId, ResourceKey, StructTag, TypeTag, CORE_CODE_ADDRESS,
};
use move_vm_runtime::data_cache::RemoteCache;
use move_vm_types::natives::balance::{Balance, NativeBalance, WalletId};
use vm::errors::{PartialVMResult, VMResult};

use crate::access_path::AccessPath;

pub trait Storage {
    /// Returns the data for `key` in the storage or `None` if the key can not be found.
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;
    /// Set `key` to `value` in the storage.
    fn insert(&self, key: &[u8], value: &[u8]);
    /// Clear the storage of the given `key` and its value.
    fn remove(&self, key: &[u8]);
}

pub trait WriteEffects {
    fn delete(&self, path: AccessPath);
    fn insert(&self, path: AccessPath, blob: Vec<u8>);
}

pub struct State<S, O: Oracle> {
    store: S,
    oracle: OracleView<O>,
}

pub trait EventHandler {
    fn on_event(
        &self,
        address: AccountAddress,
        ty_tag: TypeTag,
        message: Vec<u8>,
        caller: Option<ModuleId>,
    );
}

impl<S, O> State<S, O>
where
    S: Storage,
    O: Oracle,
{
    pub fn new(store: S, oracle: O) -> State<S, O> {
        State {
            store,
            oracle: OracleView::new(oracle),
        }
    }

    pub fn get_by_path(&self, path: AccessPath) -> Option<Vec<u8>> {
        let mut key = Vec::with_capacity(AccountAddress::LENGTH + path.path.len());
        key.extend_from_slice(&path.address.to_u8());
        key.extend_from_slice(&path.path);
        self.store.get(&key)
    }
}

impl<S, O> RemoteCache for State<S, O>
where
    S: Storage,
    O: Oracle,
{
    fn get_module(&self, module_id: &ModuleId) -> VMResult<Option<Vec<u8>>> {
        let path = AccessPath::from(module_id);
        Ok(self.get_by_path(path))
    }

    fn get_resource(
        &self,
        address: &AccountAddress,
        tag: &StructTag,
    ) -> PartialVMResult<Option<Vec<u8>>> {
        if address == &CORE_CODE_ADDRESS {
            if let Some(ticker) = self.oracle.get_ticker(tag) {
                return Ok(self.oracle.get_price(&ticker));
            }
        }

        let path = AccessPath::resource_access_path(&ResourceKey::new(*address, tag.to_owned()));
        Ok(self.get_by_path(path))
    }
}

impl<S, O> WriteEffects for State<S, O>
where
    S: Storage,
    O: Oracle,
{
    fn delete(&self, path: AccessPath) {
        let mut key = Vec::with_capacity(AccountAddress::LENGTH + path.path.len());
        key.extend_from_slice(&path.address.to_u8());
        key.extend_from_slice(&path.path);
        self.store.remove(&key);
    }

    fn insert(&self, path: AccessPath, blob: Vec<u8>) {
        let mut key = Vec::with_capacity(AccountAddress::LENGTH + path.path.len());
        key.extend_from_slice(&path.address.to_u8());
        key.extend_from_slice(&path.path);
        self.store.insert(&key, &blob);
    }
}

pub trait Oracle {
    fn get_price(&self, ticker: &str) -> Option<u128>;
}

pub struct OracleView<O: Oracle> {
    oracle: O,
}

const PONT: &str = "PONT";

impl<O> OracleView<O>
where
    O: Oracle,
{
    pub fn new(oracle: O) -> OracleView<O> {
        OracleView { oracle }
    }

    pub fn get_ticker(&self, tag: &StructTag) -> Option<String> {
        fn extract_name(tag: &TypeTag) -> Option<String> {
            match tag {
                TypeTag::Struct(tg) => Some(if tg.module.as_str() == PONT {
                    PONT.to_owned()
                } else {
                    tg.name.as_str().to_owned()
                }),
                _ => None,
            }
        }

        if tag.address == CORE_CODE_ADDRESS
            && tag.module.as_str() == "Coins"
            && tag.name.as_str() == "Price"
        {
            if tag.type_params.len() == 2 {
                let first_part = extract_name(&tag.type_params[0])?;
                let second_part = extract_name(&tag.type_params[1])?;

                Some(format!(
                    "{}_{}",
                    first_part.to_uppercase(),
                    second_part.to_uppercase()
                ))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_price(&self, ticker: &str) -> Option<Vec<u8>> {
        self.oracle
            .get_price(ticker)
            .map(|price| price.to_le_bytes().to_vec())
    }
}

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

pub trait BalanceAccess {
    fn get_balance(&self, address: &AccountAddress, ticker: &str) -> Option<Balance>;
    fn deposit(&self, address: &AccountAddress, ticker: &str, amount: Balance);
    fn withdraw(&self, address: &AccountAddress, ticker: &str, amount: Balance);
}

pub struct Bank<B: BalanceAccess> {
    access: B,
}

impl<B: BalanceAccess> Bank<B> {
    pub fn new(access: B) -> Bank<B> {
        Bank { access }
    }

    pub fn deposit(&self, wallet_id: &WalletId, amount: Balance) {
        if wallet_id.module == PONT {
            self.access.deposit(&wallet_id.address, PONT, amount)
        } else {
            self.access
                .deposit(&wallet_id.address, &wallet_id.name, amount)
        }
    }

    pub fn withdrawal(&self, wallet_id: &WalletId, amount: Balance) {
        if wallet_id.module == PONT {
            self.access.withdraw(&wallet_id.address, PONT, amount)
        } else {
            self.access
                .withdraw(&wallet_id.address, &wallet_id.name, amount)
        }
    }
}

impl<B: BalanceAccess> NativeBalance for &Bank<B> {
    fn get_balance(&self, wallet_id: &WalletId) -> Option<Balance> {
        if wallet_id.module == PONT {
            self.access.get_balance(&wallet_id.address, PONT)
        } else {
            self.access.get_balance(&wallet_id.address, &wallet_id.name)
        }
    }
}

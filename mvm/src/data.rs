use crate::access_path::AccessPath;
use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::{ModuleId, ResourceKey, StructTag, TypeTag};
use move_vm_runtime::data_cache::RemoteCache;
use vm::errors::{PartialVMResult, VMResult};

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

pub struct State<S> {
    store: S,
}

pub trait EventHandler {
    fn on_event(
        &self,
        guid: Vec<u8>,
        seq_num: u64,
        ty_tag: TypeTag,
        message: Vec<u8>,
        caller: Option<ModuleId>,
    );
}

impl<S> State<S>
where
    S: Storage,
{
    pub fn new(store: S) -> State<S> {
        State { store }
    }

    pub fn get_by_path(&self, path: AccessPath) -> Option<Vec<u8>> {
        let mut key = Vec::with_capacity(AccountAddress::LENGTH + path.path.len());
        key.extend_from_slice(&path.address.to_u8());
        key.extend_from_slice(&path.path);
        self.store.get(&key)
    }
}

impl<S> RemoteCache for State<S>
where
    S: Storage,
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
        let path = AccessPath::resource_access_path(&ResourceKey::new(*address, tag.to_owned()));
        Ok(self.get_by_path(path))
    }
}

impl<S> WriteEffects for State<S>
where
    S: Storage,
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

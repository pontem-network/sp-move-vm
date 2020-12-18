use alloc::vec::Vec;
use move_vm_runtime::data_cache::RemoteCache;
use move_core_types::language_storage::{ModuleId, StructTag, ResourceKey};
use vm::errors::{VMResult, PartialVMResult};
use move_core_types::account_address::AccountAddress;
use crate::access_path::AccessPath;
use alloc::borrow::ToOwned;

pub trait Storage {
    /// Returns the data for `key` in the storage or `None` if the key can not be found.
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;
    /// Set `key` to `value` in the storage.
    fn insert(&mut self, key: &[u8], value: &[u8]);
    /// Clear the storage of the given `key` and its value.
    fn remove(&mut self, key: &[u8]);
}

pub struct State<S> {
    data: S,
}

impl<S> State<S> {
    pub fn new(data: S) -> State<S> {
        State {
            data
        }
    }
}

impl<S> RemoteCache for State<S>
        where S: Storage {
    fn get_module(&self, module_id: &ModuleId) -> VMResult<Option<Vec<u8>>> {


        unimplemented!()
    }

    fn get_resource(&self, address: &AccountAddress, tag: &StructTag) -> PartialVMResult<Option<Vec<u8>>> {
        let resource_tag = ResourceKey::new(*address, tag.to_owned());
        let path = AccessPath::resource_access_path(&resource_tag);

        unimplemented!()
    }
}
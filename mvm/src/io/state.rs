use crate::io::traits::{Storage, BalanceAccess};
use move_vm_runtime::data_cache::RemoteCache;
use crate::io::key::AccessKey;
use move_core_types::language_storage::{ModuleId, StructTag};
use vm::errors::{VMResult, PartialVMResult};
use move_core_types::account_address::AccountAddress;
use crate::io::balance::MasterOfCoin;

pub struct State<S: Storage> {
    store: S,
}

impl<S: Storage> State<S> {
    pub fn new(store: S) -> State<S> {
        State {
            store,
        }
    }
}

impl<S: Storage> RemoteCache for State<S> {
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

impl<S: Storage> WriteEffects for State<S> {
    fn delete(&self, key: AccessKey) {
        self.store.remove(key.as_ref());
    }

    fn insert(&self, key: AccessKey, blob: Vec<u8>) {
        self.store.insert(key.as_ref(), &blob);
    }
}

pub trait WriteEffects {
    fn delete(&self, path: AccessKey);
    fn insert(&self, path: AccessKey, blob: Vec<u8>);
}
use crate::io::balance::MasterOfCoin;
use crate::io::context::ExecutionContext;
use crate::io::key::AccessKey;
use crate::io::session::StateSession;
use crate::io::traits::{BalanceAccess, Storage};
use alloc::vec::Vec;
use anyhow::Error;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::{ModuleId, StructTag};
use move_core_types::resolver::{ModuleResolver, ResourceResolver};

pub struct State<S: Storage> {
    store: S,
}

impl<S: Storage> State<S> {
    pub fn new(store: S) -> State<S> {
        State { store }
    }

    pub fn state_session<'c, B: BalanceAccess>(
        &self,
        context: Option<ExecutionContext>,
        master_of_coin: &'c MasterOfCoin<B>,
    ) -> StateSession<'c, '_, State<S>, B> {
        StateSession::new(self, context, master_of_coin.session(self))
    }
}

impl<S: Storage> ModuleResolver for State<S> {
    type Error = Error;

    fn get_module(&self, module_id: &ModuleId) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(self.store.get(AccessKey::from(module_id).as_ref()))
    }
}

impl<S: Storage> ResourceResolver for State<S> {
    type Error = Error;

    fn get_resource(
        &self,
        address: &AccountAddress,
        typ: &StructTag,
    ) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(self.store.get(AccessKey::from((address, typ)).as_ref()))
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

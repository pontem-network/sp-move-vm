use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::{TypeTag, ModuleId};

pub trait EventHandler {
    fn on_event(
        &self,
        guid: Vec<u8>,
        seq_num: u64,
        ty_tag: TypeTag,
        message: Vec<u8>,
    );
}

pub trait Storage {
    /// Returns the data for `key` in the storage or `None` if the key can not be found.
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;
    /// Set `key` to `value` in the storage.
    fn insert(&self, key: &[u8], value: &[u8]);
    /// Clear the storage of the given `key` and its value.
    fn remove(&self, key: &[u8]);
}

pub type Balance = u128;

pub trait BalanceAccess {
    fn get_balance(&self, address: &AccountAddress, ticker: &str) -> Option<Balance>;
    fn deposit(&self, address: &AccountAddress, ticker: &str, amount: Balance);
    fn withdraw(&self, address: &AccountAddress, ticker: &str, amount: Balance);
}
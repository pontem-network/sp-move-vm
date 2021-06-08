use alloc::borrow::Cow;
use core::cell::RefCell;

use hashbrown::HashMap;

use diem_types::account_config::{CORE_CODE_ADDRESS, DIEM_MODULE_NAME};
use diem_types::account_config;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{StructTag, TypeTag};
use move_core_types::vm_status::known_locations::DIEM_MODULE_IDENTIFIER;
use move_vm_runtime::data_cache::RemoteCache;

use crate::io::traits::{Balance, BalanceAccess, CurrencyAccessPath};

pub const DIEM_COIN: &str = "Diem";
pub const CURRENCY_INFO: &str = "CurrencyInfo";

pub struct MasterOfCoin<B: BalanceAccess> {
    access: B,
    native_mapper: RefCell<HashMap<StructTag, Option<Vec<u8>>>>,
}

impl<B: BalanceAccess> MasterOfCoin<B> {
    pub fn new(access: B) -> MasterOfCoin<B> {
        MasterOfCoin {
            access,
            native_mapper: Default::default(),
        }
    }

    pub fn session<'b, 'r, R: RemoteCache>(&'b self, remote: &'r R) -> MasterOfCoinSession<'b, 'r, B, R> {
        MasterOfCoinSession {
            master_of_coin: self,
            balances: RefCell::new(Default::default()),
            remote,
        }
    }

    pub fn clear(&self) {
        self.native_mapper.borrow_mut().clear();
    }

    pub fn get_bridge<R: RemoteCache>(&self, remote: &R, coin: &StructTag) -> Option<Vec<u8>> {
        let mut mapper = self.native_mapper.borrow_mut();

        match mapper.get(coin) {
            Some(path) => {
                return path.to_owned();
            }
            None => {
                let addr = account_config::diem_root_address();
                let mut currency = native_currency(coin);
                let bridge = remote.get_resource(&addr, &currency).ok()?;
                if let TypeTag::Struct(coin) = currency.type_params.remove(0) {
                    mapper.insert(coin, bridge.to_owned());
                }
                return bridge;
            }
        }
    }
}

pub struct MasterOfCoinSession<'b, 'r, B: BalanceAccess, R: RemoteCache> {
    master_of_coin: &'b MasterOfCoin<B>,
    balances: RefCell<HashMap<AccountAddress, HashMap<Cow<'static, CurrencyAccessPath>, Balance>>>,
    remote: &'r R,
}

impl<'b, 'r, B: BalanceAccess, R: RemoteCache> MasterOfCoinSession<'b, 'r, B, R> {
    pub fn get_bridge(&self, coin: &StructTag) -> Option<Vec<u8>> {
        self.master_of_coin.get_bridge(self.remote, coin)
    }

    pub fn resolve(&self, address: &AccountAddress, tag: &StructTag) -> Option<Vec<u8>> {
        if tag.module.as_ref() == DIEM_MODULE_IDENTIFIER.as_ref() {
            if tag.name.as_str() == DIEM_COIN {
                if let Some(bridge) = self.get_bridge(coin_type(&tag.type_params)?) {


                }
            }
            if tag.name.as_str() == CURRENCY_INFO {
                if let Some(bridge) = self.get_bridge(coin_type(&tag.type_params)?) {


                }
            }
        }
        None
    }
}

fn coin_type(t_params: &[TypeTag]) -> Option<&StructTag> {
    if t_params.len() != 1 {
        None
    } else {
        match &t_params[0] {
            TypeTag::Struct(tag) => Some(tag),
            _ => None,
        }
    }
}

fn native_currency(coin: &StructTag) -> StructTag {
    StructTag {
        address: CORE_CODE_ADDRESS,
        module: Identifier::new("NativeCurrencies").expect("Valid identifier"),
        name: Identifier::new("NativeCurrency").expect("Valid identifier"),
        type_params: vec![TypeTag::Struct(coin.to_owned())],
    }
}
use alloc::borrow::Cow;
use core::cell::RefCell;

use anyhow::Error;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use diem_types::account_config;
use diem_types::account_config::CORE_CODE_ADDRESS;
use diem_types::resources::currency_info::CurrencyInfoResource;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{StructTag, TypeTag};
use move_core_types::vm_status::known_locations::DIEM_MODULE_IDENTIFIER;
use move_vm_runtime::data_cache::RemoteCache;
use vm::errors::PartialVMResult;

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

    pub fn session<'b, 'r, R: RemoteCache>(
        &'b self,
        remote: &'r R,
    ) -> MasterOfCoinSession<'b, 'r, B, R> {
        MasterOfCoinSession {
            master_of_coin: self,
            balances: RefCell::new(Default::default()),
            remote,
        }
    }

    pub fn clear(&self) {
        self.native_mapper.borrow_mut().clear();
    }

    fn get_bridge<R: RemoteCache>(&self, remote: &R, coin: &StructTag) -> Option<Vec<u8>> {
        let mut mapper = self.native_mapper.borrow_mut();

        match mapper.get(coin) {
            Some(path) => path.to_owned(),
            None => {
                let addr = account_config::diem_root_address();
                let mut currency = native_currency(coin);
                let bridge = remote.get_resource(&addr, &currency).ok()?
                    .and_then(|buff| bcs::from_bytes(&buff).ok());
                if let TypeTag::Struct(coin) = currency.type_params.remove(0) {
                    mapper.insert(coin, bridge.to_owned());
                }
                bridge
            }
        }
    }

    fn get_balance(&self, address: &AccountAddress, path: &CurrencyAccessPath) -> Option<Balance> {
        self.access.get_balance(address, path)
    }

    fn get_currency_info(&self, path: &CurrencyAccessPath) -> Option<CurrencyInfo> {
        self.access.get_currency_info(path)
    }
}

pub struct MasterOfCoinSession<'b, 'r, B: BalanceAccess, R: RemoteCache> {
    master_of_coin: &'b MasterOfCoin<B>,
    balances: RefCell<
        HashMap<AccountAddress, HashMap<Cow<'static, CurrencyAccessPath>, Option<Balance>>>,
    >,
    remote: &'r R,
}

impl<'b, 'r, B: BalanceAccess, R: RemoteCache> MasterOfCoinSession<'b, 'r, B, R> {
    fn get_bridge(&self, coin: &StructTag) -> Option<Vec<u8>> {
        self.master_of_coin.get_bridge(self.remote, coin)
    }

    fn get_balance(&self, address: &AccountAddress, path: Vec<u8>) -> Option<Vec<u8>> {
        let balance = self.master_of_coin.get_balance(address, &path);
        let mut balances = self.balances.borrow_mut();
        let acc = balances.entry(*address).or_insert_with(HashMap::new);
        acc.insert(Cow::Owned(path), balance.clone());
        bcs::to_bytes(&balance?).ok()
    }

    pub fn resolve(
        &self,
        address: &AccountAddress,
        tag: &StructTag,
    ) -> PartialVMResult<Option<Vec<u8>>> {
        if tag.module.as_ref() == DIEM_MODULE_IDENTIFIER.as_ref() {
            if tag.name.as_str() == DIEM_COIN {
                let bridge = coin_type(&tag.type_params).and_then(|coin| self.get_bridge(coin));
                if let Some(bridge) = bridge {
                    return Ok(self.get_balance(address, bridge));
                }
            }
            if tag.name.as_str() == CURRENCY_INFO {
                let bridge = coin_type(&tag.type_params).and_then(|coin| self.get_bridge(coin));
                if let Some(bridge) = bridge {
                    return Ok(self
                        .remote
                        .get_resource(address, tag)?
                        .and_then(|val| {
                            self.master_of_coin
                                .get_currency_info(&bridge)
                                .map(|info| (val, info))
                        })
                        .map(|(info, path)| path.apply(&info).unwrap()));
                }
            }
        }
        Ok(None)
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

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct CurrencyInfo {
    pub total_value: u128,
}

impl CurrencyInfo {
    pub fn apply(&self, buff: &[u8]) -> Result<Vec<u8>, Error> {
        let mut info = CurrencyInfoResource::try_from_bytes(buff)?;
        info.total_value = self.total_value;
        bcs::to_bytes(&info).map_err(Into::into)
    }
}

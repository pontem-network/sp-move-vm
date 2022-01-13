// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{
    access_path::AccessPath,
    account_config::constants::{
        diem_root_address, type_tag_for_currency_code, CORE_CODE_ADDRESS, DIEM_MODULE_IDENTIFIER,
    },
    event::EventHandle,
};
use anyhow::anyhow;
use anyhow::Result;
use move_core_types::{
    ident_str,
    identifier::{IdentStr, Identifier},
    language_storage::{ResourceKey, StructTag},
    move_resource::{MoveResource, MoveStructType},
};
use serde::{Deserialize, Serialize};

/// Struct that represents a CurrencyInfo resource
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrencyInfoResource {
    pub total_value: u128,
    decimals: u8,
    currency_code: Identifier,
    mint_events: EventHandle,
    burn_events: EventHandle,
}

impl MoveStructType for CurrencyInfoResource {
    const MODULE_NAME: &'static IdentStr = DIEM_MODULE_IDENTIFIER;
    const STRUCT_NAME: &'static IdentStr = ident_str!("TokenInfo");
}

impl MoveResource for CurrencyInfoResource {}

impl CurrencyInfoResource {
    pub fn new(
        total_value: u128,
        decimals: u8,
        currency_code: Identifier,
        mint_events: EventHandle,
        burn_events: EventHandle,
    ) -> Self {
        Self {
            total_value,
            decimals,
            currency_code,
            mint_events,
            burn_events,
        }
    }

    pub fn currency_code(&self) -> &IdentStr {
        &self.currency_code
    }

    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    pub fn total_value(&self) -> u128 {
        self.total_value
    }

    pub fn struct_tag_for(currency_code: Identifier) -> StructTag {
        StructTag {
            address: CORE_CODE_ADDRESS,
            module: CurrencyInfoResource::module_identifier(),
            name: CurrencyInfoResource::struct_identifier(),
            type_params: vec![type_tag_for_currency_code(currency_code)],
        }
    }

    pub fn resource_path_for(currency_code: Identifier) -> AccessPath {
        let resource_key = ResourceKey::new(
            diem_root_address(),
            CurrencyInfoResource::struct_tag_for(currency_code),
        );
        AccessPath::resource_access_path(resource_key)
    }

    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self> {
        bcs::from_bytes(bytes).map_err(|err| anyhow!("Failed to decode bcs. {:?}", err))
    }

    pub fn mint_events(&self) -> &EventHandle {
        &self.mint_events
    }

    pub fn burn_events(&self) -> &EventHandle {
        &self.burn_events
    }
}

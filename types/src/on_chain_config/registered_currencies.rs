// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::on_chain_config::OnChainConfig;
use alloc::vec::Vec;
use anyhow::{anyhow, Result};
use core::fmt;
use move_core_types::identifier::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RegisteredCurrencies {
    currency_codes: Vec<Identifier>,
}

impl fmt::Display for RegisteredCurrencies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for currency_code in self.currency_codes().iter() {
            write!(f, "{} ", currency_code)?;
        }
        write!(f, "]")
    }
}

impl RegisteredCurrencies {
    pub fn currency_codes(&self) -> &[Identifier] {
        &self.currency_codes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        bcs::from_bytes(bytes)
            .map_err(|err| anyhow!("Failed to decode RegisteredCurrencies. {:?}", err))
    }
}

impl OnChainConfig for RegisteredCurrencies {
    // registered currencies address
    const IDENTIFIER: &'static str = "RegisteredCurrencies";
}

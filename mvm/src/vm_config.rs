use crate::gas_schedule::cost_table;
use move_core_types::gas_schedule::CostTable;
use parity_scale_codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// Defines all the on chain configuration data needed by VM.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Encode, Decode)]
pub struct VMConfig {
    pub gas_schedule: CostTable,
}

impl Default for VMConfig {
    fn default() -> Self {
        VMConfig {
            gas_schedule: cost_table(),
        }
    }
}

pub mod loader {
    use crate::access_path::AccessPath;
    use crate::data::Storage;
    use crate::vm_config::VMConfig;
    use alloc::vec::Vec;
    use anyhow::{Error, Result};
    use move_core_types::account_address::AccountAddress;
    use move_core_types::identifier::Identifier;
    use move_core_types::language_storage::StructTag;
    use parity_scale_codec::{Decode, Encode};

    const IDENTIFIER: &str = "MVMConfig";
    const CONFIG_ADDRESS_STR: &str = "0xA550C18";

    fn config_address() -> AccountAddress {
        AccountAddress::from_hex_literal(CONFIG_ADDRESS_STR).expect("failed to get address")
    }

    fn make_access_path() -> AccessPath {
        let address = config_address();
        let id = Identifier::new(IDENTIFIER).expect("failed to get Identifier");

        AccessPath::new(
            address,
            AccessPath::resource_access_vec(&StructTag {
                address,
                module: id.clone(),
                name: id,
                type_params: vec![],
            }),
        )
    }

    fn make_storage_key() -> Vec<u8> {
        let path = make_access_path();
        let mut key = Vec::with_capacity(AccountAddress::LENGTH + path.path.len());
        key.extend_from_slice(&path.address.to_u8());
        key.extend_from_slice(&path.path);
        key
    }

    /// Loads vm config from storage. Returns default configuration if the config does not exists in the storage.
    pub fn load_vm_config<S: Storage>(storage: &S) -> Result<VMConfig, Error> {
        if let Some(blob) = storage.get(&make_storage_key()) {
            let mut input = blob.as_slice();
            VMConfig::decode(&mut input).map_err(|_| Error::msg("failed to decode VMConfig."))
        } else {
            Ok(VMConfig::default())
        }
    }

    /// Stores vm configuration to the storage.
    pub fn store_vm_config<S: Storage>(storage: &S, config: &VMConfig) {
        storage.insert(&make_storage_key(), &config.encode());
    }
}

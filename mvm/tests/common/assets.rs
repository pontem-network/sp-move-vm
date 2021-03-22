use serde::Deserialize;

use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::CORE_CODE_ADDRESS;
use mvm::types::{Gas, ModuleTx, ScriptArg, ScriptTx};

pub fn gas() -> Gas {
    Gas::new(10_000, 1).unwrap()
}

pub fn block_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/target/modules/0_Block.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

pub fn coins_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/target/modules/1_Coins.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

pub fn event_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/target/modules/6_Event.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

pub fn store_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/target/modules/4_Store.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

pub fn time_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/target/modules/5_Time.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

pub fn pont_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/target/modules/2_PONT.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

pub fn event_proxy_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/target/modules/9_EventProxy.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

pub fn signer_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/target/modules/3_Signer.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

pub fn pontem_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/target/modules/7_Pontem.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

pub fn account_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/target/modules/8_Account.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

pub fn emit_event_script(addr: AccountAddress, args: u64) -> ScriptTx {
    ScriptTx::new(
        include_bytes!("../assets/target/scripts/0_emit_event.mv").to_vec(),
        vec![ScriptArg::U64(args)],
        vec![],
        vec![addr],
    )
}

pub fn get_price_script(
    addr_for_eth_btc: AccountAddress,
    addr_for_btc_pont: AccountAddress,
) -> ScriptTx {
    ScriptTx::new(
        include_bytes!("../assets/target/scripts/1_get_price_test.mv").to_vec(),
        vec![],
        vec![],
        vec![addr_for_eth_btc, addr_for_btc_pont],
    )
}

pub fn store_sys_resources_script(
    addr_for_block: AccountAddress,
    addr_for_timestamp: AccountAddress,
) -> ScriptTx {
    ScriptTx::new(
        include_bytes!("../assets/target/scripts/2_store_system_resources.mv").to_vec(),
        vec![],
        vec![],
        vec![addr_for_block, addr_for_timestamp],
    )
}

pub fn store_u64_script(addr: AccountAddress, args: u64) -> ScriptTx {
    ScriptTx::new(
        include_bytes!("../assets/target/scripts/3_store_u64.mv").to_vec(),
        vec![ScriptArg::U64(args)],
        vec![],
        vec![addr],
    )
}

pub fn test_balance_script(
    addr: AccountAddress,
    addr_2: AccountAddress,
    init_usdt: u128,
    init_pont: u128,
    init_btc: u128,
) -> ScriptTx {
    ScriptTx::new(
        include_bytes!("../assets/target/scripts/4_test_balance.mv").to_vec(),
        vec![
            ScriptArg::U128(init_usdt),
            ScriptArg::U128(init_pont),
            ScriptArg::U128(init_btc),
        ],
        vec![],
        vec![addr, addr_2],
    )
}

#[derive(Deserialize)]
pub struct StoreU64 {
    pub val: u64,
}

#[derive(Deserialize)]
pub struct StoreU128 {
    pub val: u128,
}

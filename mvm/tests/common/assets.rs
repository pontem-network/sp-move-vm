use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::CORE_CODE_ADDRESS;
use mvm::types::{Gas, ModuleTx, ScriptArg, ScriptTx};
use serde::Deserialize;

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
        include_bytes!("../assets/target/modules/2_Event.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

pub fn store_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/target/modules/3_Store.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

pub fn time_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/target/modules/4_Time.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

pub fn xfi_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/target/modules/5_XFI.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

pub fn event_proxy_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/target/modules/6_EventProxy.mv").to_vec(),
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
    addr_for_btc_xfi: AccountAddress,
) -> ScriptTx {
    ScriptTx::new(
        include_bytes!("../assets/target/scripts/1_get_price_test.mv").to_vec(),
        vec![],
        vec![],
        vec![addr_for_eth_btc, addr_for_btc_xfi],
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

#[derive(Deserialize)]
pub struct StoreU64 {
    pub val: u64,
}

#[derive(Deserialize)]
pub struct StoreU128 {
    pub val: u128,
}

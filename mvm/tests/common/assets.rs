use std::convert::TryFrom;

use serde::Deserialize;

use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::CORE_CODE_ADDRESS;
use mvm::types::{Gas, ModulePackage, ModuleTx, ScriptArg, ScriptTx};

pub fn gas() -> Gas {
    Gas::new(10_000, 1).unwrap()
}

pub fn store_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/artifacts/modules/Store.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

pub fn event_proxy_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/artifacts/modules/EventProxy.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

pub fn abort_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/artifacts/modules/Abort.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

pub fn emit_event_script(addr: AccountAddress, args: u64) -> ScriptTx {
    ScriptTx::new(
        include_bytes!("../assets/artifacts/scripts/emit_event.mv").to_vec(),
        vec![ScriptArg::U64(args)],
        vec![],
        vec![addr],
    )
    .unwrap()
}

pub fn store_sys_resources_script(
    addr_for_block: AccountAddress,
    addr_for_timestamp: AccountAddress,
) -> ScriptTx {
    ScriptTx::new(
        include_bytes!("../assets/artifacts/scripts/store_system_resources.mv").to_vec(),
        vec![],
        vec![],
        vec![addr_for_block, addr_for_timestamp],
    )
    .unwrap()
}

pub fn store_u64_script(addr: AccountAddress, args: u64) -> ScriptTx {
    ScriptTx::new(
        include_bytes!("../assets/artifacts/scripts/store_u64.mv").to_vec(),
        vec![ScriptArg::U64(args)],
        vec![],
        vec![addr],
    )
    .unwrap()
}

// pub fn test_transfer_script(alice: AccountAddress, bob: AccountAddress, amount: u128) -> ScriptTx {
//     ScriptTx::new(
//         include_bytes!("../assets/target/scripts/test_balance_transfer.mv").to_vec(),
//         vec![ScriptArg::U128(amount)],
//         vec![],
//         vec![alice, bob],
//     )
//     .unwrap()
// }

// pub fn reg_coin_script(ty: TypeTag, denom: &str, decimals: u8) -> ScriptTx {
//     ScriptTx::new(
//         include_bytes!("../assets/target/scripts/register_coin.mv").to_vec(),
//         vec![
//             ScriptArg::VectorU8(denom.as_bytes().to_vec()),
//             ScriptArg::U8(decimals),
//         ],
//         vec![ty],
//         vec![CORE_CODE_ADDRESS],
//     )
//     .unwrap()
// }

pub fn error_script(addr: AccountAddress) -> ScriptTx {
    ScriptTx::new(
        include_bytes!("../assets/artifacts/scripts/error.mv").to_vec(),
        vec![],
        vec![],
        vec![addr],
    )
    .unwrap()
}

// pub fn test_balance_script(
//     addr: AccountAddress,
//     addr_2: AccountAddress,
//     init_usdt: u128,
//     init_pont: u128,
//     init_btc: u128,
// ) -> ScriptTx {
//     ScriptTx::new(
//         include_bytes!("../assets/target/scripts/test_balance.mv").to_vec(),
//         vec![
//             ScriptArg::U128(init_usdt),
//             ScriptArg::U128(init_pont),
//             ScriptArg::U128(init_btc),
//         ],
//         vec![],
//         vec![addr, addr_2],
//     )
//     .unwrap()
// }

pub fn valid_package() -> ModulePackage {
    ModulePackage::try_from(&include_bytes!("../assets/artifacts/bundles/valid_pack.pac")[..])
        .unwrap()
}

pub fn invalid_package() -> ModulePackage {
    ModulePackage::try_from(&include_bytes!("../assets/artifacts/bundles/invalid_pack.pac")[..])
        .unwrap()
}

#[derive(Deserialize)]
pub struct StoreU64 {
    pub val: u64,
}

#[derive(Deserialize)]
pub struct StoreU128 {
    pub val: u128,
}

use std::convert::TryFrom;

use serde::Deserialize;

use crate::common::mock::addr;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{StructTag, TypeTag, CORE_CODE_ADDRESS};
use mvm::io::traits::Balance;
use mvm::types::{Gas, ModulePackage, ModuleTx, ScriptArg, ScriptTx};

pub fn gas() -> Gas {
    Gas::new(10_000, 1).unwrap()
}

pub fn store_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/build/assets/bytecode_modules/Store.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

pub fn reflect_test_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/build/assets/bytecode_modules/ReflectTest.mv").to_vec(),
        addr("0x13"),
    )
}

pub fn script_book_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/build/assets/bytecode_modules/ScriptBook.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

pub fn event_proxy_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/build/assets/bytecode_modules/EventProxy.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

pub fn abort_module() -> ModuleTx {
    ModuleTx::new(
        include_bytes!("../assets/build/assets/bytecode_modules/Abort.mv").to_vec(),
        CORE_CODE_ADDRESS,
    )
}

pub fn emit_event_script(addr: AccountAddress, args: u64) -> ScriptTx {
    ScriptTx::with_script(
        include_bytes!("../assets/build/assets/bytecode_scripts/emit_event.mv").to_vec(),
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
    ScriptTx::with_script(
        include_bytes!("../assets/build/assets/bytecode_scripts/store_system_resources.mv")
            .to_vec(),
        vec![],
        vec![],
        vec![addr_for_block, addr_for_timestamp],
    )
    .unwrap()
}

pub fn store_u64_script(addr: AccountAddress, args: u64) -> ScriptTx {
    ScriptTx::with_script(
        include_bytes!("../assets/build/assets/bytecode_scripts/store_u64.mv").to_vec(),
        vec![ScriptArg::U64(args)],
        vec![],
        vec![addr],
    )
    .unwrap()
}

pub fn signer_order() -> ScriptTx {
    ScriptTx::with_script(
        include_bytes!("../assets/build/assets/bytecode_scripts/signer_order.mv").to_vec(),
        vec![],
        vec![],
        vec![addr("0x1"), addr("0x2"), addr("0x3")],
    )
    .unwrap()
}

pub fn pont_info_script(address: AccountAddress, total: u128) -> ScriptTx {
    ScriptTx::with_script(
        include_bytes!("../assets/build/assets/bytecode_scripts/pont_info.mv").to_vec(),
        vec![ScriptArg::U128(total)],
        vec![],
        vec![address],
    )
    .unwrap()
}

pub fn error_script(addr: AccountAddress) -> ScriptTx {
    ScriptTx::with_script(
        include_bytes!("../assets/build/assets/bytecode_scripts/error.mv").to_vec(),
        vec![],
        vec![],
        vec![addr],
    )
    .unwrap()
}

pub fn transfer_script(
    from: AccountAddress,
    from_balance: Balance,
    to: AccountAddress,
    to_balance: Balance,
    to_move: Balance,
) -> ScriptTx {
    ScriptTx::with_script(
        include_bytes!("../assets/build/assets/bytecode_scripts/transfer.mv").to_vec(),
        vec![
            ScriptArg::U64(from_balance),
            ScriptArg::U64(to_balance),
            ScriptArg::U64(to_move),
        ],
        vec![],
        vec![from, to],
    )
    .unwrap()
}

pub fn empty_loop(iter: u64) -> ScriptTx {
    ScriptTx::with_script(
        include_bytes!("../assets/build/assets/bytecode_scripts/empty_loop.mv").to_vec(),
        vec![ScriptArg::U64(iter)],
        vec![],
        vec![],
    )
    .unwrap()
}

pub fn math_loop(iter: u64) -> ScriptTx {
    ScriptTx::with_script(
        include_bytes!("../assets/build/assets/bytecode_scripts/math_loop.mv").to_vec(),
        vec![ScriptArg::U64(iter)],
        vec![],
        vec![],
    )
    .unwrap()
}

pub fn read_write_loop(iter: u64) -> ScriptTx {
    ScriptTx::with_script(
        include_bytes!("../assets/build/assets/bytecode_scripts/read_write_loop.mv").to_vec(),
        vec![ScriptArg::U64(iter)],
        vec![],
        vec![AccountAddress::random()],
    )
    .unwrap()
}

pub fn vector_loop(iter: u64) -> ScriptTx {
    ScriptTx::with_script(
        include_bytes!("../assets/build/assets/bytecode_scripts/vector_loop.mv").to_vec(),
        vec![ScriptArg::U64(iter)],
        vec![],
        vec![],
    )
    .unwrap()
}

pub fn reflect_type_of(addr: AccountAddress, module: &str, strct: &str) -> ScriptTx {
    ScriptTx::with_script(
        include_bytes!("../assets/build/assets/bytecode_scripts/test_reflect.mv").to_vec(),
        vec![
            ScriptArg::Address(addr),
            ScriptArg::VectorU8(module.as_bytes().to_vec()),
            ScriptArg::VectorU8(strct.as_bytes().to_vec()),
        ],
        vec![TypeTag::Struct(StructTag {
            address: addr,
            module: Identifier::new(module).unwrap(),
            name: Identifier::new(strct).unwrap(),
            type_params: vec![],
        })],
        vec![],
    )
    .unwrap()
}

pub fn valid_package() -> ModulePackage {
    ModulePackage::try_from(&include_bytes!("../assets/build/assets/bundles/valid_pack.pac")[..])
        .unwrap()
}

pub fn invalid_package() -> ModulePackage {
    ModulePackage::try_from(&include_bytes!("../assets/build/assets/bundles/invalid_pack.pac")[..])
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

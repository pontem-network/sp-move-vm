use core::convert::TryFrom;
use diem_types::account_config::diem_root_address;
use move_binary_format::access::ModuleAccess;
use move_binary_format::file_format::CompiledScript;
use move_binary_format::CompiledModule;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{ModuleId, StructTag, TypeTag, CORE_CODE_ADDRESS};
use move_core_types::value::MoveValue;
use mvm::abi::{Field, Func, ModuleAbi, StructDef, Type, TypeAbilities};
use mvm::types::{Call, ModulePackage, Transaction};

#[test]
fn test_parse_transaction() {
    let tx =
        Transaction::try_from(&include_bytes!("assets/build/assets/transaction/tx_test.mvt")[..])
            .unwrap();
    assert_eq!(tx.signers_count(), 0);
    assert!(!tx.has_root_signer());

    let script = tx.into_script(vec![]).unwrap();
    match script.call() {
        Call::Script { code } => {
            CompiledScript::deserialize(code).unwrap();
        }
        Call::ScriptFunction { .. } => unreachable!(),
    };
    assert_eq!(
        script.args(),
        &[MoveValue::U64(100).simple_serialize().unwrap()][..]
    );
    assert_eq!(
        script.type_parameters(),
        &[TypeTag::Struct(StructTag {
            address: CORE_CODE_ADDRESS,
            module: Identifier::new("Pontem").unwrap(),
            name: Identifier::new("T").unwrap(),
            type_params: vec![],
        })][..]
    )
}

#[test]
fn test_module_function() {
    let tx = Transaction::try_from(
        &include_bytes!("assets/build/assets/transaction/ScriptBook_test.mvt")[..],
    )
    .unwrap();
    assert_eq!(tx.signers_count(), 0);
    assert!(!tx.has_root_signer());
    let script = tx.into_script(vec![]).unwrap();
    match script.call() {
        Call::Script { .. } => unreachable!(),
        Call::ScriptFunction {
            mod_address,
            mod_name,
            func_name,
        } => {
            assert_eq!(*mod_address, CORE_CODE_ADDRESS);
            assert_eq!(mod_name.as_str(), "ScriptBook");
            assert_eq!(func_name.as_str(), "test");
        }
    };
}

#[test]
fn test_transaction_with_sys_signers() {
    let tx = Transaction::try_from(
        &include_bytes!("assets/build/assets/transaction/rt_signers.mvt")[..],
    )
    .unwrap();
    assert_eq!(tx.signers_count(), 0);
    assert!(tx.has_root_signer());

    let script = tx.into_script(vec![]).unwrap();
    match script.call() {
        Call::Script { code } => {
            CompiledScript::deserialize(code).unwrap();
        }
        Call::ScriptFunction { .. } => unreachable!(),
    };
    assert!(script.args().is_empty());
    assert!(script.type_parameters().is_empty());
    assert_eq!(script.signers(), &[diem_root_address()][..]);

    let tx = Transaction::try_from(
        &include_bytes!("assets/build/assets/transaction/signers_tr_with_user.mvt")[..],
    )
    .unwrap();
    assert_eq!(tx.signers_count(), 1);
    assert!(tx.has_root_signer());

    let addr = AccountAddress::random();
    let script = tx.into_script(vec![addr]).unwrap();
    match script.call() {
        Call::Script { code } => {
            CompiledScript::deserialize(code).unwrap();
        }
        Call::ScriptFunction { .. } => unreachable!(),
    };
    assert!(script.args().is_empty());
    assert!(script.type_parameters().is_empty());
    assert_eq!(script.signers(), &[diem_root_address(), addr][..]);
}

#[test]
fn test_parse_mvt() {
    let tx =
        Transaction::try_from(&include_bytes!("assets/build/assets/transaction/store_u64.mvt")[..])
            .unwrap();
    assert_eq!(tx.signers_count(), 1);
    let script = tx.into_script(vec![CORE_CODE_ADDRESS]).unwrap();
    match script.call() {
        Call::Script { code } => {
            CompiledScript::deserialize(code).unwrap();
        }
        Call::ScriptFunction { .. } => unreachable!(),
    };
    assert_eq!(
        script.args(),
        &[MoveValue::U64(13).simple_serialize().unwrap()][..]
    );
    assert_eq!(script.type_parameters(), &[][..])
}

#[test]
#[should_panic]
fn test_transaction_invalid_signer() {
    let tx = hex::decode("015fa11ceb0b0100000005010002030205050705070c10081c200000000100010002060c03000553746f72650973746f72655f7536340000000000000000000000000000000000000000000000000000000000000001000001040b000a01110002010164000000000000000107200000000000000000000000000000000000000000000000000000000000000001084466696e616e6365015400").unwrap();
    let tx = Transaction::try_from(tx.as_ref()).unwrap();
    assert_eq!(tx.signers_count(), 1);
    tx.into_script(vec![CORE_CODE_ADDRESS, CORE_CODE_ADDRESS])
        .unwrap();
}

#[test]
fn test_parse_pac() {
    let pac =
        ModulePackage::try_from(&include_bytes!("assets/build/assets/bundles/valid_pack.pac")[..])
            .unwrap();
    let tx = pac.into_tx(CORE_CODE_ADDRESS);
    let (modules, address) = tx.into_inner();

    assert_eq!(address, CORE_CODE_ADDRESS);

    let mut modules = modules
        .iter()
        .map(|module| CompiledModule::deserialize(&module).unwrap())
        .map(|module| module.name().to_string())
        .collect::<Vec<_>>();
    modules.sort();
    assert_eq!(
        modules.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        vec!["Abort", "EventProxy", "Foo", "ScriptBook", "Store"]
    );
}

#[test]
fn test_module_abi() {
    use mvm::abi::FuncVisibility::*;
    use mvm::abi::Type::*;
    use mvm::abi::TypeAbility::*;

    let bytecode = include_bytes!("assets/build/assets/bytecode_modules/EventProxy.mv");
    let abi = ModuleAbi::from(CompiledModule::deserialize(bytecode).unwrap());
    assert_eq!(
        abi,
        ModuleAbi {
            id: ModuleId::new(CORE_CODE_ADDRESS, Identifier::new("EventProxy").unwrap()),
            friends: vec![],
            structs: vec![mvm::abi::Struct {
                name: Identifier::new("U64").unwrap(),
                type_parameters: vec![],
                abilities: TypeAbilities {
                    abilities: vec![Copy, Drop, Store, Key]
                },
                fields: vec![Field {
                    name: Identifier::new("val").unwrap(),
                    tp: Type::U64
                }]
            }],
            funcs: vec![
                Func {
                    name: Identifier::new("create_val").unwrap(),
                    visibility: Public,
                    type_parameters: vec![],
                    parameters: vec![U64],
                    returns: vec![Struct(StructDef {
                        id: ModuleId::new(
                            CORE_CODE_ADDRESS,
                            Identifier::new("EventProxy").unwrap()
                        ),
                        name: Identifier::new("U64").unwrap(),
                        type_parameters: vec![]
                    })]
                },
                Func {
                    name: Identifier::new("emit_event").unwrap(),
                    visibility: Public,
                    type_parameters: vec![],
                    parameters: vec![Reference(Box::new(Signer)), U64],
                    returns: vec![]
                },
                Func {
                    name: Identifier::new("test_only").unwrap(),
                    visibility: Script,
                    type_parameters: vec![TypeAbilities { abilities: vec![] }],
                    parameters: vec![],
                    returns: vec![]
                },
            ],
        }
    );
}

use core::convert::TryFrom;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{StructTag, TypeTag, CORE_CODE_ADDRESS};
use move_vm_types::values::Value;
use mvm::types::{parse_type_params, Transaction};
use vm::file_format::CompiledScript;

#[test]
fn test_parse_type_params() {
    test(
        "0x01::Token::BTC",
        vec![TypeTag::Struct(StructTag {
            address: CORE_CODE_ADDRESS,
            module: Identifier::new("Token").unwrap(),
            name: Identifier::new("BTC").unwrap(),
            type_params: vec![],
        })],
    );

    test(
        "0x01::Token::BTC,u8",
        vec![
            TypeTag::Struct(StructTag {
                address: CORE_CODE_ADDRESS,
                module: Identifier::new("Token").unwrap(),
                name: Identifier::new("BTC").unwrap(),
                type_params: vec![],
            }),
            TypeTag::U8,
        ],
    );

    test(
        "0x01::Token::BTC;0x01::Balance::Acc<0x01::Foo::Bar>",
        vec![
            TypeTag::Struct(StructTag {
                address: CORE_CODE_ADDRESS,
                module: Identifier::new("Token").unwrap(),
                name: Identifier::new("BTC").unwrap(),
                type_params: vec![],
            }),
            TypeTag::Struct(StructTag {
                address: CORE_CODE_ADDRESS,
                module: Identifier::new("Balance").unwrap(),
                name: Identifier::new("Acc").unwrap(),
                type_params: vec![TypeTag::Struct(StructTag {
                    address: CORE_CODE_ADDRESS,
                    module: Identifier::new("Foo").unwrap(),
                    name: Identifier::new("Bar").unwrap(),
                    type_params: vec![],
                })],
            }),
        ],
    );

    test("", vec![]);

    fn test(tp: &str, expected: Vec<TypeTag>) {
        assert_eq!(parse_type_params(tp).unwrap(), expected);
    }
}

#[test]
fn test_parse_transaction() {
    let tx = Transaction::try_from(&include_bytes!("assets/target/transactions/tx_test.mvt")[..])
        .unwrap();
    assert_eq!(tx.signers_count(), 0);
    let script = tx.into_script(vec![]).unwrap();
    CompiledScript::deserialize(script.code()).unwrap();
    assert_eq!(script.args(), &[Value::u64(100)][..]);
    assert_eq!(
        script.type_parameters(),
        &[TypeTag::Struct(StructTag {
            address: CORE_CODE_ADDRESS,
            module: Identifier::new("Dfinance").unwrap(),
            name: Identifier::new("T").unwrap(),
            type_params: vec![],
        })][..]
    )
}

#[test]
fn test_parse_mvt() {
    let tx = Transaction::try_from(&include_bytes!("assets/target/transactions/store_u64.mvt")[..])
        .unwrap();
    assert_eq!(tx.signers_count(), 1);
    let script = tx.into_script(vec![CORE_CODE_ADDRESS]).unwrap();
    CompiledScript::deserialize(script.code()).unwrap();
    assert_eq!(script.args(), &[Value::u64(13)][..]);
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

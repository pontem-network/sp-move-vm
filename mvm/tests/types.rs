use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{StructTag, TypeTag, CORE_CODE_ADDRESS};
use mvm::types::parse_type_params;

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

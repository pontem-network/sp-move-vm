use move_core_types::identifier::Identifier;
use parity_scale_codec::{Decode, Encode};

#[test]
pub fn test_identifier() {
    let ident = Identifier::new("Test_Ident").unwrap();
    let buffer = ident.encode();
    assert_eq!(ident, Identifier::decode(&mut buffer.as_ref()).unwrap())
}

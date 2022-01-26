use mvm::genesis::{init_storage, GenesisConfig};

use crate::common::contains_core_module;
use crate::common::mock::StorageMock;
use mvm::io::state::State;

mod common;

#[test]
fn test_genesis_success() {
    let store = StorageMock::new();
    init_storage(store.clone(), GenesisConfig::default()).unwrap();
    let state = State::new(store);

    // Move Stdlib.
    contains_core_module(&state, "ASCII");
    contains_core_module(&state, "BCS");
    contains_core_module(&state, "BitVector");
    contains_core_module(&state, "Capability");
    contains_core_module(&state, "Debug");
    contains_core_module(&state, "Errors");
    contains_core_module(&state, "Event");
    contains_core_module(&state, "FixedPoint32");
    contains_core_module(&state, "GUID");
    contains_core_module(&state, "Hash");
    contains_core_module(&state, "Option");
    contains_core_module(&state, "Reflect");
    contains_core_module(&state, "Signer");
    contains_core_module(&state, "Vector");

    // Pontem Stdlib.
    contains_core_module(&state, "ChainId");
    contains_core_module(&state, "CoreAddresses");
    contains_core_module(&state, "Genesis");
    contains_core_module(&state, "KSM");
    contains_core_module(&state, "NativeToken");
    contains_core_module(&state, "NOX");
    contains_core_module(&state, "PontAccount");
    contains_core_module(&state, "PontBlock");
    contains_core_module(&state, "PontTimestamp");
    contains_core_module(&state, "Signature");
    contains_core_module(&state, "Token");
}

#!/usr/bin/env bash

set -e
dove build
dove call "store_u64(13)"
dove call "tx_test<0x01::Pontem::T>(100)"
dove deploy  --modules_exclude "ReflectTest"
mv ./build/assets/bundles/assets.pac ./build/assets/bundles/valid_pack.pac
dove deploy --modules_exclude "Store" "ReflectTest"
mv ./build/assets/bundles/assets.pac ./build/assets/bundles/invalid_pack.pac

dove call "rt_signers(rt)"
dove call "signers_tr_with_user(root)"
dove call "Assets::ScriptBook::test"
dove call "signer_order"


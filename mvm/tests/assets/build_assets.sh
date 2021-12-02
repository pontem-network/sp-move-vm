#!/usr/bin/env bash
set -e
echo $HOME

dove clean --global

dove build
dove tx "store_u64(13)"
dove tx "tx_test<0x01::Pontem::T>(100)"
dove build -p -o "valid_pack"
dove build -p -o "invalid_pack" --modules_exclude "Store"

dove tx "rt_signers(rt)"
dove tx "tr_signers(tr)"
dove tx "tr_and_rt_signers(root, treasury)"
dove tx "signers_tr_and_rt_with_user(root, treasury)"
dove tx "0x1::ScriptBook::test"

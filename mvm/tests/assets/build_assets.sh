#!/usr/bin/env bash

 function lockfile_waithold()
 {
    declare -ir time_beg=$(date '+%s')
    declare -ir time_max=7140  # 7140 s = 1 hour 59 min.

    while ! \
       (set -o noclobber ; \
        echo -e "DATE:$(date)\nUSER:$(whoami)\nPID:$$" > /tmp/global.lock \
       ) 2>/dev/null
    do
        if [ $(($(date '+%s') - ${time_beg})) -gt ${time_max} ] ; then
            echo "Error: waited too long for lock file /tmp/global.lock" 1>&2
            return 1
        fi
        sleep 1
    done

    return 0
 }

 function lockfile_release()
 {
    rm -f /tmp/global.lock
 }

lockfile_waithold

set -e
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

lockfile_release


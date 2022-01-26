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
rm -rf pont-stdlib
git clone https://github.com/pontem-network/pont-stdlib.git
cd pont-stdlib
git reset --hard 1f094231de16cad54f2303093a7f866474bccd12
dove build -b
cd ..

rm -rf move-stdlib
git clone https://github.com/pontem-network/move-stdlib.git
cd move-stdlib
git reset --hard 12c5488729b8377b90f247537459f16ef1383d43
dove build -b
lockfile_release

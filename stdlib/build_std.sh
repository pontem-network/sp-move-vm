#!/usr/bin/env bash
rm -rf pont-stdlib
git clone https://github.com/pontem-network/pont-stdlib.git
cd pont-stdlib
git reset --hard release-v1.0.0
dove deploy
cd ..

rm -rf move-stdlib
git clone https://github.com/pontem-network/move-stdlib.git
cd move-stdlib
git reset --hard release-v1.0.0
dove deploy

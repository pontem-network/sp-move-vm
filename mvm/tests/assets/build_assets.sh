dove clean
dove build --tree -u
dove ct "store_u64(13)"
dove ct "tx_test<0x01::Pontem::T>(100)"
dove build --tree -p -o "stdlib" -e "Abort" "EventProxy" "Store" -u
dove build --tree -p -o "invalid_pack" -e "Abort" "EventProxy" "Store" "Event" -u
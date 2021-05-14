dove clean
dove build --tree -u
dove tx "store_u64(13)"
dove tx "tx_test<0x01::Pontem::T>(100)"
dove build --tree -p -o "stdlib" -e "Abort" "EventProxy" "Store" -u
dove build --tree -p -o "invalid_pack" -e "Abort" "Store" "Event" -u
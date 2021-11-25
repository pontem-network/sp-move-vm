script {
    use Assets::Store;
    use DiemFramework::DiemBlock;
    use DiemFramework::DiemTimestamp;

    fun store_system_resources(addr_for_block: signer, addr_for_timestamp: signer) {
        Store::store_u64(&addr_for_block, DiemBlock::get_current_block_height());
        Store::store_u64(&addr_for_timestamp, DiemTimestamp::now_microseconds());
    }
}
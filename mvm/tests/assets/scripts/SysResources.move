script {
    use Assets::Store;
    use PontemFramework::PontBlock;
    use PontemFramework::PontTimestamp;

    fun store_system_resources(addr_for_block: signer, addr_for_timestamp: signer) {
        Store::store_u64(&addr_for_block, PontBlock::get_current_block_height());
        Store::store_u64(&addr_for_timestamp, PontTimestamp::now_microseconds());
    }
}

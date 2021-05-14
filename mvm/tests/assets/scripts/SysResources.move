script {
    use 0x1::Store;
    use 0x1::Block;
    use 0x1::Time;

    fun store_system_resources(addr_for_block: signer, addr_for_timestamp: signer) {
        Store::store_u64(&addr_for_block, Block::get_current_block_height());
        Store::store_u64(&addr_for_timestamp, Time::now());
    }
}
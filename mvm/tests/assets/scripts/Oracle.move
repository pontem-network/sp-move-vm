script {
    use 0x1::PONT;
    use 0x1::Coins;
    use 0x1::Store;

    fun get_price_test(addr_for_eth_btc: &signer, addr_for_btc_pont: &signer) {
        Store::store_u128(addr_for_eth_btc, Coins::get_price<Coins::ETH, Coins::BTC>());
        Store::store_u128(addr_for_btc_pont, Coins::get_price<Coins::BTC, PONT::T>());
    }
}
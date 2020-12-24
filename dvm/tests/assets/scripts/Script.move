script {
    use 0x1::Store;

    fun main(account: &signer, val: u64) {
        Store::store_u64(account, val);
    }
}
module Assets::Foo {
    use Assets::Store;

    public fun foo(account: &signer, val: u64) {
        Store::store_u64(account, val);
    }
}

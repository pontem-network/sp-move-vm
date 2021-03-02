address 0x1 {

module Event {
    struct U64 {val: u64}

    public fun new_u64(val: u64): U64 {
        U64 {val: val}
    }

    native public fun emit<T: copyable>(account: &signer, msg: T);
}
}

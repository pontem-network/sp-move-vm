module Assets::EventProxy {
    use Std::Event;

    struct U64 has copy, key, store, drop { val: u64 }

    public fun emit_event(addr: &signer, val: u64) {
        let handle = Event::new_event_handle<U64>(addr);
        Event::emit_event(&mut handle, U64 { val });
        Event::destroy_handle(handle);
    }

    public fun create_val(val: u64): U64 {
        U64 { val }
    }

    public(script) fun test_only<T>(): (u64, U64) {
        abort 1
    }
}
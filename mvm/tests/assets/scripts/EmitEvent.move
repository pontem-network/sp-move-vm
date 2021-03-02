script {
    use 0x1::Event;

    fun emit_event(signer: &signer, val: u64) {
        Event::emit(signer, Event::new_u64(val));
    }
}
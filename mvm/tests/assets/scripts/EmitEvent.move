script {
    use 0x1::Event;
    use 0x1::EventProxy;

    fun emit_event(signer: signer, val: u64) {
        EventProxy::emit_event(&signer, val);
        Event::emit(&signer, EventProxy::create_val(val));
    }
}
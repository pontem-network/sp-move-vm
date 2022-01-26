module Assets::Abort {
    public fun error(code: u64) {
        abort code
    }
}

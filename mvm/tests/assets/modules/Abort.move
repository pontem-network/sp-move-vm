address 0x1 {
module Abort {
    public fun error(code: u64) {
        abort code
    }
}
}
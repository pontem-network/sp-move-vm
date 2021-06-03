pub fn stdlib_package() -> &'static [u8] {
    include_bytes!("../move-stdlib/target/packages/move-stdlib.pac")
}
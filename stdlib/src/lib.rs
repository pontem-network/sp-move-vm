#![no_std]

pub fn stdlib_package() -> &'static [u8] {
    include_bytes!("../pont-stdlib/artifacts/bundles/move-stdlib.pac")
}

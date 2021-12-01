#![no_std]

pub fn stdlib_package() -> &'static [u8] {
    include_bytes!("../move-stdlib/build/MoveStdlib/bundles/MoveStdlib.pac")
}

pub fn pont_stdlib_package() -> &'static [u8] {
    include_bytes!("../pont-stdlib/build/PontStdlib/bundles/PontStdlib.pac")
}

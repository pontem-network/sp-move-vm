#![no_std]

pub fn stdlib_package() -> &'static [u8] {
    include_bytes!("../move-stdlib/build/move-stdlib/bundles/move-stdlib.pac")
}

pub fn diem_stdlib_package() -> &'static [u8] {
    include_bytes!("../diem-stdlib/build/DiemStdlib/bundles/DiemStdlib.pac")
}
//! Building this module successfully guarantees that the byte-struct library is no-std compatible
//! and that (at least the u128) impl of ByteStruct produces no panic branches (panic-never compatible)

#![no_std]
#![no_main]

use panic_never as _;

use byte_struct::*;

#[derive(ByteStruct)]
#[byte_struct_be]
struct Dummy {
    v1: u64,
    v2: u16
}

#[no_mangle]
pub fn _start() -> ! {
    let v = Dummy{v1: 3, v2: 8};
    let mut bytes = [0_u8; Dummy::BYTE_LEN];
    v.write_bytes(&mut bytes);
    let v_parsed = Dummy::read_bytes(&bytes);

    loop {}
}

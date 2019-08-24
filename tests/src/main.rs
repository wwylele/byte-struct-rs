use byte_struct::*;
use generic_array::*;

bitfields!(
    #[derive(PartialEq, Debug)]
    TestBitfield: u16 {
        x: 4,
        pub y: 8,
        z: 4,
    }
);

#[derive(ByteStruct, PartialEq, Debug)]
#[byte_struct_be]
struct TestSubStruct1 {
    b: u16,
    c: [TestBitfield; 2],
}

#[derive(ByteStruct, PartialEq, Debug)]
struct TestSubStruct2 {
    #[byte_struct_le] u: u32,
    #[byte_struct_be] v: u16,
}

#[derive(ByteStruct, PartialEq, Debug)]
struct TestSubStruct3 {
    s1: TestSubStruct1,
    s2: TestSubStruct2,
}

#[derive(ByteStruct, PartialEq, Debug)]
#[byte_struct_le]
struct TestStruct {
    a: u8,
    s: TestSubStruct3,
    d: GenericArray<u16, typenum::U3>,
    e: u32,
    f: u64,
    #[byte_struct_be] g: u128,
    h: f32,
    i: f64,
}

#[test]
fn main() {
    assert_eq!(TestStruct::BYTE_LEN, 59);
    let mut data = [0; TestStruct::BYTE_LEN];
    let s = TestStruct {
        a: 0x12,
        s: TestSubStruct3 {
            s1: TestSubStruct1 {
                b: 0x3456,
                c: [
                    TestBitfield {
                        x: 0xf,
                        y: 0x8f,
                        z: 0x7
                    },
                    TestBitfield {
                        x: 0x1,
                        y: 0x23,
                        z: 0x4
                    },
                ],
            },
            s2: TestSubStruct2 {
                u: 0x90807060,
                v: 0x5040
            },
        },
        d: *GenericArray::from_slice(&[0x1020, 0x3040, 0x5060]),
        e: 0x9abcdef0,
        f: 0x0123456789ABCDEF,
        g: 0xffeeddccbbaa99887766554433221100,
        h: 1.2345,
        i: 3.14159
    };
    s.write_bytes(&mut data[..]);
    assert_eq!(&data[..], &[
        0x12,
            0x34, 0x56,
            0x78, 0xff,
            0x42, 0x31,
            0x60, 0x70, 0x80, 0x90,
            0x50, 0x40,
        0x20, 0x10, 0x40, 0x30, 0x60, 0x50,
        0xf0, 0xde, 0xbc, 0x9a,
        0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01,
        0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa, 0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x00,
        0x19, 0x04, 0x9e, 0x3f,
        0x6e, 0x86, 0x1b, 0xf0, 0xf9, 0x21, 0x09, 0x40,
    ][..]);

    let data = [
        0x00,
            0x11, 0x22,
            0x33, 0x44,
            0x17, 0x28,
            0x76, 0x54, 0x32, 0x10,
            0xfe, 0xdc,
        0x44, 0x55, 0x66, 0x77, 0x88, 0x99,
        0xaa, 0xbb, 0xcc, 0xdd,
        0x10, 0x20, 0x30, 0x40, 0x51, 0x61, 0x71, 0x81,
        0x11, 0x12, 0x13, 0x14, 0x25, 0x26, 0x27, 0x28, 0x31, 0x32, 0x33, 0x34, 0x45, 0x46, 0x47, 0x48,
        0xcd, 0xcc, 0xf6, 0x42,
        0x58, 0x39, 0xb4, 0xc8, 0x76, 0xbe, 0x05, 0x40,
    ];
    let s = TestStruct::read_bytes(&data[..]);
    assert_eq!(s, TestStruct {
        a: 0x00,
        s: TestSubStruct3 {
            s1: TestSubStruct1 {
                b: 0x1122,
                c: [
                    TestBitfield {
                        x: 0x4,
                        y: 0x34,
                        z: 0x3
                    },
                    TestBitfield {
                        x: 0x8,
                        y: 0x72,
                        z: 0x1
                    },
                ]
            },
            s2: TestSubStruct2 {
                u: 0x10325476,
                v: 0xfedc,
            },
        },
        d: *GenericArray::from_slice(&[0x5544, 0x7766, 0x9988]),
        e: 0xddccbbaa,
        f: 0x8171615140302010,
        g: 0x11121314252627283132333445464748,
        h: 123.4,
        i: 2.718,
    })
}

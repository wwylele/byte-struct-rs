use byte_struct::*;

bitfields!(
    TestBitfield: u16 {
        x: 4,
        y: 8,
        z: 4,
    }
);

#[derive(ByteStructBE, PartialEq, Debug)]
struct TestSubStruct {
    b: u16,
    c: TestBitfield,
}

#[derive(ByteStructLE, PartialEq, Debug)]
struct TestStruct {
    a: u8,
    s: TestSubStruct,
    d: [u16; 3],
    e: u32,
}

#[test]
fn main() {
    assert_eq!(TestStruct::BYTE_LEN, 15);
    let mut data = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut s = TestStruct {
        a: 0x12,
        s: TestSubStruct {
            b: 0x3456,
            c: TestBitfield {
                x: 0xf,
                y: 0x8f,
                z: 0x7
            },
        },
        d: [0x1020, 0x3040, 0x5060],
        e: 0x9abcdef0,
    };
    s.write_bytes(&mut data[..]);
    assert_eq!(data, [0x12, 0x34, 0x56, 0x78, 0xff, 0x20, 0x10, 0x40, 0x30, 0x60, 0x50, 0xf0, 0xde, 0xbc, 0x9a]);

    data = [0x00, 0x11, 0x22, 0x33, 0x44, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd];
    s = TestStruct::read_bytes(&data[..]);
    assert_eq!(s, TestStruct {
        a: 0x00,
        s: TestSubStruct {
            b: 0x1122,
            c: TestBitfield {
                x: 0x4,
                y: 0x34,
                z: 0x3
            },
        },
        d: [0x5544, 0x7766, 0x9988],
        e: 0xddccbbaa,
    })
}

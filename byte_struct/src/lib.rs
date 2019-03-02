pub use byte_struct_derive::{ByteStructLE, ByteStructBE};

pub trait ByteStruct {
    fn write_bytes(&self, bytes: &mut [u8]);
    fn read_bytes(bytes: &[u8]) -> Self;
}

pub trait ByteStructImpl {
    const BYTE_LEN: usize;
    fn write_le_bytes(&self, bytes: &mut [u8]);
    fn read_le_bytes(bytes: &[u8]) -> Self;
    fn write_be_bytes(&self, bytes: &mut [u8]);
    fn read_be_bytes(bytes: &[u8]) -> Self;
}

impl ByteStructImpl for u8 {
    const BYTE_LEN: usize = 1;
    fn write_le_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_le_bytes()[..]);
    }
    fn read_le_bytes(bytes: &[u8]) -> Self {
        u8::from_le_bytes([bytes[0]])
    }
    fn write_be_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_be_bytes()[..]);
    }
    fn read_be_bytes(bytes: &[u8]) -> Self {
        u8::from_be_bytes([bytes[0]])
    }
}

impl ByteStructImpl for i8 {
    const BYTE_LEN: usize = 1;
    fn write_le_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_le_bytes()[..]);
    }
    fn read_le_bytes(bytes: &[u8]) -> Self {
        i8::from_le_bytes([bytes[0]])
    }
    fn write_be_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_be_bytes()[..]);
    }
    fn read_be_bytes(bytes: &[u8]) -> Self {
        i8::from_be_bytes([bytes[0]])
    }
}

impl ByteStructImpl for u16 {
    const BYTE_LEN: usize = 2;
    fn write_le_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_le_bytes()[..]);
    }
    fn read_le_bytes(bytes: &[u8]) -> Self {
        u16::from_le_bytes([bytes[0], bytes[1]])
    }
    fn write_be_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_be_bytes()[..]);
    }
    fn read_be_bytes(bytes: &[u8]) -> Self {
        u16::from_be_bytes([bytes[0], bytes[1]])
    }
}

impl ByteStructImpl for i16 {
    const BYTE_LEN: usize = 2;
    fn write_le_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_le_bytes()[..]);
    }
    fn read_le_bytes(bytes: &[u8]) -> Self {
        i16::from_le_bytes([bytes[0], bytes[1]])
    }
    fn write_be_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_be_bytes()[..]);
    }
    fn read_be_bytes(bytes: &[u8]) -> Self {
        i16::from_be_bytes([bytes[0], bytes[1]])
    }
}

impl ByteStructImpl for u32 {
    const BYTE_LEN: usize = 4;
    fn write_le_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_le_bytes()[..]);
    }
    fn read_le_bytes(bytes: &[u8]) -> Self {
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
    fn write_be_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_be_bytes()[..]);
    }
    fn read_be_bytes(bytes: &[u8]) -> Self {
        u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

impl ByteStructImpl for i32 {
    const BYTE_LEN: usize = 4;
    fn write_le_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_le_bytes()[..]);
    }
    fn read_le_bytes(bytes: &[u8]) -> Self {
        i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
    fn write_be_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_be_bytes()[..]);
    }
    fn read_be_bytes(bytes: &[u8]) -> Self {
        i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

impl ByteStructImpl for u64 {
    const BYTE_LEN: usize = 8;
    fn write_le_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_le_bytes()[..]);
    }
    fn read_le_bytes(bytes: &[u8]) -> Self {
        u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7]])
    }
    fn write_be_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_be_bytes()[..]);
    }
    fn read_be_bytes(bytes: &[u8]) -> Self {
        u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7]])
    }
}

impl ByteStructImpl for i64 {
    const BYTE_LEN: usize = 8;
    fn write_le_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_le_bytes()[..]);
    }
    fn read_le_bytes(bytes: &[u8]) -> Self {
        i64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7]])
    }
    fn write_be_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_be_bytes()[..]);
    }
    fn read_be_bytes(bytes: &[u8]) -> Self {
        i64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7]])
    }
}

impl ByteStructImpl for u128 {
    const BYTE_LEN: usize = 16;
    fn write_le_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_le_bytes()[..]);
    }
    fn read_le_bytes(bytes: &[u8]) -> Self {
        u128::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11],
            bytes[12], bytes[13], bytes[14], bytes[15],
            ])
    }
    fn write_be_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_be_bytes()[..]);
    }
    fn read_be_bytes(bytes: &[u8]) -> Self {
        u128::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11],
            bytes[12], bytes[13], bytes[14], bytes[15]])
    }
}

impl ByteStructImpl for i128 {
    const BYTE_LEN: usize = 16;
    fn write_le_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_le_bytes()[..]);
    }
    fn read_le_bytes(bytes: &[u8]) -> Self {
        i128::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11],
            bytes[12], bytes[13], bytes[14], bytes[15]])
    }
    fn write_be_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_be_bytes()[..]);
    }
    fn read_be_bytes(bytes: &[u8]) -> Self {
        i128::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11],
            bytes[12], bytes[13], bytes[14], bytes[15]])
    }
}

impl ByteStructImpl for f32 {
    const BYTE_LEN: usize = 4;
    fn write_le_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_bits().to_le_bytes()[..]);
    }
    fn read_le_bytes(bytes: &[u8]) -> Self {
        f32::from_bits(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
    fn write_be_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_bits().to_be_bytes()[..]);
    }
    fn read_be_bytes(bytes: &[u8]) -> Self {
        f32::from_bits(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
}

impl ByteStructImpl for f64 {
    const BYTE_LEN: usize = 8;
    fn write_le_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_bits().to_le_bytes()[..]);
    }
    fn read_le_bytes(bytes: &[u8]) -> Self {
        f64::from_bits(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7]]))
    }
    fn write_be_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.clone().to_bits().to_be_bytes()[..]);
    }
    fn read_be_bytes(bytes: &[u8]) -> Self {
        f64::from_bits(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7]]))
    }
}

macro_rules! byte_struct_array {
    ($x:expr) => {
        impl<T: ByteStructImpl + Copy + Default> ByteStructImpl for [T; $x] {
            const BYTE_LEN: usize = ($x) * T::BYTE_LEN;
            fn write_le_bytes(&self, bytes: &mut [u8]) {
                let mut pos = 0;
                let len = T::BYTE_LEN;
                for i in 0 .. ($x) {
                    self[i].write_le_bytes(&mut bytes[pos .. pos + len]);
                    pos += len;
                }
            }
            fn read_le_bytes(bytes: &[u8]) -> Self {
                let mut pos = 0;
                let len = T::BYTE_LEN;
                let mut result = [T::default(); $x];
                for i in 0 .. ($x) {
                    result[i] = <T>::read_le_bytes(&bytes[pos .. pos + len]);
                    pos += len;
                }
                result
            }
            fn write_be_bytes(&self, bytes: &mut [u8]) {
                let mut pos = 0;
                let len = T::BYTE_LEN;
                for i in 0 .. ($x) {
                    self[i].write_be_bytes(&mut bytes[pos .. pos + len]);
                    pos += len;
                }
            }
            fn read_be_bytes(bytes: &[u8]) -> Self {
                let mut pos = 0;
                let len = T::BYTE_LEN;
                let mut result = [T::default(); $x];
                for i in 0 .. ($x) {
                    result[i] = <T>::read_be_bytes(&bytes[pos .. pos + len]);
                    pos += len;
                }
                result
            }
        }
    }
}

macro_rules! bsa0 { ($x:expr) => { byte_struct_array!($x);}}
macro_rules! bsa1 { ($x:expr) => { bsa0!($x); bsa0!(1 + $x);}}
macro_rules! bsa2 { ($x:expr) => { bsa1!($x); bsa1!(2 + $x);}}
macro_rules! bsa3 { ($x:expr) => { bsa2!($x); bsa2!(4 + $x);}}
macro_rules! bsa4 { ($x:expr) => { bsa3!($x); bsa3!(8 + $x);}}
macro_rules! bsa5 { ($x:expr) => { bsa4!($x); bsa4!(16 + $x);}}
bsa5!(1);

byte_struct_array!(100);
byte_struct_array!(3000);

#[macro_export]
macro_rules! bitfields{
    (
        $(#[$outer:meta])*
        $visibility:vis $name:ident : $base:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                $field_vis:vis $field_name:ident : $field_len:expr
            ),+ $(,)?
        }
    ) => {
        $(#[$outer])*
        $visibility struct $name {
            $(
                $(#[$inner $($args)*])*
                $field_vis $field_name: $base
            ),*
        }

        impl $name {
            #[allow(unused_assignments)]
            fn from_raw(raw: $base) -> $name {
                let mut raw_v = raw;
                $(
                    let mask: $base = (1 << $field_len) - 1;
                    let $field_name = raw_v & mask;
                    raw_v >>= $field_len;
                )*
                $name{$($field_name),*}
            }
            #[allow(unused_assignments)]
            fn to_raw(&self) -> $base {
                let mut raw: $base = 0;
                let mut pos = 0;
                $(
                    raw |= self.$field_name << pos;
                    pos += $field_len;
                )*
                raw
            }
        }

        impl ByteStructImpl for $name {
            const BYTE_LEN: usize = <$base>::BYTE_LEN;
            fn write_le_bytes(&self, bytes: &mut [u8]) {
                self.to_raw().write_le_bytes(bytes);
            }
            fn read_le_bytes(bytes: &[u8]) -> Self {
                <$name>::from_raw(<$base>::read_le_bytes(bytes))
            }
            fn write_be_bytes(&self, bytes: &mut [u8]) {
                self.to_raw().write_be_bytes(bytes);
            }
            fn read_be_bytes(bytes: &[u8]) -> Self {
                <$name>::from_raw(<$base>::read_be_bytes(bytes))
            }
        }
    }
}

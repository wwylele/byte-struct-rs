//! # Byte Struct
//!
//! Pack and unpack structure as raw bytes with packed or bit field layout.
//!
//! ## Example
//! ```
//! use byte_struct::*;
//!
//! bitfields!(
//!     #[derive(PartialEq, Debug)]
//!     GIFColorTableInfo: u8 {
//!         global_color_table_flag: 1,
//!         color_resolution: 3,
//!         sort_flag: 1,
//!         global_color_table_size: 3,
//!     }
//! );
//!
//! #[derive(ByteStruct, PartialEq, Debug)]
//! #[byte_struct_le]
//! struct GIFLogicalScreenDescriptor {
//!     width: u16,
//!     height: u16,
//!     color_table_info: GIFColorTableInfo,
//!     background_color_index: u8,
//!     pixel_aspect_ratio: u8,
//! }
//!
//! fn example() {
//!     assert_eq!(GIFLogicalScreenDescriptor::BYTE_LEN, 7);
//!     let raw_descriptor = [0x03, 0x00, 0x05, 0x00, 0xF7, 0x00, 0x00];
//!     let descriptor = GIFLogicalScreenDescriptor::read_bytes(&raw_descriptor[..]);
//!     assert_eq!(descriptor, GIFLogicalScreenDescriptor{
//!         width: 3,
//!         height: 5,
//!         color_table_info: GIFColorTableInfo {
//!             global_color_table_flag: 1,
//!             color_resolution: 3,
//!             sort_flag: 1,
//!             global_color_table_size: 7,
//!         },
//!         background_color_index: 0,
//!         pixel_aspect_ratio: 0,
//!     });
//!     let mut raw_another = [0; GIFLogicalScreenDescriptor::BYTE_LEN];
//!     descriptor.write_bytes(&mut raw_another[..]);
//!     assert_eq!(raw_descriptor, raw_another);
//! }
//! ```

#![no_std]

pub use byte_struct_derive::{ByteStruct, ByteStructBE, ByteStructLE};

/// A type that can be packed into or unpacked from fixed-size bytes, but the method is unknown yet.
pub trait ByteStructLen {
    /// The length of the packed bytes of this type
    const BYTE_LEN: usize;
}

/// A data structure that can be packed into or unpacked from raw bytes.
///
/// This trait can be derived by
/// [`#[derive(ByteStruct)]`](https://docs.rs/byte_struct_derive/*/byte_struct_derive/derive.ByteStruct.html).
///
/// One can implement this trait for custom types in order to pack or unpack an object in a special way.
pub trait ByteStruct: ByteStructLen {
    /// Packs the struct into raw bytes and write to a slice
    fn write_bytes(&self, bytes: &mut [u8]);

    /// Unpacks raw bytes from a slice into a new struct
    fn read_bytes(bytes: &[u8]) -> Self;
}

/// A type that can be packed into or unpacked from raw bytes under given default byte order.
///
/// This trait is implemented for most numeric primitive types,
/// except for `bool`, `char`, `isize` and `usize`. This is also implemented for array types
/// whose element type implements `ByteStructUnspecifiedByteOrder`.
///
/// This trait is automatically implemented for all types that implements [`ByteStruct`].
/// In this case, all members of `ByteStructUnspecifiedByteOrder` are direct wrappers of [`ByteStruct`] members.
///
/// Members in this trait are meant to be called by byte_struct internal only.
/// They do not do what one might expect:
/// the byte orders specified in `read_bytes_default_*` / `write_bytes_default_*` functions
/// are only **default byte orders**.
/// The default byte order is only respected when the type itself does not carry byte order specification
/// (e.g. primitive types).
/// In contrast, since [`ByteStruct`] types always have fixed packing method,
/// the default byte order has no effect on them, and the three versions of read / write functions for them,
/// `_default_le`, `_default_be` and no-spec from [`ByteStruct`], behave exactly the same.
///
/// One can implement this trait for custom types in order to pack or unpack an object in a special way,
/// but only when the said type changes its packing method depending on the default byte order.
/// An example for this is a custom fixed-size large integer type.
/// If the packing method is independent from the default byte order, please implement [`ByteStruct`] instead.
///
/// [`ByteStruct`]: trait.ByteStruct.html
pub trait ByteStructUnspecifiedByteOrder: ByteStructLen {
    /// Packs the object into raw bytes with little-endian as the default byte order
    fn write_bytes_default_le(&self, bytes: &mut [u8]);

    /// Unpacks raw bytes into a new object with little-endian as the default byte order
    fn read_bytes_default_le(bytes: &[u8]) -> Self;

    /// Packs the object into raw bytes with big-endian as the default byte order
    fn write_bytes_default_be(&self, bytes: &mut [u8]);

    /// Unpacks raw bytes into a new object with big-endian as the default byte order
    fn read_bytes_default_be(bytes: &[u8]) -> Self;
}

impl<T: ByteStruct> ByteStructUnspecifiedByteOrder for T {
    /// A wrapper of [`ByteStruct::write_bytes`](trait.ByteStruct.html#tymethod.write_bytes)
    fn write_bytes_default_le(&self, bytes: &mut [u8]) {
        self.write_bytes(bytes);
    }

    /// A wrapper of [`ByteStruct::read_bytes`](trait.ByteStruct.html#tymethod.read_bytes)
    fn read_bytes_default_le(bytes: &[u8]) -> Self {
        Self::read_bytes(bytes)
    }

    /// A wrapper of [`ByteStruct::write_bytes`](trait.ByteStruct.html#tymethod.write_bytes)
    fn write_bytes_default_be(&self, bytes: &mut [u8]) {
        self.write_bytes(bytes);
    }

    /// A wrapper of [`ByteStruct::read_bytes`](trait.ByteStruct.html#tymethod.read_bytes)
    fn read_bytes_default_be(bytes: &[u8]) -> Self {
        Self::read_bytes(bytes)
    }
}

impl ByteStructLen for u8 {
    const BYTE_LEN: usize = 1;
}

impl ByteStructUnspecifiedByteOrder for u8 {
    fn write_bytes_default_le(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_le_bytes()[..]);
    }
    fn read_bytes_default_le(bytes: &[u8]) -> Self {
        u8::from_le_bytes([bytes[0]])
    }
    fn write_bytes_default_be(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_be_bytes()[..]);
    }
    fn read_bytes_default_be(bytes: &[u8]) -> Self {
        u8::from_be_bytes([bytes[0]])
    }
}

impl ByteStructLen for i8 {
    const BYTE_LEN: usize = 1;
}

impl ByteStructUnspecifiedByteOrder for i8 {
    fn write_bytes_default_le(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_le_bytes()[..]);
    }
    fn read_bytes_default_le(bytes: &[u8]) -> Self {
        i8::from_le_bytes([bytes[0]])
    }
    fn write_bytes_default_be(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_be_bytes()[..]);
    }
    fn read_bytes_default_be(bytes: &[u8]) -> Self {
        i8::from_be_bytes([bytes[0]])
    }
}

impl ByteStructLen for u16 {
    const BYTE_LEN: usize = 2;
}

impl ByteStructUnspecifiedByteOrder for u16 {
    fn write_bytes_default_le(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_le_bytes()[..]);
    }
    fn read_bytes_default_le(bytes: &[u8]) -> Self {
        u16::from_le_bytes([bytes[0], bytes[1]])
    }
    fn write_bytes_default_be(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_be_bytes()[..]);
    }
    fn read_bytes_default_be(bytes: &[u8]) -> Self {
        u16::from_be_bytes([bytes[0], bytes[1]])
    }
}

impl ByteStructLen for i16 {
    const BYTE_LEN: usize = 2;
}

impl ByteStructUnspecifiedByteOrder for i16 {
    fn write_bytes_default_le(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_le_bytes()[..]);
    }
    fn read_bytes_default_le(bytes: &[u8]) -> Self {
        i16::from_le_bytes([bytes[0], bytes[1]])
    }
    fn write_bytes_default_be(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_be_bytes()[..]);
    }
    fn read_bytes_default_be(bytes: &[u8]) -> Self {
        i16::from_be_bytes([bytes[0], bytes[1]])
    }
}

impl ByteStructLen for u32 {
    const BYTE_LEN: usize = 4;
}

impl ByteStructUnspecifiedByteOrder for u32 {
    fn write_bytes_default_le(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_le_bytes()[..]);
    }
    fn read_bytes_default_le(bytes: &[u8]) -> Self {
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
    fn write_bytes_default_be(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_be_bytes()[..]);
    }
    fn read_bytes_default_be(bytes: &[u8]) -> Self {
        u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

impl ByteStructLen for i32 {
    const BYTE_LEN: usize = 4;
}

impl ByteStructUnspecifiedByteOrder for i32 {
    fn write_bytes_default_le(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_le_bytes()[..]);
    }
    fn read_bytes_default_le(bytes: &[u8]) -> Self {
        i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
    fn write_bytes_default_be(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_be_bytes()[..]);
    }
    fn read_bytes_default_be(bytes: &[u8]) -> Self {
        i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

impl ByteStructLen for u64 {
    const BYTE_LEN: usize = 8;
}

impl ByteStructUnspecifiedByteOrder for u64 {
    fn write_bytes_default_le(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_le_bytes()[..]);
    }
    fn read_bytes_default_le(bytes: &[u8]) -> Self {
        u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ])
    }
    fn write_bytes_default_be(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_be_bytes()[..]);
    }
    fn read_bytes_default_be(bytes: &[u8]) -> Self {
        u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ])
    }
}

impl ByteStructLen for i64 {
    const BYTE_LEN: usize = 8;
}

impl ByteStructUnspecifiedByteOrder for i64 {
    fn write_bytes_default_le(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_le_bytes()[..]);
    }
    fn read_bytes_default_le(bytes: &[u8]) -> Self {
        i64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ])
    }
    fn write_bytes_default_be(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_be_bytes()[..]);
    }
    fn read_bytes_default_be(bytes: &[u8]) -> Self {
        i64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ])
    }
}

impl ByteStructLen for u128 {
    const BYTE_LEN: usize = 16;
}

impl ByteStructUnspecifiedByteOrder for u128 {
    fn write_bytes_default_le(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_le_bytes()[..]);
    }
    fn read_bytes_default_le(bytes: &[u8]) -> Self {
        u128::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
        ])
    }
    fn write_bytes_default_be(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_be_bytes()[..]);
    }
    fn read_bytes_default_be(bytes: &[u8]) -> Self {
        u128::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
        ])
    }
}

impl ByteStructLen for i128 {
    const BYTE_LEN: usize = 16;
}

impl ByteStructUnspecifiedByteOrder for i128 {
    fn write_bytes_default_le(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_le_bytes()[..]);
    }
    fn read_bytes_default_le(bytes: &[u8]) -> Self {
        i128::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
        ])
    }
    fn write_bytes_default_be(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_be_bytes()[..]);
    }
    fn read_bytes_default_be(bytes: &[u8]) -> Self {
        i128::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
        ])
    }
}

impl ByteStructLen for f32 {
    const BYTE_LEN: usize = 4;
}

impl ByteStructUnspecifiedByteOrder for f32 {
    fn write_bytes_default_le(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_bits().to_le_bytes()[..]);
    }
    fn read_bytes_default_le(bytes: &[u8]) -> Self {
        f32::from_bits(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
    fn write_bytes_default_be(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_bits().to_be_bytes()[..]);
    }
    fn read_bytes_default_be(bytes: &[u8]) -> Self {
        f32::from_bits(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
}

impl ByteStructLen for f64 {
    const BYTE_LEN: usize = 8;
}

impl ByteStructUnspecifiedByteOrder for f64 {
    fn write_bytes_default_le(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_bits().to_le_bytes()[..]);
    }
    fn read_bytes_default_le(bytes: &[u8]) -> Self {
        f64::from_bits(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }
    fn write_bytes_default_be(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.to_bits().to_be_bytes()[..]);
    }
    fn read_bytes_default_be(bytes: &[u8]) -> Self {
        f64::from_bits(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }
}

impl<T: ByteStructLen, const N: usize> ByteStructLen for [T; N] {
    const BYTE_LEN: usize = N * T::BYTE_LEN;
}

impl<T: ByteStructUnspecifiedByteOrder, const N: usize> ByteStructUnspecifiedByteOrder for [T; N] {
    fn write_bytes_default_le(&self, bytes: &mut [u8]) {
        let mut pos = 0;
        let len = T::BYTE_LEN;
        for element in self {
            element.write_bytes_default_le(&mut bytes[pos..pos + len]);
            pos += len;
        }
    }
    fn read_bytes_default_le(bytes: &[u8]) -> Self {
        let len = T::BYTE_LEN;
        array_init::array_init(|i| <T>::read_bytes_default_le(&bytes[i * len..(i + 1) * len]))
    }
    fn write_bytes_default_be(&self, bytes: &mut [u8]) {
        let mut pos = 0;
        let len = T::BYTE_LEN;
        for element in self {
            element.write_bytes_default_be(&mut bytes[pos..pos + len]);
            pos += len;
        }
    }
    fn read_bytes_default_be(bytes: &[u8]) -> Self {
        let len = T::BYTE_LEN;
        array_init::array_init(|i| <T>::read_bytes_default_be(&bytes[i * len..(i + 1) * len]))
    }
}

/// Generates a structure that implements [`ByteStructUnspecifiedByteOrder`] with bit field semantics.
///
/// The bit fields are packed to / unpacked from the base integer type,
/// which is then packed / unpacked using the primitive type's [`ByteStructUnspecifiedByteOrder`] implementation.
/// Therefore, the byte order of bit fields is unspecified internally, and is only specified
/// by the parent structure that derives [`ByteStruct`](trait.ByteStruct.html), just like all primitive
/// types.
///
/// Note that the memory representation of the generated structure during runtime is NOT in bit field layout.
/// This macro only provides conversion method between the plain structure and the bit-field-packed bytes.
///
/// [`ByteStructUnspecifiedByteOrder`]: trait.ByteStructUnspecifiedByteOrder.html
///
/// # Example
/// ```ignore
/// bitfields!(
///     // Specifies the struct name and the base type.
///     // The base type must be one of unsigned integer types.
///     // Attributes and visibility specifier can be attached before the struct name.
///     #[derive(PartialEq, Debug)]
///     SampleBitField: u16 {
///         // Specifies members and bit length from the least significant bit to the most.
///         // The bit layout is assumed packed, and paddings must be explicitly specified.
///         // The sum of bit length of all fields must equal the bit length of the base type.
///         // Attributes and visibility specifier can be attached before the field name.
///
///         // This creates bit field structure in the following layout:
///         //
///         // | MSB                                                        LSB |
///         // | 15| 14| 13| 12| 11| 10| 9 | 8 | 7 |  6 | 5 | 4 | 3 | 2 | 1 | 0 |
///         // |     z     |pad|               y                |       x       |
///         //
///         pub x: 4,
///         pub y: 8,
///         padding: 1,
///         pub z: 3,
///     }
/// );
///
/// // The macro above generates the structure below.
///
/// #[derive(PartialEq, Debug)]
/// struct SampleBitField {
///     pub x: u16,
///     pub y: u16,
///     padding: u16,
///     pub z: u16,
/// }
///
/// impl ByteStructUnspecifiedByteOrder for SampleBitField {
///     ...
/// }
/// ```
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

        impl ByteStructLen for $name {
            const BYTE_LEN: usize = <$base>::BYTE_LEN;
        }

        impl ByteStructUnspecifiedByteOrder for $name {
            fn write_bytes_default_le(&self, bytes: &mut [u8]) {
                self.to_raw().write_bytes_default_le(bytes);
            }
            fn read_bytes_default_le(bytes: &[u8]) -> Self {
                <$name>::from_raw(<$base>::read_bytes_default_le(bytes))
            }
            fn write_bytes_default_be(&self, bytes: &mut [u8]) {
                self.to_raw().write_bytes_default_be(bytes);
            }
            fn read_bytes_default_be(bytes: &[u8]) -> Self {
                <$name>::from_raw(<$base>::read_bytes_default_be(bytes))
            }
        }
    }
}

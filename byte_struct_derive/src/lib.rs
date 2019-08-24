//! # Derive macro for Byte Struct
//!
//! This crate provides macros for deriving the [`ByteStruct`] trait
//! defined in the [`byte_struct` crate](https://docs.rs/byte_struct).
//!
//! See [`#[derive(ByteStruct)]`](derive.ByteStruct.html) for using the macro.
//!
//! [`ByteStruct`]: https://docs.rs/byte_struct/*/byte_struct/trait.ByteStruct.html

#![recursion_limit = "128"]
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

#[derive(Clone, Copy)]
enum Endianness {
    Little,
    Big,
    Unspecified,
}

/// Derives trait [`ByteStruct`] for a data structure.
///
/// Requires all members to implement [`ByteStructUnspecifiedByteOrder`].
/// This includes most primitive types and nested structures with [`ByteStruct`] derived
/// (because [`ByteStructUnspecifiedByteOrder`] is automatically implemented for [`ByteStruct`] types)
///
/// Byte order attributes `#[byte_struct_le]` or `#[byte_struct_be]` can be attached to individual fields
/// and/or the entire structure.
///
/// When a byte order attribute are attached to a field, it selects which byte order version
/// of [`ByteStructUnspecifiedByteOrder`] member functions to use on the field
/// In other words, the attribute specifies the byte order on an byte-order-unspecified type.
/// These attributes have no effect on fields that implements [`ByteStruct`],
/// because they always have the same byte packing method regardless of externally specified byte order.
///
/// When a byte order attribute is attached to the entire struct,
/// it works as if it is attached to all fields that don't have byte order attributes.
///
/// If a field has no byte order attribute specified
/// (either explicitly attached to the field or implicitly by the attribute on the entire structure),
/// it must implement [`ByteStruct`] as well, so that its packing method is not byte-order-dependent.
/// This is true for all `ByteStruct`-derived structures, but not for primitive types.
///
/// [`ByteStruct`]: https://docs.rs/byte_struct/*/byte_struct/trait.ByteStruct.html
/// [`ByteStructUnspecifiedByteOrder`]: https://docs.rs/byte_struct/*/byte_struct/trait.ByteStructUnspecifiedByteOrder.html
///
/// ## Example
/// ```ignore
/// #[derive(ByteStruct)]
/// #[byte_struct_le]
/// struct Struct1 {
///     // Packed as little-endian.
///     a: u32,
///
///     // Packed as little-endian as well.
///     // Redundant attributes doesn't hurt.
///     #[byte_struct_le]
///     b: i16,
///
///     // Packed as big-endian.
///     // Attributes on fields override top-level attributes.
///     #[byte_struct_be]
///     c: u16,
/// }
///
/// // This struct has no top-level byte order attribute
/// #[derive(ByteStruct)]
/// struct Struct2 {
///     // Packed as big-endian.
///     // If the attribute is missing here, it won't compile.
///     #[byte_struct_be]
///     d: i64,
///
///     // Packed as little-endian.
///     #[byte_struct_le]
///     e: f32,
/// }
///
/// // This struct has no top-level byte order attribute either
/// #[derive(ByteStruct)]
/// struct Struct3 {
///     // Nested structures don't need attributes.
///     f: Struct1,
///
///     // Even if you give one, it has no effect.
///     // The endianness of fields inside Struct2 still remain as specified above.
///     #[byte_struct_le]
///     g: Struct2,
/// }
/// ```
#[proc_macro_derive(ByteStruct, attributes(byte_struct_le, byte_struct_be))]
pub fn byte_struct_macro_derive(input: TokenStream) -> TokenStream {
    byte_struct_macro_derive_impl(input, Endianness::Unspecified)
}

/// Same effect as [`#[derive(ByteStruct)] #[byte_struct_le]`](derive.ByteStruct.html)
///
/// But doesn't support byte order attributes on fields
#[deprecated]
#[proc_macro_derive(ByteStructLE)]
pub fn byte_struct_le_macro_derive(input: TokenStream) -> TokenStream {
    byte_struct_macro_derive_impl(input, Endianness::Little)
}

/// Same effect as [`#[derive(ByteStruct)] #[byte_struct_be]`](derive.ByteStruct.html)
///
/// But doesn't support byte order attributes on fields
#[deprecated]
#[proc_macro_derive(ByteStructBE)]
pub fn byte_struct_be_macro_derive(input: TokenStream) -> TokenStream {
    byte_struct_macro_derive_impl(input, Endianness::Big)
}

fn byte_struct_macro_derive_impl(input: TokenStream, endianness_input: Endianness) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let mut found_le = false;
    let mut found_be = false;
    for syn::Attribute{path: syn::Path{segments, ..}, ..} in ast.attrs {
        if segments.len() != 1 {
            continue;
        }
        match segments[0].ident.to_string().as_str() {
            "byte_struct_le" => found_le = true,
            "byte_struct_be" => found_be = true,
            _ => ()
        };
    }
    if found_be && found_le {
        panic!("Found conflicting byte_struct_le and byte_struct_be attributes");
    }
    let endianness = if found_le {
        Endianness::Little
    } else if found_be {
        Endianness::Big
    } else {
        endianness_input
    };

    let name = &ast.ident;
    if let syn::Data::Struct(syn::DataStruct{fields: syn::Fields::Named(
        syn::FieldsNamed{named, ..}), ..}) = ast.data {

        let mut ty0 = Vec::<syn::Type>::new();
        let mut ident1 = Vec::<syn::Ident>::new();
        let mut field_endianness = Vec::<Endianness>::new();
        for n in named {
            ty0.push(n.ty.clone());
            ident1.push(n.ident.unwrap().clone());
            let mut found_le = false;
            let mut found_be = false;
            for syn::Attribute{path: syn::Path{segments, ..}, ..} in n.attrs {
                if segments.len() != 1 {
                    continue;
                }
                match segments[0].ident.to_string().as_str() {
                    "byte_struct_le" => found_le = true,
                    "byte_struct_be" => found_be = true,
                    _ => ()
                };
            }
            if found_be && found_le {
                panic!("Found conflicting byte_struct_le and byte_struct_be attributes");
            }
            if found_be {
                field_endianness.push(Endianness::Big);
            } else if found_le {
                field_endianness.push(Endianness::Little);
            } else {
                field_endianness.push(endianness);
            }
        }

        let (write_bytes_fn, read_bytes_fn): (Vec<_>, Vec<_>) =
            field_endianness.iter().map(|e| {
                let name_str = match e {
                    Endianness::Little => ("write_bytes_default_le", "read_bytes_default_le"),
                    Endianness::Big => ("write_bytes_default_be", "read_bytes_default_be"),
                    Endianness::Unspecified => ("write_bytes", "read_bytes"),
                };
                (syn::Ident::new(name_str.0, Span::call_site()),
                syn::Ident::new(name_str.1, Span::call_site()))
            }).unzip();

        // quote! seems not liking using the same object twice in the content
        let ty1 = ty0.clone();
        let ty2 = ty0.clone();
        let ty3 = ty0.clone();
        let ident2 = ident1.clone();
        let ident3 = ident1.clone();
        let gen = quote! {
            impl ByteStruct for #name {
                fn write_bytes(&self, bytes: &mut [u8]) {
                    let mut cur: usize = 0;
                    #({
                        let len = <#ty1>::BYTE_LEN;
                        self.#ident1.#write_bytes_fn(&mut bytes[cur .. (cur + len)]);
                        cur += len;
                    })*
                }
                fn read_bytes(bytes: &[u8]) -> Self {
                    let mut cur: usize = 0;
                    #(
                        let len = <#ty2>::BYTE_LEN;
                        let #ident2 = <#ty3>::#read_bytes_fn(&bytes[cur .. (cur + len)]);
                        cur += len;
                    )*
                    #name { #(#ident3),* }
                }
            }

            impl ByteStructLen for #name {
                const BYTE_LEN: usize = #(<#ty0>::BYTE_LEN)+*;
            }
        };
        gen.into()

    } else {
        panic!("Only support struct with named fields!");
    }
}

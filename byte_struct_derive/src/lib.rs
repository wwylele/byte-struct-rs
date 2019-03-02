#![recursion_limit = "128"]
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

enum Endianness {
    Little,
    Big,
}

#[proc_macro_derive(ByteStructLE)]
pub fn byte_struct_le_macro_derive(input: TokenStream) -> TokenStream {
    byte_struct_macro_derive_impl(input, Endianness::Little)
}

#[proc_macro_derive(ByteStructBE)]
pub fn byte_struct_be_macro_derive(input: TokenStream) -> TokenStream {
    byte_struct_macro_derive_impl(input, Endianness::Big)
}

fn byte_struct_macro_derive_impl(input: TokenStream, endianness: Endianness) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    if let syn::Data::Struct(syn::DataStruct{fields: syn::Fields::Named(
        syn::FieldsNamed{named, ..}), ..}) = ast.data {

        let mut ty0 = Vec::<syn::Type>::new();
        let mut ident1 = Vec::<syn::Ident>::new();
        let mut write_bytes_fn = Vec::<syn::Ident>::new();
        let mut read_bytes_fn = Vec::<syn::Ident>::new();

        for n in named {
            ty0.push(n.ty.clone());
            ident1.push(n.ident.unwrap().clone());
            match endianness {
                Endianness::Little =>{
                    write_bytes_fn.push(syn::Ident::new("write_le_bytes", Span::call_site()));
                    read_bytes_fn.push(syn::Ident::new("read_le_bytes", Span::call_site()));
                },
                Endianness::Big =>{
                    write_bytes_fn.push(syn::Ident::new("write_be_bytes", Span::call_site()));
                    read_bytes_fn.push(syn::Ident::new("read_be_bytes", Span::call_site()));
                },
            }
        }

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

            impl ByteStructImpl for #name {
                const BYTE_LEN: usize = #(<#ty0>::BYTE_LEN)+*;
                fn write_le_bytes(&self, bytes: &mut [u8]) {
                    self.write_bytes(bytes);
                }
                fn read_le_bytes(bytes: &[u8]) -> Self {
                    <#name>::read_bytes(bytes)
                }
                fn write_be_bytes(&self, bytes: &mut [u8]) {
                    self.write_bytes(bytes);
                }
                fn read_be_bytes(bytes: &[u8]) -> Self {
                    <#name>::read_bytes(bytes)
                }
            }
        };
        gen.into()

    } else {
        panic!("Only support struct with named fields!");
    }
}
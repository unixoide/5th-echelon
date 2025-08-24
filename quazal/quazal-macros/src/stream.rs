extern crate proc_macro;
use quote::quote;
use syn::Data;
use syn::DeriveInput;

use crate::what_crate;

pub fn to_stream_derive_impl(input: DeriveInput) -> proc_macro2::TokenStream {
    let crt = what_crate();
    let name = input.ident;
    let (impl_generics, type_generics, where_generics) = input.generics.split_for_impl();
    let Data::Struct(input) = input.data else { panic!() };
    let fields = input.fields.into_iter().map(|f| f.ident);
    quote! {
        impl #impl_generics #crt::rmc::basic::ToStream for #name #type_generics
        #where_generics
        {
            fn to_stream<W>(&self, stream: &mut #crt::rmc::basic::WriteStream<W>) -> ::std::result::Result<usize, ::std::io::Error>
            where
                W: ::byteorder::WriteBytesExt,
            {
                let mut n = 0;
                #(
                    n += stream.write(&self.#fields)?;
                )*
                Ok(n)
            }
        }
    }
}

pub fn from_stream_derive_impl(input: DeriveInput) -> proc_macro2::TokenStream {
    let crt = what_crate();
    let name = input.ident;
    let (impl_generics, type_generics, where_generics) = input.generics.split_for_impl();
    let Data::Struct(input) = input.data else { panic!() };
    let fields = input.fields.into_iter().map(|f| f.ident);
    quote! {
        impl #impl_generics #crt::rmc::basic::FromStream for #name #type_generics
        #where_generics
        {
            fn from_stream<R>(stream: &mut #crt::rmc::basic::ReadStream<R>) -> ::std::result::Result<Self, #crt::rmc::basic::FromStreamError>
            where
                R: ::byteorder::ReadBytesExt,
            {
                Ok(Self {
                    #(
                        #fields: stream.read()?,
                    )*
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn foo() {
        let ts = parse_quote! {
            struct Foo {
                id: u32,
                name: String,
            }
        };

        let out = dbg!(to_stream_derive_impl(ts));
        println!("{out}");
    }
}

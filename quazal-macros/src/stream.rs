extern crate proc_macro;
use crate::what_crate;
use quote::quote;
use syn::*;

pub fn to_stream_derive_impl(input: DeriveInput) -> proc_macro2::TokenStream {
    let crt = what_crate();
    let name = input.ident;
    let input = match input.data {
        Data::Struct(input) => input,
        _ => panic!(),
    };
    let fields = input.fields.into_iter().map(|f| f.ident);
    quote! {
        impl #crt::rmc::basic::ToStream for #name {
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
    let input = match input.data {
        Data::Struct(input) => input,
        _ => panic!(),
    };
    let fields = input.fields.into_iter().map(|f| f.ident);
    quote! {
        impl #crt::rmc::basic::FromStream for #name {
            fn from_stream<R>(stream: &mut #crt::rmc::basic::ReadStream<R>) -> ::std::result::Result<Self, ::std::io::Error>
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
        println!("{}", out);
    }
}

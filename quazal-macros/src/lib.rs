extern crate proc_macro;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use proc_macro_crate::crate_name;
use proc_macro_crate::FoundCrate;
use quote::quote;
use quote::ToTokens;
use syn::*;

mod stream;
use stream::from_stream_derive_impl;
use stream::to_stream_derive_impl;

mod protocol;
use protocol::protocol_derive_impl;

fn what_crate() -> TokenStream {
    match crate_name("quazal").expect("quazal is in Cargo.toml") {
        FoundCrate::Itself => Ident::new("crate", Span::call_site()).to_token_stream(),
        FoundCrate::Name(name) => {
            let i = Ident::new(&name, Span::call_site());
            quote! { ::#i }
        }
    }
}

#[proc_macro_derive(ToStream)]
pub fn to_stream_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    proc_macro::TokenStream::from(to_stream_derive_impl(input))
}

#[proc_macro_derive(FromStream)]
pub fn from_stream_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    proc_macro::TokenStream::from(from_stream_derive_impl(input))
}

#[proc_macro_derive(Protocol, attributes(id))]
pub fn protocol_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    proc_macro::TokenStream::from(protocol_derive_impl(input))
}

use proc_macro2::Span;
use proc_macro2::TokenStream;
use syn::*;

fn error(span: Span, msg: &str) -> TokenStream {
    Error::new(span, msg).to_compile_error()
}

pub fn protocol_derive_impl(input: DeriveInput) -> TokenStream {
    match input.data {
        Data::Struct(s) => derive_struct(s, input.attrs),
        Data::Union(_) => error(Span::call_site(), "unions are not supported"),
        Data::Enum(_) => error(Span::call_site(), "enums are not supported"),
    }
}

fn derive_struct(_input: DataStruct, attrs: Vec<Attribute>) -> TokenStream {
    let pid: Option<u32> = attrs.iter().find_map(|a| {
        if a.path.is_ident("id") {
            let l: LitInt = a.parse_args().ok()?;
            l.base10_parse().ok()
        } else {
            None
        }
    });
    if pid.is_none() {
        return error(Span::call_site(), "id missing or invalid");
    }
    let _pid = pid.unwrap();
    todo!()
}

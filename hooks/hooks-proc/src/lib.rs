#![deny(clippy::pedantic)]

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::parse::Parser;
use syn::spanned::Spanned;
use syn::Expr;
use syn::ExprLit;
use syn::Lit;
use syn::LitBool;
use syn::Meta;
use syn::MetaList;
use syn::MetaNameValue;

fn parse_meta_bool_value(value: &Expr) -> Option<bool> {
    match value {
        Expr::Lit(ExprLit {
            lit: Lit::Bool(LitBool { value: bval, .. }),
            ..
        }) => Some(*bval),

        _ => None,
    }
}

fn parse_attributes(meta: &Meta, with_logging: &mut bool, always_call: &mut bool) -> Result<(), syn::Error> {
    match &meta {
        Meta::Path(path) => match path.get_ident().map(std::string::ToString::to_string).as_deref() {
            Some("log") => *with_logging = true,
            Some("always_call") => *always_call = true,
            Some(id) => {
                return Err(syn::Error::new(meta.span(), format!("unexpected attribute {id}")));
            }
            None => {}
        },
        Meta::List(MetaList { tokens, .. }) => {
            return Meta::parse
                .parse(tokens.clone().into())
                .and_then(|meta| parse_attributes(&meta, with_logging, always_call));
        }
        Meta::NameValue(MetaNameValue { path, value, .. }) => {
            match path.get_ident().map(std::string::ToString::to_string).as_deref() {
                Some("log") => {
                    if let Some(v) = parse_meta_bool_value(value) {
                        *with_logging = v;
                    } else {
                        return Err(syn::Error::new(meta.span(), "unexpected value"));
                    }
                }
                Some("always_call") => {
                    if let Some(v) = parse_meta_bool_value(value) {
                        *always_call = v;
                    } else {
                        return Err(syn::Error::new(meta.span(), "unexpected value"));
                    }
                }
                Some(id) => {
                    return Err(syn::Error::new(meta.span(), format!("unexpected attribute {id}")));
                }
                None => {}
            }
        }
    }
    Ok(())
}

#[proc_macro_attribute]
pub fn forwardable_export(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemFn);
    let sig = input.sig.clone();
    let name = input.sig.ident;
    let name_str = name.to_string();
    let body = input.block;
    let attrs = input.attrs;

    let abi = &sig.abi;
    let out = &sig.output;
    let mut ins = sig.inputs.clone();

    // remove mutability flags as they are not allowed in function pointers
    // or call parameters.
    for input in &mut ins {
        if let syn::FnArg::Typed(syn::PatType { pat, .. }) = input {
            if let syn::Pat::Ident(pi) = pat.as_mut() {
                pi.mutability = None;
            }
        }
    }

    let mut with_logging = true;
    let mut always_call = false;

    if let Ok(meta) = Meta::parse.parse(attr) {
        if let Err(err) = parse_attributes(&meta, &mut with_logging, &mut always_call) {
            return err.into_compile_error().into();
        }
    }

    let params = ins
        .iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Typed(t) => Some(t),
            syn::FnArg::Receiver(_) => None,
        })
        .map(|arg| arg.pat.clone());
    quote! {
        #(#attrs)*
        #[no_mangle]
        #[::tracing::instrument]
        #sig {
            static FUNC: ::std::sync::OnceLock<unsafe #abi fn(#ins) #out> = ::std::sync::OnceLock::new();
            if #with_logging {
                ::tracing::info!(#name_str);
            }
            let Some(cfg) = crate::config::get() else {
                ::tracing::error!("Config not loaded!");
                return Default::default();
            };
            let do_forward =  cfg!(feature = "forward_calls") 
            || cfg.forward_all_calls
            || cfg.forward_calls.iter().any(|s| s == #name_str);
            let result = if do_forward {
                if #with_logging {
                    ::tracing::info!("Forwarding the call");
                }
                if #always_call {
                    if #with_logging {
                        ::tracing::warn!("Hook is marked as always execute, so calling it before forwarding the call.");
                    }
                    // consuming any must_use
                    let _ = #body;
                }
                let func = FUNC.get_or_init(|| ::std::mem::transmute(crate::uplay_r1_loader::get_proc(::windows::core::s!(#name_str)).unwrap()));
                (func)(#(#params),*)
            } else {
                if #with_logging {
                    ::tracing::info!("Running the hook");
                }
                #body
            };
            if #with_logging {
                ::tracing::info!("result: {result:?}");
            }
            result
        }
    }
    .into()
}

#![deny(clippy::pedantic)]

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn forwardable_export(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemFn);
    let sig = input.sig.clone();
    let name = input.sig.ident;
    let name_str = name.to_string();
    let body = input.block;

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

    let params = ins
        .iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Typed(t) => Some(t),
            syn::FnArg::Receiver(_) => None,
        })
        .map(|arg| arg.pat.clone());
    quote! {
        //#(#attrs)*
        #[no_mangle]
        #[::tracing::instrument]
        #sig {
            ::tracing::info!(#name_str);
            let Some(cfg) = crate::config::get() else {
                ::tracing::error!("Config not loaded!");
                return Default::default();
            };
            let result = if cfg!(feature = "forward_calls") 
                            || cfg.forward_all_calls
                            || cfg.forward_calls.iter().any(|s| s == #name_str)
            {
                static FUNC: ::std::sync::OnceLock<unsafe #abi fn(#ins) #out> = ::std::sync::OnceLock::new();
                let func = FUNC.get_or_init(|| ::std::mem::transmute(crate::uplay_r1_loader::get_proc(::windows::core::s!(#name_str)).unwrap()));
                (func)(#(#params),*)
            } else {
                #body
            };
            ::tracing::info!("result: {result:?}");
            result
        }
    }
    .into()
}

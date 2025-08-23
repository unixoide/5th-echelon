#![deny(clippy::pedantic)]

extern crate proc_macro;

use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::parse_macro_input;
use syn::spanned::Spanned;
use syn::Attribute;
use syn::Data;
use syn::DataEnum;
use syn::DataStruct;
use syn::DeriveInput;
use syn::Error;
use syn::Expr;
use syn::Fields;
use syn::GenericArgument;
use syn::Ident;
use syn::Lit;
use syn::Path;
use syn::PathArguments;
use syn::Type;

fn error(span: Span, msg: &str) -> TokenStream {
    Error::new(span, msg).to_compile_error()
}

#[proc_macro_derive(DDLParser, attributes(count, skip))]
pub fn ddl_parser_dervice(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let tokens = match input.data {
        Data::Struct(s) => derive_struct(s, &input.ident, &input.attrs),
        Data::Enum(e) => derive_enum(&e, &input.ident, &input.attrs),
        Data::Union(_) => return proc_macro::TokenStream::new(),
    };

    proc_macro::TokenStream::from(tokens)
}

#[allow(clippy::too_many_lines)]
fn derive_struct(data: DataStruct, ident: &Ident, _attrs: &[Attribute]) -> proc_macro2::TokenStream {
    let name = ident;
    let func_name = Ident::new(&name.to_string().to_lowercase(), Span::call_site());

    let fields = match data.fields {
        Fields::Named(f) => f.named,
        Fields::Unnamed(_) => return error(ident.span(), "not supported"),
        Fields::Unit => {
            return quote! {
                fn #func_name(input: &[u8]) -> IResult<&[u8], #ident> {
                    Ok((input, dump_value(#name)))
                }
            }
        }
    };

    let (skipped_fields, fields): (Vec<_>, Vec<_>) = fields
        .into_iter()
        .partition(|f| f.attrs.iter().any(|a| a.path().is_ident("skip")));

    let (field_names, tokens): (Vec<Ident>, Vec<TokenStream>) = fields
        .into_iter()
        .filter_map(|f| {
            let span = f.span();
            let name = f.ident.unwrap();
            let name_str = name.to_string();
            if f.attrs.iter().any(|a| a.path().is_ident("skip")) {
                return None;
            }
            let Type::Path(mut ty) = f.ty else {
                return Some((name, error(span, "not supported")));
            };
            let ty = ty.path.segments.pop().unwrap().into_value();
            let tokens = if ty.ident == "Vec" {
                let ty = match ty.arguments {
                    PathArguments::AngleBracketed(a) => a.args,
                    _ => return Some((name, error(span, "not supported"))),
                };
                let Some(GenericArgument::Type(Type::Path(ty))) = ty.first() else {
                    return Some((name, error(span, "not supported")));
                };
                let ty_name = Ident::new(
                    &ty.path.segments.first().unwrap().ident.to_string().to_lowercase(),
                    ty.span(),
                );
                let ty_name = translate_type(ty_name);
                let count_type = f.attrs.into_iter().find_map(|attr| {
                    if attr.path().is_ident("count") {
                        let ct: Ident = match attr.parse_args() {
                            Ok(p) => p,
                            Err(e) => return Some(Err(error(attr.span(), &format!("{e:?}")))),
                        };
                        Some(Ok(translate_type(ct)))
                    } else {
                        None
                    }
                });

                let count_type = match count_type {
                    None => return Some((name, error(span, "count missing"))),
                    Some(Ok(ct)) => ct,
                    Some(Err(e)) => return Some((name, e)),
                };
                quote! {
                    dbg_dmp(length_count(#count_type, #ty_name), #name_str)
                }
            } else if ty.ident == "Box" {
                let ty = match ty.arguments {
                    PathArguments::AngleBracketed(a) => a.args,
                    _ => return Some((name, error(span, "not supported"))),
                };
                let Some(GenericArgument::Type(Type::Path(ty))) = ty.first() else {
                    return Some((name, error(span, "not supported")));
                };
                let ty_name = Ident::new(
                    &ty.path.segments.first().unwrap().ident.to_string().to_lowercase(),
                    ty.span(),
                );
                let ty_name = translate_type(ty_name);
                quote! {
                    boxed(#ty_name)
                }
            } else {
                let ty_name = Ident::new(&ty.ident.to_string().to_lowercase(), ty.span());
                translate_type(ty_name).to_token_stream()
            };
            Some((name, tokens))
        })
        .unzip();

    let skipped_fields = skipped_fields.into_iter().map(|f| {
        let name = f.ident;
        quote! {
            #name: std::default::Default::default()
        }
    });

    let name_str = name.to_string();
    if field_names.len() > 1 {
        quote! {
            fn #func_name(input: &[u8]) -> IResult<&[u8], #ident> {
                let (input, (#(#field_names),*)) = context(#name_str, tuple((#(#tokens),*))).parse(input)?;
                Ok((input, dump_value(#name { #(#field_names,)* #(#skipped_fields)* })))
            }
        }
    } else {
        quote! {
            fn #func_name(input: &[u8]) -> IResult<&[u8], #ident> {
                let (input, #(#field_names),*) = context(#name_str, dbg_dmp(#(#tokens),*, #name_str)).parse(input)?;
                Ok((input, dump_value(#name { #(#field_names,)* #(#skipped_fields)* })))
            }
        }
    }
}

fn translate_type(ty: Ident) -> Ident {
    if ty == "u8" {
        Ident::new("be_u8", ty.span())
    } else if ty == "u16" {
        Ident::new("be_u16", ty.span())
    } else if ty == "u32" {
        Ident::new("be_u32", ty.span())
    } else if ty == "u64" {
        Ident::new("be_u64", ty.span())
    } else {
        ty
    }
}

#[allow(clippy::too_many_lines)]
fn derive_enum(data: &DataEnum, ident: &Ident, attrs: &[Attribute]) -> proc_macro2::TokenStream {
    let mut tag_type = None;
    for attr in attrs {
        if attr.path().is_ident("repr") {
            let p: Path = attr.parse_args().unwrap();
            tag_type = Some(p.segments.last().unwrap().ident.to_string());
            break;
        }
    }
    let tag_type = Ident::new(
        match tag_type.as_deref() {
            Some("u8") => "be_u8",
            Some("u16") => "be_u16",
            Some("u32") => "be_u32",
            t => return Error::new(ident.span(), format!("Invalid tag type {t:?}")).to_compile_error(),
        },
        Span::call_site(),
    );
    let variants = data.variants.iter().map(|v| {
        let name = &v.ident;
        let name_str = format!("{ident}::{name}");
        let func_name = &name.to_string().to_lowercase();
        let func_name = Ident::new(func_name, Span::call_site());
        let d = &v.discriminant.as_ref().expect("discriminant required").1;
        let value: u64 = match d {
            Expr::Lit(ref l) => match l.lit {
                Lit::Int(ref i) => i.base10_parse().expect("number"),
                _ => {
                    return (
                        func_name,
                        Error::new(l.span(), "not supported").to_compile_error(),
                    )
                }
            },
            _ => {
                return (
                    func_name,
                    Error::new(d.span(), "not supported").to_compile_error(),
                )
            }
        };

        #[allow(clippy::cast_possible_truncation)]
        let tag = match tag_type.to_string().as_str() {
            "be_u8" => vec![value as u8],
            "be_u16" => (value as u16).to_be_bytes().to_vec(),
            "be_u32" => (value as u32).to_be_bytes().to_vec(),
            "be_u64" => value.to_be_bytes().to_vec(),
            _ => unreachable!(),
        };
        let inner = match &v.fields {
            Fields::Named(_) => panic!("not supported"),
            Fields::Unnamed(f) => {
                if f.unnamed.len() != 1 {
                    return (
                        func_name,
                        Error::new(v.span(), "not supported").to_compile_error(),
                    );
                }
                f.unnamed.first().unwrap()
            }
            Fields::Unit => {
                // shortcut
                return (
                    func_name.clone(),
                    quote! {
                        fn #func_name(input: &[u8]) -> IResult<&[u8], #ident> {
                            let (input, value) = context(#name_str, tag(&[#(#tag),*] as &[_])).parse(input)?;
                            Ok(
                                (
                                    input,
                                    dump_value(#ident::#name),
                                )
                            )
                        }
                    },
                )
            },
        };
        let Type::Path(inner_type) = &inner.ty else {
            return (func_name,Error::new(inner.span(), "not supported").to_compile_error());
        };
        let func_inner = Ident::new(
            &inner_type
                .path
                .segments
                .last()
                .unwrap()
                .ident
                .to_string()
                .to_lowercase(),
            inner.span(),
        );
        (
            func_name.clone(),
            quote! {
                fn #func_name(input: &[u8]) -> IResult<&[u8], #ident> {
                    let (input, value) = context(#name_str, preceded(tag(&[#(#tag),*] as &[_]), dbg_dmp(#func_inner, #name_str))).parse(input)?;
                    Ok(
                        (
                            input,
                            dump_value(#ident::#name(value)),
                        )
                    )
                }
            },
        )
    });

    let (names, tokens): (Vec<Ident>, Vec<TokenStream>) = variants.unzip();

    let func_name = Ident::new(&ident.to_string().to_lowercase(), Span::call_site());
    let name_str = ident.to_string();

    quote! {
        fn #func_name(input: &[u8]) -> IResult<&[u8], #ident> {
            context(#name_str, dbg_dmp(alt((#(#ident::#names),*)), #name_str)).parse(input)
        }
        impl #ident {
            #(
                #tokens
            )*
        }
    }
}

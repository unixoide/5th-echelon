use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

use convert_case::Case;
use convert_case::Casing;
use proc_macro2::Span;
use quote::format_ident;
use quote::quote;
use syn::parse_quote;
use syn::Ident;

use crate::parse::ClassDeclaration;
use crate::parse::Element;
use crate::parse::Namespace;
use crate::parse::ParameterType;
use crate::parse::ProtocolDeclaration;
use crate::parse::SubType;

pub fn write_modules<I, S>(path: &Path, modules: I) -> io::Result<()>
where
    S: AsRef<str>,
    I: Iterator<Item = S>,
{
    let modules: Vec<Ident> = modules
        .map(|m| Ident::new(m.as_ref(), Span::call_site()))
        .collect();
    let ts = quote! {
        #(
            pub mod #modules;
        )*
    };

    let mut f = fs::File::create(path.join("mod").with_extension("rs"))?;
    writeln!(f, "{ts}")
}

#[allow(clippy::module_name_repetitions)]
pub fn generate_source(directory: &Path, namespace: &Namespace) -> io::Result<HashSet<String>> {
    let modules: io::Result<_> = namespace
        .elements
        .iter()
        .filter_map(|element| match element {
            Element::DOClassDeclaration(_) => todo!(),
            Element::DatasetDeclaration(_) => todo!(),
            Element::Variable(_) => todo!(),
            Element::Method(_) => todo!(),
            Element::Action(_) => todo!(),
            Element::PropertyDeclaration(p) => {
                eprintln!("Missing code generator for PropertyDeclaration {}", p.name1);
                None
            }
            Element::ProtocolDeclaration(p) => generate_protocol_code(directory, p).transpose(),
            Element::Parameter(_) => todo!(),
            Element::ReturnValue(_) => todo!(),
            Element::ClassDeclaration(c) => generate_class_code(directory, c).transpose(),
            Element::TemplateDeclaration(t) => {
                eprintln!("Missing code generator for SimpleDeclaration {}", t.name1);
                None
            }
            Element::SimpleDeclaration(s) => {
                eprintln!("Missing code generator for SimpleDeclaration {}", s.name1);
                None
            }
            Element::TemplateInstance(t) => {
                eprintln!("Missing code generator for TemplateInstance {}", t.name1);
                None
            }
            Element::DDLUnitDeclaration(d) => {
                let ty = directory
                    .join(d.name1.to_case(Case::Snake))
                    .join("types.rs");
                if ty.exists() {
                    if let Err(e) = fs::remove_file(ty) {
                        return Some(Err(e));
                    }
                }

                None
            }
            Element::DupSpaceDeclaration(_) => todo!(),
        })
        .try_fold(HashMap::<String, HashSet<String>>::new(), |mut map, res| {
            let (namespace, module) = res?;
            map.entry(namespace).or_default().insert(module);
            Ok(map)
        });
    let modules = modules?;

    modules.iter().try_for_each(|(namespace, modules)| {
        write_modules(&directory.join(namespace), modules.iter())
    })?;

    Ok(modules.keys().cloned().collect())
}

fn to_ident<S: AsRef<str>>(s: S) -> syn::Ident {
    syn::Ident::new(s.as_ref(), proc_macro2::Span::call_site())
}
fn generate_class_code(
    directory: &Path,
    class: &ClassDeclaration,
) -> io::Result<Option<(String, String)>> {
    let ns_name = class.namespace.to_case(Case::Snake);
    let d = directory.join(&ns_name);
    if !d.exists() {
        fs::create_dir_all(&d)?;
    }

    let name = to_ident(&class.name1);
    let vars = class
        .variables
        .iter()
        .map(|e| match e {
            Element::Variable(v) => v,
            _ => unreachable!("unexpected element type"),
        })
        .map(|v| {
            let name = escape_name(v.name1.trim_start_matches("m_").to_case(Case::Snake));
            let name = to_ident(name);
            let ty = to_rust_type(&v.ty.ty);
            quote! {
                pub #name: #ty
            }
        })
        .collect::<Vec<_>>();

    let ts = if vars.is_empty() {
        quote! {
            #[derive(Debug, ToStream, FromStream)]
            pub struct #name;
        }
    } else {
        quote! {
            #[derive(Debug, ToStream, FromStream)]
            pub struct #name {
                #(#vars,)*
            }
        }
    };

    let file_name = d.join("types.rs");

    let is_new = !file_name.exists();

    let mut f = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)?;

    if is_new {
        writeln!(
            f,
            "#![allow(clippy::enum_variant_names, clippy::upper_case_acronyms)]"
        )?;
    }

    writeln!(f, "{ts}")?;

    Ok(Some((ns_name, "types".to_owned())))
}

#[allow(clippy::too_many_lines)]
fn generate_protocol_code(
    directory: &Path,
    protocol: &ProtocolDeclaration,
) -> io::Result<Option<(String, String)>> {
    #![allow(clippy::similar_names, clippy::cast_possible_truncation)]

    let ns_name = protocol.namespace.to_case(Case::Snake);
    let mod_name = protocol.name1.to_case(Case::Snake);

    let d = directory.join(&ns_name);
    if !d.exists() {
        fs::create_dir_all(&d)?;
    }
    let fname = d.join(&mod_name).with_extension("rs");

    /*
    let mut f = match fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&fname)
    {
        Err(e) if matches!(e.kind(), std::io::ErrorKind::AlreadyExists) => {
            eprintln!("Skipping {:?}, file already exists", fname);
            return Ok(Some((ns_name, mod_name)));
        }
        other => other,
    }?;
    */
    let mut f = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&fname)?;
    let struct_name_str = protocol.name1.to_case(Case::UpperCamel);
    let enum_name = to_ident(format!("{struct_name_str}Method"));
    let struct_name = to_ident(&struct_name_str);
    #[allow(clippy::single_match_else)]
    let id = match protocol.id {
        Some(id) => quote!(#id),
        None => quote!(todo!()),
    };
    let methods: Vec<_> = protocol
        .methods
        .iter()
        .map(|m| match m {
            Element::Method(m) => m,
            _ => unreachable!("Unexpected element type"),
        })
        .enumerate()
        .map(|(i, m)| ((i + 1) as u32, m))
        .collect();
    let method_variants = methods.iter().map(|(num, m)| {
        let name = to_ident(m.name1.to_case(Case::UpperCamel));
        quote! {
            #name = #num
        }
    });
    let num_methods = methods.len() as u32;
    let method: Vec<_> = methods
        .iter()
        .map(|(_, m)| to_ident(m.name1.to_case(Case::UpperCamel)))
        .collect();
    let method_name: Vec<_> = methods
        .iter()
        .map(|(_, m)| to_ident(m.name1.to_case(Case::Snake)))
        .collect();
    let method_resp: Vec<_> = methods
        .iter()
        .map(|(_, m)| format_ident!("{}Response", m.name1.to_case(Case::UpperCamel)))
        .collect();
    let method_req: Vec<_> = methods
        .iter()
        .map(|(_, m)| format_ident!("{}Request", m.name1.to_case(Case::UpperCamel)))
        .collect();

    let method_types = methods
        .iter()
        .zip(method_req.iter().zip(method_resp.iter()))
        .map(|((_, meth), (req, resp))| {
            let params = meth.elements1.iter().filter_map(|e| match e {
                Element::Parameter(p) => Some(p),
                Element::ReturnValue(_) => None,
                _ => unreachable!("Unexpected param type"),
            });
            let retval = meth
                .elements1
                .iter()
                .find_map(|e| match e {
                    Element::Parameter(_) => None,
                    Element::ReturnValue(r) => Some(r),
                    _ => unreachable!("Unexpected param type"),
                })
                .map(|r| {
                    // let name = to_ident(&r.name1);
                    let dtype = to_rust_type(&r.dtype1.ty);
                    quote! {
                        pub return_value: #dtype
                    }
                });
            let (req_params, resp_params): (Vec<_>, Vec<_>) =
                params.partition(|p| matches!(p.ty, ParameterType::Request));

            let req_params = req_params
                .into_iter()
                .map(|p| {
                    let name = to_ident(escape_name(p.name1.to_case(Case::Snake)));
                    let dtype = to_rust_type(&p.dtype1.ty);
                    quote! {
                        pub #name: #dtype
                    }
                })
                .collect::<Vec<_>>();

            let resp_params = resp_params
                .into_iter()
                .map(|p| {
                    let name = to_ident(escape_name(p.name1.to_case(Case::Snake)));
                    let dtype = to_rust_type(&p.dtype1.ty);
                    quote! {
                        pub #name: #dtype
                    }
                })
                .collect::<Vec<_>>();

            let mut ts1 = if req_params.is_empty() {
                quote! {
                   #[derive(Debug, FromStream)]
                   pub struct #req;
                }
            } else {
                quote! {
                    #[derive(Debug, FromStream)]
                    pub struct #req {
                        #(
                            #req_params,
                        )*
                    }
                }
            };

            let ts2 = if resp_params.is_empty() && retval.is_none() {
                quote! {
                    #[derive(Debug, ToStream)]
                    pub struct #resp;
                }
            } else {
                let retval = retval.into_iter();
                quote! {
                    #[derive(Debug, ToStream)]
                    pub struct #resp {
                        #(#retval,)*
                        #(
                            #resp_params,
                        )*
                    }
                }
            };

            ts1.extend(ts2);
            ts1
        });

    let trait_name = format_ident!("{}Trait", struct_name);

    let ts = quote! {
        #![allow(
            clippy::enum_variant_names,
            clippy::module_name_repetitions,
            clippy::too_many_lines
            clippy::upper_case_acronyms,
            clippy::wildcard_imports,
        )]

        use num_enum::TryFromPrimitive;
        use quazal::ClientInfo;
        use quazal::Context;
        use quazal::rmc::basic::FromStream;
        use quazal::rmc::basic::ToStream;
        use quazal::rmc::Error;
        use quazal::rmc::Protocol;
        use quazal::rmc::Request;
        use slog::Logger;
        use std::convert::TryFrom;
        use super::types::*;

        #[derive(Debug, TryFromPrimitive)]
        #[repr(u32)]
        enum #enum_name {
            #(#method_variants,)*
        }

        #(
            #method_types
        )*

        pub struct #struct_name<T: #trait_name<CI>, CI>(T, ::std::marker::PhantomData<CI>);

        impl<T: #trait_name<CI>, CI> #struct_name<T, CI> {
            pub fn new(implementation: T) -> Self { Self(implementation, ::std::marker::PhantomData) }
        }

        impl<T: #trait_name<CI>, CI> Protocol<CI> for #struct_name<T, CI> {
            fn id(&self) -> u16 { #id }
            fn name(&self) -> String { #struct_name_str.to_string() }
            fn num_methods(&self) -> u32 { #num_methods }

            fn handle(
                &self,
                logger: &Logger,
                ctx: &Context,
                ci: &mut ClientInfo<CI>,
                request: &Request,
            ) -> Result<Vec<u8>, Error> {
                let method = #enum_name::try_from(request.method_id).ok();
                match method {
                    None => Err(Error::UnknownMethod),
                    #(
                        Some(#enum_name::#method) => {
                            let req = #method_req::from_bytes(&request.parameters)?;
                            debug!(logger, "Request: {:?}", req);
                            let resp = self.0.#method_name(logger, ctx, ci, req);
                            debug!(logger, "Response: {:?}", resp);
                            Ok(resp?.as_bytes())
                        },
                    )*
                }
            }

            fn method_name(&self, method_id: u32) -> Option<String> {
                #enum_name::try_from(method_id).ok().map(|e| format!("{:?}", e))
            }
        }

        #[allow(unused_variables)]
        pub trait #trait_name<CI> {
            #(
                fn #method_name(
                    &self,
                    logger: &Logger,
                    ctx: &Context,
                    ci: &mut ClientInfo<CI>,
                    request: #method_req
                ) -> Result<#method_resp, Error> {
                    warn!(logger, "Method {}.{} not implemented", #struct_name_str, stringify!(#method_name));
                    Err(quazal::rmc::Error::UnimplementedMethod)
                }
            )*
        }
    };

    writeln!(f, "{ts}")?;

    Ok(Some((ns_name, mod_name)))
}

fn to_rust_type(ty: &SubType) -> syn::Path {
    match ty {
        SubType::DOClass(_) => todo!(),
        SubType::Dataset(_) => todo!(),
        SubType::Class(c) => to_rust_type_impl(c).unwrap_or_else(|| {
            let i = to_ident(c);
            parse_quote!(#i)
        }),
        SubType::Simple(s) => {
            to_rust_type_impl(s.as_str()).unwrap_or_else(|| todo!("unsupported datatype {}", s))
        }
        SubType::Template(t) => {
            let name = to_rust_type_impl(&t.template_name).unwrap_or_else(|| {
                let i = to_ident(&t.template_name);
                parse_quote!(#i)
            });
            let params = t.parameters.iter().map(to_rust_type);
            parse_quote! {
                #name<#(#params),*>
            }
        }
    }
}

fn to_rust_type_impl(s: &str) -> Option<syn::Path> {
    match s {
        "bool" => Some(parse_quote!(bool)),
        "byte" | "uint8" => Some(parse_quote!(u8)),
        "uint16" => Some(parse_quote!(u16)),
        "uint32" => Some(parse_quote!(u32)),
        "uint64" => Some(parse_quote!(u64)),
        "int8" => Some(parse_quote!(i8)),
        "int16" => Some(parse_quote!(i16)),
        "int32" => Some(parse_quote!(i32)),
        "int64" => Some(parse_quote!(i64)),
        "double" => Some(parse_quote!(f64)),
        "string" => Some(parse_quote!(String)),
        "qlist" => Some(parse_quote!(quazal::rmc::types::QList)),
        "std_list" | "qvector" => Some(parse_quote!(Vec)),
        "std_map" => Some(parse_quote!(std::collections::HashMap)),
        "qresult" => Some(parse_quote!(quazal::rmc::types::QResult)),
        "datetime" => Some(parse_quote!(quazal::rmc::types::DateTime)),
        "stationurl" => Some(parse_quote!(quazal::rmc::types::StationURL)),
        "buffer" => Some(parse_quote!(Vec<u8>)),
        "buffertail" => Some(parse_quote!(quazal::rmc::types::BufferTail)),
        "variant" => Some(parse_quote!(quazal::rmc::types::Variant)),
        "Property" => Some(parse_quote!(quazal::rmc::types::Property)),
        "PropertyVariant" => Some(parse_quote!(quazal::rmc::types::PropertyVariant)),
        "ResultRange" => Some(parse_quote!(quazal::rmc::types::ResultRange)),
        "qBuffer" => Some(parse_quote!(quazal::rmc::types::QBuffer)),
        "any" => Some(parse_quote!(quazal::rmc::types::Any)),
        "Data" => Some(parse_quote!(quazal::rmc::types::Data)),
        _ => None,
    }
}

fn escape_name<S: AsRef<str>>(s: S) -> String {
    match s.as_ref() {
        "type" => "typ",
        r => r,
    }
    .to_string()
}

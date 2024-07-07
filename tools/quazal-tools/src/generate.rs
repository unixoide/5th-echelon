#![allow(clippy::implicit_hasher)]

use std::collections::hash_map;
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

pub fn write_modules<I, S>(path: &Path, modules: I, is_top: bool) -> io::Result<()>
where
    S: AsRef<str>,
    I: Iterator<Item = S>,
{
    let modules = modules.map(|m| {
        let m = m.as_ref();
        let i = Ident::new(m, Span::call_site());
        if is_top {
            quote! {
                #[cfg(feature = #m)]
                pub mod #i;
            }
        } else {
            quote! {
                pub mod #i;
            }
        }
    });
    let ts = quote! {
        #(
            #modules
        )*
    };

    let mut f = fs::File::create(path.join("mod").with_extension("rs"))?;
    writeln!(f, "// AUTOGENERATED with quazal-tools\n{ts}")
}

#[allow(clippy::module_name_repetitions)]
pub fn generate_source(
    directory: &Path,
    namespace: &Namespace,
    import_map: &ImportMap,
) -> io::Result<HashSet<String>> {
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
                println!(
                    "[-] Missing code generator for PropertyDeclaration {}",
                    p.name1
                );
                None
            }
            Element::ProtocolDeclaration(p) => {
                generate_protocol_code(directory, p, import_map).transpose()
            }
            Element::Parameter(_) => todo!(),
            Element::ReturnValue(_) => todo!(),
            Element::ClassDeclaration(c) => {
                generate_class_code(directory, c, import_map).transpose()
            }
            Element::TemplateDeclaration(t) => {
                println!(
                    "[-] Missing code generator for TemplateDeclaration {}",
                    t.name1
                );
                None
            }
            Element::SimpleDeclaration(s) => {
                println!(
                    "[-] Missing code generator for SimpleDeclaration {}",
                    s.name1
                );
                None
            }
            Element::TemplateInstance(t) => {
                println!(
                    "[-] Missing code generator for TemplateInstance {}",
                    t.name1
                );
                None
            }
            Element::DDLUnitDeclaration(d) => {
                println!("[*] New namespace {}", d.name1);
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
    let mut modules = modules?;

    modules.iter_mut().try_for_each(|(namespace, modules)| {
        let target_dir = &directory.join(namespace);
        if !modules.contains("types") {
            // create empty types.rs file if it doesn't exist yet
            fs::File::create(target_dir.join("types.rs"))?;
            modules.insert(String::from("types"));
        }
        write_modules(target_dir, modules.iter(), false)
    })?;

    Ok(modules.keys().cloned().collect())
}

fn to_ident<S: AsRef<str>>(s: S) -> syn::Ident {
    syn::Ident::new(s.as_ref(), proc_macro2::Span::call_site())
}

fn generate_class_code(
    directory: &Path,
    class: &ClassDeclaration,
    import_map: &ImportMap,
) -> io::Result<Option<(String, String)>> {
    println!("  [*] New class {}", class.name1);
    let mut imports = Vec::new();
    let ns_name = class.namespace.to_case(Case::Snake);
    let d = directory.join(&ns_name);
    if !d.exists() {
        fs::create_dir_all(&d)?;
    }

    let name = to_ident(&class.name1);
    let mut vars = class
        .variables
        .iter()
        .map(|e| match e {
            Element::Variable(v) => v,
            _ => unreachable!("unexpected element type"),
        })
        .map(|v| {
            let name = escape_name(fix_name(
                v.name1.trim_start_matches("m_").to_case(Case::Snake),
            ));
            let name = to_ident(name);
            let ty = to_rust_type(&v.ty.ty);

            imports.append(&mut import_map.find_imports(&ty, &class.namespace));

            quote! {
                pub #name: #ty
            }
        })
        .collect::<Vec<_>>();

    if !class.base.is_empty() {
        let name = to_ident(fix_name(escape_name(class.base.to_case(Case::Snake))));
        let ty = to_rust_type(&SubType::Class(class.base.clone()));
        imports.append(&mut import_map.find_imports(&ty, &class.namespace));

        vars.insert(
            0,
            quote! {
                pub #name: #ty
            },
        );
    }

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
            "// AUTOGENERATED with quazal-tools\n\n#![allow(clippy::enum_variant_names, clippy::upper_case_acronyms)]"
        )?;
    }

    if !imports.is_empty() {
        let namespace = imports
            .iter()
            .map(|element_path| element_path.namespace.to_case(Case::Snake))
            .map(to_ident);
        let ty = imports
            .iter()
            .map(|element_path| &element_path.name)
            .map(to_ident);
        writeln!(
            f,
            "{}",
            quote! {
                #(
                    use super::super::#namespace::types::#ty;
                )*
            }
        )?;
    }

    writeln!(f, "{ts}")?;

    Ok(Some((ns_name, "types".to_owned())))
}

#[allow(clippy::too_many_lines)]
fn generate_protocol_code(
    directory: &Path,
    protocol: &ProtocolDeclaration,
    import_map: &ImportMap,
) -> io::Result<Option<(String, String)>> {
    #![allow(clippy::similar_names, clippy::cast_possible_truncation)]
    println!("  [*] New protocol {}", protocol.name1);

    let mut imports = Vec::new();
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
    let method_enum_name = format_ident!("{struct_name_str}Method");
    let server_struct_name = format_ident!("{}Server", &struct_name_str);
    let client_struct_name = format_ident!("{}Client", &struct_name_str);
    let id_const_name = format_ident!("{}_ID", struct_name_str.to_case(Case::ScreamingSnake));
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
    let method_enum_variants: Vec<_> = methods
        .iter()
        .map(|(_, m)| to_ident(m.name1.to_case(Case::UpperCamel)))
        .collect();
    let method_names: Vec<_> = methods
        .iter()
        .map(|(_, m)| to_ident(fix_name(m.name1.to_case(Case::Snake))))
        .collect();
    let method_resp_types: Vec<_> = methods
        .iter()
        .map(|(_, m)| format_ident!("{}Response", m.name1.to_case(Case::UpperCamel)))
        .collect();
    let method_req_types: Vec<_> = methods
        .iter()
        .map(|(_, m)| format_ident!("{}Request", m.name1.to_case(Case::UpperCamel)))
        .collect();

    let method_types = methods
        .iter()
        .zip(method_req_types.iter().zip(method_resp_types.iter()))
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

                    imports.append(&mut import_map.find_imports(&dtype, &protocol.namespace));

                    quote! {
                        pub return_value: #dtype
                    }
                });
            let (req_params, resp_params): (Vec<_>, Vec<_>) =
                params.partition(|p| matches!(p.ty, ParameterType::Request));

            let req_params = req_params
                .into_iter()
                .map(|p| {
                    let name = to_ident(fix_name(escape_name(p.name1.to_case(Case::Snake))));
                    let dtype = to_rust_type(&p.dtype1.ty);

                    imports.append(&mut import_map.find_imports(&dtype, &protocol.namespace));

                    quote! {
                        pub #name: #dtype
                    }
                })
                .collect::<Vec<_>>();

            let resp_params = resp_params
                .into_iter()
                .map(|p| {
                    let name = to_ident(fix_name(escape_name(p.name1.to_case(Case::Snake))));
                    let dtype = to_rust_type(&p.dtype1.ty);

                    imports.append(&mut import_map.find_imports(&dtype, &protocol.namespace));

                    quote! {
                        pub #name: #dtype
                    }
                })
                .collect::<Vec<_>>();

            let mut ts1 = if req_params.is_empty() {
                quote! {
                   #[derive(Debug, ToStream, FromStream)]
                   pub struct #req;
                }
            } else {
                quote! {
                    #[derive(Debug, ToStream, FromStream)]
                    pub struct #req {
                        #(
                            #req_params,
                        )*
                    }
                }
            };

            let ts2 = if resp_params.is_empty() && retval.is_none() {
                quote! {
                    #[derive(Debug, ToStream, FromStream)]
                    pub struct #resp;
                }
            } else {
                let retval = retval.into_iter();
                quote! {
                    #[derive(Debug, ToStream, FromStream)]
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
        })
        .collect::<Vec<_>>();

    let server_trait_name = format_ident!("{}Trait", server_struct_name);
    let client_trait_name = format_ident!("{}Trait", client_struct_name);

    let server_code = generate_server_protocol_code(
        &server_struct_name,
        &server_trait_name,
        &id_const_name,
        num_methods,
        &struct_name_str,
        &method_enum_name,
        &method_enum_variants,
        &method_req_types,
        &method_resp_types,
        &method_names,
    );
    let client_code = generate_client_protocol_code(
        &client_struct_name,
        &client_trait_name,
        &id_const_name,
        num_methods,
        &struct_name_str,
        &method_enum_name,
        &method_enum_variants,
        &method_req_types,
        &method_resp_types,
        &method_names,
    );

    let type_imports = if imports.is_empty() {
        quote!()
    } else {
        let namespace = imports
            .iter()
            .map(|element_path| element_path.namespace.to_case(Case::Snake))
            .map(to_ident);
        let ty = imports
            .iter()
            .map(|element_path| &element_path.name)
            .map(to_ident);
        quote! {
            #(
                use super::super::#namespace::types::#ty;
            )*
        }
    };

    let ts = quote! {
        #![allow(
            clippy::enum_variant_names,
            clippy::module_name_repetitions,
            clippy::too_many_lines,
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
        use quazal::rmc::ClientProtocol;
        use quazal::rmc::Request;
        use quazal::prudp::ClientRegistry;
        use slog::Logger;
        use std::convert::TryFrom;

        use super::types::*;

        #type_imports

        pub const #id_const_name: u16 = #id;

        #[derive(Debug, TryFromPrimitive)]
        #[repr(u32)]
        pub enum #method_enum_name {
            #(#method_variants,)*
        }

        #(
            #method_types
        )*

        #server_code
        #client_code
    };

    writeln!(f, "// AUTOGENERATED with quazal-tools\n{ts}")?;

    Ok(Some((ns_name, mod_name)))
}

fn generate_server_protocol_code(
    server_struct_name: &Ident,
    server_trait_name: &Ident,
    id_const_name: &Ident,
    num_methods: u32,
    struct_name_str: &str,
    method_enum_name: &Ident,
    method_enum_variants: &[Ident],
    method_req_types: &[Ident],
    method_resp_types: &[Ident],
    method_names: &[Ident],
) -> proc_macro2::TokenStream {
    quote! {
        pub struct #server_struct_name<T: #server_trait_name<CI>, CI>(T, ::std::marker::PhantomData<CI>);

        impl<T: #server_trait_name<CI>, CI> #server_struct_name<T, CI> {
            pub fn new(implementation: T) -> Self { Self(implementation, ::std::marker::PhantomData) }
        }

        impl<T: #server_trait_name<CI>, CI> Protocol<CI> for #server_struct_name<T, CI> {
            fn id(&self) -> u16 { #id_const_name }
            fn name(&self) -> String { #struct_name_str.to_string() }
            fn num_methods(&self) -> u32 { #num_methods }

            fn handle(
                &self,
                logger: &Logger,
                ctx: &Context,
                ci: &mut ClientInfo<CI>,
                request: &Request,
                client_registry: &ClientRegistry<CI>,
                socket: &std::net::UdpSocket,
            ) -> Result<Vec<u8>, Error> {
                let method = #method_enum_name::try_from(request.method_id).ok();
                match method {
                    None => Err(Error::UnknownMethod),
                    #(
                        Some(#method_enum_name::#method_enum_variants) => {
                            let req = #method_req_types::from_bytes(&request.parameters)?;
                            debug!(logger, "Request: {:?}", req);
                            let resp = self.0.#method_names(logger, ctx, ci, req, client_registry, socket);
                            debug!(logger, "Response: {:?}", resp);
                            Ok(resp?.to_bytes())
                        },
                    )*
                }
            }

            fn method_name(&self, method_id: u32) -> Option<String> {
                #method_enum_name::try_from(method_id).ok().map(|e| format!("{:?}", e))
            }
        }

        #[allow(unused_variables)]
        pub trait #server_trait_name<CI> {
            #(
                fn #method_names(
                    &self,
                    logger: &Logger,
                    ctx: &Context,
                    ci: &mut ClientInfo<CI>,
                    request: #method_req_types,
                    client_registry: &ClientRegistry<CI>,
                    _socket: &std::net::UdpSocket,
                ) -> Result<#method_resp_types, Error> {
                    warn!(logger, "Method {}.{} not implemented", #struct_name_str, stringify!(#method_names));
                    Err(quazal::rmc::Error::UnimplementedMethod)
                }
            )*
        }
    }
}

fn generate_client_protocol_code(
    client_struct_name: &Ident,
    _client_trait_name: &Ident,
    id_const_name: &Ident,
    num_methods: u32,
    struct_name_str: &str,
    method_enum_name: &Ident,
    method_enum_variants: &[Ident],
    method_req_types: &[Ident],
    method_resp_types: &[Ident],
    method_names: &[Ident],
) -> proc_macro2::TokenStream {
    quote! {
        pub struct #client_struct_name<CI>(::std::marker::PhantomData<CI>);

        impl<CI> #client_struct_name<CI> {
            pub fn new() -> Self { Self(::std::marker::PhantomData) }
        }

        impl<CI> ClientProtocol<CI> for #client_struct_name<CI> {
            fn id(&self) -> u16 { #id_const_name }
            fn name(&self) -> String { #struct_name_str.to_string() }
            fn num_methods(&self) -> u32 { #num_methods }

            fn method_name(&self, method_id: u32) -> Option<String> {
                #method_enum_name::try_from(method_id).ok().map(|e| format!("{:?}", e))
            }
        }

        #[allow(unused_variables)]
        impl<CI> #client_struct_name<CI> {
            #(
                pub fn #method_names(
                    &self,
                    logger: &Logger,
                    ctx: &Context,
                    ci: &mut ClientInfo<CI>,
                    request: #method_req_types
                ) -> Result<#method_resp_types, Error> {
                    warn!(logger, "Method {}.{} not implemented", #struct_name_str, stringify!(#method_names));
                    self.send(logger, ctx, ci, #method_enum_name::#method_enum_variants as u32, request.to_bytes());
                    Err(quazal::rmc::Error::UnimplementedMethod)
                }
            )*
        }
    }
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

fn fix_name(s: impl AsRef<str>) -> String {
    s.as_ref()
        .replace("_ur_ls", "_urls")
        .replace("_i_ds", "_ids")
        .replace("_pi_ds", "_pids")
}

#[derive(Debug)]
pub struct ElementPath {
    pub namespace: String,
    pub name: String,
}

pub struct ImportMap(HashMap<String, ElementPath>);

impl ImportMap {
    fn find_imports(&self, path: &syn::Path, current_namespace: &str) -> Vec<&ElementPath> {
        let mut imports = Vec::new();
        let actual_ty = path.segments.last().unwrap();
        let mut possible_imports = vec![actual_ty.ident.to_string()];

        if !actual_ty.arguments.is_empty() {
            possible_imports.extend(match actual_ty.arguments {
                syn::PathArguments::AngleBracketed(ref args) => {
                    args.args.iter().map(|arg| match arg {
                        syn::GenericArgument::Type(syn::Type::Path(path)) => {
                            path.path.segments.last().unwrap().ident.to_string()
                        }
                        _ => todo!(),
                    })
                }
                _ => todo!(),
            });
        }
        for possible_import in possible_imports {
            if let Some(element_path) = self.0.get(&possible_import) {
                if element_path.namespace != current_namespace {
                    imports.push(element_path);
                }
            }
        }
        imports
    }
}

#[must_use]
pub fn build_import_map(namespaces: &[Namespace]) -> ImportMap {
    let mut import_map: HashMap<String, ElementPath> = HashMap::new();
    for ns in namespaces {
        for el in &ns.elements {
            match el {
                Element::DOClassDeclaration(_)
                | Element::DatasetDeclaration(_)
                | Element::Variable(_)
                | Element::Method(_)
                | Element::Action(_)
                | Element::PropertyDeclaration(_)
                | Element::ProtocolDeclaration(_)
                | Element::Parameter(_)
                | Element::ReturnValue(_)
                | Element::TemplateDeclaration(_)
                | Element::SimpleDeclaration(_)
                | Element::TemplateInstance(_)
                | Element::DDLUnitDeclaration(_)
                | Element::DupSpaceDeclaration(_) => { /* ignore */ }
                Element::ClassDeclaration(class) => {
                    // TODO: change code generator to prefer local data types over imports
                    if ["RVConnectionData", "LoginData"].contains(&class.name1.as_str()) {
                        println!(
                            "[-] Ignoring {}.{} due to name clash",
                            class.namespace, class.name1
                        );
                        continue;
                    }
                    match import_map.entry(class.name1.clone()) {
                        hash_map::Entry::Occupied(entry) => {
                            panic!("Import clash: Type {} already imported from {}, but also exists in {}", entry.key(), entry.get().namespace, class.namespace);
                        }
                        hash_map::Entry::Vacant(entry) => entry.insert(ElementPath {
                            namespace: class.namespace.clone(),
                            name: class.name1.clone(),
                        }),
                    };
                }
            }
        }
    }

    import_map.remove("Property"); // we have our own implementation
    import_map.remove("PropertyVariant"); // we have our own implementation
    import_map.remove("ResultRange"); // we have our own implementation
    import_map.remove("Data"); // we have our own implementation

    ImportMap(import_map)
}
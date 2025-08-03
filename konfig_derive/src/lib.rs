use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(konfig, Konfig))]
struct KonfigOpts {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), KonfigField>,

    #[darling(default)]
    env_prefix: Option<String>,

    #[darling(default)]
    allow_env: bool,

    #[darling(default)]
    allow_cli: bool,

    #[darling(default)]
    allow_config: bool,
}

#[derive(Debug, FromField)]
#[darling(attributes(konfig, skip_konfig, skip))]
struct KonfigField {
    ident: Option<syn::Ident>,

    #[darling(default)]
    env_name: Option<String>,

    #[darling(default)]
    cli_name: Option<String>,

    #[darling(default)]
    skip_konfig: bool,

    #[darling(default)]
    skip: bool,
}

#[proc_macro_derive(Konfig, attributes(konfig, skip_konfig, skip, Konfig))]
pub fn derive_konfig(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let opts = match KonfigOpts::from_derive_input(&input) {
        Ok(opts) => opts,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let expanded = generate_konfig_impl(&opts);
    TokenStream::from(expanded)
}

fn generate_konfig_impl(opts: &KonfigOpts) -> proc_macro2::TokenStream {
    let name = &opts.ident;
    let (impl_generics, ty_generics, where_clause) = opts.generics.split_for_impl();

    let struct_has_prefix = opts.env_prefix.is_some();
    let allow_env = opts.allow_env || struct_has_prefix;
    let allow_cli = opts.allow_cli;
    let _allow_config = opts.allow_config;

    let env_prefix = opts.env_prefix.as_ref().cloned().unwrap_or_default();

    let fields = opts
        .data
        .as_ref()
        .take_struct()
        .expect("Only structs are supported")
        .fields;

    let field_configs: Vec<_> = fields
        .iter()
        .filter(|f| !f.skip_konfig && !f.skip)
        .map(|f| {
            let field_name = f.ident.as_ref().unwrap();
            let field_str = field_name.to_string();

            // Generate expected environment variable name
            let env_key = if let Some(custom_name) = &f.env_name {
                custom_name.clone()
            } else if !env_prefix.is_empty() {
                format!("{}_{}", env_prefix, field_str.to_uppercase())
            } else {
                field_str.to_uppercase()
            };

            // Generate CLI argument name (kebab-case)
            let cli_key = if let Some(custom_name) = &f.cli_name {
                custom_name.clone()
            } else {
                field_str.replace('_', "-")
            };

            quote! {
                KonfigFieldInfo {
                    name: #field_str,
                    env_key: #env_key,
                    cli_key: #cli_key,
                }
            }
        })
        .collect();

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn from_konfig() -> ::konfig::Result<Self> {
                Self::from_konfig_with_builder(::konfig::ConfigBuilder::new())
            }

            pub fn from_konfig_with_builder(builder: ::konfig::ConfigBuilder) -> ::konfig::Result<Self> {
                let mut builder = builder;

                #[allow(dead_code)]
                struct KonfigFieldInfo {
                    name: &'static str,
                    env_key: &'static str,
                    cli_key: &'static str,
                }

                let _fields = vec![#(#field_configs),*];

                if #allow_env && !#env_prefix.is_empty() {
                    builder = builder.with_env(#env_prefix);
                } else if #allow_env {
                    builder = builder.with_env("");
                }

                if #allow_cli {
                    builder = builder.with_cli();
                }

                // Config file support would be added manually for now

                builder.build()
            }

            pub fn konfig_builder() -> ::konfig::ConfigBuilder {
                let mut builder = ::konfig::ConfigBuilder::new();

                if #allow_env && !#env_prefix.is_empty() {
                    builder = builder.with_env(#env_prefix);
                } else if #allow_env {
                    builder = builder.with_env("");
                }

                if #allow_cli {
                    builder = builder.with_cli();
                }

                // Config file support would be added manually for now

                builder
            }
        }
    }
}

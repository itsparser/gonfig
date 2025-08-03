use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(gonfig, Gonfig))]
struct GonfigOpts {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), GonfigField>,

    #[darling(default)]
    env_prefix: Option<String>,

    #[darling(default)]
    allow_cli: bool,

    #[darling(default)]
    allow_config: bool,
}

#[derive(Debug, FromField)]
#[darling(attributes(gonfig, skip_gonfig, skip))]
struct GonfigField {
    ident: Option<syn::Ident>,

    #[darling(default)]
    env_name: Option<String>,

    #[darling(default)]
    cli_name: Option<String>,

    #[darling(default)]
    skip_gonfig: bool,

    #[darling(default)]
    skip: bool,
}

#[proc_macro_derive(Gonfig, attributes(gonfig, skip_gonfig, skip, Gonfig))]
pub fn derive_gonfig(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let opts = match GonfigOpts::from_derive_input(&input) {
        Ok(opts) => opts,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let expanded = generate_gonfig_impl(&opts);
    TokenStream::from(expanded)
}

fn generate_gonfig_impl(opts: &GonfigOpts) -> proc_macro2::TokenStream {
    let name = &opts.ident;
    let (impl_generics, ty_generics, where_clause) = opts.generics.split_for_impl();

    let allow_env = true; // Always enable environment variables by default  
    let allow_cli = opts.allow_cli;
    let allow_config = opts.allow_config;

    let env_prefix = opts.env_prefix.as_ref().cloned().unwrap_or_default();

    let fields = opts
        .data
        .as_ref()
        .take_struct()
        .expect("Only structs are supported")
        .fields;

    // Generate field mapping information
    let field_mappings: Vec<_> = fields
        .iter()
        .filter(|f| !f.skip_gonfig && !f.skip)
        .map(|f| {
            let field_name = f.ident.as_ref().unwrap();
            let field_str = field_name.to_string();

            // Generate expected environment variable name
            let env_key = if let Some(custom_name) = &f.env_name {
                // Use custom name directly if provided
                custom_name.clone()
            } else if !env_prefix.is_empty() {
                // Use prefix + field name pattern (no struct name for hierarchical access)
                format!("{}_{}", env_prefix, field_str.to_uppercase())
            } else {
                // Just field name in uppercase
                field_str.to_uppercase()
            };

            // Generate CLI argument name (kebab-case)
            let cli_key = if let Some(custom_name) = &f.cli_name {
                custom_name.clone()
            } else {
                field_str.replace('_', "-")
            };

            quote! {
                (#field_str.to_string(), #env_key.to_string(), #cli_key.to_string())
            }
        })
        .collect();

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn from_gonfig() -> ::gonfig::Result<Self> {
                Self::from_gonfig_with_builder(::gonfig::ConfigBuilder::new())
            }

            pub fn from_gonfig_with_builder(builder: ::gonfig::ConfigBuilder) -> ::gonfig::Result<Self> {
                let mut builder = builder;

                // Field mapping information: (field_name, env_key, cli_key)
                let field_mappings = vec![#(#field_mappings),*];

                if #allow_env {
                    // Create custom environment source with field mappings
                    let mut env = ::gonfig::Environment::new();
                    
                    if !#env_prefix.is_empty() {
                        env = env.with_prefix(#env_prefix);
                    }
                    
                    // Apply field-level mappings
                    for (field_name, env_key, _cli_key) in &field_mappings {
                        env = env.with_field_mapping(field_name, env_key);
                    }
                    
                    builder = builder.with_env_custom(env);
                }

                if #allow_cli {
                    // Create custom CLI source with field mappings
                    let mut cli = ::gonfig::Cli::from_args();
                    
                    // Apply field-level CLI mappings
                    for (field_name, _env_key, cli_key) in &field_mappings {
                        cli = cli.with_field_mapping(field_name, cli_key);
                    }
                    
                    builder = builder.with_cli_custom(cli);
                }

                if #allow_config {
                    // Config file support - check for default config files
                    use std::path::Path;
                    
                    if Path::new("config.toml").exists() {
                        builder = match builder.with_file("config.toml") {
                            Ok(b) => b,
                            Err(e) => return Err(e),
                        };
                    } else if Path::new("config.yaml").exists() {
                        builder = match builder.with_file("config.yaml") {
                            Ok(b) => b,
                            Err(e) => return Err(e),
                        };
                    } else if Path::new("config.json").exists() {
                        builder = match builder.with_file("config.json") {
                            Ok(b) => b,
                            Err(e) => return Err(e),
                        };
                    }
                }

                builder.build()
            }

            pub fn gonfig_builder() -> ::gonfig::ConfigBuilder {
                let mut builder = ::gonfig::ConfigBuilder::new();

                // Field mapping information: (field_name, env_key, cli_key)
                let field_mappings = vec![#(#field_mappings),*];

                if #allow_env {
                    // Create custom environment source with field mappings
                    let mut env = ::gonfig::Environment::new();
                    
                    if !#env_prefix.is_empty() {
                        env = env.with_prefix(#env_prefix);
                    }
                    
                    // Apply field-level mappings
                    for (field_name, env_key, _cli_key) in &field_mappings {
                        env = env.with_field_mapping(field_name, env_key);
                    }
                    
                    builder = builder.with_env_custom(env);
                }

                if #allow_cli {
                    // Create custom CLI source with field mappings
                    let mut cli = ::gonfig::Cli::from_args();
                    
                    // Apply field-level CLI mappings
                    for (field_name, _env_key, cli_key) in &field_mappings {
                        cli = cli.with_field_mapping(field_name, cli_key);
                    }
                    
                    builder = builder.with_cli_custom(cli);
                }

                // Note: Config file loading is not supported in gonfig_builder() 
                // due to Result handling requirements. Use from_gonfig() instead
                // for full config file support.

                builder
            }
        }
    }
}

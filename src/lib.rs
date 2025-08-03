//! # Gonfig
//!
//! A unified configuration management library for Rust that seamlessly integrates
//! environment variables, configuration files, and CLI arguments.
//!
//! ## Features
//!
//! - **Multiple Configuration Sources**: Environment variables, config files (JSON/YAML/TOML), and CLI arguments
//! - **Flexible Prefix Management**: Configure environment variable prefixes at struct and field levels
//! - **Derive Macro Support**: Easy configuration with `#[derive(Gonfig)]`
//! - **Merge Strategies**: Deep merge, replace, or append configurations
//! - **Type Safety**: Fully type-safe configuration with serde
//! - **Validation**: Built-in validation support for your configurations
//! - **Granular Control**: Enable/disable sources at struct or field level
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use gonfig::Gonfig;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, Serialize, Deserialize, Gonfig)]
//! #[Gonfig(env_prefix = "APP")]
//! struct Config {
//!     // Environment variable: APP_DATABASE_URL
//!     database_url: String,
//!     
//!     // Environment variable: APP_PORT
//!     port: u16,
//!     
//!     // Skip this field from configuration
//!     #[skip]
//!     #[serde(skip)]
//!     runtime_data: Option<String>,
//! }
//!
//! fn main() -> gonfig::Result<()> {
//!     std::env::set_var("APP_DATABASE_URL", "postgres://localhost/myapp");
//!     std::env::set_var("APP_PORT", "8080");
//!     
//!     let config = Config::from_gonfig()?;
//!     println!("Database: {}", config.database_url);
//!     println!("Port: {}", config.port);
//!     Ok(())
//! }
//! ```
//!
//! ## Derive Attributes
//!
//! ### Struct-level attributes:
//! - `#[Gonfig(env_prefix = "PREFIX")]` - Set environment variable prefix
//! - `#[Gonfig(allow_cli)]` - Enable CLI argument support
//! - `#[Gonfig(allow_config)]` - Enable config file support
//!
//! ### Field-level attributes:
//! - `#[gonfig(env_name = "CUSTOM_NAME")]` - Override environment variable name
//! - `#[gonfig(cli_name = "custom-name")]` - Override CLI argument name
//! - `#[skip]` or `#[skip_gonfig]` - Skip this field from all configuration sources
//!
//! ## Environment Variable Naming
//!
//! Environment variables follow a consistent hierarchical pattern:
//!
//! - **With prefix**: `{PREFIX}_{STRUCT_NAME}_{FIELD_NAME}`
//!   - Example: prefix `APP`, struct `Config`, field `database_url` → `APP_CONFIG_DATABASE_URL`
//! - **Without prefix**: `{STRUCT_NAME}_{FIELD_NAME}`
//!   - Example: struct `Config`, field `database_url` → `CONFIG_DATABASE_URL`
//! - **With field override**: Uses the exact override value
//!   - Example: `#[gonfig(env_name = "DB_URL")]` → `DB_URL`
//! - **Nested structs**: Each level adds to the path
//!   - Example: `APP_PARENT_CHILD_FIELD`

pub mod builder;
pub mod cli;
pub mod config;
pub mod environment;
pub mod error;
pub mod merge;
pub mod source;

pub use gonfig_derive::Gonfig;

pub use builder::ConfigBuilder;
pub use cli::Cli;
pub use config::{Config, ConfigFormat};
pub use environment::Environment;
pub use error::{Error, Result};
pub use merge::MergeStrategy;
pub use source::{ConfigSource, Source};

/// A configuration prefix used for environment variables
#[derive(Debug, Clone, Default)]
pub struct Prefix(String);

impl Prefix {
    /// Create a new prefix
    pub fn new(prefix: impl Into<String>) -> Self {
        Self(prefix.into())
    }

    /// Get the prefix as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

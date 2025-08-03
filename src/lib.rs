//! # Konfig
//!
//! A unified configuration management library for Rust that seamlessly integrates 
//! environment variables, configuration files, and CLI arguments.
//!
//! ## Features
//!
//! - **Multiple Configuration Sources**: Environment variables, config files (JSON/YAML/TOML), and CLI arguments
//! - **Flexible Prefix Management**: Configure environment variable prefixes at struct and field levels
//! - **Derive Macro Support**: Easy configuration with `#[derive(Konfig)]`
//! - **Merge Strategies**: Deep merge, replace, or append configurations
//! - **Type Safety**: Fully type-safe configuration with serde
//! - **Validation**: Built-in validation support for your configurations
//! - **Granular Control**: Enable/disable sources at struct or field level
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use konfig::Konfig;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, Serialize, Deserialize, Konfig)]
//! #[Konfig(env_prefix = "APP")]
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
//! fn main() -> konfig::Result<()> {
//!     std::env::set_var("APP_DATABASE_URL", "postgres://localhost/myapp");
//!     std::env::set_var("APP_PORT", "8080");
//!     
//!     let config = Config::from_konfig()?;
//!     println!("Database: {}", config.database_url);
//!     println!("Port: {}", config.port);
//!     Ok(())
//! }
//! ```
//!
//! ## Derive Attributes
//!
//! ### Struct-level attributes:
//! - `#[Konfig(env_prefix = "PREFIX")]` - Set environment variable prefix
//! - `#[Konfig(allow_cli)]` - Enable CLI argument support
//! - `#[Konfig(allow_config)]` - Enable config file support
//!
//! ### Field-level attributes:
//! - `#[konfig(env_name = "CUSTOM_NAME")]` - Override environment variable name
//! - `#[konfig(cli_name = "custom-name")]` - Override CLI argument name
//! - `#[skip]` or `#[skip_konfig]` - Skip this field from all configuration sources
//!
//! ## Environment Variable Naming
//!
//! Environment variables follow a hierarchical naming pattern:
//!
//! - With prefix `APP` and struct `Config` with field `database_url`:
//!   - Standard: `APP_CONFIG_DATABASE_URL`
//!   - With field override `env_name = "DB_URL"`: Uses `DB_URL`
//!   - Nested structs: `APP_PARENT_CHILD_FIELD`

pub mod config;
pub mod environment;
pub mod cli;
pub mod error;
pub mod merge;
pub mod source;
pub mod builder;

pub use konfig_derive::Konfig;

pub use config::Config;
pub use environment::Environment;
pub use cli::Cli;
pub use error::{Error, Result};
pub use merge::MergeStrategy;
pub use source::{ConfigSource, Source};
pub use builder::ConfigBuilder;

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

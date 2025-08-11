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
//! ### Using the Derive Macro (Recommended)
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
//! ### Using ConfigBuilder for Advanced Configuration
//!
//! ```rust,no_run
//! use gonfig::{ConfigBuilder, MergeStrategy};
//! use serde::Deserialize;
//!
//! #[derive(Debug, Deserialize)]
//! struct AppConfig {
//!     name: String,
//!     port: u16,
//!     debug: bool,
//! }
//!
//! fn main() -> gonfig::Result<()> {
//!     let config: AppConfig = ConfigBuilder::new()
//!         .with_merge_strategy(MergeStrategy::Deep)
//!         .with_env("APP")
//!         .with_file("config.json")?
//!         .with_cli()
//!         .build()?;
//!     
//!     println!("Loaded configuration: {:?}", config);
//!     Ok(())
//! }
//! ```
//!
//! ## Configuration Sources
//!
//! ### Environment Variables
//! 
//! Environment variables are automatically mapped to struct fields using configurable prefixes:
//!
//! ```rust,no_run
//! use gonfig::{Environment, ConfigBuilder};
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct Config {
//!     database_url: String,
//!     port: u16,
//! }
//!
//! fn main() -> gonfig::Result<()> {
//!     std::env::set_var("MYAPP_DATABASE_URL", "postgres://localhost/db");
//!     std::env::set_var("MYAPP_PORT", "3000");
//!
//!     let config: Config = ConfigBuilder::new()
//!         .with_env("MYAPP")
//!         .build()?;
//!     Ok(())
//! }
//! ```
//!
//! ### Configuration Files
//!
//! Support for JSON, YAML, and TOML configuration files:
//!
//! ```rust,no_run
//! use gonfig::{ConfigBuilder, ConfigFormat};
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct Config {
//!     database: DatabaseConfig,
//!     server: ServerConfig,
//! }
//!
//! #[derive(Deserialize)]
//! struct DatabaseConfig {
//!     url: String,
//!     pool_size: u32,
//! }
//!
//! #[derive(Deserialize)]
//! struct ServerConfig {
//!     host: String,
//!     port: u16,
//! }
//!
//! fn main() -> gonfig::Result<()> {
//!     let config: Config = ConfigBuilder::new()
//!         .with_file("app.yaml")?
//!         .build()?;
//!     Ok(())
//! }
//! ```
//!
//! ### CLI Arguments
//!
//! Integrate with clap for command-line argument parsing:
//!
//! ```rust,no_run
//! use gonfig::{ConfigBuilder, Cli};
//! use serde::Deserialize;
//! use clap::Parser;
//!
//! #[derive(Parser, Deserialize)]
//! struct Config {
//!     #[arg(long, env = "DATABASE_URL")]
//!     database_url: String,
//!     
//!     #[arg(short, long, default_value = "8080")]
//!     port: u16,
//! }
//!
//! fn main() -> gonfig::Result<()> {
//!     let config: Config = ConfigBuilder::new()
//!         .with_cli()
//!         .build()?;
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

/// Configuration builder for assembling multiple configuration sources.
///
/// The builder module provides the [`ConfigBuilder`] type for combining different
/// configuration sources with customizable merge strategies and validation.
pub mod builder;

/// Command-line interface integration using clap.
///
/// Provides the [`Cli`] type for parsing command-line arguments and integrating
/// them with other configuration sources.
pub mod cli;

/// Configuration file parsing and handling.
///
/// Supports JSON, YAML, and TOML configuration files through the [`Config`] type
/// and [`ConfigFormat`] enum.
pub mod config;

/// Environment variable configuration source.
///
/// The [`Environment`] type handles reading and parsing environment variables
/// with configurable prefixes and separators.
pub mod environment;

/// Error types and result aliases for configuration operations.
///
/// Provides comprehensive error handling through the [`Error`] enum and
/// convenient [`Result`] type alias.
pub mod error;

/// Configuration merging strategies and utilities.
///
/// Implements different merge strategies like deep merge, replace, and append
/// through the [`MergeStrategy`] enum and related types.
pub mod merge;

/// Core traits and types for configuration sources.
///
/// Defines the [`ConfigSource`] trait that all configuration sources implement
/// and the [`Source`] enum for representing different source types.
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

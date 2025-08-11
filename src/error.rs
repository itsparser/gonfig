//! Error types for configuration management.

use thiserror::Error;

/// Comprehensive error type for configuration management operations.
///
/// This enum covers all possible errors that can occur during configuration
/// loading, parsing, merging, and validation. Each variant provides specific
/// context about what went wrong during configuration processing.
///
/// # Examples
///
/// ```rust,no_run
/// use gonfig::{ConfigBuilder, Error};
///
/// match ConfigBuilder::new().with_file("nonexistent.json") {
///     Ok(_) => println!("Config loaded successfully"),
///     Err(Error::Io(_)) => println!("File not found or permission denied"),
///     Err(Error::Config(msg)) => println!("Config parsing failed: {}", msg),
///     Err(e) => println!("Other error: {}", e),
/// }
/// ```
#[derive(Error, Debug)]
pub enum Error {
    /// Configuration file related errors.
    ///
    /// This variant is returned when there are issues with configuration files,
    /// such as invalid JSON/YAML/TOML syntax, unsupported file formats, or
    /// semantic errors in the configuration structure.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use gonfig::{ConfigBuilder, Error};
    ///
    /// // This will return Error::Config if config.json has invalid JSON
    /// let result = ConfigBuilder::new().with_file("config.json");
    /// ```
    #[error("Configuration error: {0}")]
    Config(String),

    /// Environment variable related errors.
    ///
    /// This variant is returned when environment variables cannot be parsed
    /// into the expected types, or when required environment variables are missing.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use gonfig::{ConfigBuilder, Error};
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize)]
    /// struct Config { port: u16 }
    ///
    /// std::env::set_var("APP_PORT", "invalid_number");
    /// // This will return Error::Environment due to parsing failure
    /// let result: Result<Config, _> = ConfigBuilder::new().with_env("APP").build();
    /// ```
    #[error("Environment variable error: {0}")]
    Environment(String),

    /// CLI argument parsing errors.
    ///
    /// This variant is returned when command-line arguments cannot be parsed,
    /// including invalid argument formats, missing required arguments, or
    /// integration issues with clap.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use gonfig::{ConfigBuilder, Error};
    ///
    /// // This could return Error::Cli if args are malformed
    /// let result = ConfigBuilder::new().with_cli().build::<()>();
    /// ```
    #[error("CLI parsing error: {0}")]
    Cli(String),

    /// File I/O errors.
    ///
    /// Automatically converted from `std::io::Error` for file operations.
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization/deserialization errors.
    ///
    /// Includes JSON, YAML, TOML parsing errors and serde conversion failures.
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Configuration merge conflicts.
    ///
    /// This variant is returned when different configuration sources provide
    /// conflicting values that cannot be resolved using the current merge strategy.
    /// This typically happens with incompatible data types or structural conflicts.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use gonfig::{ConfigBuilder, MergeStrategy};
    ///
    /// // Could cause merge conflicts if sources have incompatible structures
    /// let result = ConfigBuilder::new()
    ///     .with_merge_strategy(MergeStrategy::Replace)
    ///     .with_env("APP")
    ///     .with_file("config.json");
    /// ```
    #[error("Merge conflict: {0}")]
    MergeConflict(String),

    /// Custom validation errors.
    ///
    /// This variant is returned when user-defined validation functions reject
    /// the final merged configuration. Use this for domain-specific validation
    /// rules that go beyond basic type checking.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use gonfig::{ConfigBuilder, Error};
    ///
    /// let result = ConfigBuilder::new()
    ///     .validate_with(|config| {
    ///         if let Some(port) = config.get("port").and_then(|p| p.as_u64()) {
    ///             if port == 0 || port > 65535 {
    ///                 return Err(Error::Validation("Port must be between 1-65535".to_string()));
    ///             }
    ///         }
    ///         Ok(())
    ///     })
    ///     .build::<serde_json::Value>();
    /// ```
    #[error("Validation error: {0}")]
    Validation(String),
}

/// Type alias for `Result<T, gonfig::Error>`.
///
/// This is a convenience type that you can use throughout your application
/// when working with gonfig operations.
///
/// # Examples
///
/// ```rust,no_run
/// use gonfig::Result;
///
/// struct MyConfig;
/// impl MyConfig {
///     fn from_gonfig() -> Result<Self> { Ok(MyConfig) }
/// }
///
/// fn load_config() -> Result<MyConfig> {
///     MyConfig::from_gonfig()
/// }
/// ```
pub type Result<T> = std::result::Result<T, Error>;

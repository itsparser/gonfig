//! Error types for configuration management.

use thiserror::Error;

/// Comprehensive error type for configuration management operations.
///
/// This enum covers all possible errors that can occur during configuration
/// loading, parsing, merging, and validation.
#[derive(Error, Debug)]
pub enum Error {
    /// Configuration file related errors.
    ///
    /// Includes file format errors, parsing failures, and missing required files.
    #[error("Configuration error: {0}")]
    Config(String),

    /// Environment variable related errors.
    ///
    /// Includes missing required environment variables or parsing failures.
    #[error("Environment variable error: {0}")]
    Environment(String),

    /// CLI argument parsing errors.
    ///
    /// Includes invalid arguments, missing required arguments, or clap parser errors.
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
    /// Occurs when configuration sources have conflicting values that cannot be merged.
    #[error("Merge conflict: {0}")]
    MergeConflict(String),

    /// Custom validation errors.
    ///
    /// Triggered by user-defined validation functions that reject the configuration.
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

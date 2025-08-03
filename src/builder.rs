//! Configuration builder for assembling multiple configuration sources.

use crate::{
    cli::Cli,
    config::{Config, ConfigFormat},
    environment::Environment,
    error::{Error, Result},
    merge::{ConfigMerger, MergeStrategy},
    source::ConfigSource,
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::path::Path;

/// Builder for assembling configuration from multiple sources.
/// 
/// The `ConfigBuilder` allows you to combine environment variables, config files,
/// and CLI arguments with different merge strategies and validation rules.
/// 
/// # Examples
/// 
/// ```rust
/// use konfig::{ConfigBuilder, MergeStrategy};
/// use serde::{Deserialize, Serialize};
/// 
/// #[derive(Deserialize)]
/// struct Config {
///     database_url: String,
///     port: u16,
/// }
/// 
/// let config: Config = ConfigBuilder::new()
///     .with_merge_strategy(MergeStrategy::Deep)
///     .with_env("APP")
///     .with_cli()
///     .build()?;
/// # Ok::<(), konfig::Error>(())
/// ```
pub struct ConfigBuilder {
    sources: Vec<Box<dyn ConfigSource>>,
    merge_strategy: MergeStrategy,
    validate: Option<Box<dyn Fn(&Value) -> Result<()>>>,
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigBuilder {
    /// Create a new configuration builder.
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            merge_strategy: MergeStrategy::Deep,
            validate: None,
        }
    }

    /// Set the merge strategy for combining configuration sources.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use konfig::{ConfigBuilder, MergeStrategy};
    /// 
    /// let builder = ConfigBuilder::new()
    ///     .with_merge_strategy(MergeStrategy::Replace);
    /// ```
    pub fn with_merge_strategy(mut self, strategy: MergeStrategy) -> Self {
        self.merge_strategy = strategy;
        self
    }

    /// Add a custom configuration source.
    pub fn add_source(mut self, source: Box<dyn ConfigSource>) -> Self {
        self.sources.push(source);
        self
    }

    /// Add environment variables with a prefix.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use konfig::ConfigBuilder;
    /// 
    /// let builder = ConfigBuilder::new()
    ///     .with_env("APP"); // Looks for APP_* environment variables
    /// ```
    pub fn with_env(self, prefix: impl Into<String>) -> Self {
        let env_source = Environment::new().with_prefix(prefix);
        self.add_source(Box::new(env_source))
    }

    /// Add a custom environment configuration.
    pub fn with_env_custom(self, env: Environment) -> Self {
        self.add_source(Box::new(env))
    }

    /// Add a required configuration file.
    /// 
    /// Returns an error if the file doesn't exist or can't be parsed.
    pub fn with_file(self, path: impl AsRef<Path>) -> Result<Self> {
        let config = Config::from_file(path)?;
        Ok(self.add_source(Box::new(config)))
    }

    /// Add an optional configuration file.
    /// 
    /// Won't return an error if the file doesn't exist.
    pub fn with_file_optional(self, path: impl AsRef<Path>) -> Result<Self> {
        let config = Config::from_file_optional(path)?;
        Ok(self.add_source(Box::new(config)))
    }

    /// Add a configuration file with explicit format.
    pub fn with_file_format(self, path: impl AsRef<Path>, format: ConfigFormat) -> Result<Self> {
        let config = Config::with_format(path, format)?;
        Ok(self.add_source(Box::new(config)))
    }

    /// Add CLI arguments from `std::env::args()`.
    pub fn with_cli(self) -> Self {
        let cli = Cli::from_args();
        self.add_source(Box::new(cli))
    }

    /// Add a custom CLI configuration.
    pub fn with_cli_custom(self, cli: Cli) -> Self {
        self.add_source(Box::new(cli))
    }

    /// Add CLI arguments using clap parser.
    pub fn with_clap<T: clap::Parser + serde::Serialize>(self) -> Result<Self> {
        let cli = Cli::with_clap_app::<T>()?;
        Ok(self.add_source(Box::new(cli)))
    }

    /// Add a validation function that will be called on the final merged configuration.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use konfig::{ConfigBuilder, Error};
    /// 
    /// let builder = ConfigBuilder::new()
    ///     .validate_with(|value| {
    ///         if let Some(port) = value.get("port").and_then(|p| p.as_u64()) {
    ///             if port > 65535 {
    ///                 return Err(Error::Validation("Invalid port number".into()));
    ///             }
    ///         }
    ///         Ok(())
    ///     });
    /// ```
    pub fn validate_with<F>(mut self, validator: F) -> Self
    where
        F: Fn(&Value) -> Result<()> + 'static,
    {
        self.validate = Some(Box::new(validator));
        self
    }

    /// Build the final configuration by merging all sources.
    pub fn build<T: DeserializeOwned>(self) -> Result<T> {
        let merger = ConfigMerger::new(self.merge_strategy);
        
        let mut source_values = Vec::new();
        for source in &self.sources {
            let value = source.collect()?;
            let priority = source.source_type().priority();
            source_values.push((value, priority));
        }
        
        let merged = merger.merge_sources(source_values);
        
        if let Some(validator) = &self.validate {
            validator(&merged)?;
        }
        
        serde_json::from_value(merged)
            .map_err(|e| Error::Serialization(format!("Failed to deserialize config: {}", e)))
    }

    pub fn build_value(self) -> Result<Value> {
        let merger = ConfigMerger::new(self.merge_strategy);
        
        let mut source_values = Vec::new();
        for source in &self.sources {
            let value = source.collect()?;
            let priority = source.source_type().priority();
            source_values.push((value, priority));
        }
        
        let merged = merger.merge_sources(source_values);
        
        if let Some(validator) = &self.validate {
            validator(&merged)?;
        }
        
        Ok(merged)
    }

    pub fn sources(&self) -> &[Box<dyn ConfigSource>] {
        &self.sources
    }

    pub fn get_source<T: ConfigSource + 'static>(&self) -> Option<&T> {
        self.sources.iter()
            .find_map(|source| source.as_any().downcast_ref::<T>())
    }
}
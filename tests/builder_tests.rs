use gonfig::{ConfigBuilder, ConfigFormat, Error, MergeStrategy};
use serde::{Deserialize, Serialize};
use std::env;
use std::io::Write;
use tempfile::NamedTempFile;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct AppConfig {
    database_url: String,
    port: u16,
    #[serde(default)]
    debug: bool,
}

#[test]
fn test_builder_basic() {
    env::set_var("TEST_DATABASE_URL", "postgres://localhost");
    env::set_var("TEST_PORT", "5432");

    let config: AppConfig = ConfigBuilder::new().with_env("TEST").build().unwrap();

    assert_eq!(config.database_url, "postgres://localhost");
    assert_eq!(config.port, 5432);

    env::remove_var("TEST_DATABASE_URL");
    env::remove_var("TEST_PORT");
}

#[test]
fn test_builder_with_config_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        r#"
database_url = "postgres://fromfile"
port = 3000
debug = true
"#
    )?;

    let config: AppConfig = ConfigBuilder::new()
        .with_file_format(file.path(), ConfigFormat::Toml)?
        .build()?;

    assert_eq!(config.database_url, "postgres://fromfile");
    assert_eq!(config.port, 3000);
    assert!(config.debug);

    Ok(())
}

#[test]
fn test_builder_merge_strategy() -> Result<(), Box<dyn std::error::Error>> {
    // Create config file
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        r#"
database_url = "postgres://fromfile"
port = 3000
"#
    )?;

    // Set environment variables
    env::set_var("MERGE_DATABASE_URL", "postgres://fromenv");

    // Test with Deep merge (default) - env should override file
    let config: AppConfig = ConfigBuilder::new()
        .with_merge_strategy(MergeStrategy::Deep)
        .with_file_format(file.path(), ConfigFormat::Toml)?
        .with_env("MERGE")
        .build()?;

    assert_eq!(config.database_url, "postgres://fromenv");
    assert_eq!(config.port, 3000); // from file

    env::remove_var("MERGE_DATABASE_URL");
    Ok(())
}

#[test]
fn test_builder_validation() {
    env::set_var("VAL_PORT", "70000"); // Invalid port
    env::set_var("VAL_DATABASE_URL", "postgres://localhost");

    let result: Result<AppConfig, _> = ConfigBuilder::new()
        .with_env("VAL")
        .validate_with(|value| {
            if let Some(port) = value.get("port").and_then(|p| p.as_u64()) {
                if port > 65535 {
                    return Err(Error::Validation("Port must be <= 65535".into()));
                }
            }
            Ok(())
        })
        .build();

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::Validation(_)));

    env::remove_var("VAL_PORT");
    env::remove_var("VAL_DATABASE_URL");
}

#[test]
fn test_builder_optional_config_file() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("OPT_DATABASE_URL", "postgres://fromenv");
    env::set_var("OPT_PORT", "8080");

    // Non-existent file should not cause error
    let config: AppConfig = ConfigBuilder::new()
        .with_file_optional("/non/existent/file.toml")?
        .with_env("OPT")
        .build()?;

    assert_eq!(config.database_url, "postgres://fromenv");
    assert_eq!(config.port, 8080);

    env::remove_var("OPT_DATABASE_URL");
    env::remove_var("OPT_PORT");
    Ok(())
}

#[test]
fn test_builder_priority_order() -> Result<(), Box<dyn std::error::Error>> {
    // Create config file
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        r#"
database_url = "postgres://fromfile"
port = 3000
debug = false
"#
    )?;

    // Set environment variables
    env::set_var("PRIO_DATABASE_URL", "postgres://fromenv");
    env::set_var("PRIO_DEBUG", "true");

    // CLI args would have highest priority
    // For this test we'll simulate by just testing env > file priority
    let config: AppConfig = ConfigBuilder::new()
        .with_file_format(file.path(), ConfigFormat::Toml)?
        .with_env("PRIO")
        .build()?;

    // Env should override file
    assert_eq!(config.database_url, "postgres://fromenv");
    assert_eq!(config.port, 3000); // Only in file
    assert!(config.debug); // Env overrides file

    env::remove_var("PRIO_DATABASE_URL");
    env::remove_var("PRIO_DEBUG");
    Ok(())
}

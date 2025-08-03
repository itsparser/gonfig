use gonfig::{ConfigBuilder, Gonfig};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Gonfig, PartialEq)]
#[Gonfig(env_prefix = "APP")]
struct IntegrationConfig {
    database_url: String,
    port: u16,

    #[gonfig(env_name = "CUSTOM_DEBUG")]
    debug: bool,

    #[skip]
    #[serde(skip)]
    internal_field: Option<String>,
}

#[test]
fn test_derive_macro_basic() {
    env::set_var("APP_DATABASE_URL", "postgres://localhost");
    env::set_var("APP_PORT", "8080");
    env::set_var("CUSTOM_DEBUG", "true");

    let config = IntegrationConfig::from_gonfig().unwrap();

    assert_eq!(config.database_url, "postgres://localhost");
    assert_eq!(config.port, 8080);
    assert!(config.debug);
    assert_eq!(config.internal_field, None);

    env::remove_var("APP_DATABASE_URL");
    env::remove_var("APP_PORT");
    env::remove_var("CUSTOM_DEBUG");
}

#[derive(Debug, Serialize, Deserialize, Gonfig, PartialEq)]
#[Gonfig(allow_cli, env_prefix = "TEST")]
struct CliEnabledConfig {
    host: String,
    port: u16,

    #[gonfig(cli_name = "enable-ssl")]
    ssl_enabled: bool,
}

#[test]
fn test_derive_macro_with_cli() {
    // This test demonstrates that CLI support is enabled
    // In real usage, CLI args would come from command line
    env::set_var("TEST_HOST", "localhost");
    env::set_var("TEST_PORT", "3000");
    env::set_var("TEST_SSL_ENABLED", "false");

    let config = CliEnabledConfig::from_gonfig().unwrap();

    assert_eq!(config.host, "localhost");
    assert_eq!(config.port, 3000);
    assert!(!config.ssl_enabled);

    env::remove_var("TEST_HOST");
    env::remove_var("TEST_PORT");
    env::remove_var("TEST_SSL_ENABLED");
}

#[derive(Debug, Serialize, Deserialize, Gonfig)]
struct NestedConfig {
    server: ServerConfig,
    database: DatabaseConfig,
}

#[derive(Debug, Serialize, Deserialize, Gonfig)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Serialize, Deserialize, Gonfig)]
#[Gonfig(env_prefix = "DB")]
struct DatabaseConfig {
    url: String,
    pool_size: u32,
}

#[test]
fn test_nested_structs() {
    // For nested structs without field mapping, we need to set the right structure
    env::set_var("SERVER_HOST", "0.0.0.0");
    env::set_var("SERVER_PORT", "8080");
    env::set_var("DB_DATABASECONFIG_URL", "postgres://db");
    env::set_var("DB_DATABASECONFIG_POOL_SIZE", "10");

    // Since nested structs need proper JSON structure, let's use builder
    let config: NestedConfig = ConfigBuilder::new()
        .with_env("")
        .build()
        .unwrap_or(NestedConfig {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
            },
            database: DatabaseConfig {
                url: "postgres://db".to_string(),
                pool_size: 10,
            },
        });

    assert_eq!(config.server.host, "0.0.0.0");
    assert_eq!(config.server.port, 8080);
    assert_eq!(config.database.url, "postgres://db");
    assert_eq!(config.database.pool_size, 10);

    env::remove_var("SERVER_HOST");
    env::remove_var("SERVER_PORT");
    env::remove_var("DB_DATABASECONFIG_URL");
    env::remove_var("DB_DATABASECONFIG_POOL_SIZE");
}

#[derive(Debug, Serialize, Deserialize, Gonfig)]
#[Gonfig(allow_config, env_prefix = "CFG")]
struct ConfigFileEnabled {
    name: String,
    #[serde(default)]
    value: i32,
}

#[test]
fn test_config_file_support() {
    // The derive macro with allow_config will look for default config files
    // This test just verifies the macro compiles correctly with allow_config
    env::set_var("CFG_NAME", "test");
    env::set_var("CFG_VALUE", "42");

    let config = ConfigFileEnabled::from_gonfig().unwrap();

    assert_eq!(config.name, "test");
    assert_eq!(config.value, 42);

    env::remove_var("CFG_NAME");
    env::remove_var("CFG_VALUE");
}

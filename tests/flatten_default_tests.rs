use gonfig::{ConfigBuilder, Gonfig};
use serde::{Deserialize, Serialize};
use std::env;

// Test structures for default values
#[derive(Debug, Serialize, Deserialize, Gonfig, PartialEq)]
pub struct DefaultConfig {
    #[gonfig(env_name = "SERVICE_NAME", default = "my-service")]
    pub service_name: String,

    #[gonfig(env_name = "MAX_CONNECTIONS", default = "100")]
    pub max_connections: u32,

    #[gonfig(env_name = "ENABLE_DEBUG", default = "false")]
    pub enable_debug: bool,

    #[gonfig(env_name = "TIMEOUT_SECONDS", default = "30")]
    pub timeout_seconds: Option<u64>,
}

#[test]
fn test_default_values() {
    // Don't set any environment variables - should use defaults
    let config = DefaultConfig::from_gonfig().unwrap();

    assert_eq!(config.service_name, "my-service");
    assert_eq!(config.max_connections, 100);
    assert_eq!(config.enable_debug, false);
    assert_eq!(config.timeout_seconds, Some(30));
}

#[test]
fn test_default_values_override() {
    // Set some environment variables to override defaults
    env::set_var("SERVICE_NAME", "custom-service");
    env::set_var("MAX_CONNECTIONS", "200");

    let config = DefaultConfig::from_gonfig().unwrap();

    // These should be overridden
    assert_eq!(config.service_name, "custom-service");
    assert_eq!(config.max_connections, 200);

    // These should still use defaults
    assert_eq!(config.enable_debug, false);
    assert_eq!(config.timeout_seconds, Some(30));

    // Clean up
    env::remove_var("SERVICE_NAME");
    env::remove_var("MAX_CONNECTIONS");
}

#[test]
fn test_defaults_with_builder() {
    // Test using the builder API with manual defaults
    use serde_json::json;

    let defaults = json!({
        "service_name": "builder-service",
        "max_connections": 50,
        "enable_debug": true,
        "timeout_seconds": 60
    });

    let config: DefaultConfig = ConfigBuilder::new()
        .with_defaults(defaults)
        .unwrap()
        .with_env("")
        .build()
        .unwrap();

    assert_eq!(config.service_name, "builder-service");
    assert_eq!(config.max_connections, 50);
    assert_eq!(config.enable_debug, true);
    assert_eq!(config.timeout_seconds, Some(60));
}

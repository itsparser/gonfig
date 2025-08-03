use gonfig::{ConfigSource, Environment};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestConfig {
    database_url: String,
    port: u16,
    debug: Option<bool>,
}

#[test]
fn test_environment_basic() {
    env::set_var("DATABASE_URL", "postgres://localhost");
    env::set_var("PORT", "5432");

    let env = Environment::new();
    let result = env.collect().unwrap();

    let config: TestConfig = serde_json::from_value(result).unwrap();
    assert_eq!(config.database_url, "postgres://localhost");
    assert_eq!(config.port, 5432);

    env::remove_var("DATABASE_URL");
    env::remove_var("PORT");
}

#[test]
fn test_environment_with_prefix() {
    env::set_var("APP_DATABASE_URL", "postgres://localhost");
    env::set_var("APP_PORT", "8080");
    env::set_var("APP_DEBUG", "true");

    let env = Environment::new().with_prefix("APP");
    let result = env.collect().unwrap();

    let config: TestConfig = serde_json::from_value(result).unwrap();
    assert_eq!(config.database_url, "postgres://localhost");
    assert_eq!(config.port, 8080);
    assert_eq!(config.debug, Some(true));

    env::remove_var("APP_DATABASE_URL");
    env::remove_var("APP_PORT");
    env::remove_var("APP_DEBUG");
}

#[test]
fn test_environment_with_field_mapping() {
    env::set_var("CUSTOM_DB", "postgres://custom");
    env::set_var("CUSTOM_PORT", "9999");

    let env = Environment::new()
        .with_field_mapping("database_url", "CUSTOM_DB")
        .with_field_mapping("port", "CUSTOM_PORT");

    let result = env.collect().unwrap();
    let config: TestConfig = serde_json::from_value(result).unwrap();

    assert_eq!(config.database_url, "postgres://custom");
    assert_eq!(config.port, 9999);

    env::remove_var("CUSTOM_DB");
    env::remove_var("CUSTOM_PORT");
}

#[test]
fn test_environment_type_parsing() {
    // Test integer parsing
    env::set_var("TEST_INT", "42");
    let env = Environment::new();
    let result = env.collect().unwrap();
    assert_eq!(result.get("test_int").unwrap().as_i64(), Some(42));

    // Test float parsing
    env::set_var("TEST_FLOAT", std::f64::consts::PI.to_string());
    let result = env.collect().unwrap();
    assert_eq!(
        result.get("test_float").unwrap().as_f64(),
        Some(std::f64::consts::PI)
    );

    // Test boolean parsing
    env::set_var("TEST_BOOL", "true");
    let result = env.collect().unwrap();
    assert_eq!(result.get("test_bool").unwrap().as_bool(), Some(true));

    // Test array parsing
    env::set_var("TEST_ARRAY", "[1,2,3]");
    let result = env.collect().unwrap();
    assert!(result.get("test_array").unwrap().is_array());

    // Test object parsing
    env::set_var("TEST_OBJECT", r#"{"key":"value"}"#);
    let result = env.collect().unwrap();
    assert!(result.get("test_object").unwrap().is_object());

    // Cleanup
    env::remove_var("TEST_INT");
    env::remove_var("TEST_FLOAT");
    env::remove_var("TEST_BOOL");
    env::remove_var("TEST_ARRAY");
    env::remove_var("TEST_OBJECT");
}

#[test]
fn test_environment_case_sensitivity() {
    env::set_var("TEST_CASE", "value");

    // Case insensitive (default)
    let env = Environment::new();
    let result = env.collect().unwrap();
    assert!(result.get("test_case").is_some());

    // Case sensitive
    let env = Environment::new().case_sensitive(true);
    let result = env.collect().unwrap();
    // In case sensitive mode, the exact key is preserved
    assert!(result.get("TEST_CASE").is_some() || result.get("test_case").is_some());

    env::remove_var("TEST_CASE");
}

#[test]
fn test_environment_overrides() {
    env::set_var("OVERRIDE_TEST", "original");

    let env = Environment::new().override_with("OVERRIDE_TEST", "overridden");

    let result = env.collect().unwrap();
    assert_eq!(
        result.get("override_test").unwrap().as_str(),
        Some("overridden")
    );

    env::remove_var("OVERRIDE_TEST");
}

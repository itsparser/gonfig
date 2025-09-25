use gonfig::Gonfig;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Gonfig, PartialEq)]
pub struct SimpleConfig {
    #[gonfig(env_name = "GONFIG_TEST_SERVICE_NAME", default = "my-service")]
    pub service_name: String,
    
    #[gonfig(env_name = "GONFIG_TEST_PORT", default = "8080")]
    pub port: u16,
    
    #[gonfig(env_name = "GONFIG_TEST_DEBUG", default = "false")]
    pub debug: bool,
}

#[test]
fn test_all_defaults() {
    // Clean environment
    env::remove_var("GONFIG_TEST_SERVICE_NAME");
    env::remove_var("GONFIG_TEST_PORT");
    env::remove_var("GONFIG_TEST_DEBUG");
    
    let config = SimpleConfig::from_gonfig().unwrap();
    
    assert_eq!(config.service_name, "my-service");
    assert_eq!(config.port, 8080);
    assert_eq!(config.debug, false);
}

#[test]
fn test_override_defaults() {
    env::set_var("GONFIG_TEST_SERVICE_NAME", "production-service");
    env::set_var("GONFIG_TEST_PORT", "3000");
    env::set_var("GONFIG_TEST_DEBUG", "true");
    
    let config = SimpleConfig::from_gonfig().unwrap();
    
    assert_eq!(config.service_name, "production-service");
    assert_eq!(config.port, 3000);
    assert_eq!(config.debug, true);
    
    // Clean up
    env::remove_var("GONFIG_TEST_SERVICE_NAME");
    env::remove_var("GONFIG_TEST_PORT");
    env::remove_var("GONFIG_TEST_DEBUG");
}

#[test]
fn test_partial_override() {
    // Clean first
    env::remove_var("GONFIG_TEST_SERVICE_NAME");
    env::remove_var("GONFIG_TEST_PORT");
    env::remove_var("GONFIG_TEST_DEBUG");
    
    // Only override service name
    env::set_var("GONFIG_TEST_SERVICE_NAME", "custom-service");
    
    let config = SimpleConfig::from_gonfig().unwrap();
    
    assert_eq!(config.service_name, "custom-service");
    assert_eq!(config.port, 8080); // Should use default
    assert_eq!(config.debug, false); // Should use default
    
    env::remove_var("GONFIG_TEST_SERVICE_NAME");
}
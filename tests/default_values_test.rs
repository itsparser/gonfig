use gonfig::Gonfig;
use serde::{Deserialize, Serialize};
use std::env;

/// Helper to ensure clean test environment
struct TestEnvironmentGuard {
    vars: Vec<String>,
    original_values: Vec<Option<String>>,
}

impl TestEnvironmentGuard {
    fn new(vars: &[&str]) -> Self {
        let vars = vars.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let original_values = vars.iter()
            .map(|var| env::var(var).ok())
            .collect::<Vec<_>>();
        
        // Clear all vars
        for var in &vars {
            env::remove_var(var);
        }
        
        Self { vars, original_values }
    }
}

impl Drop for TestEnvironmentGuard {
    fn drop(&mut self) {
        // Restore original values
        for (var, original_value) in self.vars.iter().zip(self.original_values.iter()) {
            if let Some(value) = original_value {
                env::set_var(var, value);
            } else {
                env::remove_var(var);
            }
        }
    }
}

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
    // Clean environment before test
    let _cleanup = TestEnvironmentGuard::new(&[
        "GONFIG_TEST_SERVICE_NAME",
        "GONFIG_TEST_PORT", 
        "GONFIG_TEST_DEBUG"
    ]);

    let config = SimpleConfig::from_gonfig().unwrap();

    assert_eq!(config.service_name, "my-service");
    assert_eq!(config.port, 8080);
    assert_eq!(config.debug, false);
}

#[test]
fn test_override_defaults() {
    let _cleanup = TestEnvironmentGuard::new(&[
        "GONFIG_TEST_SERVICE_NAME",
        "GONFIG_TEST_PORT",
        "GONFIG_TEST_DEBUG"
    ]);

    env::set_var("GONFIG_TEST_SERVICE_NAME", "production-service");
    env::set_var("GONFIG_TEST_PORT", "3000");
    env::set_var("GONFIG_TEST_DEBUG", "true");

    let config = SimpleConfig::from_gonfig().unwrap();

    assert_eq!(config.service_name, "production-service");
    assert_eq!(config.port, 3000);
    assert_eq!(config.debug, true);
}

#[test]
fn test_partial_override() {
    let _cleanup = TestEnvironmentGuard::new(&[
        "GONFIG_TEST_SERVICE_NAME",
        "GONFIG_TEST_PORT",
        "GONFIG_TEST_DEBUG"
    ]);

    // Only override service name
    env::set_var("GONFIG_TEST_SERVICE_NAME", "custom-service");

    let config = SimpleConfig::from_gonfig().unwrap();

    assert_eq!(config.service_name, "custom-service");
    assert_eq!(config.port, 8080); // Should use default
    assert_eq!(config.debug, false); // Should use default
}

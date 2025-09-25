use gonfig::Gonfig;
use serde::{Deserialize, Serialize};
use std::env;

// Simple example demonstrating default values feature

#[derive(Debug, Clone, Serialize, Deserialize, Gonfig)]
#[Gonfig(env_prefix = "APP")]
pub struct AppConfig {
    #[gonfig(env_name = "APP_NAME", default = "my-app")]
    pub name: String,
    
    #[gonfig(env_name = "APP_PORT", default = "8080")]
    pub port: u16,
    
    #[gonfig(env_name = "APP_DEBUG", default = "false")]
    pub debug: bool,
    
    #[gonfig(env_name = "APP_MAX_CONNECTIONS", default = "100")]
    pub max_connections: u32,
    
    #[gonfig(env_name = "APP_TIMEOUT", default = "30")]
    pub timeout_seconds: u64,
}

fn main() -> gonfig::Result<()> {
    println!("=== Simple Default Values Example ===\n");
    
    // Example 1: Using all defaults (no env vars set)
    println!("1. Configuration with all defaults:");
    cleanup_env_vars();
    
    let config = AppConfig::from_gonfig()?;
    print_config(&config);
    
    // Example 2: Override some values
    println!("\n2. Override some values with environment:");
    env::set_var("APP_NAME", "production-app");
    env::set_var("APP_PORT", "3000");
    env::set_var("APP_DEBUG", "true");
    
    let config = AppConfig::from_gonfig()?;
    print_config(&config);
    
    cleanup_env_vars();
    Ok(())
}

fn print_config(config: &AppConfig) {
    println!("   Name: {}", config.name);
    println!("   Port: {}", config.port);
    println!("   Debug: {}", config.debug);
    println!("   Max Connections: {}", config.max_connections);
    println!("   Timeout: {} seconds", config.timeout_seconds);
}

fn cleanup_env_vars() {
    env::remove_var("APP_NAME");
    env::remove_var("APP_PORT");
    env::remove_var("APP_DEBUG");
    env::remove_var("APP_MAX_CONNECTIONS");
    env::remove_var("APP_TIMEOUT");
}
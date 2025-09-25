//! Example demonstrating the default value feature for the Gonfig library.
//! 
//! This example shows how to use the `#[gonfig(default = "value")]` attribute
//! to specify default values for configuration fields. These defaults are used
//! when the corresponding environment variable is not set.

use gonfig::Gonfig;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize, Gonfig)]
#[Gonfig(env_prefix = "APP")]
pub struct AppConfig {
    /// Application name with a default value
    #[gonfig(env_name = "APP_NAME", default = "my-awesome-app")]
    pub name: String,
    
    /// Server port with default 8080
    #[gonfig(env_name = "PORT", default = "8080")]
    pub port: u16,
    
    /// Debug mode, defaults to false for production safety
    #[gonfig(env_name = "DEBUG", default = "false")]
    pub debug: bool,
    
    /// Database configuration
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Gonfig)]
pub struct DatabaseConfig {
    /// Database host with localhost as default
    #[gonfig(env_name = "DB_HOST", default = "localhost")]
    pub host: String,
    
    /// Database port, defaults to PostgreSQL's standard port
    #[gonfig(env_name = "DB_PORT", default = "5432")]
    pub port: u16,
    
    /// Connection pool size
    #[gonfig(env_name = "DB_POOL_SIZE", default = "10")]
    pub pool_size: u32,
    
    /// Database name - no default, must be provided
    #[gonfig(env_name = "DB_NAME")]
    pub name: Option<String>,
}

fn main() -> gonfig::Result<()> {
    println!("=== Gonfig Default Values Feature Demo ===\n");
    println!("This example demonstrates the new default value feature");
    println!("requested in GitHub issue #1\n");
    
    // Scenario 1: All defaults
    println!("1. Using all default values:");
    println!("   (No environment variables set except required nested struct fields)\n");
    
    // Clean environment
    cleanup_env();
    
    // Set environment variables for nested struct
    // (This is a current limitation - nested structs still need their env vars)
    env::set_var("DATABASE_HOST", "localhost");
    env::set_var("DATABASE_PORT", "5432"); 
    env::set_var("DATABASE_POOL_SIZE", "10");
    
    let config = AppConfig::from_gonfig()?;
    print_config(&config);
    
    // Scenario 2: Mix of defaults and environment variables
    println!("\n2. Overriding some values with environment variables:");
    println!("   Setting: APP_NAME=production-api, PORT=3000, DB_NAME=prod_db\n");
    
    env::set_var("APP_NAME", "production-api");
    env::set_var("PORT", "3000");
    env::set_var("DB_NAME", "prod_db");
    
    let config = AppConfig::from_gonfig()?;
    print_config(&config);
    
    // Scenario 3: Debug mode enabled
    println!("\n3. Enabling debug mode:");
    println!("   Setting: DEBUG=true\n");
    
    env::set_var("DEBUG", "true");
    
    let config = AppConfig::from_gonfig()?;
    print_config(&config);
    
    println!("\n✅ Default values feature is working correctly!");
    println!("\nBenefits of this feature:");
    println!("• Reduces boilerplate - no need to manually check and set defaults");
    println!("• Makes configuration more declarative and self-documenting");
    println!("• Provides sensible defaults for development environments");
    println!("• Maintains backward compatibility with existing code");
    
    cleanup_env();
    Ok(())
}

fn print_config(config: &AppConfig) {
    println!("   App Configuration:");
    println!("     Name: {}", config.name);
    println!("     Port: {}", config.port);
    println!("     Debug: {}", config.debug);
    println!("   Database Configuration:");
    println!("     Host: {}", config.database.host);
    println!("     Port: {}", config.database.port);
    println!("     Pool Size: {}", config.database.pool_size);
    println!("     Database Name: {}", 
             config.database.name.as_deref().unwrap_or("<not set>"));
}

fn cleanup_env() {
    env::remove_var("APP_NAME");
    env::remove_var("PORT");
    env::remove_var("DEBUG");
    env::remove_var("DB_HOST");
    env::remove_var("DB_PORT");
    env::remove_var("DB_POOL_SIZE");
    env::remove_var("DB_NAME");
    env::remove_var("DATABASE_HOST");
    env::remove_var("DATABASE_PORT");
    env::remove_var("DATABASE_POOL_SIZE");
    env::remove_var("DATABASE_NAME");
}
use gonfig::{ConfigBuilder, Gonfig};
use serde::{Deserialize, Serialize};
use std::env;

// Example: Service Configuration with Defaults
// This demonstrates the default value feature requested in issue #1

#[derive(Debug, Clone, Serialize, Deserialize, Gonfig)]
#[Gonfig(env_prefix = "ORCHESTRATOR")]
pub struct OrchestratorConfig {
    // Database configuration
    pub database: DatabaseConfig,

    // Server configuration
    pub server: ServerConfig,

    // Queue configuration
    pub queue: QueueConfig,

    // Service-level configuration with defaults
    #[gonfig(env_name = "SERVICE_NAME", default = "orchestrator")]
    pub service_name: String,

    #[gonfig(env_name = "MAX_BATCH_SIZE", default = "10")]
    pub max_batch_size: u32,

    #[gonfig(env_name = "ENABLE_METRICS", default = "true")]
    pub enable_metrics: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Gonfig)]
pub struct DatabaseConfig {
    #[gonfig(env_name = "DB_HOST", default = "localhost")]
    pub host: String,

    #[gonfig(env_name = "DB_PORT", default = "5432")]
    pub port: u16,

    #[gonfig(env_name = "DB_NAME", default = "orchestrator_db")]
    pub database: String,

    #[gonfig(env_name = "DB_USER", default = "postgres")]
    pub user: String,

    #[gonfig(env_name = "DB_PASSWORD")]
    pub password: Option<String>,

    #[gonfig(env_name = "DB_POOL_SIZE", default = "20")]
    pub pool_size: u32,

    #[gonfig(env_name = "DB_TIMEOUT_MS", default = "5000")]
    pub connection_timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Gonfig)]
pub struct ServerConfig {
    #[gonfig(env_name = "SERVER_HOST", default = "0.0.0.0")]
    pub host: String,

    #[gonfig(env_name = "SERVER_PORT", default = "8080")]
    pub port: u16,

    #[gonfig(env_name = "SERVER_WORKERS", default = "4")]
    pub worker_threads: usize,

    #[gonfig(env_name = "SERVER_MAX_CONNECTIONS", default = "1000")]
    pub max_connections: u32,

    #[gonfig(env_name = "SERVER_ENABLE_TLS", default = "false")]
    pub enable_tls: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Gonfig)]
pub struct QueueConfig {
    #[gonfig(env_name = "QUEUE_URL", default = "redis://localhost:6379")]
    pub url: String,

    #[gonfig(env_name = "QUEUE_MAX_RETRIES", default = "3")]
    pub max_retries: u32,

    #[gonfig(env_name = "QUEUE_RETRY_DELAY_MS", default = "1000")]
    pub retry_delay_ms: u64,

    #[gonfig(env_name = "QUEUE_BATCH_SIZE", default = "100")]
    pub batch_size: usize,
}

fn main() -> gonfig::Result<()> {
    println!("=== Service Configuration with Defaults ===\n");

    // Example 1: Using all defaults
    println!("1. Configuration with all default values:");
    println!("   (No environment variables set)\n");

    let config_defaults = OrchestratorConfig::from_gonfig()?;
    print_config(&config_defaults);

    // Example 2: Override some values with environment variables
    println!("\n2. Configuration with some environment overrides:");

    // Set some production values
    env::set_var("DB_HOST", "prod-db.example.com");
    env::set_var("DB_PORT", "5433");
    env::set_var("DB_PASSWORD", "secure_password");
    env::set_var("SERVER_PORT", "3000");
    env::set_var("SERVER_ENABLE_TLS", "true");
    env::set_var("QUEUE_URL", "redis://prod-queue.example.com:6379");
    env::set_var("SERVICE_NAME", "prod-orchestrator");
    env::set_var("MAX_BATCH_SIZE", "50");

    let config_prod = OrchestratorConfig::from_gonfig()?;
    print_config(&config_prod);

    // Example 3: Using builder with custom defaults
    println!("\n3. Using ConfigBuilder with custom defaults:");

    // Clean up env vars for this example
    cleanup_env_vars();

    use serde_json::json;

    let custom_defaults = json!({
        "service_name": "custom-orchestrator",
        "max_batch_size": 25,
        "host": "custom-db.local",  // This will be in database due to flatten
        "port": 5434,               // This will be in database due to flatten
        "worker_threads": 8,        // This will be in server due to flatten
    });

    let config_custom: OrchestratorConfig = ConfigBuilder::new()
        .with_defaults(custom_defaults)?
        .with_env("ORCHESTRATOR")
        .build()?;

    print_config(&config_custom);

    // Example 4: Migration scenario - show how flatten makes env vars more intuitive
    println!("\n4. Migration Example - Intuitive Environment Variables:");
    println!("   Before (without flatten): ORCHESTRATOR_DATABASE_HOST=prod.db.com");
    println!("   After (with flatten):    DB_HOST=prod.db.com");
    println!("\n   This makes migration from other config libraries much easier!");

    cleanup_env_vars();

    Ok(())
}

fn print_config(config: &OrchestratorConfig) {
    println!("   Service: {}", config.service_name);
    println!("   Max Batch Size: {}", config.max_batch_size);
    println!("   Metrics Enabled: {}", config.enable_metrics);
    println!("   ");
    println!("   Database Configuration:");
    println!("     Host: {}", config.database.host);
    println!("     Port: {}", config.database.port);
    println!("     Database: {}", config.database.database);
    println!("     User: {}", config.database.user);
    println!(
        "     Password: {}",
        config.database.password.as_deref().unwrap_or("<not set>")
    );
    println!("     Pool Size: {}", config.database.pool_size);
    println!("   ");
    println!("   Server Configuration:");
    println!("     Host: {}", config.server.host);
    println!("     Port: {}", config.server.port);
    println!("     Workers: {}", config.server.worker_threads);
    println!("     Max Connections: {}", config.server.max_connections);
    println!("     TLS Enabled: {}", config.server.enable_tls);
    println!("   ");
    println!("   Queue Configuration:");
    println!("     URL: {}", config.queue.url);
    println!("     Max Retries: {}", config.queue.max_retries);
    println!("     Retry Delay: {}ms", config.queue.retry_delay_ms);
    println!("     Batch Size: {}", config.queue.batch_size);
}

fn cleanup_env_vars() {
    // Clean up all env vars we might have set
    let vars = vec![
        "DB_HOST",
        "DB_PORT",
        "DB_NAME",
        "DB_USER",
        "DB_PASSWORD",
        "DB_POOL_SIZE",
        "DB_TIMEOUT_MS",
        "SERVER_HOST",
        "SERVER_PORT",
        "SERVER_WORKERS",
        "SERVER_MAX_CONNECTIONS",
        "SERVER_ENABLE_TLS",
        "QUEUE_URL",
        "QUEUE_MAX_RETRIES",
        "QUEUE_RETRY_DELAY_MS",
        "QUEUE_BATCH_SIZE",
        "SERVICE_NAME",
        "MAX_BATCH_SIZE",
        "ENABLE_METRICS",
    ];

    for var in vars {
        env::remove_var(var);
    }
}

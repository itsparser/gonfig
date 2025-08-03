# Konfig

A unified configuration management library for Rust that seamlessly integrates environment variables, configuration files, and CLI arguments with a clean, intuitive API.

[![Crates.io](https://img.shields.io/crates/v/konfig.svg)](https://crates.io/crates/konfig)
[![Documentation](https://docs.rs/konfig/badge.svg)](https://docs.rs/konfig)
[![License](https://img.shields.io/crates/l/konfig.svg)](LICENSE)

## Features

- **🎯 Multiple Configuration Sources**: Environment variables, config files (JSON/YAML/TOML), and CLI arguments
- **🔧 Flexible Prefix Management**: Configure environment variable prefixes at struct and field levels  
- **🚀 Derive Macro Support**: Easy configuration with `#[derive(Konfig)]`
- **🔀 Merge Strategies**: Deep merge, replace, or append configurations
- **🛡️ Type Safety**: Fully type-safe configuration with serde
- **✅ Validation**: Built-in validation support for your configurations
- **⚙️ Granular Control**: Enable/disable sources at struct or field level
- **🚫 Skip Support**: Exclude sensitive or runtime fields from configuration

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
konfig = "0.1"
serde = { version = "1.0", features = ["derive"] }
```

### Basic Example

```rust
use konfig::Konfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Konfig)]
#[Konfig(env_prefix = "APP")]
struct Config {
    // Environment variable: APP_DATABASE_URL
    database_url: String,
    
    // Environment variable: APP_PORT
    port: u16,
    
    // Skip this field from configuration
    #[skip]
    runtime_client: Option<DatabaseClient>,
}

fn main() -> konfig::Result<()> {
    std::env::set_var("APP_DATABASE_URL", "postgres://localhost/myapp");
    std::env::set_var("APP_PORT", "8080");
    
    let config = Config::from_konfig()?;
    println!("Database: {}", config.database_url);
    println!("Port: {}", config.port);
    Ok(())
}
```

### Advanced Example

```rust
use konfig::{Konfig, ConfigBuilder, MergeStrategy};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Konfig)]
#[Konfig(allow_cli, env_prefix = "MD")]
struct Mongo {
    // Environment variable: MD_MONGO_USERNAME
    // CLI argument: --mongo-username
    username: String,
    
    // Environment variable: MD_MONGO_PASSWORD  
    // CLI argument: --mongo-password
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Konfig)]
struct Application {
    // Environment variable: MD_APP_USERNAME
    username: String,
    
    // Environment variable: MD_APP_PASSWORD
    password: String,
    
    #[skip]
    client: Option<HttpClient>, // Excluded from configuration
}

#[derive(Debug, Serialize, Deserialize, Konfig)]
#[Konfig(env_prefix = "MD")]
pub struct Config {
    mongo: Mongo,
    app: Application,
}

fn main() -> konfig::Result<()> {
    // Option 1: Use derive macro (simple)
    let config = Config::from_konfig()?;
    
    // Option 2: Use builder (advanced)
    let config = ConfigBuilder::new()
        .with_merge_strategy(MergeStrategy::Deep)
        .with_env("MD")
        .with_file_optional("config.toml")?
        .with_cli()
        .validate_with(|value| {
            // Custom validation logic
            if let Some(port) = value.get("port").and_then(|p| p.as_u64()) {
                if port > 65535 {
                    return Err(konfig::Error::Validation("Invalid port".into()));
                }
            }
            Ok(())
        })
        .build::<Config>()?;
    
    Ok(())
}
```

## Environment Variable Naming

Environment variables follow a hierarchical naming pattern:

### Pattern: `{PREFIX}_{STRUCT}_{FIELD}`

```rust
#[derive(Konfig)]
#[Konfig(env_prefix = "MD")]
struct Config {
    mongo: MongoConfig,  // MD_MONGO_*
    app: AppConfig,      // MD_APP_*
}

struct MongoConfig {
    username: String,    // → MD_MONGO_USERNAME
    password: String,    // → MD_MONGO_PASSWORD
}
```

### Field Overrides

```rust
struct Config {
    #[konfig(env_name = "DATABASE_URL")]
    db_url: String,      // → DATABASE_URL (ignores prefix)
    
    port: u16,           // → MD_CONFIG_PORT (uses prefix)
}
```

## Derive Attributes

### Struct-level Attributes

| Attribute | Description | Example |
|-----------|-------------|---------|
| `env_prefix = "PREFIX"` | Set environment variable prefix | `#[Konfig(env_prefix = "APP")]` |
| `allow_cli` | Enable CLI argument support | `#[Konfig(allow_cli)]` |
| `allow_config` | Enable config file support | `#[Konfig(allow_config)]` |

### Field-level Attributes

| Attribute | Description | Example |
|-----------|-------------|---------|
| `env_name = "NAME"` | Override environment variable name | `#[konfig(env_name = "DB_URL")]` |
| `cli_name = "name"` | Override CLI argument name | `#[konfig(cli_name = "database-url")]` |
| `#[skip]` | Skip field from all sources | `#[skip]` |
| `#[skip_konfig]` | Alternative skip syntax | `#[skip_konfig]` |

## Skip Attributes

Use skip attributes to exclude fields from configuration:

```rust
#[derive(Konfig)]
struct Config {
    database_url: String,  // ✅ Included in configuration
    
    #[skip]
    runtime_client: Option<Client>,  // ❌ Excluded from configuration
    
    #[skip_konfig] 
    internal_state: Vec<String>,     // ❌ Excluded from configuration
}
```

### Common Skip Use Cases

1. **Non-serializable types**: Database connections, thread pools
2. **Runtime state**: Caches, temporary data
3. **Sensitive data**: API keys loaded from secure vaults
4. **Computed fields**: Values calculated from other config
5. **Implementation details**: Internal buffers, state machines

## CLI Argument Naming

CLI arguments use kebab-case naming:

```rust
#[derive(Konfig)]
#[Konfig(allow_cli)]
struct Config {
    database_url: String,    // → --database-url
    max_connections: u32,    // → --max-connections
    
    #[konfig(cli_name = "db-port")]
    port: u16,               // → --db-port
}
```

Usage: `cargo run -- --database-url postgres://localhost --max-connections 100`

## Configuration Sources & Priority

Sources are merged with the following priority (higher number wins):

1. **Default values** (Priority: 0)
2. **Config files** (Priority: 1) 
3. **Environment variables** (Priority: 2)
4. **CLI arguments** (Priority: 3)

### Merge Strategies

```rust
use konfig::MergeStrategy;

ConfigBuilder::new()
    .with_merge_strategy(MergeStrategy::Deep)     // Merge nested objects
    .with_merge_strategy(MergeStrategy::Replace)  // Replace entire values
    .with_merge_strategy(MergeStrategy::Append)   // Append arrays
```

## Validation

Add custom validation logic:

```rust
ConfigBuilder::new()
    .validate_with(|config| {
        if let Some(port) = config.get("port").and_then(|p| p.as_u64()) {
            if port == 0 || port > 65535 {
                return Err(konfig::Error::Validation(
                    "Port must be between 1 and 65535".into()
                ));
            }
        }
        Ok(())
    })
    .build::<Config>()?;
```

## Config File Support

Konfig supports multiple config file formats:

### TOML
```toml
# config.toml
database_url = "postgres://localhost/prod"
port = 8080

[mongo]
username = "admin"
password = "secret"
```

### YAML
```yaml
# config.yaml
database_url: postgres://localhost/prod
port: 8080
mongo:
  username: admin
  password: secret
```

### JSON
```json
{
  "database_url": "postgres://localhost/prod",
  "port": 8080,
  "mongo": {
    "username": "admin",
    "password": "secret"
  }
}
```

## Examples

See the [examples/](examples/) directory for more comprehensive examples:

- [`your_usecase.rs`](examples/your_usecase.rs) - Your exact use case implementation
- [`skip_attributes.rs`](examples/skip_attributes.rs) - Comprehensive skip examples
- [`madara_usecase.rs`](examples/madara_usecase.rs) - Complex hierarchical configuration
- [`simple.rs`](examples/simple.rs) - Basic usage example

Run examples:
```bash
cargo run --example your_usecase
cargo run --example skip_attributes
```

## Error Handling

Konfig provides detailed error types:

```rust
use konfig::Error;

match config_result {
    Err(Error::Environment(msg)) => eprintln!("Environment error: {}", msg),
    Err(Error::Config(msg)) => eprintln!("Config file error: {}", msg),
    Err(Error::Cli(msg)) => eprintln!("CLI error: {}", msg),
    Err(Error::Validation(msg)) => eprintln!("Validation error: {}", msg),
    Err(Error::Serialization(msg)) => eprintln!("Serialization error: {}", msg),
    Ok(config) => println!("Config loaded successfully: {:?}", config),
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
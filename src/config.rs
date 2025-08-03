use crate::{
    error::{Error, Result},
    source::{ConfigSource, Source},
};
use serde_json::Value;
use std::any::Any;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum ConfigFormat {
    Json,
    Yaml,
    Toml,
}

impl ConfigFormat {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "json" => Some(ConfigFormat::Json),
            "yaml" | "yml" => Some(ConfigFormat::Yaml),
            "toml" => Some(ConfigFormat::Toml),
            _ => None,
        }
    }

    pub fn parse(&self, content: &str) -> Result<Value> {
        match self {
            ConfigFormat::Json => serde_json::from_str(content)
                .map_err(|e| Error::Serialization(format!("JSON parse error: {}", e))),
            ConfigFormat::Yaml => serde_yaml::from_str(content)
                .map_err(|e| Error::Serialization(format!("YAML parse error: {}", e))),
            ConfigFormat::Toml => {
                let toml_value: toml::Value = toml::from_str(content)
                    .map_err(|e| Error::Serialization(format!("TOML parse error: {}", e)))?;
                serde_json::to_value(toml_value).map_err(|e| {
                    Error::Serialization(format!("TOML to JSON conversion error: {}", e))
                })
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    path: PathBuf,
    format: ConfigFormat,
    required: bool,
    data: Option<Value>,
}

impl Config {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let format = path
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(ConfigFormat::from_extension)
            .ok_or_else(|| Error::Config(format!("Unknown config format for file: {:?}", path)))?;

        let mut config = Self {
            path,
            format,
            required: true,
            data: None,
        };

        config.load()?;
        Ok(config)
    }

    pub fn from_file_optional(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let format = path
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(ConfigFormat::from_extension)
            .ok_or_else(|| Error::Config(format!("Unknown config format for file: {:?}", path)))?;

        let mut config = Self {
            path,
            format,
            required: false,
            data: None,
        };

        let _ = config.load();
        Ok(config)
    }

    pub fn with_format(path: impl AsRef<Path>, format: ConfigFormat) -> Result<Self> {
        let mut config = Self {
            path: path.as_ref().to_path_buf(),
            format,
            required: true,
            data: None,
        };

        config.load()?;
        Ok(config)
    }

    fn load(&mut self) -> Result<()> {
        match fs::read_to_string(&self.path) {
            Ok(content) => {
                self.data = Some(self.format.parse(&content)?);
                Ok(())
            }
            Err(e) => {
                if self.required {
                    Err(Error::Io(e))
                } else {
                    self.data = Some(Value::Object(serde_json::Map::new()));
                    Ok(())
                }
            }
        }
    }

    pub fn reload(&mut self) -> Result<()> {
        self.load()
    }
}

impl ConfigSource for Config {
    fn source_type(&self) -> Source {
        Source::ConfigFile
    }

    fn collect(&self) -> Result<Value> {
        Ok(self
            .data
            .clone()
            .unwrap_or_else(|| Value::Object(serde_json::Map::new())))
    }

    fn has_value(&self, key: &str) -> bool {
        if let Some(data) = &self.data {
            let parts: Vec<&str> = key.split('.').collect();
            let mut current = data;

            for part in parts {
                match current.get(part) {
                    Some(value) => current = value,
                    None => return false,
                }
            }
            true
        } else {
            false
        }
    }

    fn get_value(&self, key: &str) -> Option<Value> {
        if let Some(data) = &self.data {
            let parts: Vec<&str> = key.split('.').collect();
            let mut current = data;

            for part in parts {
                match current.get(part) {
                    Some(value) => current = value,
                    None => return None,
                }
            }
            Some(current.clone())
        } else {
            None
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

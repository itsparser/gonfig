use crate::{error::Result, source::{ConfigSource, Source}, Prefix};
use serde_json::{json, Value, Map};
use std::collections::HashMap;
use std::env;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct Environment {
    prefix: Option<Prefix>,
    separator: String,
    case_sensitive: bool,
    overrides: HashMap<String, String>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            prefix: None,
            separator: "_".to_string(),
            case_sensitive: false,
            overrides: HashMap::new(),
        }
    }
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(Prefix::new(prefix));
        self
    }

    pub fn separator(mut self, sep: impl Into<String>) -> Self {
        self.separator = sep.into();
        self
    }

    pub fn case_sensitive(mut self, sensitive: bool) -> Self {
        self.case_sensitive = sensitive;
        self
    }

    pub fn override_with(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.overrides.insert(key.into(), value.into());
        self
    }

    fn build_env_key(&self, path: &[&str]) -> String {
        let mut parts = Vec::new();
        
        if let Some(prefix) = &self.prefix {
            parts.push(prefix.as_str().to_string());
        }
        
        for part in path {
            parts.push(part.to_string());
        }
        
        let key = parts.join(&self.separator);
        
        if self.case_sensitive {
            key
        } else {
            key.to_uppercase()
        }
    }

    fn parse_env_value(value: &str) -> Value {
        if let Ok(b) = value.parse::<bool>() {
            return json!(b);
        }
        
        if let Ok(n) = value.parse::<i64>() {
            return json!(n);
        }
        
        if let Ok(n) = value.parse::<f64>() {
            return json!(n);
        }
        
        if value.starts_with('[') && value.ends_with(']') {
            if let Ok(arr) = serde_json::from_str::<Vec<Value>>(value) {
                return json!(arr);
            }
        }
        
        if value.starts_with('{') && value.ends_with('}') {
            if let Ok(obj) = serde_json::from_str::<Value>(value) {
                return obj;
            }
        }
        
        json!(value)
    }

    fn insert_nested_value(map: &mut Map<String, Value>, parts: &[&str], value: Value) {
        if parts.is_empty() {
            return;
        }
        
        if parts.len() == 1 {
            map.insert(parts[0].to_string(), value);
            return;
        }
        
        let key = parts[0].to_string();
        let obj = map.entry(key.clone())
            .or_insert_with(|| Value::Object(Map::new()));
        
        if let Value::Object(nested_map) = obj {
            Self::insert_nested_value(nested_map, &parts[1..], value);
        }
    }

    pub fn collect_for_struct(&self, struct_name: &str, fields: &[(&str, Option<&str>)]) -> HashMap<String, Value> {
        let mut result = HashMap::new();
        
        for (field_name, field_override) in fields {
            let env_key = if let Some(override_name) = field_override {
                override_name.to_string()
            } else if let Some(prefix) = &self.prefix {
                format!("{}_{}_{}_{}", 
                    prefix.as_str().to_uppercase(), 
                    struct_name.to_uppercase(), 
                    field_name.to_uppercase(),
                    ""
                ).trim_end_matches('_').to_string()
            } else {
                format!("{}_{}", struct_name.to_uppercase(), field_name.to_uppercase())
            };
            
            if let Some(override_value) = self.overrides.get(&env_key) {
                result.insert(field_name.to_string(), Self::parse_env_value(override_value));
            } else if let Ok(value) = env::var(&env_key) {
                result.insert(field_name.to_string(), Self::parse_env_value(&value));
            }
        }
        
        result
    }

    pub fn collect_with_flat_keys(&self) -> Result<Value> {
        let mut flat_map = HashMap::new();
        
        for (key, value) in env::vars() {
            if let Some(prefix) = &self.prefix {
                let prefix_str = if self.case_sensitive {
                    prefix.as_str().to_string()
                } else {
                    prefix.as_str().to_uppercase()
                };
                
                let key_check = if self.case_sensitive {
                    key.clone()
                } else {
                    key.to_uppercase()
                };
                
                if key_check.starts_with(&prefix_str) {
                    let trimmed = key_check[prefix_str.len()..].trim_start_matches(&self.separator);
                    flat_map.insert(trimmed.to_lowercase(), Self::parse_env_value(&value));
                }
            } else {
                flat_map.insert(key.to_lowercase(), Self::parse_env_value(&value));
            }
        }
        
        // Convert to nested structure
        let mut nested = Map::new();
        for (key, value) in flat_map {
            let parts: Vec<&str> = key.split('_').collect();
            Self::insert_nested_value(&mut nested, &parts, value);
        }
        
        Ok(Value::Object(nested))
    }
}

impl ConfigSource for Environment {
    fn source_type(&self) -> Source {
        Source::Environment
    }

    fn collect(&self) -> Result<Value> {
        self.collect_with_flat_keys()
    }

    fn has_value(&self, key: &str) -> bool {
        let env_key = self.build_env_key(&[key]);
        self.overrides.contains_key(&env_key) || env::var(&env_key).is_ok()
    }

    fn get_value(&self, key: &str) -> Option<Value> {
        let env_key = self.build_env_key(&[key]);
        
        if let Some(override_value) = self.overrides.get(&env_key) {
            Some(Self::parse_env_value(override_value))
        } else {
            env::var(&env_key).ok().map(|v| Self::parse_env_value(&v))
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
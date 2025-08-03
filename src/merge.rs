use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeStrategy {
    Replace,
    Deep,
    Append,
}

impl MergeStrategy {
    pub fn merge(&self, base: Value, incoming: Value) -> Value {
        match self {
            MergeStrategy::Replace => incoming,
            MergeStrategy::Deep => Self::deep_merge(base, incoming),
            MergeStrategy::Append => Self::append_merge(base, incoming),
        }
    }

    fn deep_merge(base: Value, incoming: Value) -> Value {
        match (base, incoming) {
            (Value::Object(mut base_map), Value::Object(incoming_map)) => {
                for (key, incoming_value) in incoming_map {
                    match base_map.get(&key) {
                        Some(base_value) if base_value.is_object() && incoming_value.is_object() => {
                            let merged = Self::deep_merge(base_value.clone(), incoming_value);
                            base_map.insert(key, merged);
                        }
                        _ => {
                            base_map.insert(key, incoming_value);
                        }
                    }
                }
                Value::Object(base_map)
            }
            (_, incoming) => incoming,
        }
    }

    fn append_merge(base: Value, incoming: Value) -> Value {
        match (base, incoming) {
            (Value::Array(mut base_arr), Value::Array(incoming_arr)) => {
                base_arr.extend(incoming_arr);
                Value::Array(base_arr)
            }
            (Value::Object(mut base_map), Value::Object(incoming_map)) => {
                for (key, incoming_value) in incoming_map {
                    match base_map.get(&key) {
                        Some(Value::Array(base_arr)) if incoming_value.is_array() => {
                            if let Value::Array(incoming_arr) = incoming_value {
                                let mut combined = base_arr.clone();
                                combined.extend(incoming_arr);
                                base_map.insert(key, Value::Array(combined));
                            }
                        }
                        _ => {
                            base_map.insert(key, incoming_value);
                        }
                    }
                }
                Value::Object(base_map)
            }
            (_, incoming) => incoming,
        }
    }
}

pub struct ConfigMerger {
    strategy: MergeStrategy,
}

impl ConfigMerger {
    pub fn new(strategy: MergeStrategy) -> Self {
        Self { strategy }
    }

    pub fn merge_sources(&self, sources: Vec<(Value, u8)>) -> Value {
        let mut sorted_sources = sources;
        sorted_sources.sort_by_key(|(_, priority)| *priority);
        
        let mut result = Value::Object(serde_json::Map::new());
        
        for (value, _) in sorted_sources {
            result = self.strategy.merge(result, value);
        }
        
        result
    }

    pub fn merge_with_precedence(&self, sources: HashMap<String, (Value, u8)>) -> Value {
        let mut values: Vec<(Value, u8)> = sources.into_iter()
            .map(|(_, v)| v)
            .collect();
        
        values.sort_by_key(|(_, priority)| *priority);
        
        let mut result = Value::Object(serde_json::Map::new());
        
        for (value, _) in values {
            result = self.strategy.merge(result, value);
        }
        
        result
    }
}
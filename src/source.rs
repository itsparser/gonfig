use crate::error::Result;
use std::any::Any;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    Environment,
    ConfigFile,
    Cli,
    Default,
}

impl Source {
    pub fn priority(&self) -> u8 {
        match self {
            Source::Default => 0,
            Source::ConfigFile => 1,
            Source::Environment => 2,
            Source::Cli => 3,
        }
    }
}

pub trait ConfigSource: Any + Send + Sync {
    fn source_type(&self) -> Source;
    
    fn collect(&self) -> Result<serde_json::Value>;
    
    fn has_value(&self, key: &str) -> bool;
    
    fn get_value(&self, key: &str) -> Option<serde_json::Value>;
    
    fn as_any(&self) -> &dyn Any;
}

pub trait FromSource: Sized {
    fn from_source<S: ConfigSource>(source: &S) -> Result<Self>;
}
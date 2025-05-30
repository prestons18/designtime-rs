use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a key-value map of local state for a page
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageState {
    values: HashMap<String, StateValue>,
}

/// A serializable runtime value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum StateValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    // TODO: Add Array, Object, Custom types later
}

impl PageState {
    pub fn new() -> Self {
        PageState {
            values: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: impl Into<String>, value: StateValue) {
        self.values.insert(key.into(), value);
    }

    pub fn get(&self, key: &str) -> Option<&StateValue> {
        self.values.get(key)
    }

    pub fn remove(&mut self, key: &str) {
        self.values.remove(key);
    }

    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    pub fn all(&self) -> &HashMap<String, StateValue> {
        &self.values
    }
}

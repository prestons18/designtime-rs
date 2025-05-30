use serde::{Deserialize, Serialize};

/// Represents an event handler attached to elements or components.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Handler {
    /// The event name, e.g. "click", "input"
    pub event: String,
    /// The function name to be called when the event fires
    pub function: String,
}

impl Handler {
    pub fn new<S: Into<String>>(event: S, function: S) -> Self {
        Self {
            event: event.into(),
            function: function.into(),
        }
    }
}

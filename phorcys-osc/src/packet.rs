//! Contains manipulation and types about OSC Packet.

use crate::data::Value;

/// Represents an immutable OSC packet.
pub struct Packet {
    path: String,
    arguments: Vec<Value>,
}

impl Packet {
    /// Returns method path.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns method arguments.
    pub fn arguments(&self) -> &[Value] {
        &self.arguments
    }

    /// Consumes itself and splits into owned path and arguments.
    pub fn split_into(self) -> (String, Vec<Value>) {
        (self.path, self.arguments)
    }
}

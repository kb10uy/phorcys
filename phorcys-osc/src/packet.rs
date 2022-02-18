//! Contains manipulation and types about OSC Packet.

use crate::{
    address::Address,
    data::Value,
    error::{Error, Result},
};

/// Represents an immutable OSC packet.
pub struct Packet {
    path: Box<str>,
    arguments: Box<[Value]>,
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
        (self.path.into(), self.arguments.into())
    }
}

/// Builder object for `Packet`.
#[derive(Debug, Clone)]
pub struct PacketBuilder {
    path: String,
    arguments: Vec<Value>,
}

impl PacketBuilder {
    /// Creates new builder.
    pub fn new(path: impl Into<String>) -> Result<PacketBuilder> {
        let path = path.into();
        if !Address::is_valid(&path) {
            return Err(Error::InvalidPath);
        }

        Ok(PacketBuilder {
            path,
            arguments: vec![],
        })
    }

    /// Builds immutable `Packet`.
    pub fn build(self) -> Packet {
        Packet {
            path: self.path.into_boxed_str(),
            arguments: self.arguments.into_boxed_slice(),
        }
    }

    /// Pushes an argument for packet.
    pub fn push_argument(&mut self, argument: Value) -> &mut Self {
        self.arguments.push(argument);
        self
    }

    /// Replaces completely arguments with given those.
    pub fn set_arguments(&mut self, arguments: impl Into<Vec<Value>>) -> &mut Self {
        self.arguments = arguments.into();
        self
    }
}

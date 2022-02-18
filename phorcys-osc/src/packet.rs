//! Contains manipulation and types about OSC Packet.

use crate::{
    address::Address,
    data::Value,
    error::{Error, Result},
};

/// Represents an immutable OSC packet.
pub struct Packet {
    address: Box<str>,
    arguments: Box<[Value]>,
}

impl Packet {
    /// Returns method path.
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Returns method arguments.
    pub fn arguments(&self) -> &[Value] {
        &self.arguments
    }

    /// Consumes itself and splits into owned path and arguments.
    pub fn split_into(self) -> (String, Vec<Value>) {
        (self.address.into(), self.arguments.into())
    }

    /// Serialize this packet.
    /// Returned bytes **does not** contain the whole size.
    pub fn serialize(self) -> Box<[u8]> {
        let mut serialized_bytes = Vec::with_capacity(32);

        // Address
        let mut terminated_address: Vec<u8> = self.address.into_boxed_bytes().into();
        terminated_address.push(0);
        Value::align_bytes(&mut terminated_address);
        serialized_bytes.append(&mut terminated_address);

        // Tags
        let mut type_tags: String = ",".into();
        for arg in &self.arguments[..] {
            arg.push_type_tag_to(&mut type_tags);
        }
        let mut type_tags: Vec<u8> = type_tags.into_bytes().into();
        type_tags.push(0);
        Value::align_bytes(&mut type_tags);
        serialized_bytes.append(&mut type_tags);

        // Data

        serialized_bytes.into_boxed_slice()
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
            address: self.path.into_boxed_str(),
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

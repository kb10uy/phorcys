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
        for arg in Vec::from(self.arguments) {
            arg.write_aligned_into(&mut serialized_bytes);
        }

        serialized_bytes.into_boxed_slice()
    }
}

impl Packet {
    /// Splits raw bytes array into address, types tag, and argument data.
    fn split_bytes(mut bytes: Vec<u8>) -> Result<(String, String, Vec<u8>)> {
        // Check alignment
        if bytes.len() % 4 != 0 {
            return Err(Error::UnalignedData);
        }

        // Slice address
        let rest_bytes = &bytes[..];
        let address_first_nul = match rest_bytes.iter().position(|&b| b == 0x00) {
            None => return Err(Error::NotTerminated),
            Some(0) => return Err(Error::InvalidAddress),
            Some(i) => i,
        };
        let address: Vec<u8> = (&bytes[..address_first_nul]).into();
        let address = String::from_utf8(address).map_err(|_| Error::InvalidAddress)?;
        let address_aligned = Value::aligned_length(address_first_nul + 1);

        // Slice types tag
        let rest_bytes = &rest_bytes[address_aligned..];
        let tag_first_nul = match rest_bytes.iter().position(|&b| b == 0x00) {
            None => return Err(Error::NotTerminated),
            Some(0) => return Err(Error::InvalidTag),
            Some(i) => i,
        };
        let tag: Vec<u8> = (&bytes[..tag_first_nul]).into();
        let tag = String::from_utf8(tag).map_err(|_| Error::InvalidTag)?;
        let tag_aligned = Value::aligned_length(tag_first_nul + 1);

        // Cut out arguments
        let arguments_left = bytes.split_off(address_aligned + tag_aligned);

        Ok((address, tag, arguments_left))
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
            return Err(Error::InvalidAddress);
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

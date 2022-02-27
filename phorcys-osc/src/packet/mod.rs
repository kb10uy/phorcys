pub mod bundle;
pub mod message;

pub use crate::packet::{
    bundle::{Bundle, BUNDLE_HEADER},
    message::{Message, MessageBuilder},
};

use crate::error::{Error, Result};

/// Represents a whole OSC Packet.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Packet {
    /// This packet is OSC Message.
    Message(Message),

    /// This packet is OSC Bundle.
    Bundle(Bundle),
}

impl Packet {
    /// Serializes this packet.
    /// Returned bytes **does not** contain the whole size.
    pub fn serialize(self) -> Box<[u8]> {
        match self {
            Packet::Message(m) => m.serialize(),
            Packet::Bundle(b) => b.serialize(),
        }
    }

    /// Deserializes the bytes into OSC Packet.
    pub fn deserialize(bytes: &[u8]) -> Result<Packet> {
        if bytes.len() == 0 {
            return Err(Error::NotEnoughData);
        }

        match bytes[0] {
            b'/' => Ok(Packet::Message(Message::deserialize(bytes)?)),
            b'#' => Ok(Packet::Bundle(Bundle::deserialize(bytes)?)),
            _ => Err(Error::InvalidAddress),
        }
    }
}

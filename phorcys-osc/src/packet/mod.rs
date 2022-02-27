pub mod bundle;
pub mod message;

pub use crate::packet::{
    bundle::{Bundle, BUNDLE_HEADER},
    message::{Message, MessageBuilder},
};

/// Represents a whole OSC Packet.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Packet {
    /// This packet is OSC Message.
    Message(Message),

    /// This packet is OSC Bundle.
    Bundle(Bundle),
}

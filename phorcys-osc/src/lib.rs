//! OSC (Open Sound Control) protocol implementation.

pub mod address;
pub mod data;
pub mod error;
pub mod packet;

#[cfg(feature = "address-pattern")]
pub mod address_pattern;

/// Prelude module, prefixed with `Osc` to avoid identifier conflict.
pub mod prelude {
    pub use crate::address::Address as OscAddress;
    pub use crate::data::Value as OscValue;
    pub use crate::error::{Error as OscError, Result as OscResult};
    pub use crate::packet::{
        Bundle as OscBundle, Message as OscMessage, MessageBuilder as OscMessageBuilder,
        Packet as OscPacket,
    };

    #[cfg(feature = "address-pattern")]
    pub use crate::address_pattern::{
        AddressPattern as OscAddressPattern, Expression as OscExpression,
    };
}

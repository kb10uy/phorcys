//! OSC (Open Sound Control) protocol implementation.

pub mod address;
pub mod data;
pub mod error;
pub mod packet;

/// Prelude module, prefixed with `Osc` to avoid identifier conflict.
pub mod prelude {
    pub use crate::address::Address as OscAddress;
    pub use crate::data::Value as OscValue;
    pub use crate::error::{Error as OscError, Result as OscResult};
    pub use crate::packet::{Packet as OscPacket, PacketBuilder as OscPacketBuilder};
}

/// Constructs directly `Packet`.
#[macro_export]
macro_rules! osc_packet {
    ($addr:literal) => {
        PacketBuilder::new($addr).map(|b| b.build())
    };
    ($addr:literal, $($args:expr),* $(,)?) => {
        PacketBuilder::new($addr).map(|mut b| {
            let arguments = vec![
                $($args.into()),*,
            ];
            b.set_arguments(arguments);
            b.build()
        })
    };
}

//! OSC (Open Sound Control) protocol implementation.

pub mod address;
pub mod data;
pub mod error;
pub mod packet;

/// Constructs directly `Packet`.
#[macro_export]
macro_rules! osc_packet {
    ($addr:literal) => {
        PacketBuilder::new($addr).map(|b| b.build())
    };
    ($addr:literal { $($args:expr),* $(,)? }) => {
        PacketBuilder::new($addr).map(|mut b| {
            let arguments = vec![
                $($args.into()),*,
            ];
            b.set_arguments(arguments);
            b.build()
        })
    };
}

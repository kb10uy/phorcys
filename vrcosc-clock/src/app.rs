use crate::data::DateTimePart;

use std::net::SocketAddr;

use clap::Parser;

/// Sends DateTime information to VRChat as OSC packets.
#[derive(Debug, Clone, Parser)]
#[clap(author, version)]
pub struct Arguments {
    /// Sends hours part.
    /// 
    /// Source value is 0 to 23.
    /// Relative value range is [0, 1).
    #[clap(short, long, parse(try_from_str = DateTimePart::parse))]
    pub hour: Option<DateTimePart>,

    /// Sends minute part.
    /// 
    /// Source value is 0 to 59.
    /// Relative value range is [0, 1).
    #[clap(short, long, parse(try_from_str = DateTimePart::parse))]
    pub minute: Option<DateTimePart>,

    /// Sends second part.
    /// 
    /// Source value is 0 to 59.
    /// Relative value range is [0, 1).
    #[clap(short, long, parse(try_from_str = DateTimePart::parse))]
    pub second: Option<DateTimePart>,

    /// Sends month part.
    /// 
    /// Source value is 1 to 12.
    /// Relative value range is (0, 1].
    #[clap(short = 'M', long, parse(try_from_str = DateTimePart::parse))]
    pub month: Option<DateTimePart>,

    /// Sends day part.
    /// 
    /// Source value is 1 to 31.
    /// Relative value range is (0, 1].
    #[clap(short = 'D', long, parse(try_from_str = DateTimePart::parse))]
    pub day: Option<DateTimePart>,

    /// The interval of sending data, in seconds.
    #[clap(short, long, default_value = "2")]
    pub interval: usize,

    /// The UDP socket address to which vrcosc-clock sends packets.
    #[clap(short, long, default_value = "127.0.0.1:9000")]
    pub port: SocketAddr,
}

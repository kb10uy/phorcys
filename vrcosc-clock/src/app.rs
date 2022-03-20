use std::net::SocketAddr;

use clap::Parser;

/// Sends DateTime information to VRChat as OSC packets.
#[derive(Debug, Clone, Parser)]
#[clap(author, version)]
pub struct Arguments {
    /// The interval of sending data, in seconds.
    #[clap(short, long, default_value = "2")]
    pub interval: usize,

    /// The UDP socket address to which vrcosc-clock sends packets.
    #[clap(short, long, default_value = "127.0.0.1:9000")]
    pub port: SocketAddr,

    /// Script filenames.
    pub scripts: Vec<String>,
}

use async_std::net::SocketAddr;
use clap::Parser;

/// Receives OSC packets from VRChat and provides querying interface.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Parser)]
#[clap(author, version)]
pub struct Arguments {
    /// Specifies socket addresses for sending packets to VRChat.
    #[clap(short, long, default_value = "127.0.0.1:9000")]
    pub send_address: SocketAddr,

    /// Specifies socket addresses for receiving packets from VRChat.
    #[clap(short, long, default_value = "127.0.0.1:9001")]
    pub receive_address: SocketAddr,

    /// Specifies the MIDI input device for use.
    /// If not given or index is out of bound, it will list available devices.
    #[clap(short, long)]
    pub midi_port: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Application {}

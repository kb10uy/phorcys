use std::collections::HashMap;

use async_std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use clap::Parser;
use phorcys_config::Configuration;
use phorcys_osc::data::Value;

/// Receives OSC packets from VRChat and provides querying interface.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Parser)]
#[clap(author, version)]
pub struct Arguments {
    /// Specifies function to execute.
    #[clap(subcommand)]
    pub command: CommandKind,
}

/// Subcommands.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Parser)]
#[clap(author, version)]
pub enum CommandKind {
    /// Lists all the available MIDI input devices.
    ListMidiDevices,

    /// Receives OSC packet from VRChat and exports their values.
    Export(ExportCommandArguments),

    /// Starts proxy server between MIDI message and OSC message to VRChat.
    ProxyMidiArguments(ProxyMidiArguments),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Parser)]
pub struct ExportCommandArguments {
    /// Socket addresses for receiving packets from VRChat.
    #[clap(short, long, default_value = "127.0.0.1:9001")]
    pub receive_address: SocketAddr,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Parser)]
pub struct ProxyMidiArguments {
    /// Socket addresses for sending packets to VRChat.
    #[clap(short, long, default_value = "127.0.0.1:9001")]
    pub send_address: SocketAddr,

    /// MIDI input device number for use.
    #[clap(short, long, default_value = "0")]
    pub midi_port: usize,

    /// Filename of VRChat OSC API configuration JSON.
    #[clap(short, long)]
    pub avatar_configuration: String,

    /// Surpresses validation of avatar ID.
    #[clap(short = 'N', long)]
    pub no_avatar_validation: bool,

    /// Filename of entries table.
    pub entries_table: String,
}

#[derive(Debug, Clone)]
pub struct Application {
    /// OSC packet source configuration.
    pub configuration: Arc<Configuration>,

    /// Latest values received from VRChat.
    pub latest_values: Arc<Mutex<HashMap<String, Value>>>,
}

mod app;
mod command;
mod table;

use crate::{
    app::{Arguments, CommandKind},
    command::{export_values, list_midi_devices, proxy_midi},
};

use anyhow::Result;
use clap::Parser;
use flexi_logger::Logger;

#[async_std::main]
async fn main() -> Result<()> {
    Logger::try_with_env()?.start()?;
    let arguments = Arguments::parse();

    match arguments.command {
        CommandKind::ListMidiDevices => list_midi_devices()?,
        CommandKind::Export(args) => export_values(args).await?,
        CommandKind::ProxyMidiArguments(args) => proxy_midi(args).await?,
    }

    Ok(())
}

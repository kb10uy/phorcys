mod app;
mod midi;
mod prompt;
mod vrchat;

use crate::{
    app::Arguments,
    midi::{list_midi_devices, start_midi_input},
    vrchat::start_vrchat_communication,
};

use anyhow::Result;
use clap::Parser;
use flexi_logger::Logger;

#[async_std::main]
async fn main() -> Result<()> {
    Logger::try_with_env()?.start()?;
    let arguments = Arguments::parse();

    let midi_port = match arguments.midi_port {
        Some(i) => i,
        None => {
            list_midi_devices()?;
            return Ok(());
        }
    };

    start_vrchat_communication(arguments.send_address, arguments.receive_address).await?;
    start_midi_input(midi_port).await?;

    Ok(())
}

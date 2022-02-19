use std::time::Duration;

use anyhow::{bail, Result};
use async_std::task::sleep;
use log::{debug, error};
use midir::MidiInput;
use midly::live::LiveEvent;

/// Lists all MIDI inputs.
pub fn list_midi_devices() -> Result<()> {
    let midi_client = MidiInput::new("phorcys-miditable")?;
    let ports = midi_client.ports();

    eprintln!("Available MIDI input devices:");
    for (index, port) in ports.into_iter().enumerate() {
        println!("{:4}: {}", index, midi_client.port_name(&port)?);
    }
    Ok(())
}

/// Start to receive MIDI message from specified indexed device.
/// This function never returns.
pub async fn start_midi_input(device_index: usize) -> Result<()> {
    let midi_client = MidiInput::new("phorcys-miditable")?;
    let ports = midi_client.ports();
    if device_index >= ports.len() {
        bail!("Invalid device index");
    }

    let _input_port = midi_client.connect(
        &ports[device_index],
        "phorcys-miditable-input",
        on_midi_message,
        (),
    );

    loop {
        sleep(Duration::from_millis(1000)).await;
    }
}

/// MIDI message callback.
fn on_midi_message<T: Send>(timestamp: u64, message: &[u8], _data: &mut T) {
    let event = match LiveEvent::parse(message) {
        Ok(ev) => ev,
        Err(err) => {
            error!("MIDI message parse error: {}", err);
            return;
        }
    };

    debug!("MIDI Event: [@{:12}] {:?}", timestamp, event);
}

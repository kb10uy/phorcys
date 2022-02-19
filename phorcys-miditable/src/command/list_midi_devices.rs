use anyhow::Result;
use midir::MidiInput;

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

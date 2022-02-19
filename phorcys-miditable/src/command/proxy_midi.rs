use crate::{
    app::ProxyMidiArguments,
    table::{ParametersTable, ParametersTableEntry},
};

use std::{
    collections::HashMap,
    sync::mpsc::{channel, Receiver, Sender},
    time::Duration,
};

use anyhow::{bail, Result};
use async_std::{
    fs::read_to_string,
    net::UdpSocket,
    sync::Arc,
    task::{sleep, spawn},
};
use futures::try_join;
use log::{error, info, trace, warn};
use midir::MidiInput;
use midly::{live::LiveEvent, MidiMessage};
use phorcys_config::{Configuration as VrcConfig, Parameter as VrcParameter};
use phorcys_osc::{data::Value as OscValue, packet::PacketBuilder as OscPacketBuilder};

#[derive(Debug)]
struct Context {
    avatar_parameters: HashMap<String, VrcParameter>,
    table_entries: HashMap<(Option<u8>, u8), ParametersTableEntry>,
    send_socket: UdpSocket,
}

/// proxy-midi subcommand.
pub async fn proxy_midi(args: ProxyMidiArguments) -> Result<()> {
    // Read configs
    let avatar_config_source = read_to_string(args.avatar_configuration).await?;
    let parameters_table_source = read_to_string(args.entries_table).await?;
    let avatar_config: VrcConfig = serde_json::from_str(&avatar_config_source)?;
    let parameters_table: ParametersTable = toml::from_str(&parameters_table_source)?;

    // Avatar ID validation
    if !args.no_avatar_validation && avatar_config.id != parameters_table.avatar_id {
        error!(
            "Avatar ID not matched: avatar config has \"{}\", parameters table has \"{}\"",
            avatar_config.id, parameters_table.avatar_id
        );
        bail!("Avatar ID validation error");
    }

    // Construct context
    info!("Using {} as sending socket address", args.send_address);
    let send_socket = UdpSocket::bind({
        let mut bind_addr = args.send_address;
        bind_addr.set_port(0);
        bind_addr
    })
    .await?;
    send_socket.connect(args.send_address).await?;

    let avatar_parameters = avatar_config
        .parameters
        .into_iter()
        .map(|p| (p.name.clone(), p))
        .collect();

    let table_entries = parameters_table
        .entries
        .into_iter()
        .map(|p| ((p.midi_channel, p.midi_note), p))
        .collect();

    let context = Arc::new(Context {
        avatar_parameters,
        table_entries,
        send_socket,
    });

    // Execute threads
    let (tx, rx) = channel();
    try_join!(
        spawn(midi_worker(args.midi_port, tx)),
        spawn(osc_worker(context, rx)),
    )?;
    Ok(())
}

/// Processes MIDI messages.
async fn midi_worker(midi_port: usize, tx: Sender<(u8, u8)>) -> Result<()> {
    let midi_client = MidiInput::new("phorcys-miditable")?;
    let ports = midi_client.ports();
    if midi_port >= ports.len() {
        bail!("Invalid device index");
    }

    info!(
        "Started to watch MIDI input \"{}\"",
        midi_client.port_name(&ports[midi_port])?
    );
    let _input_port = midi_client.connect(
        &ports[midi_port],
        "phorcys-miditable-input",
        on_midi_message,
        tx,
    );

    loop {
        sleep(Duration::from_millis(1000)).await;
    }
}

/// Processes and distributes transfered MIDI messages.
async fn osc_worker(context: Arc<Context>, rx: Receiver<(u8, u8)>) -> Result<()> {
    while let Ok((channel, key)) = rx.recv() {
        spawn(process_midi_to_entry(context.clone(), channel, key));
    }
    Ok(())
}

/// Called from transfered MIDI message.
async fn process_midi_to_entry(context: Arc<Context>, channel: u8, key: u8) {
    trace!("Trying MIDI message: Ch.{:02} {}", channel, key);

    let entry = context
        .table_entries
        .get(&(None, key))
        .or_else(|| context.table_entries.get(&(Some(channel), key)));
    let entry = match entry {
        Some(e) => e,
        None => return,
    };
    for (param_name, value) in &entry.parameters {
        let target_param = match context.avatar_parameters.get(param_name) {
            Some(param) => param,
            None => {
                warn!("Parameter \"{}\" not found on avatar side", param_name);
                continue;
            }
        };
        let target_address = match &target_param.input {
            Some(input) => &input.address,
            None => {
                warn!("Parameter \"{}\" does not have input", param_name);
                continue;
            }
        };
        let param_value = match &value {
            toml::Value::Integer(i) => OscValue::Int32(*i as i32),
            toml::Value::Float(f) => OscValue::Float32(*f as f32),
            toml::Value::Boolean(b) => OscValue::Boolean(*b),
            _ => {
                warn!(
                    "Invalid data found when sending parameter \"{}\"",
                    param_name
                );
                continue;
            }
        };

        let packet = match OscPacketBuilder::new(target_address) {
            Ok(b) => b.push_argument(param_value).build(),
            Err(e) => {
                warn!("Failed to construct OSC packet: {}", e);
                continue;
            }
        };

        match context.send_socket.send(&packet.serialize()).await {
            Ok(_) => {
                info!("Sent OSC packet: \"{}\" <- {:?}", target_address, value);
            }
            Err(e) => {
                warn!("Failed to send OSC packet to VRChat: {}", e);
                continue;
            }
        }
    }
}

/// MIDI message callback.
fn on_midi_message(timestamp: u64, message: &[u8], tx: &mut Sender<(u8, u8)>) {
    let event = match LiveEvent::parse(message) {
        Ok(ev) => ev,
        Err(err) => {
            error!("MIDI message parse error: {}", err);
            return;
        }
    };
    trace!("MIDI Event: [@{:12}] {:?}", timestamp, event);

    match event {
        LiveEvent::Midi { channel, message } => match message {
            MidiMessage::NoteOn { key, vel } if vel > 0 => {
                tx.send((channel.into(), key.into()))
                    .expect("MPSC channel error");
            }
            _ => (),
        },
        _ => (),
    }
}

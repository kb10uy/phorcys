mod app;
mod sender;

use crate::{app::Arguments, sender::DateTimeSender};

use std::time::Duration;

use anyhow::{bail, Result};
use async_std::{net::UdpSocket, sync::Arc, task::sleep};
use clap::StructOpt;
use flexi_logger::Logger;
use log::error;
use time::OffsetDateTime;

#[async_std::main]
async fn main() -> Result<()> {
    Logger::try_with_env()?.start()?;
    let args = Arguments::parse();

    if args.scripts.len() == 0 {
        bail!("No scripts given. Aborting.");
    }

    // run(&senders, args.interval as u64, args.port).await?;
    Ok(())
}

async fn run_script<'lua>(
    sender: DateTimeSender<'lua>,
    socket: Arc<UdpSocket>,
    interval: f64,
) -> ! {
    loop {
        sleep(Duration::from_secs_f64(interval)).await;

        let datetime = OffsetDateTime::now_local().expect("Cannot obtain local time");
        let messages = match sender.convert(datetime) {
            Ok(messages) => messages,
            Err(e) => {
                error!("Conversion error: {}", e);
                continue;
            }
        };
        for message in messages {
            match socket.send(&message.serialize()).await {
                Ok(_) => (),
                Err(e) => {
                    error!("Failed to send packet: {}", e);
                }
            }
        }
    }
}

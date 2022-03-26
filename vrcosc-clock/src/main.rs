mod app;
mod sender;

use crate::{
    app::Arguments,
    sender::{run_script, SenderConstructor},
};

use std::time::Duration;

use anyhow::{bail, Result};
use async_std::task::{sleep, spawn};
use clap::StructOpt;
use flexi_logger::Logger;

#[async_std::main]
async fn main() -> Result<()> {
    Logger::try_with_env()?.start()?;
    let args = Arguments::parse();
    let constructors = match args.scripts.len() {
        0 => bail!("No scripts given. Aborting."),
        x @ 17.. => bail!("Too many {} scripts given.", x),
        _ => args.scripts.into_iter().map(|s| SenderConstructor {
            script_filename: s,
            osc_socket_address: args.port,
            interval: Duration::from_secs_f64(args.interval),
        }),
    };

    for constructor in constructors {
        spawn(run_script(constructor));
    }

    loop {
        sleep(Duration::from_secs(1)).await;
    }
}

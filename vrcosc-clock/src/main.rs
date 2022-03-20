mod app;
mod runner;

use crate::app::Arguments;

use anyhow::{bail, Result};
use clap::StructOpt;
use flexi_logger::Logger;

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

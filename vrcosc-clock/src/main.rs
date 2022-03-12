mod app;
mod runner;

use crate::{
    app::Arguments,
    runner::{run, DateTimePartSender},
};

use anyhow::Result;
use clap::StructOpt;
use flexi_logger::Logger;

#[async_std::main]
async fn main() -> Result<()> {
    Logger::try_with_env()?.start()?;
    let args = Arguments::parse();

    let mut senders = vec![];
    if let Some(part) = args.hour {
        senders.push(DateTimePartSender::new(&part, |t| t.hour() as usize, 24));
    }
    if let Some(part) = args.minute {
        senders.push(DateTimePartSender::new(&part, |t| t.minute() as usize, 60));
    }
    if let Some(part) = args.second {
        senders.push(DateTimePartSender::new(&part, |t| t.second() as usize, 60));
    }

    run(&senders, args.interval as u64, args.port).await?;
    Ok(())
}

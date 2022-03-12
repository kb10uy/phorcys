mod app;
mod data;
mod runner;

use crate::{
    app::Arguments,
    data::PartsConfig,
    runner::{run, DateTimePartSender},
};

use anyhow::{bail, Result};
use async_std::fs::read_to_string;
use clap::StructOpt;
use data::DateTimePart;
use flexi_logger::Logger;

#[async_std::main]
async fn main() -> Result<()> {
    Logger::try_with_env()?.start()?;
    let args = Arguments::parse();

    let senders = if let Some(filename) = &args.config_file {
        let toml_str = read_to_string(filename).await?;
        let table = toml::from_str(&toml_str)?;
        senders_by_config(&table)
    } else {
        Ok(senders_by_commandline(&args))
    }?;

    if senders.len() == 0 {
        bail!("No part defined. Aborting.");
    }

    run(&senders, args.interval as u64, args.port).await?;
    Ok(())
}

fn senders_by_commandline(args: &Arguments) -> Vec<DateTimePartSender> {
    let mut senders = vec![];
    if let Some(part) = &args.hour {
        senders.push(DateTimePartSender::new(&part, |t| t.hour() as usize, 24));
    }
    if let Some(part) = &args.minute {
        senders.push(DateTimePartSender::new(&part, |t| t.minute() as usize, 60));
    }
    if let Some(part) = &args.second {
        senders.push(DateTimePartSender::new(&part, |t| t.second() as usize, 60));
    }
    if let Some(part) = &args.month {
        senders.push(DateTimePartSender::new(&part, |t| t.month() as usize, 13));
    }
    if let Some(part) = &args.day {
        senders.push(DateTimePartSender::new(&part, |t| t.day() as usize, 32));
    }

    senders
}

fn senders_by_config(config: &PartsConfig) -> Result<Vec<DateTimePartSender>> {
    let mut senders = vec![];
    if let Some(part) = &config.hour {
        let part = DateTimePart::parse_config_table(&part)?;
        senders.push(DateTimePartSender::new(&part, |t| t.hour() as usize, 24));
    }
    if let Some(part) = &config.minute {
        let part = DateTimePart::parse_config_table(&part)?;
        senders.push(DateTimePartSender::new(&part, |t| t.minute() as usize, 60));
    }
    if let Some(part) = &config.second {
        let part = DateTimePart::parse_config_table(&part)?;
        senders.push(DateTimePartSender::new(&part, |t| t.second() as usize, 60));
    }
    if let Some(part) = &config.month {
        let part = DateTimePart::parse_config_table(&part)?;
        senders.push(DateTimePartSender::new(&part, |t| t.month() as usize, 13));
    }
    if let Some(part) = &config.day {
        let part = DateTimePart::parse_config_table(&part)?;
        senders.push(DateTimePartSender::new(&part, |t| t.day() as usize, 32));
    }

    Ok(senders)
}

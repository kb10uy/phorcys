use crate::app::{DateTimePart, ValueFormat};

use std::time::Duration;

use anyhow::Result;
use async_std::{
    net::{SocketAddr, UdpSocket},
    task::sleep,
};
use log::{debug, info};
use phorcys_osc::prelude::*;
use time::{format_description::parse as parse_time_format, OffsetDateTime};

/// Sends a part of DateTime.
pub struct DateTimePartSender {
    /// Base `DateTimePart`.
    pub part_information: DateTimePart,

    /// DateTime format string for extraction of target part.
    pub datetime_extraction: Box<dyn Fn(OffsetDateTime) -> usize>,

    /// Divider.
    pub value_divider: usize,
}

impl DateTimePartSender {
    /// Creates a new sender.
    pub fn new<F: Fn(OffsetDateTime) -> usize + 'static>(
        part: &DateTimePart,
        extract: F,
        divider: usize,
    ) -> DateTimePartSender {
        DateTimePartSender {
            part_information: part.clone(),
            datetime_extraction: Box::new(extract),
            value_divider: divider,
        }
    }

    /// Constructs an OSC packet from DateTime.
    pub fn construct_packet(&self, datetime: OffsetDateTime) -> Result<OscMessage> {
        let extacted_value = (self.datetime_extraction)(datetime);
        let builder = OscMessageBuilder::new(self.part_information.target_address.as_ref())?;
        let msg = match &self.part_information.format {
            ValueFormat::Absolute(div) => {
                let send_value = extacted_value % self.value_divider % *div as usize;
                builder
                    .push_argument(OscValue::Int32(send_value as i32))
                    .build()
            }
            ValueFormat::Relative(max) => {
                let ratio = extacted_value as f32 / self.value_divider as f32;
                builder
                    .push_argument(OscValue::Float32(ratio * max))
                    .build()
            }
        };
        Ok(msg)
    }
}

/// Runs main thread.
pub async fn run(senders: &[DateTimePartSender], interval: u64, address: SocketAddr) -> Result<()> {
    let log_format = parse_time_format("[hour]:[minute]:[second]")?;

    info!("Using {} as sending socket address", address);
    let send_socket = UdpSocket::bind({
        let mut bind_addr = address;
        bind_addr.set_port(0);
        bind_addr
    })
    .await?;
    send_socket.connect(address).await?;

    info!("Started to send packets...");
    loop {
        let now = OffsetDateTime::now_local()?;
        debug!("Sending {}", now.format(&log_format)?);

        for sender in senders {
            let message = sender.construct_packet(now)?.serialize();
            send_socket.send(&message).await?;
        }
        sleep(Duration::from_secs(interval)).await;
    }
}

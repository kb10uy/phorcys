use anyhow::Result;
use async_std::net::UdpSocket;
use flexi_logger::Logger;
use phorcys_config::xsoverlay::{NotificationBuilder, NotificationDescription};
use serde_json::to_vec as json_to_vec;

#[async_std::main]
async fn main() -> Result<()> {
    Logger::try_with_env()?.start()?;

    let socket = UdpSocket::bind("127.0.0.1:0").await?;
    socket.connect("127.0.0.1:42069").await?;

    let notification = NotificationBuilder::new(
        "phorcys-example",
        "Hello, XSOverlay!",
        NotificationDescription::Popup,
    )
    .build();
    let notification_json = json_to_vec(&notification)?;
    socket.send(&notification_json).await?;

    Ok(())
}

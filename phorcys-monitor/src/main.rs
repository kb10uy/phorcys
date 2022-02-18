use anyhow::Result;

use async_std::net::UdpSocket;
use flexi_logger::Logger;
use log::{info, warn};
use phorcys_osc::packet::Packet;

#[async_std::main]
async fn main() -> Result<()> {
    Logger::try_with_env()?.start()?;

    let listener = UdpSocket::bind("127.0.0.1:9001").await?;
    loop {
        let mut buffer = vec![0; 8192];
        let (read_bytes, peer) = listener.recv_from(&mut buffer).await?;
        info!("Data arrived: {} bytes from {}", read_bytes, peer);

        let bytes = (&buffer[..read_bytes]).into();
        let packet = match Packet::deserialize(bytes) {
            Ok(p) => p,
            Err(e) => {
                warn!("Packet deserialization error: {}", e);
                continue;
            }
        };
        info!("Packet: {:?}", packet);
    }
}

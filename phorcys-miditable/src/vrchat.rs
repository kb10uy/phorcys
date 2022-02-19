use anyhow::Result;
use async_std::{
    channel::{unbounded, Receiver},
    net::{SocketAddr, UdpSocket},
    task::spawn,
};
use log::{debug, error, warn};
use phorcys_osc::packet::Packet;

/// Starts send worker and receive worker.
pub async fn start_vrchat_communication(send_at: SocketAddr, recv_at: SocketAddr) -> Result<()> {
    let send_socket = UdpSocket::bind(send_at).await?;
    let recv_socket = UdpSocket::bind(recv_at).await?;
    let (tx, rx) = unbounded();

    spawn(send_worker(send_socket, rx));
    spawn(receive_packets(recv_socket));

    // TODO: return it
    drop(tx);

    Ok(())
}

async fn receive_packets(recv_socket: UdpSocket) -> Result<()> {
    let mut buffer = vec![0; 65536];
    loop {
        let (read_bytes, _peer) = match recv_socket.recv_from(&mut buffer).await {
            Ok((r, p)) => (r, p),
            Err(e) => {
                error!("Failed to reveive packet: {}", e);
                continue;
            }
        };

        let packet = (&buffer[..read_bytes]).into();
        spawn(process_osc_packet(packet));
    }
}

/// Processes single OSC packet from VRChat.
async fn process_osc_packet(bytes: Vec<u8>) -> Result<()> {
    let packet = Packet::deserialize(bytes)?;
    debug!("OSC Packet: {:?}", packet);

    Ok(())
}

/// Takes channel RX and sends them to VRChat.
async fn send_worker(_send_socket: UdpSocket, rx: Receiver<()>) -> Result<()> {
    while let Ok(_) = rx.recv().await {
        continue;
    }

    warn!("Send worker terminated");
    Ok(())
}

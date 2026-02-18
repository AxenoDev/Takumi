use hmac::digest::const_oid::Arc;
use tokio::{
    net::{TcpStream, unix::SocketAddr},
    sync::mpsc,
};
use uuid::Uuid;

use crate::protocol::Packet;

pub struct ConnectedPlayer {
    pub uuid: Uuid,
    pub username: String,
    pub remote_addr: SocketAddr,
    pub current_server: Option<String>,
    // Channel to send packets to this player's client connection.
    pub packet_tx: mpsc::Sender<Packet>,
}

impl ConnectedPlayer {
    pub fn new(
        uuid: Uuid,
        username: String,
        remote_addr: SocketAddr,
        packet_tx: mpsc::Sender<Packet>,
    ) -> Self {
        ConnectedPlayer {
            uuid,
            username,
            remote_addr,
            current_server: None,
            packet_tx,
        }
    }

    pub async fn send_packet(&self, packet: Packet) -> bool {
        self.packet_tx.send(packet).await.is_ok()
    }

    pub async fn disconnect(&self, reason: &str) {
        use crate::protocol::packets::build_play_disconnect;
        let _ = self.send_packet(build_play_disconnect(reason)).await;
    }
}

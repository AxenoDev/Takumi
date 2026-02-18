use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::ConnectedPlayer;

#[derive(Clone, Default)]
pub struct PlayerRegistry {
    pub players: Arc<RwLock<HashMap<Uuid, Arc<ConnectedPlayer>>>>,
}

impl PlayerRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn add(&self, player: Arc<ConnectedPlayer>) {
        self.players.write().await.insert(player.uuid, player);
    }

    pub async fn remove(&self, uuid: &Uuid) -> Option<Arc<ConnectedPlayer>> {
        self.players.write().await.remove(uuid)
    }

    pub async fn get(&self, uuid: &Uuid) -> Option<Arc<ConnectedPlayer>> {
        self.players.read().await.get(uuid).cloned()
    }

    pub async fn count(&self) -> usize {
        self.players.read().await.len()
    }

    pub async fn list(&self) -> Vec<Arc<ConnectedPlayer>> {
        self.players.read().await.values().cloned().collect()
    }

    pub async fn broadcast(&self, packet: crate::protocol::Packet) {
        let players = self.list().await;
        for player in players {
            let _ = player.send_packet(packet.clone()).await;
        }
    }
}

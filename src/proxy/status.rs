use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use tracing::debug;

use crate::configuration::TakumiConfig;
use crate::player::PlayerRegistry;
use crate::protocol::MinecraftCodec;
use crate::protocol::packets::{
    SB_STATUS_PING, SB_STATUS_REQUEST, build_status_pong, build_status_response,
};
use crate::protocol::types::read_varlong;

pub struct StatusHandler<'a> {
    framed: &'a mut Framed<TcpStream, MinecraftCodec>,
    addr: SocketAddr,
    config: Arc<TakumiConfig>,
    players: Arc<PlayerRegistry>,
}

impl<'a> StatusHandler<'a> {
    pub fn new(
        framed: &'a mut Framed<TcpStream, MinecraftCodec>,
        addr: SocketAddr,
        config: Arc<TakumiConfig>,
        players: Arc<PlayerRegistry>,
    ) -> Self {
        StatusHandler {
            framed,
            addr,
            config,
            players,
        }
    }

    pub async fn handle(&mut self) -> anyhow::Result<()> {
        match self.framed.next().await {
            Some(Ok(p)) if p.id == SB_STATUS_REQUEST => {}
            Some(Ok(p)) => {
                debug!("Expected status request, got {:#04x}", p.id);
                return Ok(());
            }
            _ => return Ok(()),
        }

        let online = self.players.count().await;

        let status_json = serde_json::json!({
            "version": {
                "name": format!("Takumi {}", env!("CARGO_PKG_VERSION")),
                "protocol": 774,
            },
            "players": {
                "max": &self.config.proxy.max_players,
                "online": online,
                "sample": [],
                "list": self.players.list().await.iter().map(|p| {
                    serde_json::json!({
                        "name": p.username,
                        "id": p.uuid.to_string(),
                    })
                }).collect::<Vec<_>>(),
            },
            "description": {
                "text": &self.config.proxy.motd,
            },
            "enforcesSecureChat": false,
        });

        self.framed
            .send(build_status_response(&status_json.to_string()))
            .await?;

        match self.framed.next().await {
            Some(Ok(p)) if p.id == SB_STATUS_PING => {
                let mut data = p.data.clone();
                let payload = read_varlong(&mut data)?;
                self.framed.send(build_status_pong(payload)).await?;
            }
            _ => {}
        }

        Ok(())
    }
}

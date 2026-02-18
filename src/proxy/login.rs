use std::sync::Arc;

use anyhow::bail;
use bytes::{Buf, Bytes};
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use tracing::{debug, info};
use tracing_subscriber::fmt::format;
use uuid::Uuid;

use crate::{
    configuration::TakumiConfig,
    player::PlayerRegistry,
    protocol::{
        MinecraftCodec, Packet,
        packets::{SB_LOGIN_ACK, SB_LOGIN_START, build_login_success},
        types::{read_bool, read_string, read_uuid},
    },
};

pub struct LoginHandler {
    framed: Framed<TcpStream, MinecraftCodec>,
    addr: SocketAddr,
    config: Arc<TakumiConfig>,
    players: Arc<PlayerRegistry>,
}

impl LoginHandler {
    pub fn new(
        framed: Framed<TcpStream, MinecraftCodec>,
        addr: SocketAddr,
        config: Arc<TakumiConfig>,
        players: Arc<PlayerRegistry>,
    ) -> Self {
        Self {
            framed,
            addr,
            config,
            players,
        }
    }

    pub async fn handle(&mut self) -> anyhow::Result<()> {
        let login_start: Packet = self.next_packet().await?;
        if login_start.id != SB_LOGIN_START {
            bail!(
                "Expected login start packet (id {:#04x}), got {:#04x}",
                SB_LOGIN_START,
                login_start.id
            );
        }

        let mut data: Bytes = login_start.data.clone();
        let username = read_string(&mut data)?;

        let _player_uuid_hint = if data.has_remaining() {
            let has_uuid = read_bool(&mut data)?;
            if has_uuid && data.remaining() >= 16 {
                Some(read_uuid(&mut data)?)
            } else {
                None
            }
        } else {
            None
        };

        info!("Login attempt from {} ({})", self.addr, username);

        if self.players.count().await >= self.config.proxy.max_players as usize {
            use crate::protocol::packets::build_login_disconnect;
            let _ = self
                .framed
                .send(build_login_disconnect("Server is full"))
                .await;
            return Ok(());
        }

        // TODO: If online mode is enabled, perform authentication with Mojang's servers here.
        let (uuid, final_username) = {
            let online_uuid = Uuid::new_v5(
                &Uuid::NAMESPACE_DNS,
                format!("OnlinePlayer:{}", username).as_bytes(),
            );
            (online_uuid, username.clone())
        };

        debug!("Assigned UUID {} to player {}", uuid, final_username);

        self.framed
            .send(build_login_success(&uuid, &final_username))
            .await?;

        let ack = self.next_packet().await?;
        if ack.id != SB_LOGIN_ACK {
            bail!(
                "Expected login ack packet (id {:#04x}), got {:#04x}",
                SB_LOGIN_ACK,
                ack.id
            );
        }

        info!("Player {} ({}) authenticated", final_username, uuid);

        Ok(())
    }

    async fn next_packet(&mut self) -> anyhow::Result<Packet> {
        match self.framed.next().await {
            Some(Ok(p)) => Ok(p),
            Some(Err(e)) => Err(e),
            None => bail!("Connection closed during login"),
        }
    }
}

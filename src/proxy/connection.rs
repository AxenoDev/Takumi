use futures_util::StreamExt;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use tracing::{debug, info, warn};

use crate::{
    configuration::TakumiConfig,
    player::{handshake::HandshakeHandler, registry::PlayerRegistry},
    protocol::{MinecraftCodec, State},
    proxy::status::StatusHandler,
};

pub struct ConnectionHandler {
    stream: TcpStream,
    addr: SocketAddr,
    config: Arc<TakumiConfig>,
    players: Arc<PlayerRegistry>,
}

impl ConnectionHandler {
    pub fn new(
        stream: TcpStream,
        addr: SocketAddr,
        config: Arc<TakumiConfig>,
        players: Arc<PlayerRegistry>,
    ) -> Self {
        ConnectionHandler {
            stream,
            addr,
            config,
            players,
        }
    }

    pub async fn handle(mut self) -> anyhow::Result<()> {
        let mut framed = Framed::new(self.stream, MinecraftCodec::new());

        let handshake = match framed.next().await {
            Some(Ok(p)) => p,
            Some(Err(e)) => return Err(e),
            None => return Ok(()),
        };

        let next_state = HandshakeHandler::handle(&handshake)?;
        debug!("Handshake complete, next state: {:?}", next_state);

        match next_state {
            State::Status => {
                StatusHandler::new(
                    &mut framed,
                    self.addr,
                    Arc::clone(&self.config),
                    Arc::clone(&self.players),
                )
                .handle()
                .await?;
            }
            _ => {
                warn!("Unexpected next state {:?} from {}", next_state, self.addr);
            }
        }

        info!("Connection from {:?} closed", self.addr);
        Ok(())
    }
}

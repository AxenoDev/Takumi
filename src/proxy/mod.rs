use std::sync::Arc;

use anyhow::Result;
use tokio::net::TcpListener;
use tracing::{error, info};

use crate::{
    configuration::TakumiConfig, player::PlayerRegistry, proxy::connection::ConnectionHandler,
    server,
};

pub mod connection;
pub mod status;

pub struct TakumiProxy {
    config: Arc<TakumiConfig>,
    players: Arc<PlayerRegistry>,
}

impl TakumiProxy {
    pub fn new(config: Arc<TakumiConfig>) -> Result<Self> {
        let players = Arc::new(PlayerRegistry::new());
        Ok(Self { config, players })
    }

    pub async fn run(&self) -> Result<()> {
        let server_addr = format!("{}:{}", self.config.proxy.bind, self.config.proxy.port);
        let listener = TcpListener::bind(&server_addr).await?;
        info!("Proxy listening on {}", server_addr);

        loop {
            let (stream, addr) = listener.accept().await?;
            info!("New connection from {}", addr);

            let config = Arc::clone(&self.config);
            let players = Arc::clone(&self.players);

            tokio::spawn(async move {
                let handler = connection::ConnectionHandler::new(stream, addr, config, players);
                if let Err(e) = handler.handle().await {
                    error!("Error handling connection from {:?}: {e}", addr);
                }
            });
        }
    }
}

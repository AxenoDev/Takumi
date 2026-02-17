use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use tracing::{debug, warn};

use crate::protocol::MinecraftCodec;

#[derive(Debug, Clone)]
pub struct BackendServer {
    pub name: String,
    pub address: String,
}

impl BackendServer {
    pub fn new(name: String, address: String) -> Self {
        Self {
            name: name.into(),
            address: address.into(),
        }
    }

    pub async fn connect(&self) -> anyhow::Result<Framed<TcpStream, MinecraftCodec>> {
        debug!(
            "Connecting to backend server {} at {}",
            self.name, self.address
        );
        let stream = TcpStream::connect(&self.address).await?;
        stream.set_nodelay(true)?;
        Ok(Framed::new(stream, MinecraftCodec::new()))
    }

    pub async fn is_online(&self) -> bool {
        match TcpStream::connect(&self.address).await {
            Ok(_) => true,
            Err(_) => {
                warn!(
                    "Backend server {} at {} is offline",
                    self.name, self.address
                );
                false
            }
        }
    }
}

mod configuration;
mod player;
mod protocol;
mod proxy;
mod server;

use crate::{configuration::TakumiConfig, proxy::TakumiProxy};
use anyhow::Ok;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{self, EnvFilter, fmt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = match TakumiConfig::load("config.toml") {
        std::result::Result::Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load config: {e}");
            let default_config = TakumiConfig::default();
            if let Err(save_err) = default_config.save("config.toml") {
                eprintln!("Failed to save default config: {save_err}");
            }
            default_config
        }
    };

    let filter = EnvFilter::try_new("info").unwrap_or_else(|_| EnvFilter::new("info"));

    fmt().with_env_filter(filter).with_target(false).init();

    print_banner();

    info!(
        "Starting Takumi Proxy on {}:{}",
        config.proxy.bind, config.proxy.port
    );

    let proxy = TakumiProxy::new(Arc::new(config))?;

    tokio::select! {
        res = proxy.run() => {
            if let Err(e) = res {
                eprintln!("Proxy encountered a fatal error: {e}");
            }
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Shutdown signal received, shutting down...");
        }
    }

    info!("Goodbye!");
    Ok(())
}

fn print_banner() {
    println!(
        r#"
  _____     _                    _ 
 |_   _|_ _| | ___   _ _ __ ___ (_)
   | |/ _` | |/ / | | | '_ ` _ \| |
   | | (_| |   <| |_| | | | | | | |
   |_|\__,_|_|\_\\__,_|_| |_| |_|_|
 High-performance Minecraft Proxy - v{}
"#,
        env!("CARGO_PKG_VERSION")
    );
}

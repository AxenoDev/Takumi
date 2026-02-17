mod config;

use crate::config::TakumiConfig;
use anyhow::Ok;
use tracing::info;
use tracing_subscriber::EnvFilter;

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

    print_banner();

    info!("Starting Takumi Proxy on {}:{}", config.proxy.bind, config.proxy.port);

    info!("Goodbye!");
    Ok(())
}

fn print_banner() {
    println!(
        r#"
  ████████╗ █████╗ ██╗  ██╗██╗   ██╗███╗   ███╗██╗
  ╚══██╔══╝██╔══██╗██║ ██╔╝██║   ██║████╗ ████║██║
     ██║   ███████║█████╔╝ ██║   ██║██╔████╔██║██║
     ██║   ██╔══██║██╔═██╗ ██║   ██║██║╚██╔╝██║██║
     ██║   ██║  ██║██║  ██╗╚██████╔╝██║ ╚═╝ ██║██║
     ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚═╝     ╚═╝╚═╝
  High-performance Minecraft Proxy — v{}
"#,
        env!("CARGO_PKG_VERSION")
    );
}

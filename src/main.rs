mod blocklist;
mod config;
mod handler;

use crate::config::Config;
use hickory_resolver::Resolver;
use hickory_resolver::TokioResolver;
use hickory_resolver::config::ResolverConfig;
use hickory_resolver::name_server::TokioConnectionProvider;
// use hickory_resolver::{TokioResolver, ResolverOpts, ResolverConfig};
use hickory_server::ServerFuture;
// use std::net::SocketAddr;

use crate::blocklist::Blocklist;
use crate::handler::DnsHandler;
use std::sync::Arc;
// use std::time::Duration;
// use tokio::net::{TcpListener, UdpSocket};

pub struct AppState {
    pub config: Config,
    pub blocklist: Blocklist,
    pub resolver: TokioResolver,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🚀 DNS Sinker - Initializing...\n");

    let config = Config::load("config.toml")?;
    println!("✅ Configuration loaded");

    let blocklist = Blocklist::new("blocklist.txt");

    let resolver = Resolver::builder_with_config(
        ResolverConfig::default(),
        TokioConnectionProvider::default(),
    )
    .build();
    println!("✅ Upstream resolver initialized");

    let app_state = Arc::new(AppState {
        config: config,
        blocklist,
        resolver,
    });

    let handler = DnsHandler { state: app_state };

    // 6. Create Server
    let mut server = ServerFuture::new(handler);

 
    Ok(())
}

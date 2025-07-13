mod api;
mod tunnel;
mod state;
mod config;

use api::build_routes;
use state::{load_state};
use config::{load_config};
use tunnel::{TunnelHandle, start_tunnel};

use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

pub type TunnelMap = Arc<Mutex<HashMap<u16, TunnelHandle>>>;

#[tokio::main]
async fn main() {
    let config_map = load_config().await;
    if config_map.is_empty() {
        eprintln!("No configuration found. Exiting.");
        return;
    }
    let config = config_map.get("default").expect("Missing 'default' config");

    let socks_proxy = format!("{}:{}", config.socks5_proxy_host, config.socks5_proxy_port);
    let api_addr: SocketAddr = config.listen_addr.parse().expect("Invalid listen_addr");

    let map: TunnelMap = Arc::new(Mutex::new(HashMap::new()));

    for info in load_state().await {
        let h = start_tunnel(info.clone(), socks_proxy.clone()).await;
        map.lock().await.insert(info.port, TunnelHandle { info, handle: h });
    }

    let routes = build_routes(map.clone(), socks_proxy.clone());
    println!("API listening on http://{}/", api_addr);
    warp::serve(routes).run(api_addr).await;
}

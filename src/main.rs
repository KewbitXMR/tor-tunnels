mod api;
mod tunnel;
mod state;

use api::build_routes;
use state::{load_state};
use tunnel::{TunnelHandle, start_tunnel};

use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

pub type TunnelMap = Arc<Mutex<HashMap<u16, TunnelHandle>>>;

#[tokio::main]
async fn main() {
    let socks_proxy = "127.0.0.1:9066".to_string();
    let api_addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();

    let map: TunnelMap = Arc::new(Mutex::new(HashMap::new()));

    for info in load_state().await {
        let h = start_tunnel(info.clone(), socks_proxy.clone()).await;
        map.lock().await.insert(info.port, TunnelHandle { info, handle: h });
    }

    let routes = build_routes(map.clone(), socks_proxy.clone());
    println!("API listening on http://{}/", api_addr);
    warp::serve(routes).run(api_addr).await;
}

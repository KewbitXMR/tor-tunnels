use crate::{TunnelMap, TunnelHandle};
use crate::tunnel::{start_tunnel, TunnelInfo};
use crate::state::save_state;

use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, Filter, Rejection, Reply};
use warp::reply; // ✅ Required for reply::with_status and reply::json

#[derive(Serialize)]
struct ErrorMessage {
    error: String,
}

enum TunnelResponse {
    NoContent,
    NotFound(String),
}

impl Reply for TunnelResponse {
    fn into_response(self) -> warp::reply::Response {
        match self {
            TunnelResponse::NoContent => {
                // Return empty body with 204 No Content
                reply::with_status(reply::reply(), StatusCode::NO_CONTENT).into_response()
            },
            TunnelResponse::NotFound(msg) => {
                let json = reply::json(&ErrorMessage { error: msg });
                reply::with_status(json, StatusCode::NOT_FOUND).into_response()
            }
        }
    }
}

pub fn build_routes(map: TunnelMap, socks_proxy: String) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let map_filter = warp::any().map(move || map.clone());
    let socks_filter = warp::any().map(move || socks_proxy.clone());

    let spawn = warp::path("spawn")
        .and(warp::post())
        .and(warp::body::json())
        .and(map_filter.clone())
        .and(socks_filter.clone())
        .and_then(spawn_tunnel);

    let list = warp::path("list")
        .and(warp::get())
        .and(map_filter.clone())
        .and_then(list_tunnels);

    let destroy = warp::path!("destroy" / u16)
        .and(warp::delete())
        .and(map_filter.clone())
        .and_then(destroy_tunnel);

    spawn.or(list).or(destroy)
}

#[derive(Deserialize)]
struct SpawnReq {
    onion: String,
    port: Option<u16>,
}

async fn spawn_tunnel(body: SpawnReq, map: TunnelMap, socks: String) -> Result<impl Reply, Rejection> {
    let port = if let Some(p) = body.port {
        p
    } else {
        let mut p = 5000u16;
        loop {
            if !map.lock().await.contains_key(&p) { break p; }
            p += 1;
        }
    };

    if map.lock().await.contains_key(&port) {
        return Ok(
            warp::reply::with_status("Port in use", StatusCode::CONFLICT)
                .into_response()
        );
    }

    let info = TunnelInfo { port, onion: body.onion.clone() };
    let handle = start_tunnel(info.clone(), socks).await;
    map.lock().await.insert(port, TunnelHandle { info: info.clone(), handle });
    save_state(&map).await;
    Ok(
        warp::reply::with_status(warp::reply::json(&info), StatusCode::OK)
            .into_response()
    )
}

async fn list_tunnels(map: TunnelMap) -> Result<impl Reply, Rejection> {
    let infos: Vec<_> = map.lock().await.values().map(|h| h.info.clone()).collect();
    Ok(warp::reply::json(&infos))
}

async fn destroy_tunnel(port: u16, map: TunnelMap) -> Result<impl Reply, Rejection> {
    // Scope 1: remove the tunnel while locked
    let maybe_handle = {
        let mut lock = map.lock().await;
        lock.remove(&port)
    };

    if let Some(h) = maybe_handle {
        h.handle.abort();

        // Scope 2: save state after the lock has been released
        save_state(&map).await;

        println!("[port {}] Tunnel destroyed", port);
        Ok(TunnelResponse::NoContent)
    } else {
        Ok(TunnelResponse::NotFound("Tunnel not found".into()))
    }
}
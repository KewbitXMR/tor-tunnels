use serde::{Serialize, Deserialize};
use tokio::{io::copy_bidirectional, net::{TcpListener, TcpStream}, task::JoinHandle};
use tokio_socks::tcp::Socks5Stream;
use std::error::Error;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TunnelInfo {
    pub port: u16,
    pub onion: String,
}

pub struct TunnelHandle {
    pub info: TunnelInfo,
    pub handle: JoinHandle<()>,
}

pub async fn start_tunnel(info: TunnelInfo, socks_proxy: String) -> JoinHandle<()> {
    tokio::spawn(async move {
        let listen_addr = format!("127.0.0.1:{}", info.port);
        let listener = match TcpListener::bind(&listen_addr).await {
            Ok(l) => l,
            Err(e) => {
                eprintln!("[port {}] Failed to bind: {}", info.port, e);
                return;
            }
        };
        println!("[port {}] Tunnel → {} ready", info.port, info.onion);

        loop {
            match listener.accept().await {
                Ok((client_stream, client_addr)) => {
                    let onion = info.onion.clone();
                    let socks = socks_proxy.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handle_client(client_stream, &socks, &onion).await {
                            eprintln!("[port {}] {}", info.port, e);
                        }
                        println!("[port {}] Closed {}", info.port, client_addr);
                    });
                }
                Err(e) => eprintln!("[port {}] accept error: {}", info.port, e),
            }
        }
    })
}

async fn handle_client(mut client: TcpStream, socks5: &str, dest: &str) -> Result<(), Box<dyn Error>> {
    let mut remote = Socks5Stream::connect(socks5, dest).await?.into_inner();
    copy_bidirectional(&mut client, &mut remote).await?;
    Ok(())
}
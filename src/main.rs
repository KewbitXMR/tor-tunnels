use tokio::net::{TcpListener, TcpStream};
use tokio::io::copy_bidirectional;
use tokio_socks::tcp::Socks5Stream;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Configuration parameters
    let listen_addr = "127.0.0.1:5656";
    let socks5_proxy = "127.0.0.1:9066"; // Arti's SOCKS5 proxy
    let hidden_service_addr = "5i6blbmuflq4s4im6zby26a7g22oef6kyp7vbwyru6oq5e36akzo3ayd.onion:2001"; // Replace with your hidden service address and port

    // Bind the TCP listener
    let listener = TcpListener::bind(listen_addr).await?;
    println!("Tunnel is listening on {}", listen_addr);

    loop {
        // Accept incoming connections
        let (client_stream, client_addr) = listener.accept().await?;
        println!("Accepted connection from {}", client_addr);

        // Clone variables for the async block
        let hidden_service_addr = hidden_service_addr.to_string();
        let socks5_proxy = socks5_proxy.to_string();

        // Handle the client connection asynchronously
        tokio::spawn(async move {
            if let Err(e) = handle_client(client_stream, &socks5_proxy, &hidden_service_addr).await {
                eprintln!("Error handling client {}: {:?}", client_addr, e);
            }
        });
    }
}

// Function to handle client connections
async fn handle_client(
    mut client_stream: TcpStream,
    socks5_proxy: &str,
    hidden_service_addr: &str,
) -> Result<(), Box<dyn Error>> {
    // Connect to the hidden service via the SOCKS5 proxy
    let mut remote_stream = Socks5Stream::connect(socks5_proxy, hidden_service_addr).await?.into_inner();
    println!("Connected to hidden service at {}", hidden_service_addr);

    // Bi-directional data transfer between client and hidden service
    copy_bidirectional(&mut client_stream, &mut remote_stream).await?;

    println!("Connection to {} closed", hidden_service_addr);
    Ok(())
}


This application creates a TCP tunnel that allows clients to connect to a Tor hidden service through a SOCKS5 proxy. It listens for incoming TCP connections, connects to the hidden service using the provided SOCKS5 proxy, and then forwards data between the client and the hidden service.

## Requirements

- Rust (latest stable version)
- Dependencies:
  - `tokio`: For asynchronous runtime.
  - `tokio-socks`: A library to work with SOCKS5 proxies in Tokio.
  - `tokio-io`: For handling bidirectional I/O between the client and hidden service.

## Installation

1. Clone the repository:
    ```
    git clone <repository-url>
    cd <repository-name>
    ```

2. Add dependencies to your `Cargo.toml`:
    ```toml
    [dependencies]
    tokio = { version = "1", features = ["full"] }
    tokio-socks = "0.1.4"
    ```

3. Build the project:
    ```
    cargo build --release
    ```

## Configuration

Before running the application, you need to configure the following parameters:

- `listen_addr`: The local address and port the tunnel will listen on for incoming connections (e.g., `127.0.0.1:5656`).
- `socks5_proxy`: The SOCKS5 proxy address to use for connecting to the Tor network (e.g., `127.0.0.1:9050` for a local Tor proxy).
- `hidden_service_addr`: The address and port of the hidden service you want to tunnel to (e.g., `5i6blbmuflq4s4im6zby26a7g22oef6kyp7vbwyru6oq5e36akzo3ayd.onion:2001`).

You can adjust these values in the `main` function of `src/main.rs` as needed.

## Usage

1. Run the application:
    ```
    cargo run --release
    ```

    The server will start listening on the `listen_addr` (e.g., `127.0.0.1:5656`) for incoming connections.

2. Clients can connect to the specified address and port. The server will then establish a connection to the hidden service through the SOCKS5 proxy and forward the data between the client and the hidden service.

## How It Works

- The `main` function binds a TCP listener to `listen_addr` and waits for incoming connections.
- For each incoming connection, a new asynchronous task is spawned using `tokio::spawn`.
- In the `handle_client` function:
  - A connection to the hidden service is established via the provided SOCKS5 proxy using `Socks5Stream::connect`.
  - Bi-directional data transfer occurs between the client and the hidden service using `tokio::io::copy_bidirectional`.

## EXPERIMENTAL
It's only useful for one perpose at the moment as I'm focusing on something else, currently you have to build it with the hidden service hardcoded, but in future it will be a library which you can create, destory tunnels in runtime, perhaps exposing an RPC API for other none rust applications too.

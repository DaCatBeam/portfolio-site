use portfolio_site::run;
use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use std::process;

#[tokio::main]
async fn main() {
    let host = Ipv4Addr::new(127, 0, 0, 1);
    let port = 3000;
    let socket_addr = SocketAddr::new(IpAddr::V4(host), port);

    println!("Starting server at: {}", socket_addr);

    if let Err(msg) = run(socket_addr).await {
        eprintln!("Error: {:?}", msg);
        process::exit(1);
    }
}

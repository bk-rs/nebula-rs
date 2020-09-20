/*
cargo run -p nebula-graph-demo-async-std --bin graph_client 127.0.0.1 3699 user 'password'
*/

use std::env;
use std::io;

use async_std::net::TcpStream;

use fbthrift_transport::AsyncTransport;
use nebula_graph_client::{AsyncGraphClient, Query as _};

#[async_std::main]
async fn main() -> io::Result<()> {
    run().await
}

async fn run() -> io::Result<()> {
    let domain = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("DOMAIN").unwrap_or("127.0.0.1".to_owned()));
    let port: u16 = env::args()
        .nth(2)
        .unwrap_or_else(|| env::var("PORT").unwrap_or("3699".to_owned()))
        .parse()
        .unwrap();
    let username = env::args()
        .nth(3)
        .unwrap_or_else(|| env::var("USERNAME").unwrap_or("user".to_owned()));
    let password = env::args()
        .nth(4)
        .unwrap_or_else(|| env::var("PASSWORD").unwrap_or("password".to_owned()));

    println!("graph_client {} {} {} {}", domain, port, username, password);

    //
    let addr = format!("{}:{}", domain, port);
    let stream = TcpStream::connect(addr).await?;

    //
    let transport = AsyncTransport::new(stream, None);
    let client = AsyncGraphClient::new(transport);

    let session = client
        .authenticate(username.as_str(), password.as_str())
        .await
        .unwrap();

    let out = session.show_hosts().await.unwrap();
    println!("{:?}", out);

    println!("done");

    Ok(())
}

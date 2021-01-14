/*
cargo run -p nebula-demo-async-std --bin v2_graph_client 127.0.0.1 9669 user 'password'
*/

use std::env;
use std::io;

use async_std::net::TcpStream;

use fbthrift_transport::{
    futures_io::transport::AsyncTransport, DefaultAsyncTransportConfiguration,
};
use nebula_client::v2::GraphClient;

#[async_std::main]
async fn main() -> io::Result<()> {
    run().await
}

async fn run() -> io::Result<()> {
    let domain = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("DOMAIN").unwrap_or_else(|_| "127.0.0.1".to_owned()));
    let port: u16 = env::args()
        .nth(2)
        .unwrap_or_else(|| env::var("PORT").unwrap_or_else(|_| "9669".to_owned()))
        .parse()
        .unwrap();
    let username = env::args()
        .nth(3)
        .unwrap_or_else(|| env::var("USERNAME").unwrap_or_else(|_| "user".to_owned()));
    let password = env::args()
        .nth(4)
        .unwrap_or_else(|| env::var("PASSWORD").unwrap_or_else(|_| "password".to_owned()));

    println!(
        "v2_graph_client {} {} {} {}",
        domain, port, username, password
    );

    //
    let addr = format!("{}:{}", domain, port);
    let stream = TcpStream::connect(addr).await?;

    //
    let transport = AsyncTransport::new(stream, DefaultAsyncTransportConfiguration::default());
    let client = GraphClient::new(transport);

    let mut session = client
        .authenticate(&username.as_bytes().to_vec(), &password.as_bytes().to_vec())
        .await
        .unwrap();

    let res = session.execute(&b"SHOW HOSTS;".to_vec()).await.unwrap();
    println!("{:?}", res);

    println!("done");

    Ok(())
}

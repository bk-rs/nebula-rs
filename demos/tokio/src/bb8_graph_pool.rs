/*
cargo run -p nebula-graph-demo-tokio --bin bb8_graph_pool 127.0.0.1 3699 user 'password'
*/

use std::env;
use std::io;

use bb8_nebula_graph::{NebulaGraphClientConfiguration, NebulaGraphConnectionManager};
use fbthrift_transport::AsyncTransportConfiguration;
use nebula_graph_client::{GraphTransportResponseHandler, Query as _};

#[tokio::main]
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
    let space = env::args().nth(5).or_else(|| env::var("SPACE").ok());

    println!(
        "bb8_graph_pool {} {} {} {} {:?}",
        domain, port, username, password, space
    );

    //
    let client_configuration =
        NebulaGraphClientConfiguration::new(domain, port, username, password, space);
    let transport_configuration = AsyncTransportConfiguration::new(GraphTransportResponseHandler);
    let manager = NebulaGraphConnectionManager::new(client_configuration, transport_configuration);
    let pool = bb8::Pool::builder().max_size(10).build(manager).await?;

    //
    let mut session = pool.get().await.unwrap();
    let out = session.show_hosts().await.unwrap();
    println!("{:?}", out);

    //
    let mut session = pool.get().await.unwrap();
    let out = session.show_hosts().await.unwrap();
    println!("{:?}", out);

    println!("done");

    Ok(())
}

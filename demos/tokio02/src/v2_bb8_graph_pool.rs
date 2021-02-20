/*
cargo run -p nebula-demo-tokio02 --bin v2_bb8_graph_pool 127.0.0.1 9669 user 'password'
*/

use std::env;
use std::error;

use bb8_nebula::tokio02::v2::{GraphClientConfiguration, GraphConnectionManager};
use fbthrift_transport::AsyncTransportConfiguration;
use nebula_client::v2::GraphTransportResponseHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    run().await
}

async fn run() -> Result<(), Box<dyn error::Error>> {
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
    let space = env::args().nth(5).or_else(|| env::var("SPACE").ok());

    println!(
        "v2_bb8_graph_pool {} {} {} {} {:?}",
        domain, port, username, password, space
    );

    //
    let client_configuration =
        GraphClientConfiguration::new(domain, port, username, password, space);
    let transport_configuration = AsyncTransportConfiguration::new(GraphTransportResponseHandler);
    let manager = GraphConnectionManager::new(client_configuration, transport_configuration);
    let pool = bb8::Pool::builder().max_size(1).build(manager).await?;

    //
    {
        let mut session = pool.get().await?;
        let res = session.execute(&"SHOW HOSTS;".as_bytes().to_vec()).await?;
        println!("{:?}", res);
    }

    //
    {
        let mut session = pool.get().await?;
        let res = session.execute(&"SHOW HOSTS;".as_bytes().to_vec()).await?;
        println!("{:?}", res);
    }

    println!("done");

    Ok(())
}

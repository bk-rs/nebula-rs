/*
cargo run -p nebula-demo-async-std --bin mobc_graph_pool 127.0.0.1 3699 user 'password'
*/

use std::env;
use std::error;
use std::time::Duration;

use fbthrift_transport::AsyncTransportConfiguration;
use mobc_nebula::{GraphClientConfiguration, GraphConnectionManager};
use nebula_client::{GraphQuery as _, GraphTransportResponseHandler};

#[async_std::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    run().await
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let domain = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("DOMAIN").unwrap_or_else(|_| "127.0.0.1".to_owned()));
    let port: u16 = env::args()
        .nth(2)
        .unwrap_or_else(|| env::var("PORT").unwrap_or_else(|_| "3699".to_owned()))
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
        "mobc_graph_pool {} {} {} {} {:?}",
        domain, port, username, password, space
    );

    //
    let client_configuration =
        GraphClientConfiguration::new(domain, port, username, password, space);
    let transport_configuration = AsyncTransportConfiguration::new(GraphTransportResponseHandler);
    let manager = GraphConnectionManager::new(client_configuration, transport_configuration);

    /*
    etc/nebula-graphd.conf
    --session_idle_timeout_secs=60000
    */
    let pool = mobc::Pool::builder()
        .max_open(1)
        .max_idle_lifetime(Some(Duration::from_secs(7200)))
        .build(manager);

    //
    {
        let mut session = pool.get().await?;
        let out = session.show_hosts().await?;
        println!("{:?}", out);
    }

    //
    {
        let mut session = pool.get().await?;
        let out = session.show_hosts().await?;
        println!("{:?}", out);
    }

    println!("done");

    Ok(())
}

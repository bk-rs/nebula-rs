/*
cargo run -p nebula-graph-demo-async-std --bin v2_mobc_graph_pool 127.0.0.1 3699 user 'password'
*/

use std::env;
use std::io;

use mobc_nebula_graph::v2::{NebulaGraphClientConfiguration, NebulaGraphConnectionManager};

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
    let space = env::args().nth(5).or_else(|| env::var("SPACE").ok());

    println!(
        "v2_mobc_graph_pool {} {} {} {} {:?}",
        domain, port, username, password, space
    );

    //
    let client_configuration =
        NebulaGraphClientConfiguration::new(domain, port, username, password, space);
    let manager = NebulaGraphConnectionManager::new(client_configuration, None);
    let pool = mobc::Pool::builder().max_open(10).build(manager);

    //
    let session = pool.get().await.unwrap();

    let res = session.execute(&b"SHOW HOSTS;".to_vec()).await.unwrap();
    println!("{:?}", res);

    println!("done");

    Ok(())
}

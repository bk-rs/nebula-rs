/*
cargo run -p nebula-demo-async-std --bin graph_crud 127.0.0.1 3699 user 'password' 'space'
*/

use std::env;
use std::error;
use std::time::Duration;

use async_std::net::TcpStream;
use async_std::task;

use chrono::{serde::ts_seconds, DateTime, Utc};
use fbthrift_transport::{futures_io::transport::AsyncTransport, AsyncTransportConfiguration};
use nebula_client::{GraphClient, GraphQuery as _, GraphTransportResponseHandler};
use serde::Deserialize;

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
    let space = env::args()
        .nth(5)
        .unwrap_or_else(|| env::var("SPACE").unwrap_or_else(|_| "space".to_owned()));

    println!(
        "graph_crud {} {} {} {} {}",
        domain, port, username, password, space
    );

    //
    let addr = format!("{}:{}", domain, port);
    let stream = TcpStream::connect(addr).await?;

    //
    let transport = AsyncTransport::new(
        stream,
        AsyncTransportConfiguration::new(GraphTransportResponseHandler),
    );
    let client = GraphClient::new(transport);

    let mut session = client
        .authenticate(username.as_str(), password.as_str())
        .await?;

    session.query(&format!("USE {}", space)).await?;

    //
    let tag_name = "FOO";
    let vid = 1;

    session
        .query(&format!("CREATE TAG IF NOT EXISTS {tag_name} (field_string string, field_int int, field_double double, field_bool bool, field_timestamp timestamp)", tag_name = tag_name))
        .await?;

    session
        .query(&format!(
            "CREATE TAG INDEX IF NOT EXISTS {tag_name}_index0 ON {tag_name}(field_string )",
            tag_name = tag_name
        ))
        .await?;

    task::sleep(Duration::from_secs(3)).await;

    session
        .query(&format!(r#"INSERT VERTEX {tag_name} (field_string, field_int, field_double, field_bool, field_timestamp) VALUE {vid}:("1", 2, 3.3, true, now())"#, tag_name = tag_name, vid = vid))
        .await?;

    task::sleep(Duration::from_secs(3)).await;

    #[derive(Deserialize, Debug)]
    struct Foo {
        #[serde(rename(deserialize = "VertexID"))]
        vid: u64,
        #[serde(rename(deserialize = "FOO.field_string"))]
        field_string: String,
        #[serde(rename(deserialize = "FOO.field_int"))]
        field_int: u64,
        #[serde(rename(deserialize = "FOO.field_double"))]
        field_double: f64,
        #[serde(rename(deserialize = "FOO.field_bool"))]
        field_bool: bool,
        #[serde(rename(deserialize = "FOO.field_timestamp"), with = "ts_seconds")]
        field_timestamp: DateTime<Utc>,
    }
    let query_output = session
        .query_as::<Foo>(&format!(r#"LOOKUP ON {tag_name} WHERE {tag_name}.field_string == "1" YIELD {tag_name}.field_string, {tag_name}.field_int, {tag_name}.field_double, {tag_name}.field_bool, {tag_name}.field_timestamp"#, tag_name = tag_name))
        .await?;
    println!("{:?}", query_output);

    session
        .query(&format!("DELETE VERTEX {vid}", vid = vid))
        .await?;

    session
        .query(&format!(
            "DROP TAG INDEX IF EXISTS {tag_name}_index0",
            tag_name = tag_name
        ))
        .await?;

    session
        .query(&format!(
            "DROP TAG IF EXISTS {tag_name}",
            tag_name = tag_name
        ))
        .await?;

    println!("done");

    Ok(())
}

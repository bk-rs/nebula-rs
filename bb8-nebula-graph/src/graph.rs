use std::io;
use std::result;

use async_trait::async_trait;
use bb8;
use fbthrift_transport::AsyncTransport;
use nebula_graph_client::{AsyncGraphClient, AsyncGraphSession};
use tokio::net::TcpStream;

#[derive(Debug, Clone)]
pub struct NebulaGraphClientConfiguration {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub space: Option<String>,
}

impl NebulaGraphClientConfiguration {
    pub fn new(
        host: String,
        port: u16,
        username: String,
        password: String,
        space: Option<String>,
    ) -> Self {
        Self {
            host,
            port,
            username,
            password,
            space,
        }
    }

    async fn get_async_connection(
        &self,
    ) -> result::Result<AsyncGraphSession<AsyncTransport<TcpStream>>, io::Error> {
        let addr = format!("{}:{}", self.host, self.port);
        let stream = TcpStream::connect(&addr).await?;

        let transport = AsyncTransport::new(stream, None);

        let client = AsyncGraphClient::new(transport);

        let session = client
            .authenticate(&self.username, &self.password)
            .await
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        if let Some(ref space) = self.space {
            session
                .execute(&format!("USE {}", space))
                .await
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        }

        Ok(session)
    }
}

#[derive(Clone, Debug)]
pub struct NebulaGraphConnectionManager {
    configuration: NebulaGraphClientConfiguration,
}

impl NebulaGraphConnectionManager {
    pub fn new(configuration: NebulaGraphClientConfiguration) -> Self {
        Self { configuration }
    }
}

#[async_trait]
impl bb8::ManageConnection for NebulaGraphConnectionManager {
    type Connection = AsyncGraphSession<AsyncTransport<TcpStream>>;
    type Error = io::Error;

    async fn connect(&self) -> result::Result<Self::Connection, Self::Error> {
        self.configuration.get_async_connection().await
    }

    async fn is_valid(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        Ok(conn)
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        false
    }
}

use std::io;
use std::result;

use async_trait::async_trait;
use bb8;
use fbthrift_transport::{
    fbthrift_transport_response_handler::ResponseHandler, AsyncTransport,
    AsyncTransportConfiguration,
};
use nebula_client::v2::{GraphClient, GraphSession};
use tokio::net::TcpStream;

#[derive(Debug, Clone)]
pub struct GraphClientConfiguration {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub space: Option<String>,
}

impl GraphClientConfiguration {
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
}

#[derive(Clone)]
pub struct GraphConnectionManager<H>
where
    H: ResponseHandler,
{
    client_configuration: GraphClientConfiguration,
    transport_configuration: AsyncTransportConfiguration<H>,
}

impl<H> GraphConnectionManager<H>
where
    H: ResponseHandler + Send + Sync + 'static + Unpin,
{
    pub fn new(
        client_configuration: GraphClientConfiguration,
        transport_configuration: AsyncTransportConfiguration<H>,
    ) -> Self {
        Self {
            client_configuration,
            transport_configuration,
        }
    }

    async fn get_async_connection(
        &self,
    ) -> result::Result<GraphSession<AsyncTransport<TcpStream, H>>, io::Error> {
        let addr = format!(
            "{}:{}",
            self.client_configuration.host, self.client_configuration.port
        );
        let stream = TcpStream::connect(&addr).await?;

        let transport = AsyncTransport::new(stream, self.transport_configuration.clone());

        let client = GraphClient::new(transport);

        let mut session = client
            .authenticate(
                &self.client_configuration.username.as_bytes().to_vec(),
                &self.client_configuration.password.as_bytes().to_vec(),
            )
            .await
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        if let Some(ref space) = self.client_configuration.space {
            session
                .execute(&format!("USE {}", space).as_bytes().to_vec())
                .await
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        }

        Ok(session)
    }
}

#[async_trait]
impl<H> bb8::ManageConnection for GraphConnectionManager<H>
where
    H: ResponseHandler + Send + Sync + 'static + Unpin,
{
    type Connection = GraphSession<AsyncTransport<TcpStream, H>>;
    type Error = io::Error;

    async fn connect(&self) -> result::Result<Self::Connection, Self::Error> {
        self.get_async_connection().await
    }

    async fn is_valid(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        Ok(conn)
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        conn.is_close_required()
    }
}

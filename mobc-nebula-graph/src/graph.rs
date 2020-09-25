use std::io;
use std::result;

use fbthrift_transport::{
    fbthrift_transport_response_handler::ResponseHandler, AsyncTransport,
    AsyncTransportConfiguration,
};
use mobc::async_trait;
use mobc::Manager;
use nebula_graph_client::{AsyncGraphClient, AsyncGraphSession};

#[cfg(feature = "async_std")]
use async_std::net::TcpStream;
#[cfg(feature = "tokio")]
use tokio02::net::TcpStream;

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
}

#[derive(Clone)]
pub struct NebulaGraphConnectionManager<H>
where
    H: ResponseHandler,
{
    client_configuration: NebulaGraphClientConfiguration,
    transport_configuration: AsyncTransportConfiguration<H>,
}

impl<H> NebulaGraphConnectionManager<H>
where
    H: ResponseHandler + Send + Sync + 'static + Unpin,
{
    pub fn new(
        client_configuration: NebulaGraphClientConfiguration,
        transport_configuration: AsyncTransportConfiguration<H>,
    ) -> Self {
        Self {
            client_configuration,
            transport_configuration,
        }
    }

    async fn get_async_connection(
        &self,
    ) -> result::Result<AsyncGraphSession<AsyncTransport<TcpStream, H>>, io::Error> {
        let addr = format!(
            "{}:{}",
            self.client_configuration.host, self.client_configuration.port
        );
        let stream = TcpStream::connect(&addr).await?;

        let transport = AsyncTransport::new(stream, self.transport_configuration.clone());

        let client = AsyncGraphClient::new(transport);

        let mut session = client
            .authenticate(
                &self.client_configuration.username,
                &self.client_configuration.password,
            )
            .await
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        if let Some(ref space) = self.client_configuration.space {
            session
                .execute(&format!("USE {}", space))
                .await
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        }

        Ok(session)
    }
}

#[async_trait]
impl<H> Manager for NebulaGraphConnectionManager<H>
where
    H: ResponseHandler + Send + Sync + 'static + Unpin,
{
    type Connection = AsyncGraphSession<AsyncTransport<TcpStream, H>>;
    type Error = io::Error;

    async fn connect(&self) -> result::Result<Self::Connection, Self::Error> {
        self.get_async_connection().await
    }

    async fn check(&self, mut conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        conn.execute("SHOW CHARSET")
            .await
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        Ok(conn)
    }

    fn validate(&self, conn: &mut Self::Connection) -> bool {
        !conn.is_close_required()
    }
}

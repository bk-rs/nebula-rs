use std::result;

use bytes::Bytes;
use fbthrift::{ApplicationException, ApplicationExceptionErrorCode, BinaryProtocol, Transport};
use nebula_fbthrift_graph_v2::{
    client::{GraphService, GraphServiceImpl},
    errors::graph_service::{AuthenticateError, ExecuteError, ExecuteJsonError, SignoutError},
    types::{ErrorCode, ExecutionResponse},
};

//
//
//
struct AsyncGraphConnection<T>
where
    T: Transport,
    Bytes: ::fbthrift::Framing<DecBuf = ::fbthrift::FramingDecoded<T>>,
    ::fbthrift::ProtocolEncoded<BinaryProtocol>:
        ::fbthrift::BufMutExt<Final = ::fbthrift::FramingEncodedFinal<T>>,
{
    service: GraphServiceImpl<BinaryProtocol, T>,
}

impl<T> AsyncGraphConnection<T>
where
    T: Transport,
    Bytes: ::fbthrift::Framing<DecBuf = ::fbthrift::FramingDecoded<T>>,
    ::fbthrift::ProtocolEncoded<BinaryProtocol>:
        ::fbthrift::BufMutExt<Final = ::fbthrift::FramingEncodedFinal<T>>,
{
    fn new(transport: T) -> Self {
        Self {
            service: GraphServiceImpl::<BinaryProtocol, _>::new(transport),
        }
    }
}

//
//
//
pub struct AsyncGraphClient<T>
where
    T: Transport,
    Bytes: ::fbthrift::Framing<DecBuf = ::fbthrift::FramingDecoded<T>>,
    ::fbthrift::ProtocolEncoded<BinaryProtocol>:
        ::fbthrift::BufMutExt<Final = ::fbthrift::FramingEncodedFinal<T>>,
{
    connection: AsyncGraphConnection<T>,
}

impl<T> AsyncGraphClient<T>
where
    T: Transport,
    Bytes: ::fbthrift::Framing<DecBuf = ::fbthrift::FramingDecoded<T>>,
    ::fbthrift::ProtocolEncoded<BinaryProtocol>:
        ::fbthrift::BufMutExt<Final = ::fbthrift::FramingEncodedFinal<T>>,
{
    pub fn new(transport: T) -> Self {
        Self {
            connection: AsyncGraphConnection::new(transport),
        }
    }

    pub async fn authenticate(
        self,
        username: &Vec<u8>,
        password: &Vec<u8>,
    ) -> result::Result<AsyncGraphSession<T>, AuthenticateError> {
        let res = self
            .connection
            .service
            .authenticate(username, password)
            .await?;

        if res.error_code != ErrorCode::SUCCEEDED {
            return Err(ApplicationException::new(
                ApplicationExceptionErrorCode::Unknown,
                res.error_msg
                    .map(|x| String::from_utf8_lossy(&x).to_string())
                    .unwrap_or_else(|| "Unknown".to_owned()),
            )
            .into());
        }
        let session_id = res.session_id.ok_or_else(|| {
            ApplicationException::new(
                ApplicationExceptionErrorCode::InternalError,
                "Missing session_id".to_owned(),
            )
        })?;

        Ok(AsyncGraphSession::new(self.connection, session_id))
    }
}

//
//
//
pub struct AsyncGraphSession<T>
where
    T: Transport,
    Bytes: ::fbthrift::Framing<DecBuf = ::fbthrift::FramingDecoded<T>>,
    ::fbthrift::ProtocolEncoded<BinaryProtocol>:
        ::fbthrift::BufMutExt<Final = ::fbthrift::FramingEncodedFinal<T>>,
{
    connection: AsyncGraphConnection<T>,
    session_id: i64,
    close_required: bool,
}

impl<T> AsyncGraphSession<T>
where
    T: Transport,
    Bytes: ::fbthrift::Framing<DecBuf = ::fbthrift::FramingDecoded<T>>,
    ::fbthrift::ProtocolEncoded<BinaryProtocol>:
        ::fbthrift::BufMutExt<Final = ::fbthrift::FramingEncodedFinal<T>>,
{
    fn new(connection: AsyncGraphConnection<T>, session_id: i64) -> Self {
        Self {
            connection,
            session_id,
            close_required: false,
        }
    }

    pub async fn signout(self) -> result::Result<(), SignoutError> {
        self.connection.service.signout(self.session_id).await
    }

    pub async fn execute(
        &mut self,
        stmt: &Vec<u8>,
    ) -> result::Result<ExecutionResponse, ExecuteError> {
        let res = self
            .connection
            .service
            .execute(self.session_id, stmt)
            .await?;

        match res.error_code {
            ErrorCode::E_SESSION_INVALID | ErrorCode::E_SESSION_TIMEOUT => {
                self.close_required = true;
            }
            _ => {}
        }

        Ok(res)
    }

    pub async fn execute_json(&self, stmt: &Vec<u8>) -> result::Result<Vec<u8>, ExecuteJsonError> {
        self.connection
            .service
            .executeJson(self.session_id, stmt)
            .await
    }

    pub fn is_close_required(&self) -> bool {
        self.close_required
    }
}

use std::io;
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
struct GraphConnection<T>
where
    T: Transport,
    Bytes: ::fbthrift::Framing<DecBuf = ::fbthrift::FramingDecoded<T>>,
    ::fbthrift::ProtocolEncoded<BinaryProtocol>:
        ::fbthrift::BufMutExt<Final = ::fbthrift::FramingEncodedFinal<T>>,
{
    service: GraphServiceImpl<BinaryProtocol, T>,
}

impl<T> GraphConnection<T>
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
pub struct GraphClient<T>
where
    T: Transport,
    Bytes: ::fbthrift::Framing<DecBuf = ::fbthrift::FramingDecoded<T>>,
    ::fbthrift::ProtocolEncoded<BinaryProtocol>:
        ::fbthrift::BufMutExt<Final = ::fbthrift::FramingEncodedFinal<T>>,
{
    connection: GraphConnection<T>,
}

impl<T> GraphClient<T>
where
    T: Transport,
    Bytes: ::fbthrift::Framing<DecBuf = ::fbthrift::FramingDecoded<T>>,
    ::fbthrift::ProtocolEncoded<BinaryProtocol>:
        ::fbthrift::BufMutExt<Final = ::fbthrift::FramingEncodedFinal<T>>,
{
    pub fn new(transport: T) -> Self {
        Self {
            connection: GraphConnection::new(transport),
        }
    }

    #[allow(clippy::ptr_arg)]
    pub async fn authenticate(
        self,
        username: &Vec<u8>,
        password: &Vec<u8>,
    ) -> result::Result<GraphSession<T>, AuthenticateError> {
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

        Ok(GraphSession::new(self.connection, session_id))
    }
}

//
//
//
pub struct GraphSession<T>
where
    T: Transport,
    Bytes: ::fbthrift::Framing<DecBuf = ::fbthrift::FramingDecoded<T>>,
    ::fbthrift::ProtocolEncoded<BinaryProtocol>:
        ::fbthrift::BufMutExt<Final = ::fbthrift::FramingEncodedFinal<T>>,
{
    connection: GraphConnection<T>,
    session_id: i64,
    close_required: bool,
}

impl<T> GraphSession<T>
where
    T: Transport,
    Bytes: ::fbthrift::Framing<DecBuf = ::fbthrift::FramingDecoded<T>>,
    ::fbthrift::ProtocolEncoded<BinaryProtocol>:
        ::fbthrift::BufMutExt<Final = ::fbthrift::FramingEncodedFinal<T>>,
{
    fn new(connection: GraphConnection<T>, session_id: i64) -> Self {
        Self {
            connection,
            session_id,
            close_required: false,
        }
    }

    pub async fn signout(self) -> result::Result<(), SignoutError> {
        self.connection.service.signout(self.session_id).await
    }

    #[allow(clippy::ptr_arg)]
    pub async fn execute(
        &mut self,
        stmt: &Vec<u8>,
    ) -> result::Result<ExecutionResponse, ExecuteError> {
        let res = match self.connection.service.execute(self.session_id, stmt).await {
            Ok(res) => res,
            Err(ExecuteError::ThriftError(err)) => {
                if let Some(io_err) = err.downcast_ref::<io::Error>() {
                    // "ExecuteError Broken pipe (os error 32)"
                    if io_err.kind() == io::ErrorKind::BrokenPipe {
                        self.close_required = true;
                    }
                }

                return Err(ExecuteError::ThriftError(err));
            }
            Err(err) => return Err(err),
        };

        match res.error_code {
            ErrorCode::E_SESSION_INVALID | ErrorCode::E_SESSION_TIMEOUT => {
                self.close_required = true;
            }
            _ => {}
        }

        Ok(res)
    }

    #[allow(clippy::ptr_arg)]
    pub async fn execute_json(
        &mut self,
        stmt: &Vec<u8>,
    ) -> result::Result<Vec<u8>, ExecuteJsonError> {
        let res = match self
            .connection
            .service
            .executeJson(self.session_id, stmt)
            .await
        {
            Ok(res) => res,
            Err(ExecuteJsonError::ThriftError(err)) => {
                if let Some(io_err) = err.downcast_ref::<io::Error>() {
                    // "ExecuteJsonError Broken pipe (os error 32)"
                    if io_err.kind() == io::ErrorKind::BrokenPipe {
                        self.close_required = true;
                    }
                }

                return Err(ExecuteJsonError::ThriftError(err));
            }
            Err(err) => return Err(err),
        };

        Ok(res)
    }

    pub fn is_close_required(&self) -> bool {
        self.close_required
    }
}

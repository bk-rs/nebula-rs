use std::io::{self, Cursor};

use fbthrift::{
    binary_protocol::BinaryProtocolDeserializer, ApplicationException, Deserialize, MessageType,
    ProtocolReader,
};
use fbthrift_transport_response_handler::ResponseHandler;
use nebula_fbthrift_storage::services::storage_service::{ScanEdgeExn, ScanVertexExn};

#[derive(Clone)]
pub struct StorageTransportResponseHandler;

impl ResponseHandler for StorageTransportResponseHandler {
    fn try_make_static_response_bytes(
        &mut self,
        _service_name: &'static str,
        fn_name: &'static str,
        _request_bytes: &[u8],
    ) -> io::Result<Option<Vec<u8>>> {
        match fn_name {
            "StorageService.scanVertex" | "StorageService.scanEdge" => Ok(None),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Unknown method {}", fn_name),
            )),
        }
    }

    fn parse_response_bytes(&mut self, response_bytes: &[u8]) -> io::Result<Option<usize>> {
        let mut des = BinaryProtocolDeserializer::new(Cursor::new(response_bytes));
        let (name, message_type, _) = match des.read_message_begin(|v| v.to_vec()) {
            Ok(v) => v,
            Err(_) => return Ok(None),
        };

        match &name[..] {
            b"scanVertex" | b"scanEdge" => {}
            _ => return Ok(None),
        };

        match message_type {
            MessageType::Reply => {
                match &name[..] {
                    b"scanVertex" => {
                        let _: ScanVertexExn = match Deserialize::read(&mut des) {
                            Ok(v) => v,
                            Err(_) => return Ok(None),
                        };
                    }
                    b"scanEdge" => {
                        let _: ScanEdgeExn = match Deserialize::read(&mut des) {
                            Ok(v) => v,
                            Err(_) => return Ok(None),
                        };
                    }
                    _ => unreachable!(),
                };
            }
            MessageType::Exception => {
                let _: ApplicationException = match Deserialize::read(&mut des) {
                    Ok(v) => v,
                    Err(_) => return Ok(None),
                };
            }
            MessageType::Call | MessageType::Oneway | MessageType::InvalidMessageType => {}
        }

        match des.read_message_end() {
            Ok(v) => v,
            Err(_) => return Ok(None),
        };

        Ok(Some(des.into_inner().position() as usize))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    #[test]
    fn test_try_make_static_response_bytes() -> Result<(), Box<dyn error::Error>> {
        let mut handler = StorageTransportResponseHandler;

        assert_eq!(
            handler.try_make_static_response_bytes(
                "StorageService",
                "StorageService.scanVertex",
                b"FOO"
            )?,
            None
        );
        assert_eq!(
            handler.try_make_static_response_bytes(
                "StorageService",
                "StorageService.scanEdge",
                b"FOO"
            )?,
            None
        );
        match handler.try_make_static_response_bytes("StorageService", "StorageService.foo", b"FOO")
        {
            Ok(_) => assert!(false),
            Err(err) => {
                assert_eq!(err.kind(), io::ErrorKind::Other);

                assert_eq!(err.to_string(), "Unknown method StorageService.foo");
            }
        }

        Ok(())
    }
}

use std::io;

use bytes::Bytes;
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
        request_bytes: &[u8],
    ) -> io::Result<Option<Vec<u8>>> {
        let mut des = BinaryProtocolDeserializer::<Bytes>::new(Bytes::from(request_bytes.to_vec()));
        let (name, _, _) = des
            .read_message_begin(|v| v.to_vec())
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        match &name[..] {
            b"scanVertex" | b"scanEdge" => Ok(None),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Unknown method {:?}", name),
                ))
            }
        }
    }

    fn parse_response_bytes(&mut self, response_bytes: &[u8]) -> io::Result<Option<usize>> {
        let n = response_bytes.len();

        let mut des =
            BinaryProtocolDeserializer::<Bytes>::new(Bytes::from(response_bytes.to_vec()));
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

        Ok(Some(n - des.into_inner().len()))
    }
}

use std::io::{self, Cursor};

use bytes::{Buf, BytesMut};
use fbthrift::{
    binary_protocol::{BinaryProtocolDeserializer, BinaryProtocolSerializer},
    ApplicationException, Deserialize, MessageType, ProtocolReader, ProtocolWriter, Serialize,
};
use fbthrift_transport_response_handler::ResponseHandler;
use nebula_fbthrift_graph_v2::services::graph_service::{
    AuthenticateExn, ExecuteExn, ExecuteJsonExn, SignoutExn,
};

#[derive(Clone)]
pub struct GraphTransportResponseHandler;

impl ResponseHandler for GraphTransportResponseHandler {
    fn try_make_static_response_bytes(
        &mut self,
        _service_name: &'static str,
        _fn_name: &'static str,
        request_bytes: &[u8],
    ) -> io::Result<Option<Vec<u8>>> {
        let mut des = BinaryProtocolDeserializer::new(Cursor::new(request_bytes));
        let (name, message_type, seqid) = des
            .read_message_begin(|v| v.to_vec())
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        match &name[..] {
            b"authenticate" => Ok(None),
            b"signout" => {
                if message_type != MessageType::Call {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Unexpected message type {:?}", message_type),
                    ));
                }

                let buf = BytesMut::with_capacity(1024);
                let mut ser = BinaryProtocolSerializer::<BytesMut>::with_buffer(buf);

                ser.write_message_begin("signout", MessageType::Reply, seqid);
                ser.write_message_end();

                SignoutExn::Success(()).write(&mut ser);

                let res_buf = ser.finish().bytes().to_vec();

                Ok(Some(res_buf))
            }
            b"execute" => Ok(None),
            b"executeJson" => Ok(None),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Unknown method {:?}", name),
            )),
        }
    }

    fn parse_response_bytes(&mut self, response_bytes: &[u8]) -> io::Result<Option<usize>> {
        let n = response_bytes.len();

        let mut des = BinaryProtocolDeserializer::new(Cursor::new(response_bytes));
        let (name, message_type, _) = match des.read_message_begin(|v| v.to_vec()) {
            Ok(v) => v,
            Err(_) => return Ok(None),
        };

        match &name[..] {
            b"authenticate" => {}
            b"signout" => unreachable!(),
            b"execute" => {}
            b"executeJson" => {}
            _ => return Ok(None),
        };

        match message_type {
            MessageType::Reply => {
                match &name[..] {
                    b"authenticate" => {
                        let _: AuthenticateExn = match Deserialize::read(&mut des) {
                            Ok(v) => v,
                            Err(_) => return Ok(None),
                        };
                    }
                    b"execute" => {
                        let _: ExecuteExn = match Deserialize::read(&mut des) {
                            Ok(v) => v,
                            Err(_) => return Ok(None),
                        };
                    }
                    b"executeJson" => {
                        let _: ExecuteJsonExn = match Deserialize::read(&mut des) {
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

        Ok(Some(n - des.into_inner().position() as usize))
    }
}

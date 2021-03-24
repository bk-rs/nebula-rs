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
        fn_name: &'static str,
        request_bytes: &[u8],
    ) -> io::Result<Option<Vec<u8>>> {
        match fn_name {
            "GraphService.authenticate" => Ok(None),
            "GraphService.signout" => {
                let mut des = BinaryProtocolDeserializer::new(Cursor::new(request_bytes));
                let (name, message_type, seqid) = des
                    .read_message_begin(|v| v.to_vec())
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

                if name != b"signout" {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Unexpected name {:?}", name),
                    ));
                }

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
            "GraphService.execute" => Ok(None),
            "GraphService.executeJson" => Ok(None),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Unknown method {}", fn_name),
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    #[test]
    fn test_try_make_static_response_bytes() -> Result<(), Box<dyn error::Error>> {
        let mut handler = GraphTransportResponseHandler;

        assert_eq!(
            handler.try_make_static_response_bytes(
                "GraphService",
                "GraphService.authenticate",
                b"FOO"
            )?,
            None
        );
        assert_eq!(
            handler.try_make_static_response_bytes(
                "GraphService",
                "GraphService.execute",
                b"FOO"
            )?,
            None
        );
        assert_eq!(
            handler.try_make_static_response_bytes(
                "GraphService",
                "GraphService.executeJson",
                b"FOO"
            )?,
            None
        );
        match handler.try_make_static_response_bytes("GraphService", "GraphService.foo", b"FOO") {
            Ok(_) => assert!(false),
            Err(err) => {
                assert_eq!(err.kind(), io::ErrorKind::Other);

                assert_eq!(err.to_string(), "Unknown method GraphService.foo");
            }
        }

        Ok(())
    }

    #[test]
    fn test_try_make_static_response_bytes_with_signout() -> Result<(), Box<dyn error::Error>> {
        let mut handler = GraphTransportResponseHandler;

        //
        // Ref https://github.com/bk-rs/nebula-rs/blob/e500e6f93b0ffcd009038c2a51b41a6aa3488b18/nebula-fbthrift/nebula-fbthrift-graph-v2/src/lib.rs#L1346
        //
        let request = ::fbthrift::serialize!(::fbthrift::BinaryProtocol, |p| {
            ::fbthrift::protocol::write_message(
                p,
                "signout",
                ::fbthrift::MessageType::Call,
                // Note: we send a 0 message sequence ID from clients because
                // this field should not be used by the server (except for some
                // language implementations).
                0,
                |p| {
                    p.write_struct_begin("args");
                    p.write_field_begin("arg_sessionId", ::fbthrift::TType::I64, 1i16);
                    ::fbthrift::Serialize::write(&1, p);
                    p.write_field_end();
                    p.write_field_stop();
                    p.write_struct_end();
                },
            )
        });

        match handler.try_make_static_response_bytes(
            "GraphService",
            "GraphService.signout",
            request.bytes(),
        ) {
            Ok(Some(_)) => {}
            _ => assert!(false),
        }

        Ok(())
    }
}

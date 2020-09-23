use std::io;

use bytes::{Buf, Bytes, BytesMut};
use fbthrift::{
    binary_protocol::{BinaryProtocolDeserializer, BinaryProtocolSerializer},
    ApplicationException, Deserialize, MessageType, ProtocolReader, ProtocolWriter, Serialize,
};
use fbthrift_transport_response_handler::ResponseHandler;
use nebula_fbthrift_graph::services::graph_service::{AuthenticateExn, ExecuteExn, SignoutExn};

#[derive(Clone)]
pub struct GraphTransportResponseHandler;

impl ResponseHandler for GraphTransportResponseHandler {
    fn try_make_response_bytes(
        &self,
        request_bytes: &[u8],
    ) -> io::Result<(Vec<u8>, Option<Vec<u8>>)> {
        let mut des = BinaryProtocolDeserializer::<Bytes>::new(Bytes::from(request_bytes.to_vec()));
        let (name, message_type, seqid) = des
            .read_message_begin(|v| v.to_vec())
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        match message_type {
            MessageType::Call => {}
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Unexpected message type {:?}", message_type),
                ))
            }
        }

        match &name[..] {
            b"authenticate" => Ok((name, None)),
            b"signout" => {
                let buf = BytesMut::with_capacity(1024);
                let mut ser = BinaryProtocolSerializer::<BytesMut>::with_buffer(buf);

                ser.write_message_begin("signout", MessageType::Reply, seqid);
                ser.write_message_end();

                SignoutExn::Success(()).write(&mut ser);

                let res_buf = ser.finish().bytes().to_vec();

                return Ok((name, Some(res_buf)));
            }
            b"execute" => Ok((name, None)),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Unknown method {:?}", name),
                ))
            }
        }
    }

    fn parse_response_bytes(
        &self,
        _name: &[u8],
        response_bytes: &[u8],
    ) -> io::Result<Option<usize>> {
        let n = response_bytes.len();

        let mut des =
            BinaryProtocolDeserializer::<Bytes>::new(Bytes::from(response_bytes.to_vec()));
        let (name, message_type, _) = des
            .read_message_begin(|v| v.to_vec())
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        match &name[..] {
            b"authenticate" => {}
            b"signout" => unreachable!(),
            b"execute" => {}
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Unknown method {:?}", name),
                ))
            }
        };

        match message_type {
            MessageType::Reply => {
                match &name[..] {
                    b"authenticate" => {
                        let _: AuthenticateExn = Deserialize::read(&mut des)
                            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
                    }
                    b"execute" => {
                        let _: ExecuteExn = Deserialize::read(&mut des)
                            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
                    }
                    _ => unreachable!(),
                };
            }
            MessageType::Exception => {
                let _: ApplicationException = Deserialize::read(&mut des)
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
            }
            MessageType::Call | MessageType::Oneway | MessageType::InvalidMessageType => {}
        }

        des.read_message_end()
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        Ok(Some(n - des.into_inner().len()))
    }
}

use std::result;

use bytes::Bytes;
use fbthrift::{BinaryProtocol, Transport};
use nebula_fbthrift_meta::{
    client::{MetaService, MetaServiceImpl},
    errors::meta_service::{GetSpaceError, ListPartsError, ListTagsError},
    types::{
        GetSpaceReq, GetSpaceResp, ListEdgesReq, ListEdgesResp, ListPartsReq, ListPartsResp,
        ListTagsReq, ListTagsResp,
    },
};

//
//
//
struct MetaConnection<T>
where
    T: Transport,
    Bytes: ::fbthrift::Framing<DecBuf = ::fbthrift::FramingDecoded<T>>,
    ::fbthrift::ProtocolEncoded<BinaryProtocol>:
        ::fbthrift::BufMutExt<Final = ::fbthrift::FramingEncodedFinal<T>>,
{
    service: MetaServiceImpl<BinaryProtocol, T>,
}

impl<T> MetaConnection<T>
where
    T: Transport,
    Bytes: ::fbthrift::Framing<DecBuf = ::fbthrift::FramingDecoded<T>>,
    ::fbthrift::ProtocolEncoded<BinaryProtocol>:
        ::fbthrift::BufMutExt<Final = ::fbthrift::FramingEncodedFinal<T>>,
{
    fn new(transport: T) -> Self {
        Self {
            service: MetaServiceImpl::<BinaryProtocol, _>::new(transport),
        }
    }
}

//
//
//
pub struct MetaClient<T>
where
    T: Transport,
    Bytes: ::fbthrift::Framing<DecBuf = ::fbthrift::FramingDecoded<T>>,
    ::fbthrift::ProtocolEncoded<BinaryProtocol>:
        ::fbthrift::BufMutExt<Final = ::fbthrift::FramingEncodedFinal<T>>,
{
    connection: MetaConnection<T>,
}

impl<T> MetaClient<T>
where
    T: Transport,
    Bytes: ::fbthrift::Framing<DecBuf = ::fbthrift::FramingDecoded<T>>,
    ::fbthrift::ProtocolEncoded<BinaryProtocol>:
        ::fbthrift::BufMutExt<Final = ::fbthrift::FramingEncodedFinal<T>>,
{
    pub fn new(transport: T) -> Self {
        Self {
            connection: MetaConnection::new(transport),
        }
    }

    pub async fn get_space(&self, space_name: &str) -> result::Result<GetSpaceResp, GetSpaceError> {
        let req = GetSpaceReq {
            space_name: space_name.to_owned(),
        };
        let res = self.connection.service.getSpace(&req).await?;

        Ok(res)
    }

    // part_id from 1
    pub async fn list_parts(
        &self,
        space_id: i32,
        part_ids: Vec<i32>,
    ) -> result::Result<ListPartsResp, ListPartsError> {
        let req = ListPartsReq { space_id, part_ids };
        let res = self.connection.service.listParts(&req).await?;

        Ok(res)
    }

    pub async fn list_tags(&self, space_id: i32) -> result::Result<ListTagsResp, ListTagsError> {
        let req = ListTagsReq { space_id };
        let res = self.connection.service.listTags(&req).await?;

        Ok(res)
    }

    pub async fn list_edges(&self, space_id: i32) -> result::Result<ListEdgesResp, ListTagsError> {
        let req = ListEdgesReq { space_id };
        let res = self.connection.service.listEdges(&req).await?;

        Ok(res)
    }
}

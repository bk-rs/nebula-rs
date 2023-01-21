use bytes::Bytes;
use fbthrift::{BinaryProtocol, Transport};
use nebula_fbthrift_meta_v1::{
    client::{MetaService, MetaServiceImpl},
    errors::meta_service::{GetSpaceError, ListPartsError, ListSpacesError, ListTagsError},
    types::{
        GetSpaceReq, GetSpaceResp, ListEdgesReq, ListEdgesResp, ListPartsReq, ListPartsResp,
        ListSpacesReq, ListSpacesResp, ListTagsReq, ListTagsResp,
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

    pub async fn list_spaces(&self) -> Result<ListSpacesResp, ListSpacesError> {
        self.connection
            .service
            .listSpaces(&ListSpacesReq {
                ..Default::default()
            })
            .await
    }

    pub async fn get_space(&self, space_name: &str) -> Result<GetSpaceResp, GetSpaceError> {
        self.connection
            .service
            .getSpace(&GetSpaceReq {
                space_name: space_name.to_owned(),
                ..Default::default()
            })
            .await
    }

    // part_id from 1
    pub async fn list_parts(
        &self,
        space_id: i32,
        part_ids: Vec<i32>,
    ) -> Result<ListPartsResp, ListPartsError> {
        self.connection
            .service
            .listParts(&ListPartsReq {
                space_id,
                part_ids,
                ..Default::default()
            })
            .await
    }

    pub async fn list_tags(&self, space_id: i32) -> Result<ListTagsResp, ListTagsError> {
        self.connection
            .service
            .listTags(&ListTagsReq {
                space_id,
                ..Default::default()
            })
            .await
    }

    pub async fn list_edges(&self, space_id: i32) -> Result<ListEdgesResp, ListTagsError> {
        self.connection
            .service
            .listEdges(&ListEdgesReq {
                space_id,
                ..Default::default()
            })
            .await
    }
}

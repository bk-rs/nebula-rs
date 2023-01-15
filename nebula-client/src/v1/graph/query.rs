use core::time::Duration;

use async_trait::async_trait;
use nebula_fbthrift_graph::{
    errors::graph_service::ExecuteError,
    types::{ErrorCode, ExecutionResponse},
};

use serde::de::DeserializeOwned;
use serde_nebula_fbthrift_graph::de::{data::DataDeserializeError, deserialize_execution_response};

#[async_trait]
pub trait GraphQuery {
    async fn query_as<D: DeserializeOwned>(
        &mut self,
        stmt: &str,
    ) -> Result<GraphQueryOutput<D>, GraphQueryError>;

    async fn query(&mut self, stmt: &str) -> Result<GraphQueryOutput<()>, GraphQueryError> {
        self.query_as(stmt).await
    }

    async fn show_hosts(&mut self) -> Result<GraphQueryOutput<Host>, GraphQueryError> {
        self.query_as(STMT_SHOW_HOSTS).await
    }
    async fn show_spaces(&mut self) -> Result<GraphQueryOutput<Space>, GraphQueryError> {
        self.query_as(STMT_SHOW_SPACES).await
    }
}

#[derive(Debug)]
pub struct GraphQueryOutput<D>
where
    D: DeserializeOwned,
{
    pub latency: Duration,
    pub space_name: Option<String>,
    pub data_set: Vec<D>,
}

impl<D> GraphQueryOutput<D>
where
    D: DeserializeOwned,
{
    pub fn new(res: ExecutionResponse) -> Result<Self, GraphQueryError> {
        let latency = Duration::from_micros(res.latency_in_us as u64);
        let space_name = res.space_name.clone();
        let data_set = deserialize_execution_response::<D>(&res)
            .map_err(GraphQueryError::DataDeserializeError)?;

        Ok(Self {
            latency,
            space_name,
            data_set,
        })
    }
}

//
//
//
#[derive(Debug)]
pub enum GraphQueryError {
    ExecuteError(ExecuteError),
    ResponseError(ErrorCode, Option<String>),
    DataDeserializeError(DataDeserializeError),
}

impl core::fmt::Display for GraphQueryError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ExecuteError(err) => write!(f, "ExecuteError {err}"),
            Self::ResponseError(err_code, err_msg) => {
                write!(f, "ResponseError err_code:{err_code} err_msg:{err_msg:?}",)
            }
            Self::DataDeserializeError(err) => write!(f, "DataDeserializeError {err}"),
        }
    }
}

impl std::error::Error for GraphQueryError {
    fn description(&self) -> &str {
        match self {
            Self::ExecuteError(_) => "ExecuteError",
            Self::ResponseError(_, _) => "ResponseError",
            Self::DataDeserializeError(_) => "DataDeserializeError",
        }
    }
}

//
//
//
use serde::Deserialize;

const STMT_SHOW_HOSTS: &str = "SHOW HOSTS;";
#[derive(Deserialize, Debug)]
pub struct Host {
    #[serde(rename(deserialize = "Ip"))]
    pub ip: String,
    #[serde(rename(deserialize = "Port"))]
    pub port: String,
    #[serde(rename(deserialize = "Status"))]
    pub status: String,
    #[serde(rename(deserialize = "Leader count"))]
    pub leader_count: u64,
    #[serde(rename(deserialize = "Leader distribution"))]
    pub leader_distribution: String,
    #[serde(rename(deserialize = "Partition distribution"))]
    pub partition_distribution: String,
}

const STMT_SHOW_SPACES: &str = "SHOW SPACES;";
#[derive(Deserialize, Debug)]
pub struct Space {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::{Error as IoError, ErrorKind as IoErrorKind};

    #[test]
    fn impl_std_fmt_display() {
        let err = GraphQueryError::ResponseError(ErrorCode::E_DISCONNECTED, None);
        println!("{err}");
    }

    #[test]
    fn impl_std_error_error() {
        let err = IoError::new(
            IoErrorKind::Other,
            GraphQueryError::ResponseError(ErrorCode::E_DISCONNECTED, None),
        );
        println!("{err}");
    }
}

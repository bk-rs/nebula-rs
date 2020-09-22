use std::error;
use std::fmt;
use std::result;
use std::time::Duration;

use async_trait::async_trait;
use nebula_fbthrift_graph::{
    errors::graph_service::ExecuteError,
    types::{ErrorCode, ExecutionResponse},
};

cfg_if::cfg_if! {
    if #[cfg(feature = "serde_feature")] {
        use serde::de::DeserializeOwned;
        use serde_nebula_fbthrift_graph::de::{deserialize_execution_response, data::DataDeserializeError};

        #[async_trait]
        pub trait Query
        {
            async fn query_as<D: DeserializeOwned>(&mut self, stmt: &str) -> result::Result<QueryOutput<D>, QueryError>;

            async fn query(&mut self, stmt: &str) -> result::Result<QueryOutput<()>, QueryError> {
                self.query_as(stmt).await
            }

            async fn show_hosts(&mut self) -> result::Result<QueryOutput<Host>, QueryError> {
                self.query_as(STMT_SHOW_HOSTS).await
            }
            async fn show_spaces(&mut self) -> result::Result<QueryOutput<Space>, QueryError> {
                self.query_as(STMT_SHOW_SPACES).await
            }
        }

        #[derive(Debug)]
        pub struct QueryOutput<D> where D: DeserializeOwned {
            pub latency: Duration,
            pub space_name: Option<String>,
            pub data_set: Vec<D>,
        }

        impl<D> QueryOutput<D> where D: DeserializeOwned {
            pub fn new(res: ExecutionResponse) -> result::Result<Self, QueryError> {
                let latency = Duration::from_micros(res.latency_in_us as u64);
                let space_name = res.space_name.clone();
                let data_set = deserialize_execution_response::<D>(&res).map_err(|err| QueryError::DataDeserializeError(err))?;

                Ok(Self {
                    latency,
                    space_name,
                    data_set,
                })
            }
        }
    } else {
        #[async_trait]
        pub trait Query {
            async fn query(&mut self, stmt: &str) -> result::Result<QueryOutput, QueryError>;
        }

        #[derive(Debug)]
        pub struct QueryOutput {
            pub latency: Duration,
            pub space_name: Option<String>,
            pub data_set: (),
        }

        impl QueryOutput {
            pub fn new(res: ExecutionResponse) -> Self {
                let latency = Duration::from_micros(res.latency_in_us as u64);
                let space_name = res.space_name;
                let data_set = ();

                Self {
                    latency,
                    space_name,
                    data_set,
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum QueryError {
    ExecuteError(ExecuteError),
    ResponseError(ErrorCode, Option<String>),
    #[cfg(feature = "serde_feature")]
    DataDeserializeError(DataDeserializeError),
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ExecuteError(err) => write!(f, "ExecuteError {}", err),
            Self::ResponseError(err_code, err_msg) => write!(
                f,
                "ResponseError err_code:{} err_msg:{:?}",
                err_code, err_msg
            ),
            #[cfg(feature = "serde_feature")]
            Self::DataDeserializeError(err) => write!(f, "DataDeserializeError {}", err),
        }
    }
}

impl error::Error for QueryError {
    fn description(&self) -> &str {
        match self {
            Self::ExecuteError(_) => "ExecuteError",
            Self::ResponseError(_, _) => "ResponseError",
            #[cfg(feature = "serde_feature")]
            Self::DataDeserializeError(_) => "DataDeserializeError",
        }
    }
}

//
//
//
cfg_if::cfg_if! {
    if #[cfg(feature = "serde_feature")] {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io;

    #[test]
    fn impl_std_fmt_display() -> io::Result<()> {
        let err = QueryError::ResponseError(ErrorCode::E_DISCONNECTED, None);
        println!("{}", err.to_string());

        Ok(())
    }

    #[test]
    fn impl_std_error_error() -> io::Result<()> {
        let err = io::Error::new(
            io::ErrorKind::Other,
            QueryError::ResponseError(ErrorCode::E_DISCONNECTED, None),
        );
        println!("{}", err.to_string());

        Ok(())
    }
}

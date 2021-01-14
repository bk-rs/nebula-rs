#[cfg(feature = "graph")]
use bb805 as bb8;
#[cfg(feature = "graph")]
use fbthrift_transport::tokio02_io::transport::AsyncTransport;
#[cfg(feature = "graph")]
use tokio02_::net::TcpStream;

#[cfg(feature = "graph")]
#[path = "graph.rs"]
pub mod graph;

#[cfg(feature = "graph")]
pub use graph::{GraphClientConfiguration, GraphConnectionManager};

pub mod v2 {
    #[cfg(feature = "graph")]
    use super::{bb8, AsyncTransport, TcpStream};

    #[cfg(feature = "graph")]
    #[path = "graph.rs"]
    pub mod graph;

    #[cfg(feature = "graph")]
    pub use graph::{GraphClientConfiguration, GraphConnectionManager};
}

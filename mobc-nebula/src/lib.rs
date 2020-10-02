cfg_if::cfg_if! {
    if #[cfg(all(feature = "async_std", not(feature = "tokio")))] {
        #[cfg(feature = "graph")]
        pub mod graph;
        #[cfg(feature = "graph")]
        pub use graph::{GraphClientConfiguration, GraphConnectionManager};

        pub mod v2;
    } else if #[cfg(all(not(feature = "async_std"), feature = "tokio"))] {
        #[cfg(feature = "graph")]
        pub mod graph;
        #[cfg(feature = "graph")]
        pub use graph::{GraphClientConfiguration, GraphConnectionManager};

        pub mod v2;
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(feature = "async_std", not(feature = "tokio")))] {
        pub mod graph;
        pub use graph::{NebulaGraphClientConfiguration, NebulaGraphConnectionManager};

        pub mod v2;
    } else if #[cfg(all(not(feature = "async_std"), feature = "tokio"))] {
        pub mod graph;
        pub use graph::{NebulaGraphClientConfiguration, NebulaGraphConnectionManager};

        pub mod v2;
    }
}

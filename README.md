## Examples

### tokio

* [v2 bb8 graph pool](demos/tokio/src/v2_bb8_graph_pool.rs)
* [v2 graph client](demos/tokio/src/v2_graph_client.rs)
* [v1 bb8 graph pool](demos/tokio/src/v1_bb8_graph_pool.rs)
* [v1 graph client](demos/tokio/src/v1_graph_client.rs)

## Dev

```
cargo clippy --all-features --tests -- -D clippy::all
cargo +nightly clippy --all-features --tests -- -D clippy::all

cargo fmt -- --check

cargo build-all-features
cargo test-all-features -- --nocapture
```

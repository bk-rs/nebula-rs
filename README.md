## Examples

### async-std

* [count edge](demos/async-std/src/count_edge.rs)
* [count vertex](demos/async-std/src/count_vertex.rs)
* [graph client](demos/async-std/src/graph_client.rs)
* [graph CRUD](demos/async-std/src/graph_crud.rs)
* [mobc graph pool](demos/async-std/src/mobc_graph_pool.rs)
* [v2 graph client](demos/async-std/src/v2_graph_client.rs)
* [v2 mobc graph pool](demos/async-std/src/v2_mobc_graph_pool.rs)

### tokio

* [bb8 graph pool](demos/tokio/src/bb8_graph_pool.rs)
* [graph client](demos/tokio/src/graph_client.rs)
* [v2 bb8 graph pool](demos/tokio/src/v2_bb8_graph_pool.rs)
* [v2 graph client](demos/tokio/src/v2_graph_client.rs)

## Dev

```
cargo +nightly clippy -p nebula-client --all-features -- -D clippy::all
cargo fmt -p nebula-client -- --check

cargo +nightly clippy -p bb8-nebula --all-features -- -D clippy::all
cargo fmt -p bb8-nebula -- --check

cargo +nightly clippy -p mobc-nebula --all-targets --all-features -- -D clippy::all
cargo fmt -p mobc-nebula -- --check

cargo +nightly clippy -p serde-nebula-fbthrift-graph --all-targets --all-features -- -D clippy::all
cargo fmt -p serde-nebula-fbthrift-graph -- --check
```

```
cargo build-all-features
```

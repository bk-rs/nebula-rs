## Examples

[Refer to](https://github.com/bk-rs/nebula-rs/wiki/Test-Nebula-Graph)

### tokio

* [v3 bb8 graph pool](demos/tokio/src/v3_bb8_graph_pool.rs)
* [v3 graph client](demos/tokio/src/v3_graph_client.rs)

## Dev

```
cargo clippy --all-features --tests -- -D clippy::all
cargo +nightly clippy --all-features --tests -- -D clippy::all
cargo fmt -p bb8-nebula -p nebula-demo-tokio -p mobc-nebula -p nebula-client -p serde-nebula-fbthrift-graph --check --

cargo build-all-features
cargo test-all-features -- --nocapture
```

## Publish order

nebula-fbthrift/nebula-fbthrift-double

nebula-fbthrift/*

serde-nebula-fbthrift-graph

nebula-client

bb8-nebula mobc-nebula

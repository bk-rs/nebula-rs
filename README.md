## Examples

### async-std

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
cargo build-all-features
```

## Build fbthrift libs

[build fbthrift](https://github.com/bk-rs/fbthrift-transport/blob/master/README_fbthrift.md)

```
cd
git clone https://github.com/vesoft-inc/nebula.git && cd nebula
git checkout v1.0.1

thrift1 --out /tmp --gen mstch_rust src/interface/common.thrift
mv /tmp/lib.rs nebula-fbthrift-common/src/lib.rs
sed -i 's/pub value_type: ::std::option::Option<crate::types::ValueType>,$/pub value_type: ::std::option::Option<Box<crate::types::ValueType>>,/' nebula-fbthrift-common/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/interface/graph.thrift
mv /tmp/lib.rs nebula-fbthrift-graph/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/interface/meta.thrift
mv /tmp/lib.rs nebula-fbthrift-meta/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/interface/raftex.thrift
mv /tmp/lib.rs nebula-fbthrift-raftex/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/interface/storage.thrift
mv /tmp/lib.rs nebula-fbthrift-storage/src/lib.rs
```

## Build fbthrift libs v2

```
cd
git clone https://github.com/vesoft-inc/nebula-common.git && cd nebula-common
sed -i 's/^} (cpp.enum_strict cpp.type = "nebula::NullType")$/} (cpp.type = "nebula::NullType")/' src/common/interface/common.thrift

thrift1 --out /tmp --gen mstch_rust src/common/interface/common.thrift
mv /tmp/lib.rs nebula-fbthrift-common-v2/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/common/interface/graph.thrift
mv /tmp/lib.rs nebula-fbthrift-graph-v2/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/common/interface/meta.thrift
mv /tmp/lib.rs nebula-fbthrift-meta-v2/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/common/interface/raftex.thrift
mv /tmp/lib.rs nebula-fbthrift-raftex-v2/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/common/interface/storage.thrift
mv /tmp/lib.rs nebula-fbthrift-storage-v2/src/lib.rs
```

### Update nebula-fbthrift-common-v2/src/lib.rs

```
sed -i 's/^    #\[derive(Clone, Debug, PartialEq)\]$/    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]/' nebula-fbthrift-common-v2/src/lib.rs

echo 'pub mod double;' >> nebula-fbthrift-common-v2/src/lib.rs

sed -i 's/^        fVal(::std::primitive::f64),$/        fVal(crate::double::Double),/' nebula-fbthrift-common-v2/src/lib.rs
```

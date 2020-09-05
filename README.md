## Examples

### async-std

* [graph client](demos/async-std/src/graph_client.rs)
* [v2 graph client](demos/async-std/src/v2_graph_client.rs)

### tokio

* [graph client](demos/tokio/src/graph_client.rs)
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
mv /tmp/lib.rs nebula-graph-fbthrift-common/src/lib.rs
sed -i 's/pub value_type: ::std::option::Option<crate::types::ValueType>,$/pub value_type: ::std::option::Option<Box<crate::types::ValueType>>,/' nebula-graph-fbthrift-common/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/interface/graph.thrift
mv /tmp/lib.rs nebula-graph-fbthrift-graph/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/interface/meta.thrift
mv /tmp/lib.rs nebula-graph-fbthrift-meta/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/interface/raftex.thrift
mv /tmp/lib.rs nebula-graph-fbthrift-raftex/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/interface/storage.thrift
mv /tmp/lib.rs nebula-graph-fbthrift-storage/src/lib.rs
```

## Build fbthrift libs v2

```
cd
git clone https://github.com/vesoft-inc/nebula-common.git && cd nebula-common
sed -i 's/^} (cpp.enum_strict cpp.type = "nebula::NullType")$/} (cpp.type = "nebula::NullType")/' src/common/interface/common.thrift

thrift1 --out /tmp --gen mstch_rust src/common/interface/common.thrift
mv /tmp/lib.rs nebula-graph-fbthrift-common-v2/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/common/interface/graph.thrift
mv /tmp/lib.rs nebula-graph-fbthrift-graph-v2/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/common/interface/meta.thrift
mv /tmp/lib.rs nebula-graph-fbthrift-meta-v2/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/common/interface/raftex.thrift
mv /tmp/lib.rs nebula-graph-fbthrift-raftex-v2/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/common/interface/storage.thrift
mv /tmp/lib.rs nebula-graph-fbthrift-storage-v2/src/lib.rs
```

### Update nebula-graph-fbthrift-common-v2/src/lib.rs

```
sed -i 's/^    #\[derive(Clone, Debug, PartialEq)\]$/    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]/' nebula-graph-fbthrift-common-v2/src/lib.rs

echo 'pub mod double;' >> nebula-graph-fbthrift-common-v2/src/lib.rs

sed -i 's/^        fVal(::std::primitive::f64),$/        fVal(crate::double::Double),/' nebula-graph-fbthrift-common-v2/src/lib.rs
```

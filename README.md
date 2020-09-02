## Examples

### async-std

* [graph client](demos/async-std/src/graph_client.rs)

### tokio

* [graph client](demos/tokio/src/graph_client.rs)

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

## Build fbthrift libs

First of all, [build fbthrift](https://github.com/bk-rs/fbthrift-git-rs/wiki/Build-fbthrift-on-Ubuntu-20.04)

```
mkdir -p /tmp/nebula-fbthrift-{common,graph,meta,raftex,storage}/src
mkdir -p /tmp/nebula-fbthrift-{common,graph,meta,raftex,storage}-v2/src
```

### Build fbthrift libs v1

```
cd ~
git clone https://github.com/vesoft-inc/nebula.git && cd nebula
git checkout v1.2.0

thrift1 --out /tmp --gen mstch_rust src/interface/common.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-common/src/lib.rs

sed -i 's/pub value_type: ::std::option::Option<crate::types::ValueType>,$/pub value_type: ::std::option::Option<Box<crate::types::ValueType>>,/' /tmp/nebula-fbthrift-common/src/lib.rs


thrift1 --out /tmp --gen mstch_rust src/interface/graph.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-graph/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/interface/meta.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-meta/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/interface/raftex.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-raftex/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/interface/storage.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-storage/src/lib.rs
```

### Build fbthrift libs v2

```
cd ~
git clone https://github.com/vesoft-inc/nebula-common.git && cd nebula-common
git checkout v2.0.0

sed -i 's/^} (cpp.enum_strict cpp.type = "nebula::NullType")$/} (cpp.type = "nebula::NullType")/' src/common/interface/common.thrift

thrift1 --out /tmp --gen mstch_rust src/common/interface/common.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-common-v2/src/lib.rs

sed -i 's/^    #\[derive(Clone, Debug, PartialEq)\]$/    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]/' /tmp/nebula-fbthrift-common-v2/src/lib.rs
echo 'pub mod double;' >> /tmp/nebula-fbthrift-common-v2/src/lib.rs
sed -i 's/^        fVal(::std::primitive::f64),$/        fVal(crate::double::Double),/' /tmp/nebula-fbthrift-common-v2/src/lib.rs

sed -i 's/: crate::types::Value,$/: Box<crate::types::Value>,/' /tmp/nebula-fbthrift-common-v2/src/lib.rs


thrift1 --out /tmp --gen mstch_rust src/common/interface/graph.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-graph-v2/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/common/interface/meta.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-meta-v2/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/common/interface/raftex.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-raftex-v2/src/lib.rs

thrift1 --out /tmp --gen mstch_rust src/common/interface/storage.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-storage-v2/src/lib.rs
```

### Clippy

```
mv /tmp/nebula-fbthrift-common/src/lib.rs nebula-fbthrift-common/src/lib.rs
mv /tmp/nebula-fbthrift-graph/src/lib.rs nebula-fbthrift-graph/src/lib.rs
mv /tmp/nebula-fbthrift-meta/src/lib.rs nebula-fbthrift-meta/src/lib.rs
mv /tmp/nebula-fbthrift-raftex/src/lib.rs nebula-fbthrift-raftex/src/lib.rs
mv /tmp/nebula-fbthrift-storage/src/lib.rs nebula-fbthrift-storage/src/lib.rs

mv /tmp/nebula-fbthrift-common-v2/src/lib.rs nebula-fbthrift-common-v2/src/lib.rs
mv /tmp/nebula-fbthrift-graph-v2/src/lib.rs nebula-fbthrift-graph-v2/src/lib.rs
mv /tmp/nebula-fbthrift-meta-v2/src/lib.rs nebula-fbthrift-meta-v2/src/lib.rs
mv /tmp/nebula-fbthrift-raftex-v2/src/lib.rs nebula-fbthrift-raftex-v2/src/lib.rs
mv /tmp/nebula-fbthrift-storage-v2/src/lib.rs nebula-fbthrift-storage-v2/src/lib.rs
```

```
sed -i '5i\#![allow(clippy::all)]' nebula-fbthrift-common/src/lib.rs
sed -i '5i\#![allow(clippy::all)]' nebula-fbthrift-graph/src/lib.rs
sed -i '5i\#![allow(clippy::all)]' nebula-fbthrift-meta/src/lib.rs
sed -i '5i\#![allow(clippy::all)]' nebula-fbthrift-raftex/src/lib.rs
sed -i '5i\#![allow(clippy::all)]' nebula-fbthrift-storage/src/lib.rs 

sed -i '5i\#![allow(clippy::all)]' nebula-fbthrift-common-v2/src/lib.rs
sed -i '5i\#![allow(clippy::all)]' nebula-fbthrift-graph-v2/src/lib.rs 
sed -i '5i\#![allow(clippy::all)]' nebula-fbthrift-meta-v2/src/lib.rs
sed -i '5i\#![allow(clippy::all)]' nebula-fbthrift-raftex-v2/src/lib.rs
sed -i '5i\#![allow(clippy::all)]' nebula-fbthrift-storage-v2/src/lib.rs
```

```
sed -i '5i\#![allow(bare_trait_objects)]' nebula-fbthrift-common/src/lib.rs
sed -i '5i\#![allow(bare_trait_objects)]' nebula-fbthrift-graph/src/lib.rs
sed -i '5i\#![allow(bare_trait_objects)]' nebula-fbthrift-meta/src/lib.rs
sed -i '5i\#![allow(bare_trait_objects)]' nebula-fbthrift-raftex/src/lib.rs
sed -i '5i\#![allow(bare_trait_objects)]' nebula-fbthrift-storage/src/lib.rs 

sed -i '5i\#![allow(bare_trait_objects)]' nebula-fbthrift-common-v2/src/lib.rs
sed -i '5i\#![allow(bare_trait_objects)]' nebula-fbthrift-graph-v2/src/lib.rs 
sed -i '5i\#![allow(bare_trait_objects)]' nebula-fbthrift-meta-v2/src/lib.rs
sed -i '5i\#![allow(bare_trait_objects)]' nebula-fbthrift-raftex-v2/src/lib.rs
sed -i '5i\#![allow(bare_trait_objects)]' nebula-fbthrift-storage-v2/src/lib.rs
```

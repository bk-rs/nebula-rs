#!/usr/bin/env bash

set -ex

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

# 
# First of all, [build fbthrift](https://github.com/bk-rs/fbthrift-git-rs/wiki/Build-fbthrift-on-Ubuntu)
# 

<<'PREPARE'
cd ~
git clone https://github.com/vesoft-inc/nebula.git nebula_v3 && cd nebula_v3
git checkout v3.3.0
cd


cd ~
git clone https://github.com/vesoft-inc/nebula.git nebula_v2 && cd nebula_v2
git checkout v2.6.2
cd


cd ~
git clone https://github.com/vesoft-inc/nebula.git nebula_v1 && cd nebula_v1
git checkout v1.2.1
cd
PREPARE

# 
rm -rf /tmp/nebula-fbthrift-{common,graph,meta,raftex,storage}-v3
rm -rf /tmp/nebula-fbthrift-{common,graph,meta,raftex,storage}-v2
rm -rf /tmp/nebula-fbthrift-{common,graph,meta,raftex,storage}-v1

mkdir -p /tmp/nebula-fbthrift-{common,graph,meta,raftex,storage}-v3/src
mkdir -p /tmp/nebula-fbthrift-{common,graph,meta,raftex,storage}-v2/src
mkdir -p /tmp/nebula-fbthrift-{common,graph,meta,raftex,storage}-v1/src

# 
# v2
# 
cd ~/nebula_v2

sed -i 's/^} (cpp.enum_strict cpp.type = "nebula::NullType")$/} (cpp.type = "nebula::NullType")/' src/interface/common.thrift


rm -rf /tmp/lib.rs
thrift1 --out /tmp --gen mstch_rust src/interface/common.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-common-v2/src/lib.rs

sed -i 's/^    #\[derive(Clone, Debug, PartialEq)\]$/    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]/' /tmp/nebula-fbthrift-common-v2/src/lib.rs
echo 'pub mod double;' >> /tmp/nebula-fbthrift-common-v2/src/lib.rs
sed -i 's/^        fVal(::std::primitive::f64),$/        fVal(crate::double::Double),/' /tmp/nebula-fbthrift-common-v2/src/lib.rs
sed -i 's/: crate::types::Value,$/: Box<crate::types::Value>,/' /tmp/nebula-fbthrift-common-v2/src/lib.rs


rm -rf /tmp/lib.rs
thrift1 --out /tmp --gen mstch_rust src/interface/graph.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-graph-v2/src/lib.rs


rm -rf /tmp/lib.rs
thrift1 --out /tmp --gen mstch_rust src/interface/meta.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-meta-v2/src/lib.rs


rm -rf /tmp/lib.rs
thrift1 --out /tmp --gen mstch_rust src/interface/raftex.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-raftex-v2/src/lib.rs


rm -rf /tmp/lib.rs
thrift1 --out /tmp --gen mstch_rust src/interface/storage.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-storage-v2/src/lib.rs


sed -i '5i\#![allow(bare_trait_objects)]' /tmp/nebula-fbthrift-common-v2/src/lib.rs
sed -i '5i\#![allow(bare_trait_objects)]' /tmp/nebula-fbthrift-graph-v2/src/lib.rs 
sed -i '5i\#![allow(bare_trait_objects)]' /tmp/nebula-fbthrift-meta-v2/src/lib.rs
sed -i '5i\#![allow(bare_trait_objects)]' /tmp/nebula-fbthrift-raftex-v2/src/lib.rs
sed -i '5i\#![allow(bare_trait_objects)]' /tmp/nebula-fbthrift-storage-v2/src/lib.rs

cd

# 
# v1
# 
cd ~/nebula_v1


rm -rf /tmp/lib.rs
thrift1 --out /tmp --gen mstch_rust src/interface/common.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-common-v1/src/lib.rs

sed -i 's/pub value_type: ::std::option::Option<crate::types::ValueType>,$/pub value_type: ::std::option::Option<Box<crate::types::ValueType>>,/' /tmp/nebula-fbthrift-common-v1/src/lib.rs


rm -rf /tmp/lib.rs
thrift1 --out /tmp --gen mstch_rust src/interface/graph.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-graph-v1/src/lib.rs


rm -rf /tmp/lib.rs
thrift1 --out /tmp --gen mstch_rust src/interface/meta.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-meta-v1/src/lib.rs


rm -rf /tmp/lib.rs
thrift1 --out /tmp --gen mstch_rust src/interface/raftex.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-raftex-v1/src/lib.rs


rm -rf /tmp/lib.rs
thrift1 --out /tmp --gen mstch_rust src/interface/storage.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-storage-v1/src/lib.rs


sed -i '5i\#![allow(bare_trait_objects)]' /tmp/nebula-fbthrift-common-v1/src/lib.rs
sed -i '5i\#![allow(bare_trait_objects)]' /tmp/nebula-fbthrift-graph-v1/src/lib.rs
sed -i '5i\#![allow(bare_trait_objects)]' /tmp/nebula-fbthrift-meta-v1/src/lib.rs
sed -i '5i\#![allow(bare_trait_objects)]' /tmp/nebula-fbthrift-raftex-v1/src/lib.rs
sed -i '5i\#![allow(bare_trait_objects)]' /tmp/nebula-fbthrift-storage-v1/src/lib.rs 

cd

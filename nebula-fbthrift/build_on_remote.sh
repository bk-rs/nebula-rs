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
git checkout v3.6.0
cd

PREPARE

# 
rm -rf /tmp/nebula-fbthrift-{common,graph,meta,raftex,storage}-v3

mkdir -p /tmp/nebula-fbthrift-{common,graph,meta,raftex,storage}-v3/src

# 
# v3
# 
cd ~/nebula_v3



rm -rf /tmp/{lib, types}.rs
thrift1 -I ~/fbthrift --out /tmp --gen mstch_rust src/interface/common.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-common-v3/src/lib.rs
mv /tmp/types.rs /tmp/nebula-fbthrift-common-v3/src/types.rs

sed -i 's/^    pub const version/    \/\/ pub const version/' /tmp/nebula-fbthrift-common-v3/src/lib.rs
echo 'pub mod double;' >> /tmp/nebula-fbthrift-common-v3/src/lib.rs

sed -i 's/^#\[derive(Clone, PartialEq, Debug)\]$/#[derive(Clone, PartialEq, Debug, Eq, PartialOrd, Ord)]/' /tmp/nebula-fbthrift-common-v3/src/types.rs
sed -i 's/^#\[derive(Clone, PartialEq)\]$/#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]/' /tmp/nebula-fbthrift-common-v3/src/types.rs
sed -i 's/^    fVal(::std::primitive::f64),$/    fVal(crate::double::Double),/' /tmp/nebula-fbthrift-common-v3/src/types.rs
sed -i 's/^    vVal(crate::types::Vertex),$/    vVal(Box<crate::types::Vertex>),/' /tmp/nebula-fbthrift-common-v3/src/types.rs
sed -i 's/: crate::types::Value,$/: Box<crate::types::Value>,/' /tmp/nebula-fbthrift-common-v3/src/types.rs
sed -i 's/: ::std::primitive::f64,$/: crate::double::Double,/' /tmp/nebula-fbthrift-common-v3/src/types.rs


rm -rf /tmp/{lib, types}.rs
thrift1 -I ~/fbthrift --out /tmp --gen mstch_rust src/interface/graph.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-graph-v3/src/lib.rs
mv /tmp/types.rs /tmp/nebula-fbthrift-graph-v3/src/types.rs


rm -rf /tmp/{lib, types}.rs
thrift1 -I ~/fbthrift --out /tmp --gen mstch_rust src/interface/meta.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-meta-v3/src/lib.rs
mv /tmp/types.rs /tmp/nebula-fbthrift-meta-v3/src/types.rs


rm -rf /tmp/{lib, types}.rs
thrift1 -I ~/fbthrift --out /tmp --gen mstch_rust src/interface/raftex.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-raftex-v3/src/lib.rs
mv /tmp/types.rs /tmp/nebula-fbthrift-raftex-v3/src/types.rs


rm -rf /tmp/{lib, types}.rs
thrift1 -I ~/fbthrift --out /tmp --gen mstch_rust src/interface/storage.thrift
mv /tmp/lib.rs /tmp/nebula-fbthrift-storage-v3/src/lib.rs
mv /tmp/types.rs /tmp/nebula-fbthrift-storage-v3/src/types.rs


sed -i '5i\#![allow(bare_trait_objects)]' /tmp/nebula-fbthrift-common-v3/src/lib.rs
sed -i '5i\#![allow(bare_trait_objects)]' /tmp/nebula-fbthrift-graph-v3/src/lib.rs 
sed -i '5i\#![allow(bare_trait_objects)]' /tmp/nebula-fbthrift-meta-v3/src/lib.rs
sed -i '5i\#![allow(bare_trait_objects)]' /tmp/nebula-fbthrift-raftex-v3/src/lib.rs
sed -i '5i\#![allow(bare_trait_objects)]' /tmp/nebula-fbthrift-storage-v3/src/lib.rs

cd

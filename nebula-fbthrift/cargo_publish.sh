#!/usr/bin/env bash

<<'EG'
./cargo_publish.sh v3
EG

set -ex

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

# 
type="$1"

# 
case $type in
  "v3")
    cd ${script_path_root}nebula-fbthrift-common-v3
    cargo publish -v
    sleep 2

    cd ${script_path_root}nebula-fbthrift-graph-v3
    cargo publish -v
    sleep 2

    cd ${script_path_root}nebula-fbthrift-meta-v3
    cargo publish -v
    sleep 2

    cd ${script_path_root}nebula-fbthrift-raftex-v3
    cargo publish -v
    sleep 2

    cd ${script_path_root}nebula-fbthrift-storage-v3
    cargo publish -v
    sleep 2

    ;;
esac

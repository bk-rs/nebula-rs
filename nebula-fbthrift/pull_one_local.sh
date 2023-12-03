#!/usr/bin/env bash

<<'STEPS'
# on local
fbthrift_ssh_host="1.1.1.1"
fbthrift_ssh_port=22
fbthrift_ssh_user="root"
scp -P $fbthrift_ssh_port build_on_remote.sh $fbthrift_ssh_user@$fbthrift_ssh_host:~/build_on_remote.sh

# on remote
cd ~
./build_on_remote.sh

# on local
./pull_one_local.sh $fbthrift_ssh_user $fbthrift_ssh_host $fbthrift_ssh_port
STEPS

set -ex

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

# 
ssh_username="$1"
ssh_host="$2"
ssh_port="$3"

# 
# v3
# 
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-common-v3/src/lib.rs \
                                ${script_path_root}nebula-fbthrift-common-v3/src/lib.rs
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-common-v3/src/types.rs \
                                ${script_path_root}nebula-fbthrift-common-v3/src/types.rs

scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-graph-v3/src/lib.rs \
                                ${script_path_root}nebula-fbthrift-graph-v3/src/lib.rs
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-graph-v3/src/types.rs \
                                ${script_path_root}nebula-fbthrift-graph-v3/src/types.rs

scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-meta-v3/src/lib.rs \
                                ${script_path_root}nebula-fbthrift-meta-v3/src/lib.rs
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-meta-v3/src/types.rs \
                                ${script_path_root}nebula-fbthrift-meta-v3/src/types.rs

scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-raftex-v3/src/lib.rs \
                                ${script_path_root}nebula-fbthrift-raftex-v3/src/lib.rs
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-raftex-v3/src/types.rs \
                                ${script_path_root}nebula-fbthrift-raftex-v3/src/types.rs

scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-storage-v3/src/lib.rs \
                                ${script_path_root}nebula-fbthrift-storage-v3/src/lib.rs
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-storage-v3/src/types.rs \
                                ${script_path_root}nebula-fbthrift-storage-v3/src/types.rs


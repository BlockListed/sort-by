#!/bin/bash

cat <<EOF | docker run -i --rm -v "$(realpath ..):/app/sort-by" ubuntu:22.04
apt update && apt install curl gcc -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
source /root/.cargo/env
cd /app/sort-by
cargo build --release
EOF
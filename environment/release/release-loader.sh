#!/bin/bash
SCRIPT_PATH=$(dirname $(realpath -s $0))
echo "Starting release of loader"
cd $SCRIPT_PATH && cd ../../loader
cargo clean
cargo build --target aarch64-unknown-linux-gnu --release

rsync -e "ssh -i ~/.ssh/id_rsa_backend" -avO --delete --no-perms target/aarch64-unknown-linux-gnu/release/illuvi-analytics-loader ru@backend:/opt/loader

#!/bin/bash
SCRIPT_PATH=$(dirname $(realpath -s $0))

echo "Starting release of backend"
cd $SCRIPT_PATH && cd ../../site/backend
cargo clean
cargo build --target aarch64-unknown-linux-gnu --release

rsync -e "ssh -i ~/.ssh/id_rsa_backend" -avO --whole-file --no-perms target/aarch64-unknown-linux-gnu/release/illuvi-analytics-backend ru@backend:/tmp
# rsync causes the watcher to reload the service multiple times, so have to copy move it like this
ssh -i ~/.ssh/id_rsa_backend ru@backend "mv /tmp/illuvi-analytics-backend /opt/backend"

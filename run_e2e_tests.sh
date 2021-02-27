#!/bin/bash

set -e pipefail

# start server in background process
cargo run . > /dev/null &

cd ./e2e-tests && cargo test
#!/usr/bin/env bash

set -e

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

cd $SCRIPT_DIR/..

bash scripts/pack_wasm.sh

cd web

bun i

cd ../server

cargo watch

cd ../plugin_w

wasm-pack build

cd ..

docker compose up -d

echo "Setup finished correctly"

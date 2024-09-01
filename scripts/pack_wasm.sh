#!/usr/bin/env bash

set -e

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

cd $SCRIPT_DIR/..

rm -rf web/pkg web/src/bindings

cd core
cargo test
cd ..

cd server
cargo test
cd ..

cd wasm
wasm-pack build

mv pkg ../web/

cd ../web
./node_modules/.bin/eslint --fix src/bindings

echo "Wasm pack complete!"

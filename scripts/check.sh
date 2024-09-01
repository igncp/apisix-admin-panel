#!/usr/bin/env bash

set -e

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

cd $SCRIPT_DIR/..

cargo clippy --release --all-features --all-targets -- -D warnings

cd web

./node_modules/.bin/eslint . --fix
bun run build

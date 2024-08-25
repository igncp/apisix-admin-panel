#!/usr/bin/env bash

set -e

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
VERSION="0.0.1"

cd $SCRIPT_DIR/..

bash scripts/pack_wasm.sh

cd web

bun i
rm -rf out
bun run build

cd ..

docker buildx build \
  --progress=plain \
  --platform linux/amd64,linux/arm64 \
  --push \
  -t igncp/apisix-admin-panel:$VERSION \
  .

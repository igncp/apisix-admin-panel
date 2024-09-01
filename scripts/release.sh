#!/usr/bin/env bash

set -e

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
VERSION=$(cat $SCRIPT_DIR/../VERSION | tr -d '\n')

cd $SCRIPT_DIR/..

bash scripts/pack_wasm.sh

cd web

bun i
rm -rf out
bun run build

cd ..

if [ -n "$DOCKER_PUSH" ]; then
  docker buildx build \
    --progress=plain \
    --platform linux/amd64,linux/arm64 \
    --push \
    -t igncp/apisix-admin-panel:$VERSION \
    -t igncp/apisix-admin-panel:latest \
    .
else
  docker build \
    --progress=plain \
    -t igncp/apisix-admin-panel:$VERSION \
    -t igncp/apisix-admin-panel:latest \
    .
fi

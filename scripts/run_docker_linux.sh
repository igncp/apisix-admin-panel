#!/usr/bin/env bash

set -e

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

cd $SCRIPT_DIR/..

docker run \
  --rm -it \
  --net host \
  -e PORT=3000 \
  -e APISIX_STANDALONE_CONFIG=/apisix_conf/apisix.yaml \
  -e APISIX_USERS='[{ "username": "foo", "password": "bar" }]' \
  -e APISIX_JWT_SECRET=foo \
  -v $PWD/scripts/apisix_standalone:/apisix_conf/ \
  --name apisix-admin-panel \
  igncp/apisix-admin-panel

#!/usr/bin/env bash

set -e

docker run \
  --rm -it \
  --net host \
  -e PORT=3000 \
  --name apisix-admin-panel \
  apisix-admin-panel

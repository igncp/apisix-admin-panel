#!/usr/bin/env bash

set -e

PLATFORM=$(uname -m)

rustup target add $PLATFORM-unknown-linux-musl

cd server

echo "PLATFORM: $PLATFORM"

CARGO_BUILD_TARGET=$PLATFORM-unknown-linux-musl \
  cargo build --release --target $PLATFORM-unknown-linux-musl

cd ..

mv \
  target/$PLATFORM-unknown-linux-musl/release/apisix-admin-panel \
  ./apisix-admin-panel

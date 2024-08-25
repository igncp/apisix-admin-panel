FROM rust:1-alpine3.20 AS builder
WORKDIR /app

RUN apk add --no-cache libressl-dev ca-certificates musl-dev alpine-sdk libffi-dev bash
COPY . .

RUN bash scripts/docker_buildx.sh

FROM alpine:3.20.2
WORKDIR /app

RUN apk add --no-cache openssl-dev ca-certificates

COPY --from=builder /app/apisix-admin-panel .
COPY web/out static

CMD ./apisix-admin-panel

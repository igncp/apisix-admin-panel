# APISIX Admin Panel

Unofficial web UI for managing Apache APISIX. Even if there is already the
[APISIX dashboard project](https://github.com/apache/apisix-dashboard), this
project aims to be an alternative solution.

## Introduction

- Lightweight: It should be fast, low in memory (currently ~5mb), and small in size (currently ~25mb)
- Powerful: It should support more features than the official dashboard project
- Secure: It should be secure by default, keeping the API key only in the server

To achieve this the tech stack uses Rust in the backend and in the frontend
(via WebAssembly). There is also some code using JavaScript for the frontend UI
which is built into static files.

It supports both [Standalone deployments](https://apisix.apache.org/docs/apisix/deployment-modes/#standalone) via updating the config file, and [Traditional deployments](https://apisix.apache.org/docs/apisix/deployment-modes/#traditional) via API requests.

## Try it out

You can run it locally with:

```sh
docker run \
  --rm -it \
  --net host \
  -e PORT=3000 \
  --name apisix-admin-panel \
  igncp/apisix-admin-panel:latest
```

And then access: http://localhost:3000

You can pass a custom `APISIX_ADMIN_KEY` env variable to interact with your
APISIX instance.

You can find information in the [Docker Hub page](https://hub.docker.com/r/igncp/apisix-admin-panel).

## Requirements

- APISIX v3
- Docker

## Features already available

- Create, read, update, and delete most entities (routes, consumers, consumer groups, etc.)
- Support for most plugins, and you can always input raw JSON if the input is not yet supported
- Multiple deployments modes
- Supports requiring authentication to the web UI via a form

## Development

After cloning the repository, you can run `docker compose up -d` to run a local
instance of APISIX where to test the project. PRs are welcomed.

## Roadmap for v0.1

- Complete the usage guide
- Add deployment CI with versioning

## Roadmap for v1

For the MVP, the following features are planned:

- ... Most features included in the official dashboard (some of them included here)
- Multilanguage
- Responsive
- Dark / Light mode
- Support all built-in plugins
- Input validations

## Ideas for features

- Bulk updates
- Pagination
- Support all control APIs
- E2E tests with playwright
- Export/Import
- Backups
- Search: https://apisix.apache.org/docs/apisix/admin-api/#support-filtering-query
- Use the official dashboard in the docker compose
- Trigger commands
- Deeplinks

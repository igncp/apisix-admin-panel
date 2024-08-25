# APISIX Admin Panel

Unofficial web UI for managing Apache APISIX. Even if there is already the
[APISIX dashboard project](https://github.com/apache/apisix-dashboard), this
projects aims to be an alternative solution.

## Principles

- Lightweight: Should be fast, low in memory (currently ~5mb), and small in size (currently ~10mb)
- Powerful: It should support more features than the official dashboard project
- Secure: It should be secure by default, keeping the API key only in the server

To achieve this the tech stack uses Rust in the backend and in the frontend
(via WebAssembly). There is also some code using JavaScript for the frontend UI
which is built into static files.

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

## Development

After cloning the repository, you can run `docker compose up -d` to run a local
instance of APISIX where to test the project. PRs are welcomed.

## Roadmap for v0.1

- Complete the usage guide
- Add deployment CI with versioning
- Implement CHANGELOG.md

## Roadmap for v1

For the MVP, the following features are planned:

- ... Most features included in the official dashboard (some of them included here)
- Multi-arch docker images
- Multilanguage
- Responsive
- Dark mode
- Support all built-in plugins
- Input validations
- Authentication
- Edit form

## Ideas for features

- Bulk updates
- Pagination
- Support all control APIs
- Support changing the YAML configuration via docker volume
- Support configuring custom plugins

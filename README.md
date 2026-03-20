# Widehabit

Widehabit is a personal habit-tracking service. It is designed to manage habits, schedules, and habit logs through a backend API and a browser-based interface.

## Overview

This repository is organized as a Rust workspace with three main parts:

- `server/`: backend API on `axum` and `tokio`
- `frontend/`: `leptos` CSR web client built with `trunk`
- `shared/`: common DTOs and models shared between backend and frontend

Supporting project assets live at the repository root:

- `migrations/`: Diesel migrations
- `docker/` and `docker-compose.yaml`: local infrastructure and deployment assets

## Stack

- Rust `1.92+`
- PostgreSQL `18.1`
- `axum` for HTTP
- `diesel` + `diesel-async` + `bb8` for persistence
- `utoipa` + Swagger UI for OpenAPI
- `leptos` for the frontend
- Docker Compose for local infrastructure

## Runtime Layout

- Backend binary: `widehabit-server`
- API prefix: `/api/v1`
- Prometheus metrics: `/metrics`
- Swagger UI: `/swagger-ui` in debug builds only
- Frontend development proxy: `/api/v1` -> `http://127.0.0.1:9091/api/v1`

## Environment Configuration

The project uses a local `.env` file.

Variables currently referenced by the repository include:

- `POSTGRES_HOST`
- `POSTGRES_DB`
- `POSTGRES_USER`
- `POSTGRES_PASSWORD`
- `DATABASE_URL`
- `WIDE_DATABASE_URL`
- `WIDE_LISTEN_ADDRESS`
- `WIDE_ACCESS_LT`
- `PROMETHEUS_PORT`
- `LOKI_PORT`
- `GRAFANA_PORT`
- `GRAFANA_ADMIN_PASSWORD`

The backend also supports additional `WIDE_*` settings with internal defaults:

- `WIDE_LISTEN_PORT` default: `9091`
- `WIDE_LOG_LEVEL` default: `DEBUG`
- `WIDE_JSON_LOG` default: `false`
- `WIDE_DATABASE_POOL` default: `30`
- `WIDE_JWT_SECRET`
- `WIDE_ACCESS_LT` default: `15`
- `WIDE_REFRESH_LT` default: `6`

Example local configuration:

```env
POSTGRES_HOST=localhost
POSTGRES_DB=widehabit
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres

DATABASE_URL=postgresql://postgres:postgres@localhost:5432/widehabit
WIDE_DATABASE_URL=postgresql://postgres:postgres@localhost:5432/widehabit

WIDE_LISTEN_ADDRESS=127.0.0.1
WIDE_LISTEN_PORT=9091
WIDE_LOG_LEVEL=DEBUG
WIDE_JSON_LOG=false
WIDE_DATABASE_POOL=30

WIDE_JWT_SECRET=change-me
WIDE_ACCESS_LT=15
WIDE_REFRESH_LT=6

PROMETHEUS_PORT=9090
LOKI_PORT=3100
GRAFANA_PORT=3000
GRAFANA_ADMIN_PASSWORD=admin
```

## Local Development

### Prerequisites

Install system dependencies:

```bash
# Ubuntu / Debian
sudo apt update
sudo apt install -y libpq-dev pkg-config
```

Install Rust toolchain requirements:

```bash
rustup toolchain install stable
rustup target add wasm32-unknown-unknown
cargo install diesel_cli --no-default-features --features postgres
cargo install trunk
```

### Start local infrastructure

```bash
docker compose up -d postgres
```

To run the full backend-oriented stack:

```bash
docker compose up -d --build
```

To enable monitoring too:

```bash
docker compose --profile monitoring up -d --build
```

### Apply migrations

```bash
diesel migration run
```

`diesel_cli` uses `DATABASE_URL`, while the backend itself reads `WIDE_DATABASE_URL`.

### Run the backend

From the workspace root:

```bash
cargo run -p widehabit-server
```

### Run the frontend

From `frontend/`:

```bash
trunk serve --open
```

The frontend uses the proxy configuration from `frontend/Trunk.toml` to forward `/api/v1` requests to the backend.

## Production Deployment

The repository now includes a production-oriented Compose stack in `docker-compose.prod.yaml`.

It runs:

- PostgreSQL
- Diesel migrations as a one-shot container
- `widehabit-server`
- `nginx` serving the built frontend and proxying `/api/v1` to the backend

### Prepare environment

Create a production env file from the example:

```bash
cp .env.production.example .env.production
```

Update at least:

- `POSTGRES_PASSWORD`
- `WIDE_DATABASE_URL`
- `WIDE_JWT_SECRET`

### Start the stack

```bash
docker compose -f docker-compose.prod.yaml up -d --build
```

The frontend will be available on port `1966`.

### Deployment layout note

If you want to keep only Compose manifests under `/opt/docker`, prefer using prebuilt images from a registry.
If you build images directly from this repository, keep the deployment files next to the source tree or clone the repository on the target host so the Compose build contexts remain valid.

## Useful Commands

Check the whole workspace:

```bash
cargo check --workspace
```

## Agent Workflow

If you prefer working through Codex or other agent-style flows, the repository now includes a local skill for the frontend-backed tracker API:

- Skill path: `.codex/skills/widehabit-api`
- Scope: registration, login, habit list and mutations, schedules, habit logging, and habit stats
- Default host discovery: `WIDEHABIT_API_BASE_URL`, with local fallback to `http://127.0.0.1:9091/api/v1`

Example commands:

```bash
.codex/skills/widehabit-api/scripts/login.sh --username <username> --password <password>
.codex/skills/widehabit-api/scripts/api.sh GET '/habit?page=1&limit=7'
.codex/skills/widehabit-api/scripts/api.sh POST /habit '{"name":"Read","description":"20 min"}'
```

The skill keeps a local access token and cookie jar in `/tmp/widehabit-api-skill` unless `WIDEHABIT_API_STATE_DIR` is set.

Run the password hashing helper:

```bash
cargo run -p widehabit-server --bin arghash
```

## API and Debug Endpoints

With the backend running in debug mode:

- Swagger UI: `http://127.0.0.1:9091/swagger-ui`
- OpenAPI JSON: `http://127.0.0.1:9091/api-docs/openapi.json`
- Metrics: `http://127.0.0.1:9091/metrics`

Swagger UI is compiled only for debug builds.

## Development Notes

- Put shared request and response types into `shared/` first when both frontend and backend use them
- Keep backend layering intact: router -> service -> db/repo
- Update `server/src/api/docs.rs` when changing API surface

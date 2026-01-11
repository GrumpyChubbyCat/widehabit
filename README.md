# Widehabit

Widehabit is a service designed for personal habit tracking, built with high-performance Rust.

## Project Stack

* **Programming Language:** Rust >= 1.92
* **Database:** PostgreSQL 18.1
* **Containerization & Orchestration:** Docker, Docker Compose

## Core Libraries

1. **Runtime:** `tokio`
2. **HTTP Framework:** `axum`
3. **Serialization/Deserialization:** `serde`
4. **Database Layer:** `async-diesel` with `bb8` connection pooling, `diesel_migrations`
5. **API Routing & OpenAPI:** `utoipa`

## Deployment

The application can be deployed using Docker Compose:

```bash
docker compose up -d
```

## Environment Variables (.env)

```
# Database Settings
POSTGRES_HOST=localhost
POSTGRES_DB=widehobby
POSTGRES_USER=lamantin
POSTGRES_PASSWORD=chokny1975

# Diesel CLI & App Connection String
WIDE_DATABASE_URL=postgres://lamantin:chokny1975@localhost:5432/widehobby
WIDE_DATABASE_POOL=10

# Server Settings
WIDE_LISTEN_ADDRESS=0.0.0.0
WIDE_LISTEN_PORT=9091
WIDE_LOG_LEVEL=debug

# Authentication (JWT)
WIDE_JWT_SECRET=your_super_secret_key_change_me
WIDE_ACCESS_LT=3600
WIDE_REFRESH_LT=86400
```

## Developer Guide

### Prerequisites

Install Rust, Cargo, and the required PostgreSQL development libraries:

```bash
# Ubuntu/Debian
sudo apt update && sudo apt install libpq-dev
```

### Database Setup

Install the Diesel CLI with PostgreSQL features and run migrations:

```bash
cargo install diesel_cli --no-default-features --features postgres
diesel migration run
```

### Running the Service

To start the service in development mode:

```bash
cargo run
```

## API Documentation

When running in debug mode, the Swagger UI is available at:
[http://localhost:9091/swagger-ui/](https://www.google.com/search?q=http://localhost:9091/swagger-ui/)

**Note:** The API documentation is disabled in `--release` mode and within Docker containers.
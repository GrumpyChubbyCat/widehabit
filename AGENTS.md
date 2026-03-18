# AGENTS.md

## Project Summary

Widehabit is a personal habit-tracking service built as a Rust monorepo. The project combines:

- `server/`: backend API on `axum` and `tokio`
- `frontend/`: web UI on `leptos`
- `shared/`: shared models and DTOs for backend and frontend
- PostgreSQL 18.1 for persistence
- Docker and Docker Compose for local deployment and orchestration

The workspace targets Rust `1.92+` and uses a shared dependency setup from the root `Cargo.toml`.

## How To Work In This Repository

Before changing code, orient yourself by crate and responsibility:

1. Identify the correct workspace member first: `server/`, `frontend/`, or `shared/`.
2. If the change affects both API and UI contracts, inspect `shared/src/model/` first and update shared types there.
3. Keep backend layering intact:
   - API handlers live in `server/src/api/router/`
   - business logic lives in `server/src/service/`
   - database access lives in `server/src/db/repo/`
4. For database work, use the existing `diesel-async` + `bb8` approach in the repository layer.
5. If you add or change API endpoints, update OpenAPI-related code in `server/src/api/docs.rs`.
6. For frontend styling, keep CSS split by feature/component in `frontend/style/`, with shared global styles in `frontend/style/base.css`.

## Backend Rules

- Use `InternalError` from `server/src/errors.rs` for business, database, and domain errors.
- Use `StartError` for startup and initialization failures.
- Map backend errors to HTTP responses through `server/src/api/errors.rs`.
- Do not introduce new error variants before checking the existing error model.

## Local Runbook

- Start infrastructure with `docker compose up -d`
- Run database migrations with `diesel migration run`
- Run the backend in development with `cargo run -p widehabit-server`
- In debug builds, Swagger UI is exposed at `http://localhost:9091/swagger-ui/`

Environment is configured through `.env`, including database connection, server listen settings, and JWT lifetimes.

## Development Notes

- Use `cargo run -p widehabit-server` when you want to work directly on the backend crate.
- Use `cargo check --workspace` after changes that touch more than one crate or shared contracts.
- Treat `shared/src/model/` as the source of truth for structures used by both API and UI.
- Update migrations and repository code together when database behavior changes.
- Keep frontend styling changes localized in `frontend/style/` instead of mixing them into Rust modules.

## Important Import Rule

This repository has a strict import style:

- Prefer direct imports for structs, enums, functions, and types at the top of the file.
- Avoid fully qualified paths inside implementation code.
- If names collide, import the parent modules and disambiguate with module prefixes.
- Group imports logically: standard library, third-party crates, then local modules.
- Do not use inline paths for types or functions unless it is strictly necessary and cannot be solved through normal imports.

In short: keep logic readable, keep imports explicit, and do not scatter long module paths through the code body.

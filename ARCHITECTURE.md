# ARCHITECTURE.md

## Overview

Widehabit is organized as a Rust workspace with three main crates:

- `server/`: HTTP API, business services, database access, auth, and OpenAPI docs
- `frontend/`: Leptos-based web client compiled for the browser
- `shared/`: common domain models reused by backend and frontend

Supporting infrastructure is kept at the repository root:

- `docker/` and `docker-compose.yaml` for containerized development and deployment
- `migrations/` for database schema changes
- `diesel.toml` for Diesel configuration

The product itself is a habit-tracking service built around four main concerns:

- authentication and session handling
- habit management
- schedule planning
- habit log recording and retrieval

## Development Flow

The workspace is intended to be developed as a single system rather than as isolated crates.

- Start local infrastructure with `docker compose up -d`
- Apply Diesel migrations before validating backend flows that depend on schema changes
- Run `cargo check --workspace` to verify cross-crate compatibility
- Update `shared/` first when a change affects both backend and frontend
- Keep `server/src/api/docs.rs` aligned with backend API changes so Swagger stays accurate in debug mode

## Workspace Layout

The root workspace is declared in `Cargo.toml` and includes:

- `server`
- `frontend`
- `shared`

Shared workspace dependencies include `utoipa`, `serde`, `uuid`, `chrono`, `validator`, `jsonwebtoken`, and related support crates.

## Architectural Responsibilities

### `server/`

The backend is an `axum` application running on `tokio`.

- `src/api/`: API surface, request extractors, docs, error mapping, and router assembly
- `src/api/router/`: route handlers grouped by feature: auth, habits, logs, schedules, health
- `src/service/`: business logic layer
- `src/db/`: entities, schema, and repositories
- `src/db/repo/`: persistence operations for users, habits, logs, and schedules
- `src/model/`: backend-specific auth-related models
- `src/errors.rs`: internal and startup error definitions
- `src/config.rs`: application configuration

### `frontend/`

The frontend is a Leptos web application.

- `src/api/`: client-side API integration and API error handling
- `src/pages.rs`: top-level pages
- `src/components/`: reusable UI elements such as icons and modal dialogs
- `style/`: scoped CSS files per area, with shared rules in `base.css`
- `public/assets/icons/`: static icon assets
- `index.html` and `Trunk.toml`: frontend entry and build config

### `shared/`

The shared crate contains domain models used across the stack.

- `src/model/auth.rs`
- `src/model/habit.rs`
- `src/model/log.rs`
- `src/model/schedule.rs`
- `src/model/user.rs`

Any change to request/response contracts or shared domain structures should be evaluated here first.

## Repository Structure

The architecture-relevant structure of the repository is:

```text
.
├── Cargo.toml
├── Cargo.lock
├── README.md
├── AGENTS.md
├── ARCHITECTURE.md
├── diesel.toml
├── docker
│   ├── Dockerfile.migrations
│   ├── Dockerfile.service
│   ├── prometheus.yml
│   └── promtail.yml
├── docker-compose.yaml
├── frontend
│   ├── Cargo.toml
│   ├── Trunk.toml
│   ├── index.html
│   ├── public
│   │   └── assets
│   │       └── icons
│   │           ├── logout.svg
│   │           ├── pen.svg
│   │           ├── plus.svg
│   │           ├── settings.svg
│   │           └── trash.svg
│   ├── src
│   │   ├── api
│   │   │   ├── client.rs
│   │   │   ├── errors.rs
│   │   │   └── mod.rs
│   │   ├── components
│   │   │   ├── icons.rs
│   │   │   ├── mod.rs
│   │   │   └── modals.rs
│   │   ├── lib.rs
│   │   ├── main.rs
│   │   └── pages.rs
│   ├── style
│   │   ├── auth.css
│   │   ├── base.css
│   │   ├── calendar.css
│   │   ├── habit-item.css
│   │   ├── layout.css
│   │   └── modal.css
├── migrations
│   ├── 00000000000000_diesel_initial_setup
│   ├── 2026-01-06-144155-0000_create_users_and_roles
│   ├── 2026-01-11-122605-0000_new_user_default_role
│   ├── 2026-01-11-173822-0000_add_timestamps
│   ├── 2026-01-11-173908-0000_create_habit_tables
│   ├── 2026-01-14-194721-0000_add_schedules_version_id
│   └── 2026-01-15-195208-0000_habit_logs_recreation
├── server
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── src
│       ├── api
│       │   ├── docs.rs
│       │   ├── errors.rs
│       │   ├── extractors.rs
│       │   ├── mod.rs
│       │   ├── router
│       │   │   ├── auth.rs
│       │   │   ├── habit.rs
│       │   │   ├── health.rs
│       │   │   ├── log.rs
│       │   │   ├── mod.rs
│       │   │   └── schedule.rs
│       │   └── state.rs
│       ├── bin
│       │   └── arghash.rs
│       ├── config.rs
│       ├── db
│       │   ├── entity.rs
│       │   ├── mod.rs
│       │   ├── repo
│       │   │   ├── habit.rs
│       │   │   ├── log.rs
│       │   │   ├── mod.rs
│       │   │   ├── schedule.rs
│       │   │   └── user.rs
│       │   └── schema.rs
│       ├── errors.rs
│       ├── lib.rs
│       ├── main.rs
│       ├── model
│       │   ├── auth.rs
│       │   └── mod.rs
│       └── service
│           ├── habit.rs
│           ├── log.rs
│           ├── mod.rs
│           ├── schedule.rs
│           └── user.rs
└── shared
    ├── Cargo.toml
    └── src
        ├── lib.rs
        └── model
            ├── auth.rs
            ├── habit.rs
            ├── log.rs
            ├── mod.rs
            ├── schedule.rs
            └── user.rs
```

Build outputs such as `frontend/dist/` are intentionally excluded from this view because they are generated artifacts rather than source architecture.

## Change Routing Guide

Use this routing logic when deciding where to implement a change:

- Shared DTO or model change: `shared/`
- HTTP handler, routing, extractors, OpenAPI, response mapping: `server/src/api/`
- Business rules: `server/src/service/`
- Database entities and repository access: `server/src/db/`
- UI pages, components, and client-side behavior: `frontend/src/`
- Frontend styles: `frontend/style/`
- Environment and deployment changes: root files, `docker/`, and `docker-compose.yaml`

## Development Entry Points

- Backend executable: `server/src/main.rs`
- Backend shared module surface: `server/src/lib.rs`
- Frontend executable: `frontend/src/main.rs`
- Frontend shared module surface: `frontend/src/lib.rs`
- Shared contract entrypoint: `shared/src/lib.rs`

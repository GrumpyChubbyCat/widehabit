---
name: widehabit-api
description: "Use when working with the frontend-backed Widehabit API directly instead of the UI: registration, login, listing or mutating habits, reading or writing schedules, logging habit time, checking habit stats, or scripting authenticated calls against the tracker server."
---

# Widehabit API

Use this skill when the user wants to work with Widehabit through the backend API rather than the frontend.

## Quick Start

1. Prefer an explicit tracker base URL from `WIDEHABIT_API_BASE_URL`.
2. If it is not set, use the local dev default `http://127.0.0.1:9091/api/v1`.
3. If the backend may not be running, remind yourself of the repo runbook:
   `docker compose up -d postgres`
   `diesel migration run`
   `cargo run -p widehabit-server`
4. Log in with `scripts/login.sh` to obtain an access token and refresh cookie.
5. Send follow-up requests with `scripts/api.sh`.

## Workflow

- For endpoint shapes, payload fields, and enum values, read `references/api.md`.
- Treat this skill as scoped to the API surface currently exercised by the frontend.
- Prefer the bundled scripts over handwritten `curl` when you need authenticated requests repeatedly.
- Use `scripts/api.sh` for both read and write operations. It retries once after `401` by calling `/auth/refresh` with the saved cookie jar.
- If login fails with `401`, assume the credentials are invalid and ask the user for new ones rather than guessing.
- If the host is unreachable or returns an unexpected response, surface that clearly and ask the user to confirm the base URL.
- If the user asks you to change the API, treat `shared/src/model/` as the contract source of truth, then update the server router and docs as needed.

## Scripts

- `scripts/login.sh --username <name> --password <password>`
- `scripts/api.sh GET /habit?page=1\\&limit=7`
- `scripts/api.sh POST /habit '{"name":"Read","description":"20 min"}'`
- `scripts/api.sh PUT /schedule @/tmp/schedule.json`
- `scripts/api.sh POST /log/<habit_id> '{"comment":"done"}'`

The scripts store session state in `${WIDEHABIT_API_STATE_DIR:-/tmp/widehabit-api-skill}`:

- `access_token`
- `cookies.txt`

## Notes

- The frontend-backed routes in scope are `/auth/registration`, `/auth/login`, `/habit?page=1&limit=7`, `POST /habit`, `PATCH /habit/{habit_id}`, `DELETE /habit/{habit_id}`, `GET /schedule`, `PUT /schedule`, `POST /log/{habit_id}`, and `GET /log/{habit_id}/stats`.
- Public auth routes live under `/auth`; `/habit`, `/schedule`, and `/log` require `Authorization: Bearer <token>`.
- Login returns an access token JSON body and also sets the `refresh_token` cookie.
- `DayOfWeek` values serialize as lowercase strings like `monday` and `friday`.
- In debug builds, Swagger UI is available at `http://127.0.0.1:9091/swagger-ui` and the OpenAPI JSON at `http://127.0.0.1:9091/api-docs/openapi.json`.

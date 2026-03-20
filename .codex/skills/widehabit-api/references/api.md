# Widehabit Frontend API Reference

This reference mirrors the API surface the current frontend actually uses. It is based on the Rust sources in `frontend/src/`, `server/src/api/router/`, and `shared/src/model/`.

## Base URL

- Default local base URL: `http://127.0.0.1:9091/api/v1`
- API prefix: `/api/v1`

## Auth

### `POST /auth/registration`

Request body:

```json
{
  "email": "user@example.com",
  "username": "nikita",
  "password": "secret123"
}
```

Validation:

- `email` must be a valid email
- `username` length: 3..20
- `password` length: at least 6

Response:

- `201 Created`

### `POST /auth/login`

Request body:

```json
{
  "username": "nikita",
  "password": "secret123"
}
```

Response body:

```json
{
  "access_token": "<jwt>"
}
```

Behavior:

- Sets `refresh_token` as an `HttpOnly` cookie
- That cookie is later used by `POST /auth/refresh` inside the frontend client and bundled scripts

### `POST /auth/refresh`

This is an implementation detail used by the frontend client and bundled scripts after `401`.

Response body:

```json
{
  "access_token": "<jwt>"
}
```

Notes:

- Does not require a JSON body
- Requires the saved `refresh_token` cookie

## Habits

### `POST /habit/`

Headers:

- `Authorization: Bearer <access_token>`

Request body:

```json
{
  "name": "Morning run",
  "description": "20 minutes outside"
}
```

Response body:

```json
{
  "habit_id": "uuid",
  "name": "Morning run",
  "description": "20 minutes outside",
  "status": "Progress"
}
```

### `GET /habit/?page=<n>&limit=<n>`

Returns:

```json
{
  "items": [],
  "total_count": 0,
  "page": 1,
  "page_size": 20
}
```

Notes:

- The frontend currently requests `page=1&limit=7`
- `page` and `limit` are required query params in the current server DTO

### `PATCH /habit/{habit_id}`

Request body matches `NewHabitReq`:

```json
{
  "name": "Morning run",
  "description": "25 minutes outside"
}
```

Response body:

```json
{
  "habit_id": "uuid",
  "name": "Morning run",
  "description": "25 minutes outside"
}
```

### `DELETE /habit/{habit_id}`

Response:

- `204 No Content`

## Schedules

`DayOfWeek` is serialized with lowercase names:

- `monday`
- `tuesday`
- `wednesday`
- `thursday`
- `friday`
- `saturday`
- `sunday`

### `GET /schedule/`

Returns:

```json
{
  "schedules": []
}
```

### `PUT /schedule/`

Request body:

```json
{
  "habit_id": "uuid",
  "schedules": [
    {
      "day": "monday",
      "start_time": "08:00:00",
      "end_time": "09:00:00"
    },
    {
      "day": "friday",
      "start_time": "18:00:00",
      "end_time": "18:30:00"
    }
  ]
}
```

Notes:

- `start_time` and `end_time` use `HH:MM:SS`

## Logs

### `POST /log/{habit_id}`

Request body:

```json
{
  "habit_schedule_id": null,
  "log_date": "2026-03-20",
  "actual_start": "2026-03-20T08:01:00Z",
  "actual_end": "2026-03-20T08:24:00Z",
  "comment": "done"
}
```

All fields are optional in the current DTO.

Response body:

```json
{
  "habit_log_id": "uuid",
  "habit_id": "uuid",
  "habit_schedule_id": null,
  "log_date": "2026-03-20",
  "actual_start": "2026-03-20T08:01:00Z",
  "actual_end": "2026-03-20T08:24:00Z",
  "comment": "done"
}
```

### `GET /log/{habit_id}/stats`

Returns:

```json
{
  "habit_id": "uuid",
  "total_minutes": 120
}
```

## Source Of Truth

If anything here and the code disagree, prefer these project files:

- `frontend/src/pages.rs`
- `frontend/src/components/mod.rs`
- `frontend/src/components/modals.rs`
- `server/src/api/router/auth.rs`
- `server/src/api/router/habit.rs`
- `server/src/api/router/schedule.rs`
- `server/src/api/router/log.rs`
- `shared/src/model/user.rs`
- `shared/src/model/habit.rs`
- `shared/src/model/schedule.rs`
- `shared/src/model/log.rs`
- `shared/src/model/mod.rs`

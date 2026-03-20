#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${WIDEHABIT_API_BASE_URL:-http://127.0.0.1:9091/api/v1}"
STATE_DIR="${WIDEHABIT_API_STATE_DIR:-/tmp/widehabit-api-skill}"
COOKIE_JAR="$STATE_DIR/cookies.txt"
TOKEN_FILE="$STATE_DIR/access_token"

usage() {
    cat <<'EOF'
Usage:
  api.sh <METHOD> <PATH> [JSON_BODY|@JSON_FILE]

Examples:
  api.sh GET /habit?page=1\&limit=7
  api.sh POST /habit '{"name":"Read","description":"20 min"}'
  api.sh PUT /schedule @/tmp/schedule.json

Environment:
  WIDEHABIT_API_BASE_URL
  WIDEHABIT_API_STATE_DIR
EOF
}

extract_access_token() {
    if command -v jq >/dev/null 2>&1; then
        jq -r '.access_token // empty'
        return
    fi

    sed -n 's/.*"access_token"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p'
}

normalize_base_url() {
    local url="$1"

    if [[ "$url" != http://* && "$url" != https://* ]]; then
        url="http://$url"
    fi

    while [[ "$url" == */ ]]; do
        url="${url%/}"
    done

    printf '%s' "$url"
}

refresh_access_token() {
    local refresh_body
    refresh_body="$(mktemp)"

    local refresh_code
    refresh_code="$(
        curl -sS \
            -o "$refresh_body" \
            -w '%{http_code}' \
            -b "$COOKIE_JAR" \
            -c "$COOKIE_JAR" \
            -X POST \
            "$BASE_URL/auth/refresh"
    )"

    if [ "$refresh_code" -lt 200 ] || [ "$refresh_code" -ge 300 ]; then
        rm -f "$refresh_body"
        return 1
    fi

    local new_token
    new_token="$(extract_access_token < "$refresh_body")"
    rm -f "$refresh_body"

    if [ -z "$new_token" ]; then
        return 1
    fi

    printf '%s' "$new_token" > "$TOKEN_FILE"
    return 0
}

run_request() {
    local method="$1"
    local path_part="$2"
    local body_arg="${3:-}"
    local body_file="$4"

    local url="$BASE_URL$path_part"
    local curl_args=(
        -sS
        -o "$body_file"
        -w '%{http_code}'
        -b "$COOKIE_JAR"
        -X "$method"
    )

    if [ -s "$TOKEN_FILE" ]; then
        curl_args+=(-H "Authorization: Bearer $(cat "$TOKEN_FILE")")
    fi

    if [ -n "$body_arg" ]; then
        curl_args+=(-H 'Content-Type: application/json')
        if [[ "$body_arg" == @* ]]; then
            curl_args+=(--data "@${body_arg#@}")
        else
            curl_args+=(--data "$body_arg")
        fi
    fi

    curl "${curl_args[@]}" "$url"
}

method="${1:-}"
path_part="${2:-}"
body_arg="${3:-}"

if [ "$method" = "-h" ] || [ "$method" = "--help" ]; then
    usage
    exit 0
fi

if [ -z "$method" ] || [ -z "$path_part" ]; then
    usage >&2
    exit 1
fi

if [[ "$path_part" != /* ]]; then
    path_part="/$path_part"
fi

BASE_URL="$(normalize_base_url "$BASE_URL")"

mkdir -p "$STATE_DIR"
touch "$COOKIE_JAR"

response_body="$(mktemp)"
trap 'rm -f "$response_body"' EXIT

http_code="$(run_request "$method" "$path_part" "$body_arg" "$response_body")"

if [ "$http_code" = "401" ] && refresh_access_token; then
    http_code="$(run_request "$method" "$path_part" "$body_arg" "$response_body")"
fi

if [ "$http_code" -lt 200 ] || [ "$http_code" -ge 300 ]; then
    printf 'Request failed with HTTP %s\n' "$http_code" >&2
    cat "$response_body" >&2
    exit 1
fi

cat "$response_body"

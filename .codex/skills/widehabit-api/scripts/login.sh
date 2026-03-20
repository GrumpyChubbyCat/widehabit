#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${WIDEHABIT_API_BASE_URL:-http://127.0.0.1:9091/api/v1}"
STATE_DIR="${WIDEHABIT_API_STATE_DIR:-/tmp/widehabit-api-skill}"
COOKIE_JAR="$STATE_DIR/cookies.txt"
TOKEN_FILE="$STATE_DIR/access_token"

usage() {
    cat <<'EOF'
Usage:
  login.sh --username <username> --password <password>

Environment fallbacks:
  WIDEHABIT_USERNAME
  WIDEHABIT_PASSWORD
  WIDEHABIT_API_BASE_URL
  WIDEHABIT_API_STATE_DIR
EOF
}

json_escape() {
    local value="$1"
    value=${value//\\/\\\\}
    value=${value//\"/\\\"}
    value=${value//$'\n'/\\n}
    value=${value//$'\r'/\\r}
    value=${value//$'\t'/\\t}
    printf '%s' "$value"
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

extract_access_token() {
    if command -v jq >/dev/null 2>&1; then
        jq -r '.access_token // empty'
        return
    fi

    sed -n 's/.*"access_token"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p'
}

username="${WIDEHABIT_USERNAME:-}"
password="${WIDEHABIT_PASSWORD:-}"

while [ "$#" -gt 0 ]; do
    case "$1" in
        --username)
            username="${2:-}"
            shift 2
            ;;
        --password)
            password="${2:-}"
            shift 2
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            printf 'Unknown argument: %s\n' "$1" >&2
            usage >&2
            exit 1
            ;;
    esac
done

if [ -z "$username" ] || [ -z "$password" ]; then
    printf 'Both username and password are required.\n' >&2
    usage >&2
    exit 1
fi

BASE_URL="$(normalize_base_url "$BASE_URL")"

mkdir -p "$STATE_DIR"
touch "$COOKIE_JAR"

payload=$(
    printf '{"username":"%s","password":"%s"}' \
        "$(json_escape "$username")" \
        "$(json_escape "$password")"
)

response_file="$(mktemp)"
trap 'rm -f "$response_file"' EXIT

http_code="$(
    curl -sS \
        -o "$response_file" \
        -w '%{http_code}' \
        -c "$COOKIE_JAR" \
        -H 'Content-Type: application/json' \
        -X POST \
        "$BASE_URL/auth/login" \
        --data "$payload"
)"

if [ "$http_code" -lt 200 ] || [ "$http_code" -ge 300 ]; then
    printf 'Login failed with HTTP %s\n' "$http_code" >&2
    cat "$response_file" >&2
    exit 1
fi

access_token="$(extract_access_token < "$response_file")"

if [ -z "$access_token" ]; then
    printf 'Login succeeded but access_token was not found in the response.\n' >&2
    cat "$response_file" >&2
    exit 1
fi

printf '%s' "$access_token" > "$TOKEN_FILE"
cat "$response_file"
printf '\nSaved token to %s and cookies to %s\n' "$TOKEN_FILE" "$COOKIE_JAR"

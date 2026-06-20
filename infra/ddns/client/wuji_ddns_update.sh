#!/bin/sh
set -eu

if [ -z "${DDNS_FUNCTION_URL:-}" ]; then
  echo "DDNS_FUNCTION_URL must be set" >&2
  exit 1
fi

if [ -z "${DDNS_HOSTNAME:-}" ]; then
  echo "DDNS_HOSTNAME must be set" >&2
  exit 1
fi

TOKEN_FILE="${DDNS_TOKEN_FILE:-$HOME/.config/wuji-ddns/token}"
if [ ! -r "$TOKEN_FILE" ]; then
  echo "DDNS token file is not readable: $TOKEN_FILE" >&2
  exit 1
fi

set +e
RESPONSE=$(
  DDNS_FUNCTION_URL="$DDNS_FUNCTION_URL" \
  DDNS_HOSTNAME="$DDNS_HOSTNAME" \
  DDNS_TOKEN_FILE="$TOKEN_FILE" \
  python3 <<'PY'
import json
import os
import sys
import urllib.error
import urllib.request

url = os.environ["DDNS_FUNCTION_URL"]
hostname = os.environ["DDNS_HOSTNAME"]
token_file = os.environ["DDNS_TOKEN_FILE"]

with open(token_file, "r", encoding="utf-8") as handle:
    token = handle.read().strip()

payload = json.dumps({"hostname": hostname}).encode("utf-8")
request = urllib.request.Request(
    url,
    data=payload,
    method="POST",
    headers={
        "Authorization": f"Bearer {token}",
        "Content-Type": "application/json",
    },
)

try:
    with urllib.request.urlopen(request, timeout=15) as response:
        body = response.read().decode("utf-8")
        print(json.dumps({"http_code": response.getcode(), "body": body}))
except urllib.error.HTTPError as exc:
    body = exc.read().decode("utf-8")
    print(json.dumps({"http_code": exc.code, "body": body}))
    sys.exit(1)
except Exception as exc:
    print(json.dumps({"http_code": 0, "body": f"request_error: {exc}"}))
    sys.exit(2)
PY
)
STATUS=$?
set -e
HTTP_CODE=$(printf '%s' "$RESPONSE" | python3 -c 'import json,sys; print(json.loads(sys.stdin.read())["http_code"])')
BODY=$(printf '%s' "$RESPONSE" | python3 -c 'import json,sys; print(json.loads(sys.stdin.read())["body"])')

case "$HTTP_CODE" in
  200)
    printf '%s\n' "$BODY"
    ;;
  *)
    printf 'DDNS update failed with HTTP %s: %s\n' "$HTTP_CODE" "$BODY" >&2
    exit "${STATUS:-1}"
    ;;
esac

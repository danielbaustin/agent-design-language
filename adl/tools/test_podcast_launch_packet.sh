#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/adl-podcast-launch.XXXXXX")"
server_pid=""
cleanup() {
  if [ -n "$server_pid" ]; then
    kill "$server_pid" 2>/dev/null || true
  fi
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

env -u OPENAI_API_KEY -u GEMINI_API_KEY \
ADL_OPENAI_KEY_FILE="$TMP_DIR/missing-openai.key" \
ADL_GEMINI_KEY_FILE="$TMP_DIR/missing-gemini.key" \
ADL_PODCAST_AUDIO_TEST_TONES=1 \
ADL_PODCAST_LAUNCH_WORK_DIR="$TMP_DIR/adl-podcast-launch-work" \
  bash "$ROOT_DIR/adl/tools/demo_v0918_podcast_launch.sh" "$TMP_DIR/site" >/dev/null

python3 "$ROOT_DIR/adl/tools/validate_podcast_launch_packet.py" \
  "$TMP_DIR/site" \
  "$ROOT_DIR/docs/milestones/v0.91.8/review/podcast_launch_5711/episodes.json"

port="$(python3 - <<'PY'
import socket

with socket.socket() as sock:
    sock.bind(("127.0.0.1", 0))
    print(sock.getsockname()[1])
PY
)"
python3 -m http.server "$port" --bind 127.0.0.1 --directory "$ROOT_DIR/demos" >"$TMP_DIR/http.log" 2>&1 &
server_pid="$!"
python3 - <<'PY' "$port"
import sys
import time
import urllib.request

port = sys.argv[1]
url = f"http://127.0.0.1:{port}/podcast/"
deadline = time.time() + 5
while True:
    try:
        with urllib.request.urlopen(url, timeout=1) as response:
            if response.status == 200:
                break
    except Exception:
        pass
    if time.time() > deadline:
        raise SystemExit(f"podcast test HTTP server did not become ready: {url}")
    time.sleep(0.1)
PY

python3 "$ROOT_DIR/adl/tools/validate_podcast_launch_packet.py" \
  "$ROOT_DIR/demos/podcast" \
  "$ROOT_DIR/docs/milestones/v0.91.8/review/podcast_launch_5711/episodes.json" \
  --preview-root "$ROOT_DIR/demos/_preview/podcast" \
  --http-base "http://127.0.0.1:$port"

echo "test_podcast_launch_packet: PASS"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_ROOT="${1:-$ROOT/.adl/reports/demo-hitl-editor-review}"
MANIFEST_PATH="$OUT_ROOT/editor_review_demo_manifest.v1.json"

mkdir -p "$OUT_ROOT"

INDEX_PATH="$ROOT/docs/tooling/editor/index.html"
SCRIPT_PATH="$ROOT/docs/tooling/editor/task_bundle_editor.js"
STYLE_PATH="$ROOT/docs/tooling/editor/style.css"
STP_PATH="$ROOT/docs/records/v0.85/tasks/task-v085-wp05-demo/stp.md"
SIP_PATH="$ROOT/docs/records/v0.85/tasks/task-v085-wp05-demo/sip.md"
SOR_PATH="$ROOT/docs/records/v0.85/tasks/task-v085-wp05-demo/sor.md"
DEMO_DOC="$ROOT/docs/tooling/editor/demo.md"

for path in \
  "$INDEX_PATH" \
  "$SCRIPT_PATH" \
  "$STYLE_PATH" \
  "$STP_PATH" \
  "$SIP_PATH" \
  "$SOR_PATH" \
  "$DEMO_DOC"; do
  [[ -f "$path" ]] || {
    echo "[editor-demo] missing required path: $path" >&2
    exit 1
  }
done

PORT="$(
  python3 - <<'PY'
import socket
s = socket.socket()
s.bind(("127.0.0.1", 0))
print(s.getsockname()[1])
s.close()
PY
)"

SERVER_LOG="$OUT_ROOT/http_server.log"
python3 -m http.server "$PORT" --bind 127.0.0.1 --directory "$ROOT" >"$SERVER_LOG" 2>&1 &
SERVER_PID=$!
cleanup() {
  kill "$SERVER_PID" >/dev/null 2>&1 || true
}
trap cleanup EXIT
sleep 1

curl -fsS "http://127.0.0.1:${PORT}/docs/tooling/editor/index.html" >/dev/null
curl -fsS "http://127.0.0.1:${PORT}/docs/tooling/editor/task_bundle_editor.js" >/dev/null
curl -fsS "http://127.0.0.1:${PORT}/docs/tooling/editor/style.css" >/dev/null

DRY_RUN_COMMAND="$("$ROOT/adl/tools/editor_action.sh" start --issue 870 --branch codex/870-v085-wp05-first-editor-surfaces --dry-run)"

cat >"$MANIFEST_PATH" <<EOF
{
  "schema_version": "editor_review_demo_manifest.v1",
  "served_url": "http://127.0.0.1:${PORT}/docs/tooling/editor/index.html",
  "task_bundle_paths": [
    "docs/records/v0.85/tasks/task-v085-wp05-demo/stp.md",
    "docs/records/v0.85/tasks/task-v085-wp05-demo/sip.md",
    "docs/records/v0.85/tasks/task-v085-wp05-demo/sor.md"
  ],
  "review_doc": "docs/tooling/editor/demo.md",
  "workflow_action_dry_run": "${DRY_RUN_COMMAND}"
}
EOF

echo "[editor-demo] served URL: http://127.0.0.1:${PORT}/docs/tooling/editor/index.html"
echo "[editor-demo] workflow action dry run:"
printf '%s\n' "$DRY_RUN_COMMAND"
echo "[editor-demo] manifest:"
cat "$MANIFEST_PATH"
echo
echo "[editor-demo] PASS"

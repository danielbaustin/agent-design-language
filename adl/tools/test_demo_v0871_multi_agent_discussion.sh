#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

OUT_DIR="$TMPDIR_ROOT/artifacts"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v0871_multi_agent_discussion.sh "$OUT_DIR" >/dev/null
)

TRANSCRIPT="$OUT_DIR/transcript.md"
MANIFEST="$OUT_DIR/demo_manifest.json"
SUMMARY="$OUT_DIR/runtime/runs/v0-87-1-multi-agent-tea-discussion/run_summary.json"
STEPS="$OUT_DIR/runtime/runs/v0-87-1-multi-agent-tea-discussion/steps.json"
TRACE="$OUT_DIR/runtime/runs/v0-87-1-multi-agent-tea-discussion/logs/trace_v1.json"
FIRST_TURN="$OUT_DIR/out/discussion/01-chatgpt-opening.md"
LAST_TURN="$OUT_DIR/out/discussion/05-chatgpt-toast.md"

[[ -f "$TRANSCRIPT" ]] || {
  echo "assertion failed: transcript missing" >&2
  exit 1
}
[[ -f "$MANIFEST" ]] || {
  echo "assertion failed: manifest missing" >&2
  exit 1
}
[[ -f "$SUMMARY" ]] || {
  echo "assertion failed: run summary missing" >&2
  exit 1
}
[[ -f "$STEPS" ]] || {
  echo "assertion failed: steps artifact missing" >&2
  exit 1
}
[[ -f "$TRACE" ]] || {
  echo "assertion failed: trace missing" >&2
  exit 1
}
[[ -f "$FIRST_TURN" ]] || {
  echo "assertion failed: first turn missing" >&2
  exit 1
}
[[ -f "$LAST_TURN" ]] || {
  echo "assertion failed: last turn missing" >&2
  exit 1
}

grep -Fq "Turn 1 - ChatGPT" "$TRANSCRIPT" || {
  echo "assertion failed: transcript missing ChatGPT opening" >&2
  exit 1
}
grep -Fq "Turn 2 - Claude" "$TRANSCRIPT" || {
  echo "assertion failed: transcript missing Claude reply" >&2
  exit 1
}
grep -Fq '"steps": 5' "$MANIFEST" || {
  echo "assertion failed: manifest missing five-step declaration" >&2
  exit 1
}
grep -Fq '"execution_mode": "runtime_http_compatibility_demo"' "$MANIFEST" || {
  echo "assertion failed: manifest missing execution mode" >&2
  exit 1
}
grep -Fq '"conversation"' "$STEPS" || {
  echo "assertion failed: steps artifact missing conversation metadata" >&2
  exit 1
}
grep -Fq '"speaker": "Claude"' "$STEPS" || {
  echo "assertion failed: steps artifact missing Claude turn speaker" >&2
  exit 1
}
grep -Fq "five explicit turns" "$LAST_TURN" || {
  echo "assertion failed: final turn missing bounded proof summary" >&2
  exit 1
}

echo "demo_v0871_multi_agent_discussion: ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OBS="$ROOT_DIR/adl/tools/observability.sh"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

# shellcheck disable=SC1090
source "$OBS"

export ADL_OBSERVABILITY_REPO_ROOT="$ROOT_DIR"
export ADL_OBSERVABILITY_LOG="$TMP_DIR/events.log"

adl_obs_event "pr.sh" "doctor" "started" \
  "path" "$ROOT_DIR/.adl/v0.91.5/tasks/example/sor.md" \
  "token" "super-secret-token" \
  "tmp" "/private/tmp/adl-secret"

event="$(cat "$TMP_DIR/events.log")"
case "$event" in
  *"schema=adl.observability.event.v1"* ) ;;
  *) echo "missing observability schema: $event" >&2; exit 1 ;;
esac
case "$event" in
  *"command=pr.sh"*"stage=doctor"*"result=started"* ) ;;
  *) echo "missing command/stage/result fields: $event" >&2; exit 1 ;;
esac
case "$event" in
  *"<repo>/.adl/v0.91.5/tasks/example/sor.md"* ) ;;
  *) echo "repo path was not normalized: $event" >&2; exit 1 ;;
esac
if grep -Fq "$ROOT_DIR" "$TMP_DIR/events.log"; then
  echo "event leaked absolute repo path" >&2
  exit 1
fi
if grep -Eiq 'super-secret-token|/private/tmp/adl-secret' "$TMP_DIR/events.log"; then
  echo "event leaked secret-like or tmp path material" >&2
  exit 1
fi

ADL_OBSERVABILITY=0 adl_obs_event "pr.sh" "disabled" "started"
line_count="$(wc -l <"$TMP_DIR/events.log" | tr -d ' ')"
[[ "$line_count" == "1" ]] || {
  echo "disabled observability still wrote events" >&2
  exit 1
}

echo "PASS test_control_plane_observability"

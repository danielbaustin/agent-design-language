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

TOOLING_BIN="$ROOT_DIR/adl/target/debug/adl"
if [[ ! -x "$TOOLING_BIN" ]]; then
  TOOLING_BIN=""
fi

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

(
  export ADL_OBSERVABILITY=0
  adl_obs_event "pr.sh" "disabled" "started"
)
line_count="$(wc -l <"$TMP_DIR/events.log" | tr -d ' ')"
[[ "$line_count" == "1" ]] || {
  echo "disabled observability still wrote events" >&2
  exit 1
}

stderr_capture="$TMP_DIR/stderr.log"
(
  export ADL_OBSERVABILITY_STDERR=0
  adl_obs_event "pr.sh" "json_mode" "started" \
    "artifact_ref" "$ROOT_DIR/docs/example.md"
) 2>"$stderr_capture"
[[ ! -s "$stderr_capture" ]] || {
  echo "stderr suppression still wrote terminal output" >&2
  exit 1
}
line_count="$(wc -l <"$TMP_DIR/events.log" | tr -d ' ')"
[[ "$line_count" == "2" ]] || {
  echo "stderr suppression did not preserve durable logging" >&2
  exit 1
}
grep -Fq "stage=json_mode" "$TMP_DIR/events.log" || {
  echo "json-mode event missing from durable log" >&2
  exit 1
}

bad_sink_parent="$TMP_DIR/not-a-dir"
bad_sink="$bad_sink_parent/events.log"
printf 'occupied\n' >"$bad_sink_parent"
bad_stderr="$TMP_DIR/bad-sink-stderr.log"
bad_stdout="$TMP_DIR/bad-sink-stdout.log"
validate_cmd=(
  tooling validate-structured-prompt
  --type sor
  --phase bootstrap
  --input "$ROOT_DIR/.adl/v0.91.5/tasks/issue-3838__v0-91-5-toolkit-simplification-9-9-decompose-remaining-tools-workflow-into-direct-small-binaries/sor.md"
)
if [[ -n "$TOOLING_BIN" ]]; then
  ADL_OBSERVABILITY_STDERR=0 ADL_OBSERVABILITY_LOG="$bad_sink" \
    "$TOOLING_BIN" "${validate_cmd[@]}" >"$bad_stdout" 2>"$bad_stderr"
else
  ADL_OBSERVABILITY_STDERR=0 ADL_OBSERVABILITY_LOG="$bad_sink" \
    cargo run --manifest-path "$ROOT_DIR/adl/Cargo.toml" --quiet --bin adl -- \
    "${validate_cmd[@]}" >"$bad_stdout" 2>"$bad_stderr"
fi
grep -Fq "stage=compatibility_log" "$bad_stderr" || {
  echo "bad compatibility sink did not emit fallback failure signal" >&2
  cat "$bad_stderr" >&2
  exit 1
}
grep -Fq "result=failed" "$bad_stderr" || {
  echo "compatibility sink fallback line missing failed result" >&2
  cat "$bad_stderr" >&2
  exit 1
}
if grep -Fq "$TMP_DIR" "$bad_stderr"; then
  echo "compatibility sink fallback leaked absolute temp path" >&2
  cat "$bad_stderr" >&2
  exit 1
fi

echo "PASS test_control_plane_observability"

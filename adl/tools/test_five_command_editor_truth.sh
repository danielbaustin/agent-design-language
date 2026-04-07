#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
HELP_OUT="$(
  cd "$ROOT_DIR" &&
    bash adl/tools/pr.sh --help
)"

assert_contains() {
  local pattern="$1" text="$2" label="$3"
  grep -Fq "$pattern" <<<"$text" || {
    echo "assertion failed ($label): expected to find '$pattern'" >&2
    echo "actual output:" >&2
    echo "$text" >&2
    exit 1
  }
}

assert_contains "  create" "$HELP_OUT" "help lists pr create"
assert_contains "  init" "$HELP_OUT" "help lists pr init"
assert_contains "  run" "$HELP_OUT" "help lists pr run"
assert_contains "  finish" "$HELP_OUT" "help lists pr finish"
assert_contains 'pr start <issue> ...` remains only as a legacy alias' "$HELP_OUT" "help documents pr start as legacy alias"

grep -Fq '| `pr create` | yes | no | control-plane only |' "$ROOT_DIR/docs/tooling/editor/command_adapter.md" || {
  echo "assertion failed: command_adapter.md must keep pr create control-plane only" >&2
  exit 1
}
grep -Fq '| `pr init` | yes | no | control-plane only |' "$ROOT_DIR/docs/tooling/editor/command_adapter.md" || {
  echo "assertion failed: command_adapter.md must keep pr init control-plane only" >&2
  exit 1
}
grep -Fq '| `pr start` | yes | yes, via `adl/tools/editor_action.sh start` | supported thin adapter |' "$ROOT_DIR/docs/tooling/editor/command_adapter.md" || {
  echo "assertion failed: command_adapter.md must keep pr start as the only browser-direct adapter surface" >&2
  exit 1
}
grep -Fq '| `pr run` | yes | no | control-plane only |' "$ROOT_DIR/docs/tooling/editor/command_adapter.md" || {
  echo "assertion failed: command_adapter.md must keep pr run control-plane only" >&2
  exit 1
}
grep -Fq '| `pr finish` | yes | no | control-plane only |' "$ROOT_DIR/docs/tooling/editor/command_adapter.md" || {
  echo "assertion failed: command_adapter.md must keep pr finish control-plane only" >&2
  exit 1
}

grep -Fq -- '- `pr create` is not launched from the browser in this slice' "$ROOT_DIR/docs/tooling/editor/demo.md" || {
  echo "assertion failed: editor demo must keep pr create out of browser scope" >&2
  exit 1
}
grep -Fq -- '- `pr init` is not launched from the browser in this slice' "$ROOT_DIR/docs/tooling/editor/demo.md" || {
  echo "assertion failed: editor demo must keep pr init out of browser scope" >&2
  exit 1
}
grep -Fq -- '- `pr run` is not launched from the browser in this slice' "$ROOT_DIR/docs/tooling/editor/demo.md" || {
  echo "assertion failed: editor demo must keep pr run out of browser scope" >&2
  exit 1
}
grep -Fq -- '- `pr finish` is not launched from the browser in this slice' "$ROOT_DIR/docs/tooling/editor/demo.md" || {
  echo "assertion failed: editor demo must keep pr finish out of browser scope" >&2
  exit 1
}
grep -Fq -- '- the broader lifecycle commands still run through the repo-local control plane' "$ROOT_DIR/docs/tooling/editor/five_command_demo.md" || {
  echo "assertion failed: five_command_demo.md must keep the control-plane boundary explicit" >&2
  exit 1
}

echo "editor truth checks: ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

pass() {
  echo "PASS: $1"
}

fail() {
  echo "FAIL: $1" >&2
  exit 1
}

out="$(bash adl/tools/editor_action.sh start --issue 938 --branch codex/938-v085-editor-control-plane-adapter --dry-run)"
[[ "$out" == "./adl/tools/pr.sh start 938 --slug v085-editor-control-plane-adapter" ]] || fail "dry-run should derive the slug from the branch"
pass "dry-run derives slug from branch"

out="$(bash adl/tools/editor_action.sh contract)"
[[ "$out" == *"editor_adapter_schema: editor.command_adapter.v1"* ]] || fail "contract text should include schema header"
[[ "$out" == *"maps_to: adl/tools/pr.sh start"* ]] || fail "contract text should map the adapter to pr start"
[[ "$out" == *"unsupported_browser_direct_actions:"* ]] || fail "contract text should list unsupported direct browser actions"
pass "contract text exposes the supported adapter surface"

out="$(bash adl/tools/editor_action.sh contract --format json)"
[[ "$out" == *"\"schema_version\": \"editor.command_adapter.v1\""* ]] || fail "contract json should include schema version"
[[ "$out" == *"\"unsupported_browser_direct_actions\""* ]] || fail "contract json should list unsupported direct browser actions"
pass "contract json exposes the supported adapter surface"

if bash adl/tools/editor_action.sh start --issue 938 --branch codex/939-v085-editor-review-flow-integration --dry-run >/dev/null 2>&1; then
  fail "mismatched issue and branch should fail"
fi
pass "mismatched issue and branch are rejected"

out="$(bash adl/tools/editor_action.sh start --issue 938 --branch codex/938-v085-editor-control-plane-adapter --slug v085-editor-control-plane-adapter --dry-run)"
[[ "$out" == "./adl/tools/pr.sh start 938 --slug v085-editor-control-plane-adapter" ]] || fail "explicit slug should still produce the same command"
pass "explicit slug stays consistent"

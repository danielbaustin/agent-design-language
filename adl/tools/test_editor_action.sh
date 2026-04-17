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

out="$(bash adl/tools/editor_action.sh prepare --phase run --issue 2053 --slug backlog-tools-refresh-web-task-editor-current-skills --version v0.90)"
[[ "$out" == "./adl/tools/pr.sh run 2053 --slug backlog-tools-refresh-web-task-editor-current-skills --version v0.90" ]] || fail "prepare should emit the current pr run command"
pass "prepare emits current pr run command"

out="$(bash adl/tools/editor_action.sh prepare --phase doctor-ready --issue 2053 --slug backlog-tools-refresh-web-task-editor-current-skills --version v0.90)"
[[ "$out" == "./adl/tools/pr.sh doctor 2053 --slug backlog-tools-refresh-web-task-editor-current-skills --version v0.90 --mode ready" ]] || fail "prepare should emit the ready doctor command"
pass "prepare emits ready doctor command"

out="$(bash adl/tools/editor_action.sh prepare --phase finish --issue 2053 --slug backlog-tools-refresh-web-task-editor-current-skills --version v0.90 --title "[v0.90][tools] Refresh editor" --paths "docs/tooling/editor/README.md")"
[[ "$out" == "./adl/tools/pr.sh finish 2053 --title '[v0.90][tools] Refresh editor' --paths 'docs/tooling/editor/README.md'" ]] || fail "prepare should shell-quote finish title and paths"
pass "prepare emits quoted finish command"

out="$(bash adl/tools/editor_action.sh contract)"
[[ "$out" == *"editor_adapter_schema: editor.command_adapter.v2"* ]] || fail "contract text should include schema header"
[[ "$out" == *"action: prepare"* ]] || fail "contract text should expose prepare as the supported action"
[[ "$out" == *"browser_prepared_human_run"* ]] || fail "contract text should state copy-only human-run mode"
[[ "$out" == *"unsupported_browser_direct_actions:"* ]] || fail "contract text should list unsupported direct browser actions"
pass "contract text exposes the supported adapter surface"

out="$(bash adl/tools/editor_action.sh contract --format json)"
[[ "$out" == *"\"schema_version\": \"editor.command_adapter.v2\""* ]] || fail "contract json should include schema version"
[[ "$out" == *"\"action\": \"prepare\""* ]] || fail "contract json should expose prepare as the supported action"
[[ "$out" == *"\"unsupported_browser_direct_actions\""* ]] || fail "contract json should list unsupported direct browser actions"
pass "contract json exposes the supported adapter surface"

out="$(bash adl/tools/editor_action.sh start --issue 938 --branch codex/938-v085-editor-control-plane-adapter --dry-run)"
[[ "$out" == "./adl/tools/pr.sh start 938 --slug v085-editor-control-plane-adapter" ]] || fail "legacy dry-run should derive the slug from the branch"
pass "legacy start dry-run remains compatible"

if bash adl/tools/editor_action.sh start --issue 938 --branch codex/939-v085-editor-review-flow-integration --dry-run >/dev/null 2>&1; then
  fail "mismatched issue and branch should fail"
fi
pass "mismatched issue and branch are rejected"

out="$(bash adl/tools/editor_action.sh start --issue 938 --branch codex/938-v085-editor-control-plane-adapter --slug v085-editor-control-plane-adapter --dry-run)"
[[ "$out" == "./adl/tools/pr.sh start 938 --slug v085-editor-control-plane-adapter" ]] || fail "legacy explicit slug should still produce the same command"
pass "legacy explicit slug stays consistent"

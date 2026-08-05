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
[[ "$out" == ".adl/bin/csdlc-v2/csdlc-bind --root <worktree> --request <bind-request.json>" ]] || fail "prepare should emit the current csdlc-bind command"
pass "prepare emits current csdlc-bind command"

out="$(bash adl/tools/editor_action.sh prepare --phase doctor-ready --issue 2053 --slug backlog-tools-refresh-web-task-editor-current-skills --version v0.90)"
[[ "$out" == ".adl/bin/csdlc-v2/csdlc-doctor --repo <repo> --issue 2053" ]] || fail "prepare should emit the ready csdlc-doctor command"
pass "prepare emits ready csdlc-doctor command"

out="$(bash adl/tools/editor_action.sh prepare --phase finish --issue 2053 --slug backlog-tools-refresh-web-task-editor-current-skills --version v0.90 --title "[v0.90][tools] Refresh editor" --paths "docs/tooling/editor/README.md")"
[[ "$out" == ".adl/bin/csdlc-v2/csdlc-validate --root <worktree> finalize --request <finalize-request.json>" ]] || fail "prepare should emit the finalize validation command"
pass "prepare emits finalize validation command"

out="$(bash adl/tools/editor_action.sh contract)"
[[ "$out" == *"editor_adapter_schema: editor.command_adapter.v2"* ]] || fail "contract text should include schema header"
[[ "$out" == *"action: prepare"* ]] || fail "contract text should expose prepare as the supported action"
[[ "$out" == *"browser_prepared_human_run"* ]] || fail "contract text should state copy-only human-run mode"
[[ "$out" == *"unsupported_browser_direct_actions:"* ]] || fail "contract text should list unsupported direct browser actions"
[[ "$out" == *"language_contract:"* ]] || fail "contract text should expose the editor language contract"
[[ "$out" == *"- run"* ]] || fail "contract text should include singular run in the language contract"
pass "contract text exposes the supported adapter surface"

out="$(bash adl/tools/editor_action.sh contract --format json)"
[[ "$out" == *"\"schema_version\": \"editor.command_adapter.v2\""* ]] || fail "contract json should include schema version"
[[ "$out" == *"\"action\": \"prepare\""* ]] || fail "contract json should expose prepare as the supported action"
[[ "$out" == *"\"unsupported_browser_direct_actions\""* ]] || fail "contract json should list unsupported direct browser actions"
[[ "$out" == *"\"language_contract\""* ]] || fail "contract json should expose the editor language contract"
[[ "$out" == *"\"run\""* ]] || fail "contract json should include singular run in the language contract"
pass "contract json exposes the supported adapter surface"

if bash adl/tools/editor_action.sh start --issue 938 --branch codex/938-v085-editor-control-plane-adapter --dry-run >/dev/null 2>&1; then
  fail "sunset start action should fail closed"
fi
pass "sunset start action is unavailable"

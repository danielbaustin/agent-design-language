#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/select_validation_lanes.sh"
TMP="$(mktemp -d)"
UNTRACKED_FIXTURE="$ROOT/docs/architecture/__selector_untracked_fixture__.md"
trap 'rm -rf "$TMP" "$UNTRACKED_FIXTURE"' EXIT

assert_has() {
  local file="$1"
  local needle="$2"
  if ! grep -F -- "$needle" "$file" >/dev/null; then
    echo "expected $file to contain: $needle" >&2
    echo "actual output:" >&2
    cat "$file" >&2
    exit 1
  fi
}

assert_not_has() {
  local file="$1"
  local needle="$2"
  if grep -F -- "$needle" "$file" >/dev/null; then
    echo "expected $file not to contain: $needle" >&2
    echo "actual output:" >&2
    cat "$file" >&2
    exit 1
  fi
}

docs_only="$TMP/docs-only.txt"
printf 'M\tdocs/milestones/v0.91.6/README.md\n' >"$docs_only"
bash "$SCRIPT" --changed-files "$docs_only" >"$TMP/docs.out"
assert_has "$TMP/docs.out" "aggregate_status=selected"
assert_has "$TMP/docs.out" "docs_diff_check status=selected"
assert_not_has "$TMP/docs.out" "rust_pr_fast"

prompt_template="$TMP/prompt-template.txt"
printf 'M\tdocs/templates/prompts/current.json\n' >"$prompt_template"
bash "$SCRIPT" --changed-files "$prompt_template" >"$TMP/prompt.out"
assert_has "$TMP/prompt.out" "prompt_template_contracts status=selected"
assert_not_has "$TMP/prompt.out" "docs_diff_check status=selected"

focused_rust="$TMP/focused-rust.txt"
printf 'M\tadl/src/runtime_v2/contract_schema.rs\n' >"$focused_rust"
bash "$SCRIPT" --changed-files "$focused_rust" >"$TMP/focused.out"
assert_has "$TMP/focused.out" "rust_pr_fast status=selected"
assert_has "$TMP/focused.out" "mode=focused"
assert_has "$TMP/focused.out" "filter_expression=test(contract_schema)"
assert_not_has "$TMP/focused.out" "runtime_owner_lane status=selected"

focused_rust_with_space="$TMP/focused rust paths.txt"
printf 'M\tadl/src/runtime_v2/contract_schema.rs\n' >"$focused_rust_with_space"
focused_rust_with_space_resolved="$(python3 - <<'PY' "$focused_rust_with_space"
from pathlib import Path
import sys

print(Path(sys.argv[1]).resolve())
PY
)"
bash "$SCRIPT" --changed-files "$focused_rust_with_space" >"$TMP/focused-space.out"
assert_has "$TMP/focused-space.out" "rust_pr_fast status=selected"
assert_has "$TMP/focused-space.out" "--changed-files '$focused_rust_with_space_resolved'"

shared_rust="$TMP/shared-rust.txt"
printf 'M\tadl/src/lib.rs\n' >"$shared_rust"
bash "$SCRIPT" --changed-files "$shared_rust" >"$TMP/shared.out"
assert_has "$TMP/shared.out" "aggregate_status=escalated"
assert_has "$TMP/shared.out" "rust_pr_fast status=escalated"

release_gate="$TMP/release-gate.txt"
printf 'M\t.github/workflows/ci.yaml\n' >"$release_gate"
bash "$SCRIPT" --changed-files "$release_gate" >"$TMP/release.out"
assert_has "$TMP/release.out" "aggregate_status=release_gate_required"
assert_has "$TMP/release.out" "release_gate_review status=release_gate_required"
assert_has "$TMP/release.out" "ci_path_policy_contracts status=selected"

bash "$SCRIPT" --changed-files "$focused_rust" --json >"$TMP/focused.json"
python3 - <<'PY' "$TMP/focused.json"
import json
import sys

plan = json.load(open(sys.argv[1]))
assert plan["schema_version"] == "adl.validation_lane_plan.v1"
assert plan["lanes"]["rust_pr_fast"]["mode"] == "focused"
assert plan["pr_publication_sufficient"] is True
PY

report="$TMP/report.json"
bash "$SCRIPT" --changed-files "$focused_rust" --json --report-out "$report" >/dev/null
python3 - <<'PY' "$report"
import json
import sys

plan = json.load(open(sys.argv[1]))
assert plan["lanes"]["rust_pr_fast"]["status"] == "selected"
PY

if bash "$SCRIPT" --changed-files "$shared_rust" --run >"$TMP/refuse.out" 2>"$TMP/refuse.err"; then
  echo "expected --run to refuse an escalated plan" >&2
  exit 1
fi
assert_has "$TMP/refuse.err" "refusing --run because the plan is not fully selected"

printf '# selector untracked fixture\n' >"$UNTRACKED_FIXTURE"
bash "$SCRIPT" --include-working-tree >"$TMP/include-working-tree.out"
assert_has "$TMP/include-working-tree.out" "path=docs/architecture/__selector_untracked_fixture__.md"

run_docs="$TMP/run-docs.txt"
printf 'M\tdocs/architecture/VALIDATION_LANE_SELECTOR.md\n' >"$run_docs"
bash "$SCRIPT" --changed-files "$run_docs" --run --report-out "$TMP/run-docs-report.json" >/dev/null
python3 - <<'PY' "$TMP/run-docs-report.json"
import json
import sys

plan = json.load(open(sys.argv[1]))
assert plan["run_status"] == "passed"
assert plan["lanes"]["docs_diff_check"]["run_status"] == "passed"
PY

echo "PASS test_select_validation_lanes"

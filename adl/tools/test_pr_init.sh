#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PR_SH_SRC="$ROOT_DIR/adl/tools/pr.sh"
CARD_PATHS_SRC="$ROOT_DIR/adl/tools/card_paths.sh"
PROMPT_LINT_SRC="$ROOT_DIR/adl/tools/lint_prompt_spec.sh"
PROMPT_VALIDATOR_SRC="$ROOT_DIR/adl/tools/validate_structured_prompt.sh"
STP_CONTRACT_SRC="$ROOT_DIR/adl/schemas/structured_task_prompt.contract.yaml"
SIP_CONTRACT_SRC="$ROOT_DIR/adl/schemas/structured_implementation_prompt.contract.yaml"
SOR_CONTRACT_SRC="$ROOT_DIR/adl/schemas/structured_output_record.contract.yaml"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

repo="$tmpdir/repo"
mkdir -p \
  "$repo/adl/tools" \
  "$repo/adl/schemas" \
  "$repo/.adl/issues/v0.85/bodies"

cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
cp "$PROMPT_LINT_SRC" "$repo/adl/tools/lint_prompt_spec.sh"
cp "$PROMPT_VALIDATOR_SRC" "$repo/adl/tools/validate_structured_prompt.sh"
cp "$STP_CONTRACT_SRC" "$repo/adl/schemas/structured_task_prompt.contract.yaml"
cp "$SIP_CONTRACT_SRC" "$repo/adl/schemas/structured_implementation_prompt.contract.yaml"
cp "$SOR_CONTRACT_SRC" "$repo/adl/schemas/structured_output_record.contract.yaml"
chmod +x "$repo/adl/tools/pr.sh" "$repo/adl/tools/lint_prompt_spec.sh" "$repo/adl/tools/validate_structured_prompt.sh"

cat >"$repo/.adl/issues/v0.85/bodies/issue-42-test-init.md" <<'EOF'
---
issue_card_schema: adl.issue.v1
wp: "authoring"
slug: "test-init"
title: "[v0.85][authoring] Test init"
labels:
  - "track:roadmap"
  - "version:v0.85"
issue_number: 42
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Sprint Test"
required_outcome_type:
  - "docs"
repo_inputs:
  - "adl/tools/pr.sh"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes: []
pr_start:
  enabled: true
  slug: "test-init"
---

# Issue Card

## Summary
x
## Goal
x
## Required Outcome
x
## Deliverables
x
## Acceptance Criteria
x
## Repo Inputs
x
## Dependencies
x
## Demo Expectations
x
## Non-goals
x
## Issue-Graph Notes
x
## Notes
x
## Tooling Notes
x
EOF

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  git add -A
  git commit -q -m "init"
)

assert_contains() {
  local pattern="$1" text="$2" label="$3"
  grep -Fq "$pattern" <<<"$text" || {
    echo "assertion failed ($label): expected to find '$pattern'" >&2
    echo "actual output:" >&2
    echo "$text" >&2
    exit 1
  }
}

(
  cd "$repo"

  out1="$("$BASH_BIN" adl/tools/pr.sh init 42 --slug test-init --no-fetch-issue --version v0.85)"
  assert_contains "STP      .adl/v0.85/tasks/issue-0042__test-init/stp.md" "$out1" "stp path"
  assert_contains "CONTRACT minimum v0.85 init = task-bundle directory + validated stp.md only" "$out1" "contract line"

  stp_path="$repo/.adl/v0.85/tasks/issue-0042__test-init/stp.md"
  [[ -f "$stp_path" ]] || {
    echo "assertion failed: expected stp.md to exist" >&2
    exit 1
  }
  [[ ! -e "$repo/.adl/v0.85/tasks/issue-0042__test-init/sip.md" ]] || {
    echo "assertion failed: sip.md should not be created by pr init" >&2
    exit 1
  }
  [[ ! -e "$repo/.adl/v0.85/tasks/issue-0042__test-init/sor.md" ]] || {
    echo "assertion failed: sor.md should not be created by pr init" >&2
    exit 1
  }
  cmp "$repo/.adl/issues/v0.85/bodies/issue-42-test-init.md" "$stp_path" >/dev/null || {
    echo "assertion failed: stp.md should match canonical source issue prompt" >&2
    exit 1
  }

  out2="$("$BASH_BIN" adl/tools/pr.sh init 42 --slug test-init --no-fetch-issue --version v0.85)"
  assert_contains "STP already exists" "$out2" "idempotent reuse"

  set +e
  bad="$("$BASH_BIN" adl/tools/pr.sh init 43 --slug missing --no-fetch-issue --version v0.85 2>&1)"
  status=$?
  set -e
  [[ "$status" -ne 0 ]] || {
    echo "assertion failed: missing source prompt should fail" >&2
    exit 1
  }
  assert_contains "canonical source issue prompt not found" "$bad" "missing source failure"
)

echo "pr.sh init minimal task-bundle initialization: ok"

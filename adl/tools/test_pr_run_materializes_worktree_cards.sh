#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
export ADL_TOOLING_MANIFEST_ROOT="$ROOT_DIR"
PR_SH_SRC="$ROOT_DIR/adl/tools/pr.sh"
PR_DELEGATE_SRC="$ROOT_DIR/adl/tools/pr_delegate.sh"
PR_USAGE_SRC="$ROOT_DIR/adl/tools/pr_usage.sh"
CARD_PATHS_SRC="$ROOT_DIR/adl/tools/card_paths.sh"
PROMPT_LINT_SRC="$ROOT_DIR/adl/tools/lint_prompt_spec.sh"
PROMPT_VALIDATOR_SRC="$ROOT_DIR/adl/tools/validate_structured_prompt.sh"
INPUT_TPL_SRC="$ROOT_DIR/adl/templates/cards/input_card_template.md"
OUTPUT_TPL_SRC="$ROOT_DIR/adl/templates/cards/output_card_template.md"
STP_CONTRACT_SRC="$ROOT_DIR/adl/schemas/structured_task_prompt.contract.yaml"
SIP_CONTRACT_SRC="$ROOT_DIR/adl/schemas/structured_implementation_prompt.contract.yaml"
SOR_CONTRACT_SRC="$ROOT_DIR/adl/schemas/structured_output_record.contract.yaml"
BASH_BIN="$(command -v bash)"
REAL_ADL_BIN="$ROOT_DIR/adl/target/debug/adl"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

if [[ ! -x "$REAL_ADL_BIN" ]]; then
  cargo build --manifest-path "$ROOT_DIR/adl/Cargo.toml" --bin adl >/dev/null
fi

origin="$tmpdir/origin.git"
repo="$tmpdir/repo"
mkdir -p "$repo/adl/tools" "$repo/adl/templates/cards" "$repo/adl/schemas"
cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$PR_DELEGATE_SRC" "$repo/adl/tools/pr_delegate.sh"
cp "$PR_USAGE_SRC" "$repo/adl/tools/pr_usage.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
cp "$PROMPT_LINT_SRC" "$repo/adl/tools/lint_prompt_spec.sh"
cp "$PROMPT_VALIDATOR_SRC" "$repo/adl/tools/validate_structured_prompt.sh"
cp "$INPUT_TPL_SRC" "$repo/adl/templates/cards/input_card_template.md"
cp "$OUTPUT_TPL_SRC" "$repo/adl/templates/cards/output_card_template.md"
cp "$STP_CONTRACT_SRC" "$repo/adl/schemas/structured_task_prompt.contract.yaml"
cp "$SIP_CONTRACT_SRC" "$repo/adl/schemas/structured_implementation_prompt.contract.yaml"
cp "$SOR_CONTRACT_SRC" "$repo/adl/schemas/structured_output_record.contract.yaml"
chmod +x "$repo/adl/tools/pr.sh" "$repo/adl/tools/lint_prompt_spec.sh" "$repo/adl/tools/validate_structured_prompt.sh"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  git add -A
  git commit -q -m "init"
  git branch -M main
  git init --bare -q "$origin"
  git remote add origin "$origin"
  git push -q -u origin main
  git fetch -q origin main
)

(
  cd "$repo"
  mkdir -p .adl/v0.86/bodies
  cat > .adl/v0.86/bodies/issue-910-validation-pass.md <<'EOF'
---
issue_card_schema: adl.issue.v1
wp: "unassigned"
queue: "tools"
slug: "validation-pass"
title: "[v0.86][tools] Validation pass"
labels:
  - "track:roadmap"
  - "type:task"
  - "area:tools"
  - "version:v0.86"
issue_number: 910
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Test sprint"
required_outcome_type:
  - "code"
repo_inputs:
  - "adl/tools/pr.sh"
canonical_files:
  - "adl/tools/pr.sh"
demo_required: false
demo_names: []
issue_graph_notes:
  - "Regression proof for worktree-local bundle materialization."
pr_start:
  enabled: false
  slug: "validation-pass"
---

## Summary

Regression proof for issue-mode run bundle materialization.

## Goal

Make the issue-mode run binder leave a complete local worktree issue surface.

## Required Outcome

- local issue prompt copy exists in the worktree
- local task bundle cards exist in the worktree

## Deliverables

- local issue prompt copy
- local `stp.md`, `sip.md`, `sor.md`, `spp.md`, and `srp.md`

## Acceptance Criteria

- `pr run <issue>` creates the local issue prompt copy
- `pr run <issue>` creates `stp.md`, `sip.md`, `sor.md`, `spp.md`, and `srp.md`

## Repo Inputs

- `adl/tools/pr.sh`

## Dependencies

None.

## Target Files / Surfaces

- issue-mode bind lifecycle

## Validation Plan

- run `adl/tools/pr.sh run 910 ...`

## Demo / Proof Requirements

None.

## Demo Expectations

None.

## Non-goals

- none

## Issue-Graph Notes

- shell regression only

## Notes

- authored issue prompt required

## Tooling Notes

- keep the proof focused
EOF
  export ADL_PR_RUST_BIN="$REAL_ADL_BIN"
  "$BASH_BIN" adl/tools/pr.sh run 910 --slug validation-pass --no-fetch-issue --version v0.86 --allow-open-pr-wave >/dev/null
  [[ -f ".worktrees/adl-wp-910/.adl/v0.86/bodies/issue-910-validation-pass.md" ]] || {
    echo "assertion failed: expected run to materialize worktree-local issue prompt" >&2
    exit 1
  }
  [[ -f ".worktrees/adl-wp-910/.adl/v0.86/tasks/issue-0910__validation-pass/stp.md" ]] || {
    echo "assertion failed: expected run to materialize worktree-local stp.md" >&2
    exit 1
  }
  [[ -f ".worktrees/adl-wp-910/.adl/v0.86/tasks/issue-0910__validation-pass/sip.md" ]] || {
    echo "assertion failed: expected run to materialize worktree-local sip.md" >&2
    exit 1
  }
  [[ -f ".worktrees/adl-wp-910/.adl/v0.86/tasks/issue-0910__validation-pass/sor.md" ]] || {
    echo "assertion failed: expected run to materialize worktree-local sor.md" >&2
    exit 1
  }
  [[ -f ".worktrees/adl-wp-910/.adl/v0.86/tasks/issue-0910__validation-pass/spp.md" ]] || {
    echo "assertion failed: expected run to materialize worktree-local spp.md" >&2
    exit 1
  }
  [[ -f ".worktrees/adl-wp-910/.adl/v0.86/tasks/issue-0910__validation-pass/srp.md" ]] || {
    echo "assertion failed: expected run to materialize worktree-local srp.md" >&2
    exit 1
  }
)

echo "pr.sh run issue-mode materializes worktree-local task bundle: ok"

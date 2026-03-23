#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PR_SH_SRC="$ROOT_DIR/adl/tools/pr.sh"
CARD_PATHS_SRC="$ROOT_DIR/adl/tools/card_paths.sh"
PROMPT_LINT_SRC="$ROOT_DIR/adl/tools/lint_prompt_spec.sh"
PROMPT_VALIDATOR_SRC="$ROOT_DIR/adl/tools/validate_structured_prompt.sh"
INPUT_TPL_SRC="$ROOT_DIR/adl/templates/cards/input_card_template.md"
OUTPUT_TPL_SRC="$ROOT_DIR/adl/templates/cards/output_card_template.md"
STP_CONTRACT_SRC="$ROOT_DIR/adl/schemas/structured_task_prompt.contract.yaml"
SIP_CONTRACT_SRC="$ROOT_DIR/adl/schemas/structured_implementation_prompt.contract.yaml"
SOR_CONTRACT_SRC="$ROOT_DIR/adl/schemas/structured_output_record.contract.yaml"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

origin="$tmpdir/origin.git"
repo="$tmpdir/repo"
bindir="$tmpdir/bin"
gh_log="$tmpdir/gh.log"
gh_body="$tmpdir/gh_body.md"
mkdir -p "$repo/adl/tools" "$repo/adl/templates/cards" "$repo/adl/schemas" "$repo/.adl/issues/v0.85/bodies" "$bindir"
cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
cp "$PROMPT_LINT_SRC" "$repo/adl/tools/lint_prompt_spec.sh"
cp "$PROMPT_VALIDATOR_SRC" "$repo/adl/tools/validate_structured_prompt.sh"
cp "$INPUT_TPL_SRC" "$repo/adl/templates/cards/input_card_template.md"
cp "$OUTPUT_TPL_SRC" "$repo/adl/templates/cards/output_card_template.md"
cp "$STP_CONTRACT_SRC" "$repo/adl/schemas/structured_task_prompt.contract.yaml"
cp "$SIP_CONTRACT_SRC" "$repo/adl/schemas/structured_implementation_prompt.contract.yaml"
cp "$SOR_CONTRACT_SRC" "$repo/adl/schemas/structured_output_record.contract.yaml"
chmod +x "$repo/adl/tools/pr.sh" "$repo/adl/tools/lint_prompt_spec.sh" "$repo/adl/tools/validate_structured_prompt.sh"

cat >"$repo/.adl/issues/v0.85/bodies/issue-42-test-reconcile.md" <<'EOF'
---
issue_card_schema: adl.issue.v1
wp: "authoring"
slug: "test-reconcile"
title: "[v0.85][authoring] Reconciled issue title"
labels:
  - "track:roadmap"
  - "version:v0.85"
  - "area:tools"
issue_number: 42
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Sprint Test"
required_outcome_type:
  - "code"
repo_inputs:
  - "adl/tools/pr.sh"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes: []
pr_start:
  enabled: true
  slug: "test-reconcile"
---

# Issue Card

## Summary
Reconcile the issue body from the canonical STP.
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

cat >"$bindir/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
LOG_FILE="${GH_LOG_FILE:?}"
BODY_FILE_CAPTURE="${GH_BODY_FILE_CAPTURE:?}"
printf '%s\n' "$*" >>"$LOG_FILE"

if [[ "${1:-}" == "issue" && "${2:-}" == "create" ]]; then
  echo "https://github.com/example/repo/issues/1042"
  exit 0
fi

if [[ "${1:-}" == "issue" && "${2:-}" == "view" ]]; then
  issue="${3:-}"
  shift 3
  if [[ "$issue" == "42" && "$*" == *"--json labels"* && "$*" == *"-q .labels[].name"* ]]; then
    printf '%s\n' "track:roadmap" "old:remove-me"
    exit 0
  fi
fi

if [[ "${1:-}" == "issue" && "${2:-}" == "edit" ]]; then
  shift 2
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --body-file)
        cp "$2" "$BODY_FILE_CAPTURE"
        shift 2
        ;;
      *)
        shift
        ;;
    esac
  done
  exit 0
fi

exit 1
EOF
chmod +x "$bindir/gh"

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
  export PATH="$bindir:$PATH"
  export GH_LOG_FILE="$gh_log"
  export GH_BODY_FILE_CAPTURE="$gh_body"

  out_create="$("$BASH_BIN" adl/tools/pr.sh create \
    --title "[v0.85][authoring] Transitional create path" \
    --slug transitional-create-path \
    --version v0.85 \
    --body "test body" \
    --no-start)"

  assert_contains "ISSUE_NUM=1042" "$out_create" "create path issue number"
  grep -Fq "issue create" "$gh_log" || {
    echo "assertion failed: expected create path to call gh issue create" >&2
    exit 1
  }

  : >"$gh_log"
  out_reconcile="$("$BASH_BIN" adl/tools/pr.sh create 42 --stp .adl/issues/v0.85/bodies/issue-42-test-reconcile.md)"
  assert_contains "ISSUE_NUM=42" "$out_reconcile" "reconcile issue number"
  assert_contains "MODE=reconcile" "$out_reconcile" "reconcile mode marker"
  grep -Fq -- 'issue edit 42' "$gh_log" || {
    echo "assertion failed: expected reconcile path to call gh issue edit" >&2
    exit 1
  }
  grep -Fq -- '--add-label version:v0.85' "$gh_log" || {
    echo "assertion failed: expected reconcile path to add missing STP label" >&2
    exit 1
  }
  grep -Fq -- '--add-label area:tools' "$gh_log" || {
    echo "assertion failed: expected reconcile path to add missing STP label" >&2
    exit 1
  }
  grep -Fq -- '--remove-label old:remove-me' "$gh_log" || {
    echo "assertion failed: expected reconcile path to remove stale label" >&2
    exit 1
  }
  grep -Fq "## Summary" "$gh_body" || {
    echo "assertion failed: expected reconcile body file to contain STP markdown body" >&2
    exit 1
  }
)

echo "pr.sh create create+reconcile flows: ok"

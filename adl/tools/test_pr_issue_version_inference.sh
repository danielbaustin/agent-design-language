#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
export ADL_TOOLING_MANIFEST_ROOT="$ROOT_DIR"
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
REAL_ADL_BIN="$ROOT_DIR/adl/target/debug/adl"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

if [[ ! -x "$REAL_ADL_BIN" ]]; then
  cargo build --manifest-path "$ROOT_DIR/adl/Cargo.toml" --bin adl >/dev/null
fi

origin="$tmpdir/origin.git"
repo="$tmpdir/repo"
bindir="$tmpdir/bin"
gh_log="$tmpdir/gh.log"
mkdir -p "$repo/adl/tools" "$repo/adl/templates/cards" "$repo/adl/schemas" "$bindir"
cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
cp "$PROMPT_LINT_SRC" "$repo/adl/tools/lint_prompt_spec.sh"
cp "$PROMPT_VALIDATOR_SRC" "$repo/adl/tools/validate_structured_prompt.sh"
cp "$INPUT_TPL_SRC" "$repo/adl/templates/cards/input_card_template.md"
cp "$OUTPUT_TPL_SRC" "$repo/adl/templates/cards/output_card_template.md"
cp "$STP_CONTRACT_SRC" "$repo/adl/schemas/structured_task_prompt.contract.yaml"
cp "$SIP_CONTRACT_SRC" "$repo/adl/schemas/structured_implementation_prompt.contract.yaml"
cp "$SOR_CONTRACT_SRC" "$repo/adl/schemas/structured_output_record.contract.yaml"
chmod +x "$repo/adl/tools/pr.sh"
chmod +x "$repo/adl/tools/lint_prompt_spec.sh" "$repo/adl/tools/validate_structured_prompt.sh"

cat >"$bindir/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
LOG_FILE="${GH_LOG_FILE:?}"
STATE_DIR="$(dirname "$LOG_FILE")"
TITLE_976_FILE="$STATE_DIR/issue-976.title"
LABELS_976_FILE="$STATE_DIR/issue-976.labels"
printf '%s\n' "$*" >>"$LOG_FILE"
read_labels() {
  local issue="$1"
  if [[ "$issue" == "975" ]]; then
    printf 'track:roadmap\ntype:task\narea:tools\nversion:v0.85\n'
    return 0
  fi
  if [[ "$issue" == "976" ]]; then
    if [[ -f "$LABELS_976_FILE" ]]; then
      cat "$LABELS_976_FILE"
    fi
    return 0
  fi
  return 1
}
read_title() {
  local issue="$1"
  if [[ "$issue" == "975" ]]; then
    printf '[v0.85][process] Infer current milestone card version from issue title when labels are missing\n'
    return 0
  fi
  if [[ "$issue" == "976" ]]; then
    if [[ -f "$TITLE_976_FILE" ]]; then
      cat "$TITLE_976_FILE"
    else
      printf '[v0.87.1][process] Infer dot suffixed milestone card version from issue title when labels are missing\n'
    fi
    return 0
  fi
  return 1
}
if [[ "${1:-}" == "issue" && "${2:-}" == "view" ]]; then
  issue="${3:-}"
  shift 3
  if [[ "$issue" != "975" && "$issue" != "976" ]]; then
    exit 1
  fi
  if [[ "$*" == *"--json labels"* && "$*" == *"-q .labels[].name"* ]]; then
    read_labels "$issue"
    exit 0
  fi
  if [[ "$*" == *"--json title"* && "$*" == *"-q .title"* ]]; then
    read_title "$issue"
    exit 0
  fi
fi
if [[ "${1:-}" == "issue" && "${2:-}" == "edit" ]]; then
  issue="${3:-}"
  shift 3
  if [[ "$issue" == "976" && "$*" == *"--add-label"* ]]; then
    cat <<'LABELS' >"$LABELS_976_FILE"
track:roadmap
type:task
area:tools
version:v0.87.1
LABELS
    exit 0
  fi
  if [[ "$issue" == "976" && "$*" == *"--title"* ]]; then
    printf '%s\n' '[v0.87.1][process] Infer dot suffixed milestone card version from issue title when labels are missing' >"$TITLE_976_FILE"
    exit 0
  fi
fi
exit 1
EOF
chmod +x "$bindir/gh"

canon_path() {
  local p="$1"
  mkdir -p "$p"
  (cd "$p" && pwd -P)
}

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

write_authored_source_prompt() {
  local path="$1" issue="$2" version="$3" slug="$4" title="$5"
  mkdir -p "$(dirname "$path")"
  cat >"$path" <<EOF
---
issue_card_schema: adl.issue.v1
wp: "process"
queue: "tools"
slug: "$slug"
title: "$title"
labels:
  - "track:roadmap"
  - "area:tools"
  - "version:$version"
issue_number: $issue
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Test sprint"
required_outcome_type:
  - "tests"
repo_inputs:
  - "adl/tools/pr.sh"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Test fixture for version inference lifecycle coverage."
pr_start:
  enabled: false
  slug: "$slug"
---

# Issue Card

## Summary
Test fixture for version inference lifecycle coverage.

## Goal
Verify issue version inference across init and start.

## Required Outcome
The lifecycle commands keep the inferred version in the task bundle paths.

## Deliverables
- version-specific task bundle surfaces

## Acceptance Criteria
- init writes the expected versioned root bundle
- start writes the expected versioned worktree bundle

## Repo Inputs
- adl/tools/pr.sh

## Dependencies
- none

## Demo Expectations
No demo required; tooling lifecycle fixture only.

## Non-goals
- production issue creation

## Issue-Graph Notes
- none

## Notes
- none

## Tooling Notes
- authored fixture avoids bootstrap-stub rejection during start.
EOF
}

(
  cd "$repo"
  export PATH="$bindir:$PATH"
  export GH_LOG_FILE="$gh_log"
  export ADL_PR_RUST_BIN="$REAL_ADL_BIN"

  write_authored_source_prompt \
    ".adl/v0.85/bodies/issue-975-v085-process-infer-card-version-from-issue-title.md" \
    975 \
    "v0.85" \
    "v085-process-infer-card-version-from-issue-title" \
    "[v0.85][process] Infer current milestone card version from issue title when labels are missing"
  out_init="$("$BASH_BIN" adl/tools/pr.sh init 975 --slug v085-process-infer-card-version-from-issue-title)"
  assert_contains "STATE    ISSUE_AND_BUNDLE_READY" "$out_init" "init ready state"
  [[ -f ".adl/v0.85/tasks/issue-0975__v085-process-infer-card-version-from-issue-title/sip.md" ]] || {
    echo "assertion failed: expected canonical input card under the root .adl/v0.85/tasks" >&2
    exit 1
  }
  [[ -f ".adl/v0.85/tasks/issue-0975__v085-process-infer-card-version-from-issue-title/sor.md" ]] || {
    echo "assertion failed: expected canonical output card under the root .adl/v0.85/tasks" >&2
    exit 1
  }
  grep -Fq "Version: v0.85" ".adl/v0.85/tasks/issue-0975__v085-process-infer-card-version-from-issue-title/sip.md" || {
    echo "assertion failed: expected input card version v0.85" >&2
    exit 1
  }
  grep -Fq "Branch: not bound yet" ".adl/v0.85/tasks/issue-0975__v085-process-infer-card-version-from-issue-title/sip.md" || {
    echo "assertion failed: expected init input card to stay pre-run/unbound" >&2
    exit 1
  }
  ! grep -Fq "the branch and worktree already exist" ".adl/v0.85/tasks/issue-0975__v085-process-infer-card-version-from-issue-title/sip.md" || {
    echo "assertion failed: init input card must not imply branch/worktree exists" >&2
    exit 1
  }
  grep -Fq "Title: [v0.85][process] Infer current milestone card version from issue title when labels are missing" \
    ".adl/v0.85/tasks/issue-0975__v085-process-infer-card-version-from-issue-title/sip.md" || {
    echo "assertion failed: expected preserved issue title in input card" >&2
    exit 1
  }
  if grep -Fq "v0.3" "$gh_log"; then
    echo "assertion failed: unexpected v0.3 inference in gh issue view path" >&2
    exit 1
  fi

  out_start="$("$BASH_BIN" adl/tools/pr.sh start 975 --slug v085-process-infer-card-version-from-issue-title)"
  assert_contains "WORKTREE $(canon_path "$repo/.worktrees/adl-wp-975")" "$out_start" "start prints worktree path"
  [[ -f "$repo/.worktrees/adl-wp-975/.adl/v0.85/tasks/issue-0975__v085-process-infer-card-version-from-issue-title/sip.md" ]] || {
    echo "assertion failed: expected start to create input card inside worktree-local v0.85 task bundle" >&2
    exit 1
  }
  grep -Fq "Branch: codex/975-v085-process-infer-card-version-from-issue-title" \
    "$repo/.worktrees/adl-wp-975/.adl/v0.85/tasks/issue-0975__v085-process-infer-card-version-from-issue-title/sip.md" || {
    echo "assertion failed: expected start input card to be run-bound" >&2
    exit 1
  }
  grep -Fq "the branch and worktree already exist" \
    "$repo/.worktrees/adl-wp-975/.adl/v0.85/tasks/issue-0975__v085-process-infer-card-version-from-issue-title/sip.md" || {
    echo "assertion failed: expected start input card to use run-bound wording" >&2
    exit 1
  }
  [[ -f "$repo/.worktrees/adl-wp-975/.adl/v0.85/tasks/issue-0975__v085-process-infer-card-version-from-issue-title/sor.md" ]] || {
    echo "assertion failed: expected start to create output card inside worktree-local v0.85 task bundle" >&2
    exit 1
  }
  [[ ! -e "$repo/.adl/v0.3/tasks/issue-0975__v085-process-infer-card-version-from-issue-title" ]] || {
    echo "assertion failed: unexpected v0.3 fallback task bundle after start" >&2
    exit 1
  }

  write_authored_source_prompt \
    ".adl/v0.87.1/bodies/issue-976-v0871-process-infer-dot-suffixed-version-from-title.md" \
    976 \
    "v0.87.1" \
    "v0871-process-infer-dot-suffixed-version-from-title" \
    "[v0.87.1][process] Infer dot suffixed milestone card version from issue title when labels are missing"
  out_init_dot="$("$BASH_BIN" adl/tools/pr.sh init 976 --slug v0871-process-infer-dot-suffixed-version-from-title)"
  assert_contains "STATE    ISSUE_AND_BUNDLE_READY" "$out_init_dot" "dot-suffixed init ready state"
  [[ -f ".adl/v0.87.1/tasks/issue-0976__v0871-process-infer-dot-suffixed-version-from-title/sip.md" ]] || {
    echo "assertion failed: expected canonical input card under the root .adl/v0.87.1/tasks" >&2
    exit 1
  }
  grep -Fq "Version: v0.87.1" ".adl/v0.87.1/tasks/issue-0976__v0871-process-infer-dot-suffixed-version-from-title/sip.md" || {
    echo "assertion failed: expected input card version v0.87.1" >&2
    exit 1
  }
  grep -Fq "Branch: not bound yet" ".adl/v0.87.1/tasks/issue-0976__v0871-process-infer-dot-suffixed-version-from-title/sip.md" || {
    echo "assertion failed: expected dot-suffixed init input card to stay pre-run/unbound" >&2
    exit 1
  }

  out_start_dot="$("$BASH_BIN" adl/tools/pr.sh start 976 --slug v0871-process-infer-dot-suffixed-version-from-title)"
  assert_contains "WORKTREE $(canon_path "$repo/.worktrees/adl-wp-976")" "$out_start_dot" "dot-suffixed start prints worktree path"
  [[ -f "$repo/.worktrees/adl-wp-976/.adl/v0.87.1/tasks/issue-0976__v0871-process-infer-dot-suffixed-version-from-title/sip.md" ]] || {
    echo "assertion failed: expected start to create input card inside worktree-local v0.87.1 task bundle" >&2
    exit 1
  }
  [[ ! -e "$repo/.adl/v0.3/tasks/issue-0976__v0871-process-infer-dot-suffixed-version-from-title" ]] || {
    echo "assertion failed: unexpected v0.3 fallback task bundle for dot-suffixed version after start" >&2
    exit 1
  }

)

echo "pr.sh init/start title+version inference: ok"

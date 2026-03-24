#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PR_SH_SRC="$ROOT_DIR/adl/tools/pr.sh"
CARD_PATHS_SRC="$ROOT_DIR/adl/tools/card_paths.sh"
INPUT_TPL_SRC="$ROOT_DIR/adl/templates/cards/input_card_template.md"
OUTPUT_TPL_SRC="$ROOT_DIR/adl/templates/cards/output_card_template.md"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

repo="$tmpdir/repo"
mkdir -p "$repo/adl/tools" "$repo/adl/templates/cards" "$repo/.adl/issues/v0.85/bodies"
cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
cp "$INPUT_TPL_SRC" "$repo/adl/templates/cards/input_card_template.md"
cp "$OUTPUT_TPL_SRC" "$repo/adl/templates/cards/output_card_template.md"
chmod +x "$repo/adl/tools/pr.sh"

cat >"$repo/.adl/issues/v0.85/bodies/issue-49-v085-context-defaults.md" <<'EOF'
---
issue_card_schema: adl.issue.v1
wp: "authoring"
slug: "v085-context-defaults"
title: "[v0.85][authoring] Context defaults"
labels:
  - "track:roadmap"
  - "area:tools"
  - "type:bug"
  - "version:v0.85"
issue_number: 49
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "Test"
required_outcome_type:
  - "code"
repo_inputs:
  - "docs/tooling/prompt-spec.md"
  - "adl/tools/pr.sh"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Test fixture"
pr_start:
  enabled: true
  slug: "v085-context-defaults"
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

no_gh_bin="$tmpdir/no-gh-bin"
mkdir -p "$no_gh_bin"
for cmd in awk basename cat cp cut dirname git grep head ln mkdir mktemp mv pwd readlink rm sed touch tr python3; do
  cmd_path="$(command -v "$cmd")"
  ln -s "$cmd_path" "$no_gh_bin/$cmd"
done

assert_has() {
  local pattern="$1" file="$2"
  grep -Fq -- "$pattern" "$file" || {
    echo "assertion failed: expected '$pattern' in $file" >&2
    exit 1
  }
}

(
  cd "$repo"
  PATH="$no_gh_bin" "$BASH_BIN" adl/tools/pr.sh card 49 --no-fetch-issue --slug v085-context-defaults --version v0.85 >/dev/null
  card=".adl/v0.85/tasks/issue-0049__v085-context-defaults/sip.md"
  assert_has "- Source Issue Prompt: .adl/issues/v0.85/bodies/issue-49-v085-context-defaults.md" "$card"
  assert_has "- Docs: docs/tooling/prompt-spec.md" "$card"
  assert_has "- Other: none" "$card"
)

echo "pr.sh cards context defaults: ok"

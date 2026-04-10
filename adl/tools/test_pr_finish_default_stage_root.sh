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
mockbin="$tmpdir/mockbin"
mkdir -p "$repo/adl/tools" "$repo/adl/templates/cards" "$repo/adl/schemas" "$repo/docs/tooling" "$mockbin"
cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
cp "$PROMPT_LINT_SRC" "$repo/adl/tools/lint_prompt_spec.sh"
cp "$PROMPT_VALIDATOR_SRC" "$repo/adl/tools/validate_structured_prompt.sh"
cp "$INPUT_TPL_SRC" "$repo/adl/templates/cards/input_card_template.md"
cp "$OUTPUT_TPL_SRC" "$repo/adl/templates/cards/output_card_template.md"
cp "$STP_CONTRACT_SRC" "$repo/adl/schemas/structured_task_prompt.contract.yaml"
cp "$SIP_CONTRACT_SRC" "$repo/adl/schemas/structured_implementation_prompt.contract.yaml"
cp "$SOR_CONTRACT_SRC" "$repo/adl/schemas/structured_output_record.contract.yaml"
cat > "$repo/adl/tools/README.md" <<'EOF_README'
seed tooling readme
EOF_README
cat > "$repo/docs/tooling/README.md" <<'EOF_DOC'
seed docs readme
EOF_DOC
chmod +x "$repo/adl/tools/pr.sh" "$repo/adl/tools/lint_prompt_spec.sh" "$repo/adl/tools/validate_structured_prompt.sh"

cat >"$mockbin/gh" <<'EOF_GH'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1" == "repo" && "$2" == "view" ]]; then
  echo "local/repo"
  exit 0
fi
if [[ "$1" == "pr" && "$2" == "list" ]]; then
  if [[ " $* " == *" -q "* ]]; then
    echo ""
  else
    echo "[]"
  fi
  exit 0
fi
if [[ "$1" == "pr" && "$2" == "edit" ]]; then
  body_file=""
  while [[ $# -gt 0 ]]; do
    if [[ "$1" == "--body-file" ]]; then
      body_file="$2"
      shift 2
    else
      shift
    fi
  done
  [[ -n "$body_file" ]] && cp "$body_file" "$TMP_PR_BODY"
  exit 0
fi
if [[ "$1" == "pr" && "$2" == "view" ]]; then
  if [[ " $* " == *" --json closingIssuesReferences "* ]]; then
    if [[ " $* " == *" -q "* ]]; then
      echo '1021'
    else
      echo '{"closingIssuesReferences":[{"number":1021}]}'
    fi
    exit 0
  fi
  if [[ " $* " == *" --json body "* ]]; then
    cat "$TMP_PR_BODY"
    exit 0
  fi
fi
if [[ "$1" == "pr" && "$2" == "create" ]]; then
  body_file=""
  while [[ $# -gt 0 ]]; do
    if [[ "$1" == "--body-file" ]]; then
      body_file="$2"
      shift 2
    else
      shift
    fi
  done
  [[ -n "$body_file" ]] && cp "$body_file" "$TMP_PR_BODY"
  echo "https://example.test/pr/1"
  exit 0
fi
if [[ "$1" == "issue" && "$2" == "create" ]]; then
  echo "https://example.test/issues/1"
  exit 0
fi
exit 0
EOF_GH
chmod +x "$mockbin/gh"

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

export PATH="$mockbin:$PATH"
TMP_PR_BODY="$tmpdir/pr_body.md"
export TMP_PR_BODY
export ADL_PR_RUST_BIN="$REAL_ADL_BIN"

(
  cd "$repo"

  mkdir -p .adl/v0.86/bodies
  cat > .adl/v0.86/bodies/issue-1021-finish-default-root.md <<'EOF_PROMPT'
---
issue_card_schema: adl.issue.v1
wp: "unassigned"
slug: "finish-default-root"
title: "[v0.85][authoring] Harden pr finish command behavior"
labels:
  - "track:roadmap"
  - "area:tools"
  - "type:task"
  - "version:v0.86"
issue_number: 1021
status: "active"
action: "edit"
depends_on: []
milestone_sprint: "unplanned"
required_outcome_type:
  - "code"
repo_inputs:
  - "https://github.com/example/repo/issues/1021"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Authored shell test fixture."
pr_start:
  enabled: true
  slug: "finish-default-root"
---

# [v0.85][authoring] Harden pr finish command behavior

## Summary

Authored prompt for finish-shell regression coverage.

## Goal

Provide authored prompt content so lifecycle setup can proceed.

## Required Outcome

This test issue ships code only.

## Deliverables

- authored issue prompt content

## Acceptance Criteria

- lifecycle setup accepts this prompt

## Repo Inputs

- https://github.com/example/repo/issues/1021

## Dependencies

- none

## Demo Expectations

- none

## Non-goals

- bootstrap placeholder content

## Issue-Graph Notes

- shell regression fixture

## Notes

- generated inside shell tests

## Tooling Notes

- authored fixture, not bootstrap fallback
EOF_PROMPT

  "$BASH_BIN" adl/tools/pr.sh start 1021 --slug finish-default-root --no-fetch-issue >/dev/null
  "$BASH_BIN" adl/tools/pr.sh cards 1021 --no-fetch-issue >/dev/null
  worktree="$repo/.worktrees/adl-wp-1021"
  git -C "$worktree" config user.name "Test User"
  git -C "$worktree" config user.email "test@example.com"
  mkdir -p .adl/cards/1021

  cat > .adl/cards/1021/output_1021.md <<'EOF_SOR'
# finish-default-root

Task ID: issue-1021
Run ID: issue-1021
Version: v0.3
Title: finish-default-root
Branch: codex/1021-finish-default-root
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5-codex
- Provider: local test harness
- Start Time: 2026-03-20T00:00:00Z
- End Time: 2026-03-20T00:01:00Z

## Summary
Finish default root stages both docs and code paths.
## Artifacts produced
- adl/tools/README.md
- docs/tooling/README.md
## Actions taken
- Updated one code path and one docs path.
## Main Repo Integration (REQUIRED)
- Tracked paths prepared for main-repo integration:
  - `adl/tools/README.md`
  - `docs/tooling/README.md`
- Worktree-only paths remaining:
  - `adl/tools/README.md`
  - `docs/tooling/README.md`
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: branch-local tracked edits committed and prepared for PR
- Verification performed:
  - `git diff -- adl/tools/README.md docs/tooling/README.md`
    - verifies the tracked changes intended for the PR.
- Result: PASS
## Validation
- Validation commands and their purpose:
  - `bash adl/tools/validate_structured_prompt.sh --type sor --phase completed --input .adl/cards/1021/output_1021.md`
    - verifies this completed execution record remains structurally valid.
- Results:
  - all listed commands passed
## Verification Summary
```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "completed SOR validation"
  determinism:
    status: PASS
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: PASS
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: PASS
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: not_applicable
```
## Determinism Evidence
- Determinism tests executed: completed SOR validation
- Replay verification (same inputs -> same artifacts/order): yes
- Ordering guarantees (sorting / tie-break rules used): stable tracked path set
- Artifact stability notes: default finish staging should include both docs and code edits
## Security / Privacy Checks
- Secret leakage scan performed: manual inspection
- Prompt / tool argument redaction verified: yes
- Absolute path leakage check: repo-relative references only
- Sandbox / policy invariants preserved: yes
## Replay Artifacts
- Trace bundle path(s): not applicable
- Run artifact root: not applicable
- Replay command used for verification: not applicable
- Replay result: not applicable
## Artifact Verification
- Primary proof surface: `.adl/cards/1021/output_1021.md`
- Required artifacts present: true
- Artifact schema/version checks: completed-phase SOR validation passed
- Hash/byte-stability checks: not performed
- Missing/optional artifacts and rationale: no runtime trace required for this tooling issue
## Decisions / Deviations
- none
## Follow-ups / Deferred work
- none
EOF_SOR

  echo "code path changed" >> "$worktree/adl/tools/README.md"
  echo "docs path changed" >> "$worktree/docs/tooling/README.md"

  (
    cd "$worktree"
    "$BASH_BIN" adl/tools/pr.sh finish 1021 --title "[v0.85][authoring] Harden pr finish command behavior" -f "$repo/.adl/cards/1021/input_1021.md" --output-card "$repo/.adl/cards/1021/output_1021.md" --no-checks --no-open >/dev/null
  )

  body="$(cat "$TMP_PR_BODY")"
  assert_contains "Closes #1021" "$body" "finish keeps closing linkage"
  assert_contains "## Summary" "$body" "finish renders summary section"
  assert_contains "Finish default root stages both docs and code paths." "$body" "finish uses output card summary"
  assert_contains "## Artifacts" "$body" "finish renders artifacts section"
  assert_contains "docs/tooling/README.md" "$body" "finish lists docs artifact"

  changed="$(git -C "$worktree" show --name-only --format=oneline HEAD)"
  assert_contains "adl/tools/README.md" "$changed" "finish stages code path by default"
  assert_contains "docs/tooling/README.md" "$changed" "finish stages docs path by default"
)

echo "pr.sh finish default repo-root staging: ok"

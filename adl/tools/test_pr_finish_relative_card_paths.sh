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
mockbin="$tmpdir/mockbin"
mkdir -p "$repo/adl/tools" "$repo/adl/templates/cards" "$repo/adl/schemas" "$mockbin"
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
  cp "$body_file" "$TMP_PR_BODY"
  exit 0
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
  cp "$body_file" "$TMP_PR_BODY"
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
  echo "seed" > README.md
  git add -A
  git commit -q -m "init"
  git branch -M main
  git init --bare -q "$origin"
  git remote add origin "$origin"
  git push -q -u origin main
  git fetch -q origin main
)

assert_contains() {
  local pattern="$1" text="$2"
  grep -Fq "$pattern" <<<"$text" || {
    echo "assertion failed: expected to find '$pattern'" >&2
    echo "$text" >&2
    exit 1
  }
}

TMP_PR_BODY="$tmpdir/pr_body.md"
export TMP_PR_BODY
export PATH="$mockbin:$PATH"

(
  cd "$repo"

  "$BASH_BIN" adl/tools/pr.sh start 958 --slug relative-card-paths --no-fetch-issue >/dev/null
  git -C "$tmpdir/adl-wp-958" config user.name "Test User"
  git -C "$tmpdir/adl-wp-958" config user.email "test@example.com"

  cat > .adl/cards/958/output_958.md <<'EOF_SOR'
# ADL Output Card

Task ID: issue-0958
Run ID: issue-0958
Version: v0.3
Title: relative-card-paths
Branch: codex/958-relative-card-paths
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5-codex
- Provider: local test harness
- Start Time: 2026-03-20T00:00:00Z
- End Time: 2026-03-20T00:01:00Z

## Summary
Finished relative card path test.
## Artifacts produced
- adl/tools/README.md
## Actions taken
- Updated one tracked file and rendered a PR body.
## Main Repo Integration (REQUIRED)
- Tracked paths prepared for main-repo integration:
  - `adl/tools/README.md`
- Worktree-only paths remaining:
  - `adl/tools/README.md`
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: branch-local tracked edits committed and prepared for PR
- Verification performed:
  - `git diff -- adl/tools/README.md`
    - verifies the tracked change intended for the PR.
- Result: PASS
## Validation
- Validation commands and their purpose:
  - `bash adl/tools/validate_structured_prompt.sh --type sor --phase completed --input .adl/cards/958/output_958.md`
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
- Ordering guarantees (sorting / tie-break rules used): stable card references
- Artifact stability notes: PR body should contain repo-relative card references
## Security / Privacy Checks
- Secret leakage scan performed: manual inspection
- Prompt / tool argument redaction verified: yes
- Absolute path leakage check: repo-relative card references only
- Sandbox / policy invariants preserved: yes
## Replay Artifacts
- Trace bundle path(s): not applicable
- Run artifact root: not applicable
- Replay command used for verification: not applicable
- Replay result: not applicable
## Artifact Verification
- Primary proof surface: `.adl/cards/958/output_958.md`
- Required artifacts present: true
- Artifact schema/version checks: completed-phase SOR validation passed
- Hash/byte-stability checks: not performed
- Missing/optional artifacts and rationale: no runtime trace required for this tooling issue
## Decisions / Deviations
- none
## Follow-ups / Deferred work
- none
EOF_SOR

  echo "relative body test" >> "$tmpdir/adl-wp-958/adl/tools/README.md"

  (
    cd "$tmpdir/adl-wp-958"
    "$BASH_BIN" adl/tools/pr.sh finish 958 --title "[v0.85][authoring] Prevent Absolute Host Path Leakage in Issues, Cards, and PR Bodies" --paths "adl/tools/README.md" -f "$repo/.adl/cards/958/input_958.md" --output-card "$repo/.adl/cards/958/output_958.md" --no-checks --no-open >/dev/null
  )

  body="$(cat "$TMP_PR_BODY")"
  assert_contains ".adl/cards/958/input_958.md" "$body"
  assert_contains ".adl/cards/958/output_958.md" "$body"
  if grep -Eq '/Users/|/private/|/tmp/' <<<"$body"; then
    echo "assertion failed: PR body leaked absolute host path" >&2
    echo "$body" >&2
    exit 1
  fi

  cat >"$tmpdir/issue_body_bad.md" <<'EOF_BAD'
## Goal
contains /Users/example leak
EOF_BAD
  set +e
  bad="$($BASH_BIN adl/tools/pr.sh new --title "bad issue" --body-file "$tmpdir/issue_body_bad.md" --no-start 2>&1)"
  status=$?
  set -e
  [[ "$status" -ne 0 ]] || {
    echo "assertion failed: expected pr.sh new absolute-path guard to fail" >&2
    exit 1
  }
  assert_contains "new: issue body contains disallowed absolute host path" "$bad"
)

echo "pr.sh finish/new path hygiene: ok"

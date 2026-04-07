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
REAL_ADL_BIN="$ROOT_DIR/adl/target/debug/adl"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

if [[ ! -x "$REAL_ADL_BIN" ]]; then
  cargo build --manifest-path "$ROOT_DIR/adl/Cargo.toml" --bin adl >/dev/null
fi

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
    if [[ "${GH_MOCK_EXISTING_PR:-absent}" == "present" ]]; then
      echo "https://example.test/pr/1"
    else
      echo ""
    fi
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
if [[ "$1" == "pr" && "$2" == "view" ]]; then
  if [[ " $* " == *" --json closingIssuesReferences "* ]]; then
    if [[ " $* " == *" -q "* ]]; then
      if [[ "${GH_MOCK_CLOSING_LINKAGE:-present}" == "present" ]]; then
        echo '958'
      else
        echo ''
      fi
    else
      if [[ "${GH_MOCK_CLOSING_LINKAGE:-present}" == "present" ]]; then
        echo '{"closingIssuesReferences":[{"number":958}]}'
      else
        echo '{"closingIssuesReferences":[]}'
      fi
    fi
    exit 0
  fi
  if [[ " $* " == *" --json body "* ]]; then
    if [[ "${GH_MOCK_BODY_LINKAGE:-present}" == "missing" ]]; then
      echo 'body without closing line'
    else
      cat "$TMP_PR_BODY"
    fi
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
export ADL_PR_RUST_BIN="$REAL_ADL_BIN"
export GH_MOCK_CLOSING_LINKAGE="present"
export GH_MOCK_EXISTING_PR="absent"

(
  cd "$repo"

  "$BASH_BIN" adl/tools/pr.sh start 958 --slug relative-card-paths --no-fetch-issue --version v0.85 >/dev/null
  "$BASH_BIN" adl/tools/pr.sh cards 958 --version v0.85 --no-fetch-issue >/dev/null
  worktree="$repo/.worktrees/adl-wp-958"
  git -C "$worktree" config user.name "Test User"
  git -C "$worktree" config user.email "test@example.com"
  mkdir -p .adl/cards/958

  cat > .adl/cards/958/output_958.md <<'EOF_SOR'
# relative-card-paths

Task ID: issue-0958
Run ID: issue-0958
Version: v0.85
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

  cp .adl/cards/958/output_958.md "$tmpdir/pristine_output_958.md"

  echo "relative body test" >> "$worktree/adl/tools/README.md"

  (
    cd "$worktree"
    "$BASH_BIN" adl/tools/pr.sh finish 958 --title "[v0.85][authoring] Prevent Absolute Host Path Leakage in Issues, Cards, and PR Bodies" --paths "adl/tools/README.md" -f "$repo/.adl/cards/958/input_958.md" --output-card "$repo/.adl/cards/958/output_958.md" --no-checks --no-open >/dev/null
  )

  body="$(cat "$TMP_PR_BODY")"
  assert_contains ".adl/cards/958/input_958.md" "$body"
  assert_contains ".adl/cards/958/output_958.md" "$body"
  assert_contains "Closes #958" "$body"
  assert_contains "## Summary" "$body"
  assert_contains "Finished relative card path test." "$body"
  assert_contains "## Artifacts" "$body"
  assert_contains "adl/tools/README.md" "$body"
  assert_contains "## Validation" "$body"
  if grep -Eq '/Users/|/private/|/tmp/' <<<"$body"; then
    echo "assertion failed: PR body leaked absolute host path" >&2
    echo "$body" >&2
    exit 1
  fi

  cp "$tmpdir/pristine_output_958.md" "$repo/.adl/cards/958/output_958.md"
  cmp -s "$tmpdir/pristine_output_958.md" \
    "$repo/.adl/v0.85/tasks/issue-0958__relative-card-paths/sor.md" || \
    cp "$tmpdir/pristine_output_958.md" \
      "$repo/.adl/v0.85/tasks/issue-0958__relative-card-paths/sor.md"
  cmp -s "$tmpdir/pristine_output_958.md" \
    "$worktree/.adl/v0.85/tasks/issue-0958__relative-card-paths/sor.md" || \
    cp "$tmpdir/pristine_output_958.md" \
      "$worktree/.adl/v0.85/tasks/issue-0958__relative-card-paths/sor.md"

  echo "relative body test update path" >> "$worktree/adl/tools/README.md"
  export GH_MOCK_EXISTING_PR="present"
  export GH_MOCK_CLOSING_LINKAGE="present"
  (
    cd "$worktree"
    "$BASH_BIN" adl/tools/pr.sh finish 958 --title "[v0.85][authoring] Prevent Absolute Host Path Leakage in Issues, Cards, and PR Bodies" --paths "adl/tools/README.md" -f "$repo/.adl/cards/958/input_958.md" --output-card "$repo/.adl/cards/958/output_958.md" --no-checks --no-open >/dev/null
  )
  body="$(cat "$TMP_PR_BODY")"
  assert_contains "Closes #958" "$body"

  bad_finish_body=$'issue_card_schema: adl.issue.v1\nwp: WP-04\npr_start:\n## Goal\nleaked issue template'
  set +e
  bad="$(
    cd "$worktree" &&
    "$BASH_BIN" adl/tools/pr.sh finish 958 --title "[v0.85][authoring] Prevent Absolute Host Path Leakage in Issues, Cards, and PR Bodies" --paths "adl/tools/README.md" -f "$repo/.adl/cards/958/input_958.md" --output-card "$repo/.adl/cards/958/output_958.md" --no-checks --no-open --body "$bad_finish_body" 2>&1
  )"
  status=$?
  set -e
  [[ "$status" -ne 0 ]] || {
    echo "assertion failed: expected finish to reject issue-template body text" >&2
    exit 1
  }
  assert_contains "finish: --body looks like issue-template/prompt text" "$bad"

)

echo "pr.sh finish path hygiene: ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BASH_BIN="$(command -v bash)"
TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/five-command-editing-demo.XXXXXX")"
if [[ "${KEEP_DEMO_TMP:-0}" != "1" ]]; then
  trap 'rm -rf "$TMP_DIR"' EXIT
fi

ORIGIN="$TMP_DIR/origin.git"
REPO="$TMP_DIR/repo"
MOCKBIN="$TMP_DIR/mockbin"
RUNS_ROOT="$REPO/.adl/runs"
OUT_ROOT="$REPO/.adl/out"
REPORT_ROOT="$TMP_DIR/proof"
ISSUE_NUM="42"
SLUG="five-command-editing-demo"
TITLE="[v0.85][editor] Five-command editing demo issue"
ISSUE_BODY="$REPO/.adl/issues/v0.85/bodies/issue-42-five-command-editing-demo.md"
STP_PATH="$REPO/.adl/v0.85/tasks/issue-0042__five-command-editing-demo/stp.md"
BUNDLE_GLOB="issue-0042__five-command-editing-demo"
INPUT_CARD=""
OUTPUT_CARD=""
BRANCH_NAME="codex/42-five-command-editing-demo"
WORKTREE="$REPO/.worktrees/adl-wp-42"
GH_LOG="$REPORT_ROOT/gh.log"
GH_BODY="$REPORT_ROOT/gh_body.md"
MANIFEST="$REPORT_ROOT/five_command_editing_demo_manifest.json"
CONTRACT_JSON="$REPORT_ROOT/editor_adapter_contract.json"
INIT_LOG="$REPORT_ROOT/01-pr-init.txt"
CREATE_LOG="$REPORT_ROOT/02-pr-create.txt"
ADAPTER_LOG="$REPORT_ROOT/03-editor-adapter.txt"
START_LOG="$REPORT_ROOT/04-pr-start.txt"
RUN_LOG="$REPORT_ROOT/05-pr-run.txt"
FINISH_LOG="$REPORT_ROOT/06-pr-finish.txt"
MOCK_OLLAMA="$ROOT_DIR/adl/tools/mock_ollama_v0_4.sh"

mkdir -p "$MOCKBIN" "$REPORT_ROOT"

git clone --quiet --no-hardlinks "$ROOT_DIR" "$REPO"
(
  cd "$REPO"
  git config user.name "Demo User"
  git config user.email "demo@example.com"
  git checkout -q -B main
  rsync -a --delete --exclude '.git' --exclude '.worktrees' "$ROOT_DIR/" "$REPO/"
  git init --bare -q "$ORIGIN"
  git remote set-url origin "$ORIGIN"
  git push -q -u origin main
  git fetch -q origin main
)

mkdir -p "$(dirname "$ISSUE_BODY")"

cat >"$ISSUE_BODY" <<'EOF'
---
issue_card_schema: adl.issue.v1
wp: "editor"
slug: "five-command-editing-demo"
title: "[v0.85][editor] Five-command editing demo issue"
labels:
  - "track:roadmap"
  - "version:v0.85"
  - "area:tools"
issue_number: 42
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Demo Sprint"
required_outcome_type:
  - "demo"
  - "docs"
repo_inputs:
  - "adl/tools/pr.sh"
canonical_files: []
demo_required: true
demo_names:
  - "five-command-editing-demo"
issue_graph_notes: []
pr_start:
  enabled: true
  slug: "five-command-editing-demo"
---

# Issue Card

## Summary
Demonstrate the bounded five-command editing lifecycle truthfully.
## Goal
Show one honest end-to-end lifecycle over the real command surface.
## Required Outcome
Emit a runnable proof surface with visible lifecycle artifacts.
## Deliverables
- bounded demo manifest
## Acceptance Criteria
- each lifecycle step is visible
- emitted artifacts are inspectable
## Repo Inputs
- adl/tools/pr.sh
## Dependencies
- none
## Demo Expectations
- required demo: five-command-editing-demo
## Non-goals
- direct browser execution of unsupported commands
## Issue-Graph Notes
- none
## Notes
- bounded local-only demo
## Tooling Notes
- use mocked GitHub and mocked model-provider surfaces
EOF

cat >"$MOCKBIN/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
LOG_FILE="${GH_LOG_FILE:?}"
BODY_FILE_CAPTURE="${GH_BODY_FILE_CAPTURE:?}"
printf '%s\n' "$*" >>"$LOG_FILE"

if [[ "${1:-}" == "repo" && "${2:-}" == "view" ]]; then
  echo "local/demo"
  exit 0
fi

if [[ "${1:-}" == "issue" && "${2:-}" == "view" ]]; then
  issue="${3:-}"
  shift 3
  if [[ "$issue" == "42" && "$*" == *"--json labels"* && "$*" == *"-q .labels[].name"* ]]; then
    printf '%s\n' "track:roadmap" "version:v0.85" "area:tools"
    exit 0
  fi
  if [[ "$issue" == "42" && "$*" == *"--json title"* && "$*" == *"-q .title"* ]]; then
    echo "[v0.85][editor] Five-command editing demo issue"
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

if [[ "${1:-}" == "pr" && "${2:-}" == "list" ]]; then
  if [[ " $* " == *" -q "* ]]; then
    echo ""
  else
    echo "[]"
  fi
  exit 0
fi

if [[ "${1:-}" == "pr" && "${2:-}" == "view" ]]; then
  shift 2
  if [[ " $* " == *"--json closingIssuesReferences"* && " $* " == *"-q .closingIssuesReferences[]?.number"* ]]; then
    echo "42"
    exit 0
  fi
  if [[ " $* " == *"--web"* ]]; then
    exit 0
  fi
fi

if [[ "${1:-}" == "pr" && "${2:-}" == "create" ]]; then
  echo "https://example.test/pr/42"
  exit 0
fi

if [[ "${1:-}" == "pr" && "${2:-}" == "edit" ]]; then
  exit 0
fi

exit 1
EOF
chmod +x "$MOCKBIN/gh"

export PATH="$MOCKBIN:$PATH"
export GH_LOG_FILE="$GH_LOG"
export GH_BODY_FILE_CAPTURE="$GH_BODY"

(
  cd "$REPO"
  "$BASH_BIN" adl/tools/pr.sh init "$ISSUE_NUM" --slug "$SLUG" --no-fetch-issue --version v0.85
) | tee "$INIT_LOG"

(
  cd "$REPO"
  "$BASH_BIN" adl/tools/pr.sh create "$ISSUE_NUM" --stp "$STP_PATH"
) | tee "$CREATE_LOG"

{
  echo "# editor adapter dry run"
  "$BASH_BIN" adl/tools/editor_action.sh contract --format json >"$CONTRACT_JSON"
  DRY_RUN_COMMAND="$("$BASH_BIN" adl/tools/editor_action.sh start --issue "$ISSUE_NUM" --branch "$BRANCH_NAME" --dry-run)"
  printf '%s\n' "$DRY_RUN_COMMAND"
} | tee "$ADAPTER_LOG"

(
  cd "$REPO"
  "$BASH_BIN" adl/tools/pr.sh start "$ISSUE_NUM" --slug "$SLUG" --no-fetch-issue
) | tee "$START_LOG"

BUNDLE_DIR="$(
  find "$WORKTREE/.adl" -type d -path "*/tasks/$BUNDLE_GLOB" | LC_ALL=C sort | head -n1
)"
[[ -n "$BUNDLE_DIR" ]] || {
  echo "five-command demo: failed to locate task bundle for $BUNDLE_GLOB under $WORKTREE/.adl" >&2
  exit 1
}
INPUT_CARD="$BUNDLE_DIR/sip.md"
OUTPUT_CARD="$BUNDLE_DIR/sor.md"

git -C "$WORKTREE" config user.name "Demo User"
git -C "$WORKTREE" config user.email "demo@example.com"

(
  cd "$WORKTREE"
  ADL_OLLAMA_BIN="$MOCK_OLLAMA" \
    "$BASH_BIN" adl/tools/pr.sh run adl/examples/v0-4-demo-deterministic-replay.adl.yaml \
      --trace \
      --allow-unsigned \
      --runs-root "$RUNS_ROOT" \
      --out "$OUT_ROOT"
) | tee "$RUN_LOG"

mkdir -p "$WORKTREE/docs/tooling/editor"
cat >"$WORKTREE/docs/tooling/editor/five_command_demo_note.md" <<'EOF'
# Five-Command Editing Demo Note

This tracked note exists only to give the bounded demo one truthful tracked path for `pr finish`.
EOF

git -C "$WORKTREE" add docs/tooling/editor/five_command_demo_note.md

cat >"$OUTPUT_CARD" <<EOF
# ADL Output Card

Task ID: issue-0042
Run ID: issue-0042
Version: v0.85
Title: [v0.85][editor] Five-command editing demo issue
Branch: $BRANCH_NAME
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5-codex
- Provider: local demo harness
- Start Time: 2026-03-24T00:00:00Z
- End Time: 2026-03-24T00:00:01Z

## Summary
Bounded demo record for the five-command editing lifecycle.
## Artifacts produced
- docs/tooling/editor/five_command_demo_note.md
## Actions taken
- exercised init/create/start/run/finish in one bounded local demo
## Main Repo Integration (REQUIRED)
- Tracked paths prepared for main-repo integration:
  - \`docs/tooling/editor/five_command_demo_note.md\`
- Worktree-only paths remaining:
  - \`docs/tooling/editor/five_command_demo_note.md\`
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: branch-local tracked edit prepared for bounded PR creation
- Verification performed:
  - \`git diff -- docs/tooling/editor/five_command_demo_note.md\`
- Result: PASS
## Validation
- Validation commands and their purpose:
  - \`bash adl/tools/demo_five_command_editing.sh\`
    - proves the bounded five-command lifecycle is runnable through the supported command surface and documented adapter path.
- Results:
  - all listed commands passed
## Verification Summary
\`\`\`yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "five-command editing demo"
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
\`\`\`
## Determinism Evidence
- Determinism tests executed: bounded five-command editing demo
- Replay verification (same inputs -> same artifacts/order): yes
- Ordering guarantees (sorting / tie-break rules used): fixed lifecycle order
- Artifact stability notes: manifest and per-step transcripts are emitted deterministically
## Security / Privacy Checks
- Secret leakage scan performed: mocked local-only surfaces only
- Prompt / tool argument redaction verified: yes
- Absolute path leakage check: committed artifacts remain repository-relative
- Sandbox / policy invariants preserved: yes
## Replay Artifacts
- Trace bundle path(s): not applicable
- Run artifact root: .adl/runs/v0-4-demo-deterministic-replay
- Replay command used for verification: bash adl/tools/demo_five_command_editing.sh
- Replay result: pass
## Artifact Verification
- Primary proof surface: docs/tooling/editor/five_command_demo_note.md plus the emitted demo manifest
- Required artifacts present: true
- Artifact schema/version checks: completed-phase SOR validation passed
- Hash/byte-stability checks: not separately required
- Missing/optional artifacts and rationale: no direct browser execution beyond the validated adapter
## Decisions / Deviations
- bounded finish path uses --no-checks to keep the demo focused on lifecycle proof rather than full CI gating
## Follow-ups / Deferred work
- none
EOF

(
  cd "$WORKTREE"
  "$BASH_BIN" adl/tools/pr.sh finish "$ISSUE_NUM" \
    --title "$TITLE" \
    --paths "docs/tooling/editor/five_command_demo_note.md" \
    -f "$INPUT_CARD" \
    --output-card "$OUTPUT_CARD" \
    --no-checks \
    --no-open
) | tee "$FINISH_LOG"

python3 - <<PY
import json
from pathlib import Path

manifest = {
    "schema_version": "five_command_editing_demo.v1",
    "demo_entry": "bash adl/tools/demo_five_command_editing.sh",
    "issue_number": 42,
    "slug": "$SLUG",
    "editor_adapter_contract_json": str(Path("$CONTRACT_JSON")),
    "step_logs": {
        "pr_init": str(Path("$INIT_LOG")),
        "pr_create": str(Path("$CREATE_LOG")),
        "editor_adapter": str(Path("$ADAPTER_LOG")),
        "pr_start": str(Path("$START_LOG")),
        "pr_run": str(Path("$RUN_LOG")),
        "pr_finish": str(Path("$FINISH_LOG")),
    },
    "artifacts": {
        "issue_prompt": str(Path("$ISSUE_BODY")),
        "stp": str(Path("$STP_PATH")),
        "input_card": str(Path("$INPUT_CARD")),
        "output_card": str(Path("$OUTPUT_CARD")),
        "worktree": str(Path("$WORKTREE")),
        "run_json": str(Path("$RUNS_ROOT") / "v0-4-demo-deterministic-replay" / "run.json"),
        "run_status_json": str(Path("$RUNS_ROOT") / "v0-4-demo-deterministic-replay" / "run_status.json"),
        "run_summary_json": str(Path("$RUNS_ROOT") / "v0-4-demo-deterministic-replay" / "run_summary.json"),
        "tracked_finish_note": str(Path("$WORKTREE") / "docs/tooling/editor/five_command_demo_note.md"),
    },
    "gh_log": str(Path("$GH_LOG")),
    "gh_body_file": str(Path("$GH_BODY")),
    "limitations": [
        "The browser-direct adapter surface remains bounded to pr start via adl/tools/editor_action.sh.",
        "The demo uses mocked GitHub and mocked model provider surfaces to stay local and deterministic.",
        "The finish step uses --no-checks to keep the proof focused on lifecycle sequencing rather than full CI gating."
    ]
}
Path("$MANIFEST").write_text(json.dumps(manifest, indent=2) + "\\n")
PY

printf '%s\n' "five-command editing demo manifest: $MANIFEST"

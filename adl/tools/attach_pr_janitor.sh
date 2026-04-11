#!/usr/bin/env bash
if [ -z "${BASH_VERSION:-}" ]; then
  exec bash "$0" "$@"
fi
set -euo pipefail

usage() {
  cat <<'EOF' >&2
Usage:
  adl/tools/attach_pr_janitor.sh --repo-root <path> --repo <owner/name> --issue <n> --branch <branch> --pr-url <url> --expected-pr-state <draft|ready>
EOF
}

die() {
  printf '%s\n' "ERROR: $*" >&2
  exit 2
}

REPO_ROOT=""
REPO=""
ISSUE=""
BRANCH=""
PR_URL=""
EXPECTED_PR_STATE=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --repo-root) REPO_ROOT="$2"; shift 2 ;;
    --repo) REPO="$2"; shift 2 ;;
    --issue) ISSUE="$2"; shift 2 ;;
    --branch) BRANCH="$2"; shift 2 ;;
    --pr-url) PR_URL="$2"; shift 2 ;;
    --expected-pr-state) EXPECTED_PR_STATE="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) die "unknown arg: $1" ;;
  esac
done

[[ -n "$REPO_ROOT" ]] || die "missing --repo-root"
[[ -n "$REPO" ]] || die "missing --repo"
[[ -n "$ISSUE" ]] || die "missing --issue"
[[ -n "$BRANCH" ]] || die "missing --branch"
[[ -n "$PR_URL" ]] || die "missing --pr-url"
[[ -n "$EXPECTED_PR_STATE" ]] || die "missing --expected-pr-state"
[[ -d "$REPO_ROOT" ]] || die "repo root not found: $REPO_ROOT"

command -v codex >/dev/null 2>&1 || die "codex CLI not found in PATH"
codex exec --help >/dev/null 2>&1 || die "codex exec --help failed"

bash "$REPO_ROOT/adl/tools/install_adl_operational_skills.sh" >/dev/null

ARTIFACT_ROOT="$REPO_ROOT/.adl/logs/pr-janitor/issue-${ISSUE}"
mkdir -p "$ARTIFACT_ROOT"

INPUT_FILE="$ARTIFACT_ROOT/input.yaml"
PROMPT_FILE="$ARTIFACT_ROOT/prompt.md"
RUN_LOG="$ARTIFACT_ROOT/codex.log"
LAST_MESSAGE_FILE="$ARTIFACT_ROOT/last_message.md"
PID_FILE="$ARTIFACT_ROOT/pid"

cat >"$INPUT_FILE" <<EOF
skill_input_schema: pr_janitor.v1
mode: watch_pr_url
repo_root: $REPO_ROOT
target:
  pr_number: null
  pr_url: $PR_URL
  branch: $BRANCH
  issue_number: $ISSUE
  expected_checks:
    - adl-ci
    - adl-coverage
  expected_pr_state: $EXPECTED_PR_STATE
  review_standard: standard
policy:
  repair_mode: inspect_only
  allow_pr_inference: false
  monitor_checks: true
  monitor_conflicts: true
  monitor_review_state: true
  stop_after_janitor_pass: true
EOF

cat >"$PROMPT_FILE" <<EOF
Use \$pr-janitor at $REPO_ROOT/adl/tools/skills/pr-janitor/SKILL.md with this validated input:

\`\`\`yaml
$(cat "$INPUT_FILE")
\`\`\`

Run one bounded janitor pass for this newly opened PR. Stop after the janitor pass and write a concise status summary.
EOF

nohup codex exec \
  --full-auto \
  --sandbox workspace-write \
  --cd "$REPO_ROOT" \
  --skip-git-repo-check \
  --add-dir "$REPO_ROOT" \
  --output-last-message "$LAST_MESSAGE_FILE" \
  "$(cat "$PROMPT_FILE")" >"$RUN_LOG" 2>&1 < /dev/null &

JANITOR_PID=$!
printf '%s\n' "$JANITOR_PID" >"$PID_FILE"
printf 'ATTACHED issue=%s pr=%s pid=%s log=%s\n' "$ISSUE" "$PR_URL" "$JANITOR_PID" "$RUN_LOG"

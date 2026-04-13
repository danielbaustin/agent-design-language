#!/usr/bin/env bash
if [ -z "${BASH_VERSION:-}" ]; then
  exec bash "$0" "$@"
fi
set -euo pipefail

usage() {
  cat <<'EOF' >&2
Usage:
  adl/tools/attach_post_merge_closeout.sh --repo-root <path> --repo <owner/name> --issue <n> --branch <branch> --pr-url <url>
EOF
}

die() {
  printf '%s\n' "ERROR: $*" >&2
  exit 2
}

write_summary() {
  local path="$1"
  local status="$2"
  local detail="$3"
  cat >"$path" <<EOF
status: $status
issue: $ISSUE
branch: $BRANCH
pr_url: $PR_URL
repo: $REPO
detail: $detail
normalized_surfaces:
  - canonical sor.md reconciled to closed merged truth
  - duplicate closed task bundles pruned if present
  - output card symlink relinked to canonical sor
  - safe worktree prune attempted
EOF
}

poll_once() {
  local summary_file="$1"
  local run_log="$2"
  local pr_json
  local merged_at
  local pr_state
  local issue_json
  local issue_state
  local issue_reason

  pr_json="$(gh pr view -R "$REPO" "$PR_URL" --json state,mergedAt,url 2>>"$run_log" || true)"
  if [[ -z "$pr_json" ]]; then
    return 1
  fi
  merged_at="$(printf '%s' "$pr_json" | python3 -c 'import json,sys; data=json.load(sys.stdin); print(data.get("mergedAt") or "")')"
  pr_state="$(printf '%s' "$pr_json" | python3 -c 'import json,sys; data=json.load(sys.stdin); print(data.get("state") or "")')"
  if [[ -z "$merged_at" ]]; then
    if [[ "$pr_state" == "CLOSED" ]]; then
      write_summary "$summary_file" "stopped_unmerged_pr_closed" "PR closed without merge; automatic post-merge closeout not applicable."
      return 10
    fi
    return 1
  fi

  issue_json="$(gh issue view "$ISSUE" -R "$REPO" --json state,stateReason,url 2>>"$run_log" || true)"
  if [[ -z "$issue_json" ]]; then
    return 1
  fi
  issue_state="$(printf '%s' "$issue_json" | python3 -c 'import json,sys; data=json.load(sys.stdin); print(data.get("state") or "")')"
  issue_reason="$(printf '%s' "$issue_json" | python3 -c 'import json,sys; data=json.load(sys.stdin); print(data.get("stateReason") or "")')"
  if [[ "$issue_state" != "CLOSED" || "$issue_reason" != "COMPLETED" ]]; then
    return 1
  fi

  if bash "$REPO_ROOT/adl/tools/pr.sh" closeout "$ISSUE" >>"$run_log" 2>&1; then
    write_summary "$summary_file" "normalized" "Automatic post-merge closeout completed after PR merge and issue closure."
    return 0
  fi

  write_summary "$summary_file" "failed_closeout" "PR merged and issue closed/completed, but automatic closeout failed. Inspect run.log for the bounded error."
  return 20
}

run_watch_loop() {
  local artifact_root="$1"
  local summary_file="$2"
  local run_log="$3"
  local attempt=0
  local max_attempts="${ADL_POST_MERGE_CLOSEOUT_MAX_ATTEMPTS:-240}"
  local sleep_secs="${ADL_POST_MERGE_CLOSEOUT_SLEEP_SECS:-30}"

  write_summary "$summary_file" "watching" "Waiting for PR merge and issue CLOSED/COMPLETED state before automatic closeout."
  while (( attempt < max_attempts )); do
    if poll_once "$summary_file" "$run_log"; then
      exit 0
    fi
    case "$?" in
      10) exit 0 ;;
      20) exit 20 ;;
    esac
    attempt=$((attempt + 1))
    sleep "$sleep_secs"
  done

  write_summary "$summary_file" "timeout" "Timed out waiting for merged PR plus CLOSED/COMPLETED issue state; no automatic closeout was applied."
  exit 30
}

REPO_ROOT=""
REPO=""
ISSUE=""
BRANCH=""
PR_URL=""
WATCH_MODE=0
SUMMARY_FILE=""
RUN_LOG=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --repo-root) REPO_ROOT="$2"; shift 2 ;;
    --repo) REPO="$2"; shift 2 ;;
    --issue) ISSUE="$2"; shift 2 ;;
    --branch) BRANCH="$2"; shift 2 ;;
    --pr-url) PR_URL="$2"; shift 2 ;;
    --watch) WATCH_MODE=1; shift ;;
    --summary-file) SUMMARY_FILE="$2"; shift 2 ;;
    --run-log) RUN_LOG="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) die "unknown arg: $1" ;;
  esac
done

[[ -n "$REPO_ROOT" ]] || die "missing --repo-root"
[[ -n "$REPO" ]] || die "missing --repo"
[[ -n "$ISSUE" ]] || die "missing --issue"
[[ -n "$BRANCH" ]] || die "missing --branch"
[[ -n "$PR_URL" ]] || die "missing --pr-url"
[[ -d "$REPO_ROOT" ]] || die "repo root not found: $REPO_ROOT"
command -v gh >/dev/null 2>&1 || die "gh not found in PATH"
command -v python3 >/dev/null 2>&1 || die "python3 not found in PATH"

ARTIFACT_ROOT="$REPO_ROOT/.adl/logs/post-merge-closeout/issue-${ISSUE}"
mkdir -p "$ARTIFACT_ROOT"
SUMMARY_FILE="${SUMMARY_FILE:-$ARTIFACT_ROOT/summary.md}"
RUN_LOG="${RUN_LOG:-$ARTIFACT_ROOT/run.log}"
PID_FILE="$ARTIFACT_ROOT/pid"

if [[ "$WATCH_MODE" == "1" ]]; then
  run_watch_loop "$ARTIFACT_ROOT" "$SUMMARY_FILE" "$RUN_LOG"
  exit 0
fi

nohup bash "$0" \
  --watch \
  --repo-root "$REPO_ROOT" \
  --repo "$REPO" \
  --issue "$ISSUE" \
  --branch "$BRANCH" \
  --pr-url "$PR_URL" \
  --summary-file "$SUMMARY_FILE" \
  --run-log "$RUN_LOG" >"$RUN_LOG" 2>&1 < /dev/null &

CLOSEOUT_PID=$!
printf '%s\n' "$CLOSEOUT_PID" >"$PID_FILE"
printf 'ATTACHED issue=%s pr=%s pid=%s log=%s summary=%s\n' "$ISSUE" "$PR_URL" "$CLOSEOUT_PID" "$RUN_LOG" "$SUMMARY_FILE"

#!/usr/bin/env bash
set -euo pipefail

# codexw.sh — run Codex CLI as a disciplined ADL worker.
#
# Usage:
#   codexw.sh --branch <branch> --prompt-file <file> [--root <dir>] [--receipt <file>]
#   codexw.sh --branch <branch> --prompt "<text>"     [--root <dir>] [--receipt <file>]
#
# Optional:
#   --allow-path <path>   Repeatable file-fence hint for Codex.
#   --check "<cmd>"       Repeatable checks; defaults are Rust-safe repo checks.

die(){ echo "ERROR: $*" >&2; exit 1; }
note(){ echo "• $*" >&2; }

ROOT=""
BRANCH=""
PROMPT=""
PROMPT_FILE=""
RECEIPT=""
ALLOW_PATHS=()
CHECKS=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --root) ROOT="$2"; shift 2 ;;
    --branch) BRANCH="$2"; shift 2 ;;
    --prompt) PROMPT="$2"; shift 2 ;;
    --prompt-file) PROMPT_FILE="$2"; shift 2 ;;
    --receipt) RECEIPT="$2"; shift 2 ;;
    --allow-path) ALLOW_PATHS+=("$2"); shift 2 ;;
    --check) CHECKS+=("$2"); shift 2 ;;
    -h|--help) sed -n '1,180p' "$0"; exit 0 ;;
    *) die "Unknown arg: $1" ;;
  esac
done

[[ -n "$BRANCH" ]] || die "Missing --branch"
[[ "$BRANCH" != "main" && "$BRANCH" != "master" ]] || die "Refusing to run with --branch $BRANCH"

if [[ -z "$ROOT" ]]; then
  ROOT="$(pwd)"
fi
[[ -d "$ROOT" ]] || die "Root dir not found: $ROOT"

if [[ -n "$PROMPT" && -n "$PROMPT_FILE" ]]; then
  die "Use either --prompt or --prompt-file, not both"
fi
if [[ -z "$PROMPT" && -z "$PROMPT_FILE" ]]; then
  die "Missing --prompt or --prompt-file"
fi

cd "$ROOT"

git rev-parse --is-inside-work-tree >/dev/null 2>&1 || die "Not a git repo: $ROOT"

current="$(git rev-parse --abbrev-ref HEAD)"
if [[ "$current" == "main" || "$current" == "master" ]]; then
  die "Refusing to run on $current. Switch to a feature branch first."
fi

if git show-ref --verify --quiet "refs/heads/$BRANCH"; then
  git switch "$BRANCH" >/dev/null
else
  git switch -c "$BRANCH" >/dev/null
fi

current="$(git rev-parse --abbrev-ref HEAD)"
[[ "$current" != "main" && "$current" != "master" ]] || die "Refusing to run on $current"

if [[ -n "$PROMPT_FILE" ]]; then
  [[ -f "$PROMPT_FILE" ]] || die "Prompt file not found: $PROMPT_FILE"
  PROMPT="$(cat "$PROMPT_FILE")"
fi

if [[ ${#CHECKS[@]} -eq 0 ]]; then
  CHECKS=(
    "cargo fmt"
    "cargo clippy --all-targets -- -D warnings"
    "cargo test"
  )
fi

FENCE=""
if [[ ${#ALLOW_PATHS[@]} -gt 0 ]]; then
  FENCE=$'\n\n'"FILE FENCE (hard requirement):\n""- You may only modify these paths:\n"
  for p in "${ALLOW_PATHS[@]}"; do
    FENCE+="- ${p}"$'\n'
  done
  FENCE+=$'\n'"- If you think you need other files, stop and explain why."
fi

start_ts="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
base_sha="$(git rev-parse HEAD 2>/dev/null || true)"

note "Running Codex (non-interactive)..."
codex exec \
  --full-auto \
  -C "$ROOT" \
  --prompt "$PROMPT"$'\n'"$FENCE"

note "Applying patch (codex apply)..."
codex apply

note "Running checks..."
check_log="$(mktemp)"
check_results=()
check_failed=0
set +e
for cmd in "${CHECKS[@]}"; do
  echo "\$ $cmd" | tee -a "$check_log"
  bash -lc "$cmd" 2>&1 | tee -a "$check_log"
  rc=${PIPESTATUS[0]}
  if [[ $rc -eq 0 ]]; then
    check_results+=("$cmd: ok")
  else
    check_results+=("$cmd: failed (rc=$rc)")
    check_failed=1
    break
  fi
done
set -e

end_ts="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
head_sha="$(git rev-parse HEAD 2>/dev/null || true)"
status="DONE"
if [[ $check_failed -ne 0 ]]; then
  status="FAILED"
fi

if [[ -n "$RECEIPT" ]]; then
  mkdir -p "$(dirname "$RECEIPT")"

  task_id=""
  run_id=""
  version=""
  title="codexw execution"
  if [[ -n "$PROMPT_FILE" ]]; then
    bname="$(basename "$PROMPT_FILE")"
    if [[ "$bname" =~ ^(issue-[0-9]+)__input__(v[0-9.]+)\.md$ ]]; then
      task_id="${BASH_REMATCH[1]}"
      run_id="$task_id"
      version="${BASH_REMATCH[2]}"
    fi
    card_title="$(awk -F': ' '/^Title:/ {print $2; exit}' "$PROMPT_FILE" || true)"
    if [[ -n "$card_title" ]]; then
      title="$card_title"
    fi
  fi

  {
    echo "# ADL Output Card"
    echo
    echo "Task ID: ${task_id}"
    echo "Run ID: ${run_id}"
    echo "Version: ${version}"
    echo "Title: ${title}"
    echo "Branch: ${BRANCH}"
    echo "Status: ${status}"
    echo
    echo "Execution:"
    echo "- Actor: codexw.sh"
    echo "- Provider: local codex CLI"
    echo "- Start Time: ${start_ts}"
    echo "- End Time: ${end_ts}"
    echo
    echo "## Summary"
    echo "- Ran Codex on branch ${BRANCH}."
    echo "- Applied Codex patch output and executed repo checks."
    echo
    echo "## Artifacts produced"
    echo "- Prompt source: ${PROMPT_FILE:-inline --prompt}"
    echo "- Receipt path: ${RECEIPT}"
    echo
    echo "## Actions taken"
    echo "- codex exec --full-auto"
    echo "- codex apply"
    echo
    echo "## Validation"
    echo "- Tests / checks run:"
    for cmd in "${CHECKS[@]}"; do
      echo "  - ${cmd}"
    done
    echo "- Results:"
    for line in "${check_results[@]}"; do
      echo "  - ${line}"
    done
    echo
    echo "## Decisions / Deviations"
    echo "- Base SHA: ${base_sha}"
    echo "- Head SHA: ${head_sha}"
    echo
    echo "## Follow-ups / Deferred work"
    echo "- None"
  } > "$RECEIPT"

  note "Wrote receipt: $RECEIPT"
else
  note "No --receipt specified; skipping receipt write"
fi

if [[ $check_failed -ne 0 ]]; then
  die "One or more checks failed"
fi

note "Done on branch: $BRANCH"

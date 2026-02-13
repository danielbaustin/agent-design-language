#!/usr/bin/env bash
set -euo pipefail

# codexw.sh — run Codex CLI as a disciplined ADL worker.
#
# Conveyor-belt mode (preferred):
#   codexw.sh --issue <n> [--root <dir>] [--log <path>] [--allow-unchanged-output]
#   codexw.sh <input-card-path> [--root <dir>] [--log <path>] [--allow-unchanged-output]
#
# Legacy mode (backward compatibility):
#   codexw.sh --branch <branch> --prompt-file <file> [--root <dir>] [--receipt <file>]
#   codexw.sh --branch <branch> --prompt "<text>"     [--root <dir>] [--receipt <file>]
#
# Optional legacy args:
#   --allow-path <path>   Repeatable file-fence hint for Codex.
#   --check "<cmd>"       Repeatable checks; defaults are Rust-safe repo checks.

CARD_PATHS_LIB="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/card_paths.sh"
# shellcheck disable=SC1090
source "$CARD_PATHS_LIB"

die(){
  echo "ERROR: $*" >&2
  if [[ -n "${LOG_PATH:-}" ]]; then
    echo "ERROR: $*" >> "$LOG_PATH"
  fi
  exit 1
}
note(){ echo "• $*" >&2; }

ROOT=""
BRANCH=""
PROMPT=""
PROMPT_FILE=""
RECEIPT=""
ALLOW_PATHS=()
CHECKS=()

ISSUE=""
INPUT_CARD_ARG=""
LOG_PATH=""
ALLOW_UNCHANGED_OUTPUT=0

usage() {
  sed -n '1,120p' "$0"
}

sha256_file() {
  local file="$1"
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$file" | awk '{print $1}'
    return 0
  fi
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$file" | awk '{print $1}'
    return 0
  fi
  cksum "$file" | awk '{print $1":"$2}'
}

issue_from_input_path() {
  local p="$1"
  local base
  base="$(basename "$p")"

  if [[ "$p" =~ (^|/)\.adl/cards/([0-9]+)/input_([0-9]+)\.md$ ]]; then
    [[ "${BASH_REMATCH[2]}" == "${BASH_REMATCH[3]}" ]] || die "Card path mismatch: $p"
    card_issue_normalize "${BASH_REMATCH[2]}"
    return 0
  fi

  if [[ "$base" =~ ^issue-([0-9]+)__input__v[0-9.]+\.md$ ]]; then
    card_issue_normalize "${BASH_REMATCH[1]}"
    return 0
  fi

  die "Could not parse input card path: $p"
}

run_legacy_mode() {
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
}

run_conveyor_mode() {
  local issue input_card canonical_input canonical_output card_version
  local output_pre_exists=0 output_pre_sha="" output_post_sha=""
  local codex_rc=0
  log_note() {
    local msg="$1"
    echo "• $msg" | tee -a "$LOG_PATH" >&2
  }

  if [[ -n "$ROOT" ]]; then
    [[ -d "$ROOT" ]] || die "Root dir not found: $ROOT"
    cd "$ROOT"
  fi

  if [[ -n "$ISSUE" ]]; then
    issue="$(card_issue_normalize "$ISSUE")"
    card_version="v0.3"
  else
    [[ -n "$INPUT_CARD_ARG" ]] || die "Missing --issue or input card path"
    issue="$(issue_from_input_path "$INPUT_CARD_ARG")"
    if [[ -f "$INPUT_CARD_ARG" ]]; then
      card_version="$(awk -F':' '/^Version:/ {gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2); print $2; exit}' "$INPUT_CARD_ARG" || true)"
    fi
  fi

  if [[ -z "${card_version:-}" ]]; then
    card_version="v0.3"
  fi

  input_card="$(resolve_input_card_path "$issue" "$card_version")"
  canonical_input="$(card_input_path "$issue")"
  canonical_output="$(card_output_path "$issue")"

  if [[ ! -f "$input_card" ]]; then
    die "Input card missing: $input_card"
  fi

  if [[ -n "$LOG_PATH" ]]; then
    :
  else
    LOG_PATH=".adl/logs/${issue}/codex.log"
  fi
  mkdir -p "$(dirname "$LOG_PATH")"
  touch "$LOG_PATH"

  log_note "Issue: $issue"
  log_note "Input card: $input_card"
  log_note "Canonical input: $canonical_input"
  log_note "Canonical output: $canonical_output"
  log_note "Log: $LOG_PATH"

  if [[ -f "$canonical_output" ]]; then
    output_pre_exists=1
    output_pre_sha="$(sha256_file "$canonical_output")"
  fi

  local prompt
  prompt="$(cat "$input_card")"

  log_note "Running Codex (non-interactive)..."
  set +e
  codex exec \
    --full-auto \
    -C "$(pwd)" \
    --prompt "$prompt" 2>&1 | tee -a "$LOG_PATH"
  codex_rc=${PIPESTATUS[0]}

  if [[ $codex_rc -eq 0 ]]; then
    log_note "Applying patch (codex apply)..."
    codex apply 2>&1 | tee -a "$LOG_PATH"
    codex_rc=${PIPESTATUS[0]}
  fi
  set -e

  if [[ $codex_rc -ne 0 ]]; then
    die "Codex command failed (rc=$codex_rc)"
  fi

  [[ -f "$canonical_output" ]] || die "Output card was not written: $canonical_output"
  [[ -s "$canonical_output" ]] || die "Output card is empty: $canonical_output"

  if [[ $output_pre_exists -eq 1 ]]; then
    output_post_sha="$(sha256_file "$canonical_output")"
    if [[ "$output_pre_sha" == "$output_post_sha" && $ALLOW_UNCHANGED_OUTPUT -ne 1 ]]; then
      die "Output card unchanged after run: $canonical_output (use --allow-unchanged-output to bypass)"
    fi
  fi

  log_note "Conveyor run complete."
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --root) ROOT="$2"; shift 2 ;;
    --branch) BRANCH="$2"; shift 2 ;;
    --prompt) PROMPT="$2"; shift 2 ;;
    --prompt-file) PROMPT_FILE="$2"; shift 2 ;;
    --receipt) RECEIPT="$2"; shift 2 ;;
    --allow-path) ALLOW_PATHS+=("$2"); shift 2 ;;
    --check) CHECKS+=("$2"); shift 2 ;;

    --issue) ISSUE="$2"; shift 2 ;;
    --log) LOG_PATH="$2"; shift 2 ;;
    --allow-unchanged-output) ALLOW_UNCHANGED_OUTPUT=1; shift ;;

    -h|--help) usage; exit 0 ;;

    -*) die "Unknown arg: $1" ;;
    *)
      if [[ -n "$INPUT_CARD_ARG" ]]; then
        die "Unexpected extra positional arg: $1"
      fi
      INPUT_CARD_ARG="$1"
      shift
      ;;
  esac
done

if [[ -n "$ISSUE" && -n "$INPUT_CARD_ARG" ]]; then
  die "Use either --issue or an input card path, not both"
fi

if [[ -n "$ISSUE" || -n "$INPUT_CARD_ARG" ]]; then
  if [[ -n "$BRANCH" || -n "$PROMPT" || -n "$PROMPT_FILE" || -n "$RECEIPT" || ${#ALLOW_PATHS[@]} -gt 0 || ${#CHECKS[@]} -gt 0 ]]; then
    die "Conveyor mode cannot be mixed with legacy --branch/--prompt options"
  fi
  run_conveyor_mode
else
  run_legacy_mode
fi

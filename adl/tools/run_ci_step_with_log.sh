#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/run_ci_step_with_log.sh --name <step-name> [--log-root <dir>] -- <command> [args...]

Run one CI command while preserving ADL-owned stdout, stderr, combined log, and
metadata files. The wrapped command exit code is preserved.
USAGE
}

STEP_NAME=""
LOG_ROOT="${ADL_CI_STEP_LOG_ROOT:-ci-step-logs}"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --name)
      STEP_NAME="${2:-}"
      shift 2
      ;;
    --log-root)
      LOG_ROOT="${2:-}"
      shift 2
      ;;
    --)
      shift
      break
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [ -z "$STEP_NAME" ]; then
  echo "--name is required" >&2
  usage >&2
  exit 2
fi

if [ "$#" -eq 0 ]; then
  echo "command after -- is required" >&2
  usage >&2
  exit 2
fi

COMMAND=("$@")

slug="$(
  python3 - "$STEP_NAME" <<'PY'
import re
import sys

value = sys.argv[1].strip().lower()
value = re.sub(r"[^a-z0-9._-]+", "-", value)
value = value.strip("-._")
print(value or "ci-step")
PY
)"

run_id="${GITHUB_RUN_ID:-local}"
attempt="${GITHUB_RUN_ATTEMPT:-0}"
stamp="$(date -u +%Y%m%dT%H%M%SZ)"
log_dir="$LOG_ROOT/${slug}-${run_id}-${attempt}-${stamp}"
mkdir -p "$log_dir"

stdout_log="$log_dir/stdout.log"
stderr_log="$log_dir/stderr.log"
combined_log="$log_dir/combined.log"
metadata_json="$log_dir/metadata.json"

started_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
started_epoch="$(date +%s)"

printf 'ADL CI step log start: %s\n' "$STEP_NAME" | tee "$combined_log"
printf 'started_at=%s\n' "$started_at" | tee -a "$combined_log"
printf 'command_argc=%s\n' "${#COMMAND[@]}" | tee -a "$combined_log"
printf 'command_redaction=metadata_only\n' | tee -a "$combined_log"

stdout_fifo="$log_dir/stdout.fifo"
stderr_fifo="$log_dir/stderr.fifo"
mkfifo "$stdout_fifo" "$stderr_fifo"
tee "$stdout_log" <"$stdout_fifo" &
stdout_tee_pid=$!
tee "$stderr_log" <"$stderr_fifo" >&2 &
stderr_tee_pid=$!

set +e
"${COMMAND[@]}" >"$stdout_fifo" 2>"$stderr_fifo"
status=$?
set -e

wait "$stdout_tee_pid" || true
wait "$stderr_tee_pid" || true
rm -f "$stdout_fifo" "$stderr_fifo"

finished_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
finished_epoch="$(date +%s)"
elapsed_seconds=$((finished_epoch - started_epoch))

if [ -s "$stdout_log" ]; then
  sed 's/^/[stdout] /' "$stdout_log" >>"$combined_log"
fi
if [ -s "$stderr_log" ]; then
  sed 's/^/[stderr] /' "$stderr_log" >>"$combined_log"
fi
printf 'finished_at=%s\n' "$finished_at" | tee -a "$combined_log"
printf 'exit_code=%s\n' "$status" | tee -a "$combined_log"
printf 'elapsed_seconds=%s\n' "$elapsed_seconds" | tee -a "$combined_log"

python3 - "$metadata_json" "$STEP_NAME" "$started_at" "$finished_at" "$status" "$elapsed_seconds" "$stdout_log" "$stderr_log" "$combined_log" "${COMMAND[@]}" <<'PY'
import json
import os
import sys
from pathlib import Path

(
    out_path,
    step_name,
    started_at,
    finished_at,
    exit_code,
    elapsed_seconds,
    stdout_log,
    stderr_log,
    combined_log,
    *command,
) = sys.argv[1:]

root = Path.cwd()
try:
    repo_root = Path(os.popen("git rev-parse --show-toplevel 2>/dev/null").read().strip() or root)
except Exception:
    repo_root = root

def rel(path: str) -> str:
    p = Path(path)
    if not p.is_absolute():
        p = root / p
    try:
        return p.resolve().relative_to(repo_root.resolve()).as_posix()
    except Exception:
        return p.name

def is_sensitive_token(value: str) -> bool:
    lowered = value.lower()
    return any(token in lowered for token in ("token", "secret", "api_key", "apikey", "password"))

def redact(value: str) -> str:
    if is_sensitive_token(value):
        return "<redacted>"
    return value

def redact_command(command: list[str]) -> list[str]:
    redacted = []
    redact_next = False
    for part in command:
        if redact_next:
            redacted.append("<redacted>")
            redact_next = False
            continue
        if is_sensitive_token(part):
            redacted.append("<redacted>")
            if "=" not in part:
                redact_next = True
            continue
        redacted.append(part)
    return redacted

payload = {
    "schema": "adl.ci.step_log.v1",
    "step_name": step_name,
    "started_at": started_at,
    "finished_at": finished_at,
    "exit_code": int(exit_code),
    "elapsed_seconds": int(elapsed_seconds),
    "command": redact_command(command),
    "stdout_path": rel(stdout_log),
    "stderr_path": rel(stderr_log),
    "combined_path": rel(combined_log),
}

Path(out_path).write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
PY

exit "$status"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MANAGER="$ROOT/adl/tools/validation_manager.sh"

CHANGED_FILES=""
BASE="origin/main"
HEAD="HEAD"
INCLUDE_WORKING_TREE=false
REMOTE_COMMAND=""
REMOTE_ARTIFACT_DIR=""
REMOTE_GIT_REF=""
REPORT_OUT=""
RUN=false
JSON=false

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/run_validation_manager_nessus_lane.sh [--changed-files <path> | --include-working-tree] [options]

Options:
  --changed-files <path>       Changed-file list used by the validation manager.
  --base <ref>                 Base ref when --changed-files is not supplied.
  --head <ref>                 Head ref when --changed-files is not supplied.
  --include-working-tree       Select lanes from the current working tree.
  --remote-command <command>   Explicit command for Nessus. Defaults to the single selected local lane command.
  --remote-artifact-dir <dir>  Local directory for fetched Nessus summary and log bundle.
  --remote-git-ref <ref>       Git ref checked out by the Nessus runner.
  --report-out <path>          Write validation-manager JSON report.
  --run                        Execute the selected Nessus lane.
  --json                       Print JSON instead of text.
  -h, --help                   Show this help.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --changed-files)
      CHANGED_FILES="${2:-}"
      shift 2
      ;;
    --base)
      BASE="${2:-}"
      shift 2
      ;;
    --head)
      HEAD="${2:-}"
      shift 2
      ;;
    --include-working-tree)
      INCLUDE_WORKING_TREE=true
      shift
      ;;
    --remote-command)
      REMOTE_COMMAND="${2:-}"
      shift 2
      ;;
    --remote-artifact-dir)
      REMOTE_ARTIFACT_DIR="${2:-}"
      shift 2
      ;;
    --remote-git-ref)
      REMOTE_GIT_REF="${2:-}"
      shift 2
      ;;
    --report-out)
      REPORT_OUT="${2:-}"
      shift 2
      ;;
    --run)
      RUN=true
      shift
      ;;
    --json)
      JSON=true
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "run_validation_manager_nessus_lane: unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -n "$CHANGED_FILES" && "$INCLUDE_WORKING_TREE" == true ]]; then
  echo "run_validation_manager_nessus_lane: use either --changed-files or --include-working-tree, not both" >&2
  exit 2
fi

selector_args=()
if [[ -n "$CHANGED_FILES" ]]; then
  selector_args+=(--changed-files "$CHANGED_FILES")
elif [[ "$INCLUDE_WORKING_TREE" == true ]]; then
  selector_args+=(--include-working-tree)
else
  selector_args+=(--base "$BASE" --head "$HEAD")
fi

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/adl-validation-manager-nessus.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

if [[ -n "$REMOTE_GIT_REF" ]]; then
  export ADL_NESSUS_REMOTE_GIT_REF="$REMOTE_GIT_REF"
fi

if [[ -z "$REMOTE_COMMAND" ]]; then
  profile_path="$tmpdir/profile.json"
  "$MANAGER" "${selector_args[@]}" --json >"$profile_path"
  REMOTE_COMMAND="$(python3 - "$profile_path" <<'PY'
import json
import sys

profile = json.load(open(sys.argv[1], encoding="utf-8"))
run = profile.get("run", [])
if len(run) != 1:
    raise SystemExit(
        f"run_validation_manager_nessus_lane: expected exactly one selected local lane, observed {len(run)}"
    )
command = run[0].get("command", "")
if not command:
    raise SystemExit("run_validation_manager_nessus_lane: selected local lane has no command")
print(command)
PY
)"
  if [[ -n "$CHANGED_FILES" ]]; then
    REMOTE_COMMAND="$(python3 - "$REMOTE_COMMAND" "$CHANGED_FILES" <<'PY'
import base64
import shlex
import sys
from pathlib import Path

command = sys.argv[1]
changed_files = Path(sys.argv[2]).resolve()
remote_path = ".adl/tmp/validation-manager-nessus-changed-files.txt"

local_tokens = [str(changed_files), shlex.quote(str(changed_files))]
remote_token = shlex.quote(remote_path)
rewritten = None
for token in local_tokens:
    if token in command:
        rewritten = command.replace(token, remote_token)
        break
if rewritten is None:
    raise SystemExit(
        "run_validation_manager_nessus_lane: selected command did not include the changed-files path"
    )

payload = base64.b64encode(changed_files.read_bytes()).decode("ascii")
print(
    "mkdir -p .adl/tmp && "
    f"printf %s {shlex.quote(payload)} | base64 -d > {remote_token} && "
    f"{rewritten}"
)
PY
)"
  fi
fi

manager_args=("${selector_args[@]}" --remote-runner nessus --remote-command "$REMOTE_COMMAND")
if [[ -n "$REMOTE_ARTIFACT_DIR" ]]; then
  manager_args+=(--remote-artifact-dir "$REMOTE_ARTIFACT_DIR")
fi
if [[ -n "$REPORT_OUT" ]]; then
  manager_args+=(--report-out "$REPORT_OUT")
fi
if [[ "$RUN" == true ]]; then
  manager_args+=(--run)
fi
if [[ "$JSON" == true ]]; then
  manager_args+=(--json)
fi

exec "$MANAGER" "${manager_args[@]}"

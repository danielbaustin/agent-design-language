#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MANIFEST_PATH=""
CHANGED_FILES_PATH=""
MODE="pr"
REPORT_OUT=""
WORKTREE_ROOT=""
ALLOW_CREDENTIAL_LANES=false
PRINT_PLAN=false

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/run_pvf_validation_lane.sh --manifest <path> [options]

Options:
  --manifest <path>        Manifest JSON describing PVF lanes.
  --changed-files <path>   Optional changed-file list. Lines may be "path" or
                           "STATUS<TAB>path".
  --mode <pr|release>      Runner mode. Default: pr
  --report-out <path>      Write machine-readable JSON report to this path.
  --worktree-root <path>   Bound worktree root used by lanes that require one.
  --allow-credential-lanes Explicitly allow lanes marked requires_credentials=true.
  --print-plan             Print the derived lane plan and exit.
  -h, --help               Show this help.

Status vocabulary:
  passed, failed, blocked, skipped, deferred, reused, release_gate_required
USAGE
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --manifest)
      MANIFEST_PATH="${2:-}"
      shift 2
      ;;
    --changed-files)
      CHANGED_FILES_PATH="${2:-}"
      shift 2
      ;;
    --mode)
      MODE="${2:-}"
      shift 2
      ;;
    --report-out)
      REPORT_OUT="${2:-}"
      shift 2
      ;;
    --worktree-root)
      WORKTREE_ROOT="${2:-}"
      shift 2
      ;;
    --allow-credential-lanes)
      ALLOW_CREDENTIAL_LANES=true
      shift
      ;;
    --print-plan)
      PRINT_PLAN=true
      shift
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

if [ -z "$MANIFEST_PATH" ]; then
  echo "run_pvf_validation_lane: --manifest is required" >&2
  exit 2
fi

case "$MODE" in
  pr|release) ;;
  *)
    echo "run_pvf_validation_lane: unsupported --mode '$MODE' (expected pr|release)" >&2
    exit 2
    ;;
esac

if [ ! -f "$MANIFEST_PATH" ]; then
  echo "run_pvf_validation_lane: manifest not found: $MANIFEST_PATH" >&2
  exit 2
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT
plan_tsv="$tmpdir/plan.tsv"
changed_paths_txt="$tmpdir/changed_paths.txt"

if [ -n "$CHANGED_FILES_PATH" ]; then
  awk -F '\t' '
    NF == 1 { print $1; next }
    $1 ~ /^R/ && NF >= 3 { print $3; next }
    NF >= 2 { print $2; next }
  ' "$CHANGED_FILES_PATH" > "$changed_paths_txt"
else
  : > "$changed_paths_txt"
fi

python3 - "$MANIFEST_PATH" "$plan_tsv" <<'PY'
import json
import sys
from pathlib import Path

manifest_path = Path(sys.argv[1])
plan_path = Path(sys.argv[2])
manifest = json.loads(manifest_path.read_text())
lanes = manifest.get("lanes", {})

with plan_path.open("w", encoding="utf-8") as handle:
    for lane_id, lane in lanes.items():
        row = [
            lane_id,
            lane.get("lane_class", ""),
            lane.get("release_gate_class", ""),
            lane.get("default_trigger", ""),
            "true" if lane.get("requires_worktree", False) else "false",
            "true" if lane.get("requires_credentials", False) else "false",
            json.dumps(lane.get("changed_path_hints", []), separators=(",", ":")),
            lane.get("command", ""),
        ]
        handle.write("\t".join(row) + "\n")
PY

path_matches_hints() {
  local hints_json="$1"
  python3 - "$changed_paths_txt" "$hints_json" <<'PY'
import json
import sys
from pathlib import Path

changed = [line.strip() for line in Path(sys.argv[1]).read_text().splitlines() if line.strip()]
hints = json.loads(sys.argv[2])

if not changed:
    print("unknown")
    raise SystemExit(0)

for path in changed:
    for hint in hints:
        if path == hint or path.startswith(hint):
            print("match")
            raise SystemExit(0)

print("no_match")
PY
}

json_escape_file() {
  python3 - "$1" <<'PY'
import json
import sys
from pathlib import Path

print(json.dumps(Path(sys.argv[1]).read_text()))
PY
}

declare -a lane_ids=()
state_dir="$tmpdir/state"
mkdir -p "$state_dir"

lane_file() {
  local lane_id="$1"
  local kind="$2"
  printf '%s/%s.%s\n' "$state_dir" "$lane_id" "$kind"
}

set_lane_value() {
  local lane_id="$1"
  local kind="$2"
  local value="$3"
  printf '%s' "$value" > "$(lane_file "$lane_id" "$kind")"
}

get_lane_value() {
  local lane_id="$1"
  local kind="$2"
  local path
  path="$(lane_file "$lane_id" "$kind")"
  if [ -f "$path" ]; then
    cat "$path"
  fi
}

while IFS=$'\t' read -r lane_id lane_class release_gate_class default_trigger requires_worktree requires_credentials changed_hints_json command; do
  lane_ids+=("$lane_id")
  set_lane_value "$lane_id" lane_class "$lane_class"

  if [ "$requires_worktree" = "true" ] && [ -z "$WORKTREE_ROOT" ]; then
    set_lane_value "$lane_id" status "blocked"
    set_lane_value "$lane_id" reason "requires_worktree_without_bound_root"
    continue
  fi

  if [ "$default_trigger" = "manual" ]; then
    set_lane_value "$lane_id" status "deferred"
    set_lane_value "$lane_id" reason "manual_trigger_not_selected"
    continue
  fi

  if [ "$MODE" != "release" ] && { [ "$release_gate_class" = "manual_release_gate" ] || [ "$default_trigger" = "release_only" ]; }; then
    set_lane_value "$lane_id" status "release_gate_required"
    set_lane_value "$lane_id" reason "release_mode_required"
    continue
  fi

  if [ "$default_trigger" = "changed_paths" ]; then
    match_result="$(path_matches_hints "$changed_hints_json")"
    if [ "$match_result" = "no_match" ]; then
      set_lane_value "$lane_id" status "skipped"
      set_lane_value "$lane_id" reason "changed_paths_not_matched"
      continue
    fi
  fi
done < "$plan_tsv"

if [ "$PRINT_PLAN" = true ]; then
  echo "PVF lane plan"
  echo "  mode=${MODE}"
  for lane_id in "${lane_ids[@]}"; do
    printf '  - %s status=%s class=%s reason=%s\n' \
      "$lane_id" \
      "$(get_lane_value "$lane_id" status)" \
      "$(get_lane_value "$lane_id" lane_class)" \
      "$(get_lane_value "$lane_id" reason)"
  done
  exit 0
fi

while IFS=$'\t' read -r lane_id lane_class release_gate_class default_trigger requires_worktree requires_credentials changed_hints_json command; do
  if [ -n "$(get_lane_value "$lane_id" status)" ]; then
    continue
  fi

  if [ "$requires_credentials" = "true" ] && [ "$ALLOW_CREDENTIAL_LANES" != "true" ]; then
    set_lane_value "$lane_id" status "blocked"
    set_lane_value "$lane_id" reason "credential_lane_requires_explicit_opt_in"
    continue
  fi

  output_path="$(lane_file "$lane_id" out)"
  exit_path="$(lane_file "$lane_id" exit)"
  pid_path="$(lane_file "$lane_id" pid)"
  (
    cd "$ROOT_DIR"
    export PVF_LANE_ID="$lane_id"
    export PVF_LANE_CLASS="$lane_class"
    if [ -n "$WORKTREE_ROOT" ]; then
      export PVF_WORKTREE_ROOT="$WORKTREE_ROOT"
    fi
    set +e
    bash -lc "$command" >"$output_path" 2>&1
    exit_code=$?
    set -e
    printf '%s\n' "$exit_code" > "$exit_path"
  ) &
  printf '%s\n' "$!" > "$pid_path"
done < "$plan_tsv"

for lane_id in "${lane_ids[@]}"; do
  pid_path="$(lane_file "$lane_id" pid)"
  if [ -f "$pid_path" ]; then
    wait "$(cat "$pid_path")" || true
    exit_code="$(cat "$(lane_file "$lane_id" exit)")"
    if [ "$exit_code" -eq 0 ]; then
      if grep -q '^PVF_STATUS=reused$' "$(lane_file "$lane_id" out)"; then
        set_lane_value "$lane_id" status "reused"
        set_lane_value "$lane_id" reason "artifact_reuse_confirmed"
      else
        set_lane_value "$lane_id" status "passed"
        set_lane_value "$lane_id" reason "command_succeeded"
      fi
    else
      set_lane_value "$lane_id" status "failed"
      set_lane_value "$lane_id" reason "command_failed"
    fi
  fi
done

aggregate_status="skipped"
for lane_id in "${lane_ids[@]}"; do
  status="$(get_lane_value "$lane_id" status)"
  case "$status" in
    failed)
      aggregate_status="failed"
      break
      ;;
    blocked)
      if [ "$aggregate_status" != "failed" ]; then
        aggregate_status="blocked"
      fi
      ;;
    release_gate_required)
      if [ "$aggregate_status" = "passed" ] || [ "$aggregate_status" = "reused" ] || [ "$aggregate_status" = "skipped" ]; then
        aggregate_status="release_gate_required"
      fi
      ;;
    deferred)
      if [ "$aggregate_status" = "passed" ] || [ "$aggregate_status" = "reused" ] || [ "$aggregate_status" = "skipped" ]; then
        aggregate_status="deferred"
      fi
      ;;
    skipped)
      ;;
    reused)
      if [ "$aggregate_status" = "skipped" ]; then
        aggregate_status="passed"
      fi
      ;;
    passed)
      if [ "$aggregate_status" = "skipped" ]; then
        aggregate_status="passed"
      fi
      ;;
  esac
done

echo "PVF lane summary"
echo "  mode=${MODE}"
echo "  aggregate_status=${aggregate_status}"
for lane_id in "${lane_ids[@]}"; do
  printf '  - %s status=%s class=%s reason=%s\n' \
    "$lane_id" \
    "$(get_lane_value "$lane_id" status)" \
    "$(get_lane_value "$lane_id" lane_class)" \
    "$(get_lane_value "$lane_id" reason)"
done

if [ -n "$REPORT_OUT" ]; then
  mkdir -p "$(dirname "$REPORT_OUT")"
fi

report_json="$tmpdir/report.json"
{
  printf '{\n'
  printf '  "schema_version": "adl.pvf_run.v1",\n'
  printf '  "manifest_path": %s,\n' "$(python3 - <<'PY' "$MANIFEST_PATH"
import json, sys
print(json.dumps(sys.argv[1]))
PY
)"
  printf '  "mode": %s,\n' "$(python3 - <<'PY' "$MODE"
import json, sys
print(json.dumps(sys.argv[1]))
PY
)"
  printf '  "aggregate_status": %s,\n' "$(python3 - <<'PY' "$aggregate_status"
import json, sys
print(json.dumps(sys.argv[1]))
PY
)"
  printf '  "lanes": {\n'
  first=true
  for lane_id in "${lane_ids[@]}"; do
    if [ "$first" = true ]; then
      first=false
    else
      printf ',\n'
    fi
    lane_output_path="$(lane_file "$lane_id" out)"
    lane_output_json='""'
    if [ -n "$lane_output_path" ] && [ -f "$lane_output_path" ]; then
      lane_output_json="$(json_escape_file "$lane_output_path")"
    fi
    printf '    %s: {\n' "$(python3 - <<'PY' "$lane_id"
import json, sys
print(json.dumps(sys.argv[1]))
PY
)"
    printf '      "lane_class": %s,\n' "$(python3 - <<'PY' "$(get_lane_value "$lane_id" lane_class)"
import json, sys
print(json.dumps(sys.argv[1]))
PY
)"
    printf '      "status": %s,\n' "$(python3 - <<'PY' "$(get_lane_value "$lane_id" status)"
import json, sys
print(json.dumps(sys.argv[1]))
PY
)"
    printf '      "reason": %s,\n' "$(python3 - <<'PY' "$(get_lane_value "$lane_id" reason)"
import json, sys
print(json.dumps(sys.argv[1]))
PY
)"
    printf '      "output": %s\n' "$lane_output_json"
    printf '    }'
  done
  printf '\n  }\n'
  printf '}\n'
} > "$report_json"

if [ -n "$REPORT_OUT" ]; then
  cp "$report_json" "$REPORT_OUT"
else
  cat "$report_json"
fi

if [ "$aggregate_status" = "failed" ] || [ "$aggregate_status" = "blocked" ]; then
  exit 1
fi

exit 0

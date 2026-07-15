#!/usr/bin/env bash
set -euo pipefail

PROFILE="${AWS_PROFILE:-agent-logic-admin}"
REGION="${AWS_REGION:-us-west-2}"
AWS_CLI="${ADL_AWS_CLI:-aws}"
RUN=false
MAX_AGE_MINUTES=90
ARTIFACT_DIR=""
RUN_ID=""

usage() {
  cat <<'USAGE'
Usage: adl/tools/sweep_aws_spot_orphans.sh [options]

Options:
  --run                  Terminate eligible orphaned builders. Default is dry-run.
  --profile <name>       AWS profile, or env for OIDC credentials.
  --region <region>      AWS region. Defaults to us-west-2.
  --max-age-minutes <n>  Minimum age before a builder is eligible. Minimum 30.
  --artifact-dir <dir>   Optional JSON report directory.
  --run-id <id>          Restrict live termination to one exact run id.

Only instances tagged adl:managed=true and adl:lane=spot-remote-validation
are considered. The retained EBS cache is never modified.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --run) RUN=true; shift ;;
    --profile) PROFILE="${2:-}"; shift 2 ;;
    --region) REGION="${2:-}"; shift 2 ;;
    --max-age-minutes) MAX_AGE_MINUTES="${2:-}"; shift 2 ;;
    --artifact-dir) ARTIFACT_DIR="${2:-}"; shift 2 ;;
    --run-id) RUN_ID="${2:-}"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "sweep_aws_spot_orphans: unknown argument: $1" >&2; usage >&2; exit 2 ;;
  esac
done

if [[ ! "$MAX_AGE_MINUTES" =~ ^[0-9]+$ ]] || (( MAX_AGE_MINUTES < 30 )); then
  echo "sweep_aws_spot_orphans: max age must be at least 30 minutes" >&2
  exit 2
fi
if [[ "$RUN" == true && -z "$RUN_ID" ]]; then
  echo "sweep_aws_spot_orphans: --run requires an exact --run-id" >&2
  exit 2
fi
if [[ -n "$RUN_ID" && ! "$RUN_ID" =~ ^[A-Za-z0-9][A-Za-z0-9._-]{0,127}$ ]]; then
  echo "sweep_aws_spot_orphans: run id has invalid format" >&2
  exit 2
fi

profile_args=()
if [[ "$PROFILE" != "env" && "$PROFILE" != "environment" ]]; then
  profile_args=(--profile "$PROFILE")
fi

filters=(
  Name=tag:adl:managed,Values=true
  Name=tag:adl:lane,Values=spot-remote-validation
  Name=instance-state-name,Values=pending,running,stopping,stopped
)
if [[ -n "$RUN_ID" ]]; then
  filters+=("Name=tag:adl:run_id,Values=$RUN_ID")
fi
raw="$($AWS_CLI ec2 describe-instances \
  ${profile_args[@]+"${profile_args[@]}"} \
  --region "$REGION" \
  --filters "${filters[@]}" \
  --query 'Reservations[].Instances[].{id:InstanceId,run_id:Tags[?Key==`adl:run_id`].Value|[0],launch_time:LaunchTime,state:State.Name}' \
  --output json)"

now="$(date -u +%s)"
report='[]'
rows="$(python3 - "$raw" <<'PY'
import json
import sys
value = json.loads(sys.argv[1])
if not isinstance(value, list):
    raise SystemExit("AWS describe-instances response was not a JSON array")
for item in value:
    if not isinstance(item, dict):
        raise SystemExit("AWS describe-instances response contained a non-object")
    print("\t".join(str(item.get(key) or "") for key in ("id", "run_id", "launch_time", "state")))
PY
)"
while IFS=$'\t' read -r instance_id run_id launch_time state; do
  [[ -n "$instance_id" ]] || continue
  [[ -z "$RUN_ID" || "$run_id" == "$RUN_ID" ]] || continue
  launch_epoch="$(python3 - "$launch_time" <<'PY'
import datetime
import sys
print(int(datetime.datetime.fromisoformat(sys.argv[1].replace("Z", "+00:00")).timestamp()))
PY
  )"
  age_minutes=$(( (now - launch_epoch) / 60 ))
  [[ "$age_minutes" -ge "$MAX_AGE_MINUTES" ]] || continue
  instance_hash="$(printf '%s' "$instance_id" | shasum -a 256 | awk '{print $1}')"
  if [[ "$RUN" == true ]]; then
    "$AWS_CLI" ec2 terminate-instances ${profile_args[@]+"${profile_args[@]}"} --region "$REGION" --instance-ids "$instance_id" >/dev/null
    action="termination_requested"
  else
    action="dry_run_candidate"
  fi
  printf 'orphan_sweep action=%s instance_sha256=%s run_id=%s state=%s age_minutes=%s\n' \
    "$action" "$instance_hash" "${run_id:-unknown}" "$state" "$age_minutes"
  report="$(python3 - "$report" "$instance_hash" "${run_id:-unknown}" "$state" "$age_minutes" "$action" <<'PY'
import json
import sys
items = json.loads(sys.argv[1])
items.append({"instance_id_sha256": sys.argv[2], "run_id": sys.argv[3], "state": sys.argv[4], "age_minutes": int(sys.argv[5]), "action": sys.argv[6]})
print(json.dumps(items, sort_keys=True))
PY
  )"
done <<<"$rows"

if [[ -n "$ARTIFACT_DIR" ]]; then
  mkdir -p "$ARTIFACT_DIR"
  python3 - "$ARTIFACT_DIR/orphan-sweep.json" "$RUN" "$MAX_AGE_MINUTES" "$report" <<'PY'
import json
import sys
path, run, age, report = sys.argv[1:]
with open(path, "w", encoding="utf-8") as handle:
    json.dump({"schema": "adl.aws_spot_orphan_sweep.v1", "run": run == "true", "max_age_minutes": int(age), "candidates": json.loads(report)}, handle, indent=2, sort_keys=True)
    handle.write("\n")
PY
fi

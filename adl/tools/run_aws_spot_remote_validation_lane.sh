#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

PROFILE="${AWS_PROFILE:-agent-logic-admin}"
REGION="${AWS_REGION:-us-west-2}"
ISSUE="4837"
RUN_ID="adl-wp-4837-aws-spot-$(date -u +%Y%m%d%H%M%S)"
COMMAND=""
GIT_REF=""
REPO_URL="https://github.com/danielbaustin/agent-design-language.git"
OUT_PATH=""
ARTIFACT_DIR=""
EXPECTED_PROOF="$ROOT/docs/milestones/v0.91.7/review/build_throughput/remote_validation_4603/live_run_summary_retry11_agentlogic_hotcache.json"
AWS_CLI="${ADL_AWS_CLI:-aws}"
LANE_BIN="${ADL_AWS_REMOTE_VALIDATION_BIN:-}"
RUN=false
CHECK_ACCOUNT=false
JSON=false
PRINT_COMMAND=false
INSTANCE_TYPES=()
CACHE_VOLUME_NAME="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_NAME:-adl-aws-remote-validation-cache-volume}"
CACHE_VOLUME_SIZE_GIB="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_SIZE_GIB:-100}"
CACHE_VOLUME_TYPE="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_TYPE:-gp3}"
CACHE_VOLUME_IOPS="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_IOPS:-3000}"
CACHE_VOLUME_THROUGHPUT_MBPS="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_THROUGHPUT_MBPS:-125}"
CACHE_VOLUME_DEVICE_NAME="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_DEVICE_NAME:-/dev/sdf}"
CACHE_VOLUME_MOUNT_PATH="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_MOUNT_PATH:-/mnt/adl-cache}"
SSH_KEY_NAME="${ADL_AWS_REMOTE_VALIDATION_SSH_KEY_NAME:-adl-4603-agentlogic-ssh-debug-20260701}"
SSH_PRIVATE_KEY_PATH="${ADL_AWS_REMOTE_VALIDATION_SSH_PRIVATE_KEY_PATH:-$HOME/.ssh/adl-4603-ssh-debug-20260701.pem}"
SSH_USER="${ADL_AWS_REMOTE_VALIDATION_SSH_USER:-ec2-user}"
SSH_ALLOWED_CIDR="${ADL_AWS_REMOTE_VALIDATION_SSH_ALLOWED_CIDR:-}"

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/run_aws_spot_remote_validation_lane.sh --command <shell-command> [options]

Options:
  --run                         Launch the AWS Spot remote validation lane.
  --check-account               Verify profile account against retained Agent Logic proof only.
  --print-command               Print the underlying adl-aws-remote-validation command.
  --profile <name>              AWS profile. Defaults to agent-logic-admin. Use env for OIDC/env credentials.
  --region <region>             AWS region. Defaults to us-west-2.
  --issue <number>              Issue recorded in the summary. Defaults to 4837.
  --run-id <id>                 Stable run id for artifacts.
  --command <shell-command>     Remote validation command to run.
  --git-ref <ref>               Remote git ref. Defaults to current branch/ref.
  --repo-url <url>              Remote ADL repository URL.
  --out <path>                  Summary JSON path. Defaults under .adl/tmp.
  --artifact-dir <dir>          Artifact root. Defaults beside --out.
  --instance-type <type>        Add an allowed EC2 instance type.
  --cache-volume-name <name>    Warm EBS cache volume name. Defaults to retained WP-06 cache.
  --cache-volume-size-gib <gib> Cache volume size when created. Defaults to 100.
  --cache-volume-type <type>    Cache volume type. Defaults to gp3.
  --cache-volume-iops <iops>    Cache volume IOPS. Defaults to 3000.
  --cache-volume-throughput-mbps <mbps>
                                Cache volume throughput. Defaults to 125.
  --cache-volume-device-name <device>
                                EC2 device name for attach. Defaults to /dev/sdf.
  --cache-volume-mount-path <path>
                                Remote mount path. Defaults to /mnt/adl-cache.
  --ssh-key-name <name>          EC2 key pair for live remote-tail logging.
                                Defaults to retained Agent Logic debug key.
  --ssh-private-key-path <path>  Private key for live remote-tail logging.
  --ssh-user <user>              SSH user. Defaults to ec2-user.
  --ssh-allowed-cidr <cidr>      SSH source CIDR. Defaults to auto-detected operator IP.
  --expected-proof <summary>    Retained Agent Logic proof summary used for account-hash comparison.
  --bin <path>                  adl-aws-remote-validation binary path.
  --json                        Pass --json to the underlying binary.
  -h, --help                    Show this help.

Without --run the wrapper performs account checking only when --check-account is
present, then prints a dry-run plan. It never launches EC2 unless --run is set.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --run)
      RUN=true
      shift
      ;;
    --check-account)
      CHECK_ACCOUNT=true
      shift
      ;;
    --print-command)
      PRINT_COMMAND=true
      shift
      ;;
    --profile)
      PROFILE="${2:-}"
      shift 2
      ;;
    --region)
      REGION="${2:-}"
      shift 2
      ;;
    --issue)
      ISSUE="${2:-}"
      shift 2
      ;;
    --run-id)
      RUN_ID="${2:-}"
      shift 2
      ;;
    --command)
      COMMAND="${2:-}"
      shift 2
      ;;
    --git-ref)
      GIT_REF="${2:-}"
      shift 2
      ;;
    --repo-url)
      REPO_URL="${2:-}"
      shift 2
      ;;
    --out)
      OUT_PATH="${2:-}"
      shift 2
      ;;
    --artifact-dir)
      ARTIFACT_DIR="${2:-}"
      shift 2
      ;;
    --instance-type)
      INSTANCE_TYPES+=("${2:-}")
      shift 2
      ;;
    --cache-volume-name)
      CACHE_VOLUME_NAME="${2:-}"
      shift 2
      ;;
    --cache-volume-size-gib)
      CACHE_VOLUME_SIZE_GIB="${2:-}"
      shift 2
      ;;
    --cache-volume-type)
      CACHE_VOLUME_TYPE="${2:-}"
      shift 2
      ;;
    --cache-volume-iops)
      CACHE_VOLUME_IOPS="${2:-}"
      shift 2
      ;;
    --cache-volume-throughput-mbps)
      CACHE_VOLUME_THROUGHPUT_MBPS="${2:-}"
      shift 2
      ;;
    --cache-volume-device-name)
      CACHE_VOLUME_DEVICE_NAME="${2:-}"
      shift 2
      ;;
    --cache-volume-mount-path)
      CACHE_VOLUME_MOUNT_PATH="${2:-}"
      shift 2
      ;;
    --ssh-key-name)
      SSH_KEY_NAME="${2:-}"
      shift 2
      ;;
    --ssh-private-key-path)
      SSH_PRIVATE_KEY_PATH="${2:-}"
      shift 2
      ;;
    --ssh-user)
      SSH_USER="${2:-}"
      shift 2
      ;;
    --ssh-allowed-cidr)
      SSH_ALLOWED_CIDR="${2:-}"
      shift 2
      ;;
    --expected-proof)
      EXPECTED_PROOF="${2:-}"
      shift 2
      ;;
    --bin)
      LANE_BIN="${2:-}"
      shift 2
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
      echo "run_aws_spot_remote_validation_lane: unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -z "$PROFILE" ]]; then
  echo "run_aws_spot_remote_validation_lane: --profile must not be empty" >&2
  exit 2
fi

if [[ -z "$CACHE_VOLUME_NAME" ]]; then
  echo "run_aws_spot_remote_validation_lane: cache volume name must not be empty" >&2
  exit 2
fi

if [[ -z "$GIT_REF" ]]; then
  GIT_REF="$(git -C "$ROOT" symbolic-ref --quiet --short HEAD 2>/dev/null || git -C "$ROOT" rev-parse HEAD)"
fi

if [[ -z "$OUT_PATH" ]]; then
  OUT_PATH="$ROOT/.adl/tmp/aws-spot-remote-validation/$RUN_ID/summary.json"
fi

if [[ -z "$ARTIFACT_DIR" ]]; then
  ARTIFACT_DIR="$(dirname "$OUT_PATH")/artifacts"
fi

if [[ -z "$LANE_BIN" ]]; then
  if [[ -x "$ROOT/tools/aws_remote_validation/target/debug/adl-aws-remote-validation" ]]; then
    LANE_BIN="$ROOT/tools/aws_remote_validation/target/debug/adl-aws-remote-validation"
  elif [[ -x "$ROOT/adl/target/debug/adl-aws-remote-validation" ]]; then
    LANE_BIN="$ROOT/adl/target/debug/adl-aws-remote-validation"
  else
    LANE_BIN="$ROOT/tools/aws_remote_validation/target/debug/adl-aws-remote-validation"
  fi
fi

check_account() {
  local identity_json
  local aws_profile_args=()
  if [[ "$PROFILE" != "env" && "$PROFILE" != "environment" ]]; then
    aws_profile_args=(--profile "$PROFILE")
  fi
  identity_json="$(mktemp "${TMPDIR:-/tmp}/adl-aws-identity.XXXXXX")"
  trap 'rm -f "$identity_json"' RETURN
  "$AWS_CLI" sts get-caller-identity "${aws_profile_args[@]}" --output json >"$identity_json"
  python3 - "$identity_json" "$EXPECTED_PROOF" "$PROFILE" <<'PY'
import hashlib
import json
import sys

identity_path, proof_path, profile = sys.argv[1:4]
identity = json.load(open(identity_path, encoding="utf-8"))
proof = json.load(open(proof_path, encoding="utf-8"))
account = identity.get("Account")
if not account:
    raise SystemExit("run_aws_spot_remote_validation_lane: AWS profile did not return an account")
expected = (proof.get("account_identity") or {}).get("account_id_sha256")
if not expected:
    raise SystemExit("run_aws_spot_remote_validation_lane: retained proof has no account hash")
observed = hashlib.sha256(account.encode("utf-8")).hexdigest()
if observed != expected:
    raise SystemExit(
        "run_aws_spot_remote_validation_lane: AWS profile account does not match retained Agent Logic proof"
    )
arn_present = bool(identity.get("Arn"))
user_id_present = bool(identity.get("UserId"))
print(
    f"PASS account_profile_resolved profile={profile} "
    f"account_matches_retained_proof=true arn_present={str(arn_present).lower()} "
    f"user_id_present={str(user_id_present).lower()}"
)
PY
}

if [[ "$CHECK_ACCOUNT" == true || "$RUN" == true ]]; then
  check_account
fi

if [[ -z "$COMMAND" ]]; then
  if [[ "$RUN" == true ]]; then
    echo "run_aws_spot_remote_validation_lane: --command is required when --run is set" >&2
    exit 2
  fi
fi

cmd=(
  "$LANE_BIN"
  run
  --issue "$ISSUE"
  --run-id "$RUN_ID"
  --profile "$PROFILE"
  --region "$REGION"
  --repo-url "$REPO_URL"
  --git-ref "$GIT_REF"
  --out "$OUT_PATH"
  --artifact-dir "$ARTIFACT_DIR"
  --cache-volume-name "$CACHE_VOLUME_NAME"
  --cache-volume-size-gib "$CACHE_VOLUME_SIZE_GIB"
  --cache-volume-type "$CACHE_VOLUME_TYPE"
  --cache-volume-iops "$CACHE_VOLUME_IOPS"
  --cache-volume-throughput-mbps "$CACHE_VOLUME_THROUGHPUT_MBPS"
  --cache-volume-device-name "$CACHE_VOLUME_DEVICE_NAME"
  --cache-volume-mount-path "$CACHE_VOLUME_MOUNT_PATH"
)

if [[ -n "$SSH_KEY_NAME" ]]; then
  cmd+=(--ssh-key-name "$SSH_KEY_NAME")
  cmd+=(--ssh-private-key-path "$SSH_PRIVATE_KEY_PATH")
  cmd+=(--ssh-user "$SSH_USER")
  if [[ -n "$SSH_ALLOWED_CIDR" ]]; then
    cmd+=(--ssh-allowed-cidr "$SSH_ALLOWED_CIDR")
  fi
fi

if [[ -n "$COMMAND" ]]; then
  cmd+=(--command "$COMMAND")
fi

for instance_type in ${INSTANCE_TYPES[@]+"${INSTANCE_TYPES[@]}"}; do
  cmd+=(--instance-type "$instance_type")
done

if [[ "$JSON" == true ]]; then
  cmd+=(--json)
fi

if [[ "$PRINT_COMMAND" == true ]]; then
  printf '%q ' "${cmd[@]}"
  printf '\n'
fi

if [[ "$RUN" != true ]]; then
  echo "DRY-RUN aws_spot_remote_validation profile=$PROFILE region=$REGION git_ref=$GIT_REF out=$OUT_PATH artifact_dir=$ARTIFACT_DIR cache_volume=$CACHE_VOLUME_NAME cache_mount=$CACHE_VOLUME_MOUNT_PATH ssh_tail_enabled=$([[ -n "$SSH_KEY_NAME" ]] && printf true || printf false)"
  echo "DRY-RUN no EC2 resources launched; pass --run to execute"
  exit 0
fi

if [[ ! -x "$LANE_BIN" ]]; then
  echo "run_aws_spot_remote_validation_lane: binary not executable: $LANE_BIN" >&2
  exit 2
fi

mkdir -p "$(dirname "$OUT_PATH")" "$ARTIFACT_DIR"
runner_stdout="$(mktemp "${TMPDIR:-/tmp}/adl-aws-spot-runner-stdout.XXXXXX")"
runner_stderr="$(mktemp "${TMPDIR:-/tmp}/adl-aws-spot-runner-stderr.XXXXXX")"
cleanup_runner_logs() {
  rm -f "$runner_stdout" "$runner_stderr"
}
trap cleanup_runner_logs EXIT

set +e
"${cmd[@]}" >"$runner_stdout" 2>"$runner_stderr"
runner_status="$?"
set -e

python3 - <<'PY' "$runner_stdout"
import json
import re
import sys
from pathlib import Path

path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8", errors="replace")
try:
    payload = json.loads(text)
except json.JSONDecodeError:
    redacted = re.sub(r"\b\d{12}\b", "<aws-account-id-redacted>", text)
    redacted = re.sub(r"arn:aws:[^\s,\"]+", "<aws-arn-redacted>", redacted)
    print(redacted, end="")
    raise SystemExit(0)

identity = payload.get("account_identity")
if isinstance(identity, dict):
    for key in ("account_id", "arn", "user_id"):
        if key in identity:
            identity[key] = "<redacted>"
for container_key in ("command",):
    container = payload.get(container_key)
    if isinstance(container, dict):
        for key in ("stdout_preview", "stderr_preview", "output_preview"):
            if key in container and isinstance(container[key], str):
                container[key] = re.sub(r"\b\d{12}\b", "<aws-account-id-redacted>", container[key])
                container[key] = re.sub(r"arn:aws:[^\s,\"]+", "<aws-arn-redacted>", container[key])
print(json.dumps(payload, indent=2, sort_keys=False))
PY

cat "$runner_stderr" >&2
exit "$runner_status"

#!/usr/bin/env bash
set -euo pipefail

ORIGINAL_ARGS=("$@")

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
COMMON_GIT_DIR="$(git -C "$ROOT" rev-parse --path-format=absolute --git-common-dir 2>/dev/null || true)"
PRIMARY_ROOT="${COMMON_GIT_DIR:+$(dirname "$COMMON_GIT_DIR")}"
PROCESS_BIN="${ADL_PROCESS_BIN:-${PRIMARY_ROOT:-$ROOT}/adl/target/debug/adl}"

ACTION="plan"
if [[ $# -gt 0 ]]; then
  case "$1" in
    preflight|launch|run|status|logs|ssh|stop|cleanup)
      ACTION="$1"
      shift
      ;;
  esac
fi

PROFILE="${AWS_PROFILE:-agent-logic-admin}"
REGION="${AWS_REGION:-us-west-2}"
ISSUE="5191"
RUN_ID=""
COMMAND=""
GIT_REF=""
SOURCE_COMMIT=""
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
FOLLOW=false
INSTANCE_TYPES=()
CACHE_VOLUME_NAME="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_NAME:-adl-aws-remote-validation-cache-volume}"
CACHE_VOLUME_SIZE_GIB="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_SIZE_GIB:-1000}"
CACHE_VOLUME_TYPE="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_TYPE:-gp3}"
CACHE_VOLUME_IOPS="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_IOPS:-3000}"
CACHE_VOLUME_THROUGHPUT_MBPS="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_THROUGHPUT_MBPS:-125}"
CACHE_VOLUME_DEVICE_NAME="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_DEVICE_NAME:-/dev/sdf}"
CACHE_VOLUME_MOUNT_PATH="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_MOUNT_PATH:-/mnt/adl-cache}"
REMOTE_COMMAND_TIMEOUT_SECONDS="${ADL_AWS_REMOTE_VALIDATION_COMMAND_TIMEOUT_SECONDS:-600}"
SSH_KEY_NAME="${ADL_AWS_REMOTE_VALIDATION_SSH_KEY_NAME:-adl-wp06-spot-ssh-debug-20260704}"
SSH_PRIVATE_KEY_PATH="${ADL_AWS_REMOTE_VALIDATION_SSH_PRIVATE_KEY_PATH:-$HOME/.ssh/adl-4603-ssh-debug-20260701.pem}"
SSH_USER="${ADL_AWS_REMOTE_VALIDATION_SSH_USER:-ec2-user}"
SSH_ALLOWED_CIDR="${ADL_AWS_REMOTE_VALIDATION_SSH_ALLOWED_CIDR:-}"
SSH_BIN="${ADL_SSH_BIN:-ssh}"
BUILDER_IMAGE="${ADL_AWS_SPOT_BUILDER_IMAGE:-}"
BUILDER_IMAGE_REPOSITORY="${ADL_AWS_SPOT_BUILDER_IMAGE_REPOSITORY:-adl-builder}"
BUILDER_IMAGE_TAG="${ADL_AWS_SPOT_BUILDER_IMAGE_TAG:-v0.91.7-coverage-5243}"
EXPECTED_ARCHITECTURE="${ADL_AWS_SPOT_EXPECTED_ARCHITECTURE:-x86_64}"
MIN_CACHE_FREE_GIB="${ADL_AWS_SPOT_MIN_CACHE_FREE_GIB:-10}"
ESTIMATED_HOURLY_COST_USD="${ADL_AWS_SPOT_ESTIMATED_HOURLY_COST_USD:-}"
AMI_ID="${ADL_AWS_REMOTE_VALIDATION_AMI_ID:-}"
SUBNET_ID="${ADL_AWS_REMOTE_VALIDATION_SUBNET_ID:-}"
EXPECTED_CACHE_VOLUME_ID_SHA256="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_ID_SHA256:-}"
RETAINED_CACHE_VOLUME_ID=""

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/run_aws_spot_remote_validation_lane.sh preflight [options]
  adl/tools/run_aws_spot_remote_validation_lane.sh [launch|run] --command <shell-command> [options]
  adl/tools/run_aws_spot_remote_validation_lane.sh status|logs|ssh|stop|cleanup --run-id <id> [options]

Options:
  --run                         Launch the AWS Spot remote validation lane.
  --check-account               Verify profile account against retained Agent Logic proof only.
  --print-command               Print the underlying adl-aws-remote-validation command.
  --profile <name>              AWS profile. Defaults to agent-logic-admin. Use env for OIDC/env credentials.
  --region <region>             AWS region. Defaults to us-west-2.
  --issue <number>              Issue recorded in the summary. Defaults to 5191.
  --run-id <id>                 Stable run id for artifacts.
  --command <shell-command>     Remote validation command to run.
  --git-ref <ref>               Remote git ref. Defaults to current branch/ref.
  --repo-url <url>              Remote ADL repository URL.
  --out <path>                  Summary JSON path. Defaults under .adl/tmp.
  --artifact-dir <dir>          Artifact root. Defaults beside --out.
  --instance-type <type>        Add an allowed EC2 instance type.
  --cache-volume-name <name>    Warm EBS cache volume name. Defaults to retained WP-06 cache.
  --cache-volume-size-gib <gib> Cache volume size when created. Defaults to 1000.
  --cache-volume-type <type>    Cache volume type. Defaults to gp3.
  --cache-volume-iops <iops>    Cache volume IOPS. Defaults to 3000.
  --cache-volume-throughput-mbps <mbps>
                                Cache volume throughput. Defaults to 125.
  --cache-volume-device-name <device>
                                EC2 device name for attach. Defaults to /dev/sdf.
  --cache-volume-mount-path <path>
                                Remote mount path. Defaults to /mnt/adl-cache.
                                The retained warm EBS cache is forwarded by
                                default; it is not by itself proof that a
                                builder image was used.
  --ssh-key-name <name>          EC2 key pair for live remote-tail logging.
                                Defaults to retained Agent Logic debug key.
  --ssh-private-key-path <path>  Private key for live remote-tail logging.
  --ssh-user <user>              SSH user. Defaults to ec2-user.
  --ssh-allowed-cidr <cidr>      SSH source CIDR. Defaults to the current public IP only.
  --builder-image <uri@digest>   Immutable builder image. Defaults to resolving
                                adl-builder:v0.91.7-coverage-5243 in Agent Logic ECR.
  --builder-image-repository <name>
                                ECR repository used for default digest resolution.
  --builder-image-tag <tag>      ECR tag resolved once to an immutable digest.
  --expected-architecture <arch> Expected image/runtime architecture. Defaults x86_64.
  --min-cache-free-gib <gib>     Required warm-cache headroom. Defaults 10.
  --estimated-hourly-cost-usd <usd>
                                Override the pre-run Spot hourly price estimate.
  --ami-id <id>                 Explicit AMI. Defaults to the current AL2023 SSM image.
  --subnet-id <id>              Explicit subnet. Defaults to retained hot-cache proof topology.
  --expected-cache-volume-id-sha256 <hash>
                                Expected retained EBS identity hash.
  --expected-proof <summary>    Retained Agent Logic proof summary used for account-hash comparison.
  --bin <path>                  adl-aws-remote-validation binary path.
  --json                        Pass --json to the underlying binary.
  --follow                      Follow logs until interrupted (logs action only).
  -h, --help                    Show this help.

Without --run the wrapper performs account checking only when --check-account is
present, then prints a dry-run plan. It never launches EC2 unless --run is set.
The `launch` action is the explicit asynchronous paid path; `run --run` is the
synchronous paid path. Status, logs, SSH, stop, and cleanup reuse --run-id.

Live runs always resolve or require an immutable builder-image digest, verify
the image toolchain and architecture, and execute the requested validation
inside that image. Rust validation tools are never installed on the host.
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
    --builder-image)
      BUILDER_IMAGE="${2:-}"
      shift 2
      ;;
    --builder-image-repository)
      BUILDER_IMAGE_REPOSITORY="${2:-}"
      shift 2
      ;;
    --builder-image-tag)
      BUILDER_IMAGE_TAG="${2:-}"
      shift 2
      ;;
    --expected-architecture)
      EXPECTED_ARCHITECTURE="${2:-}"
      shift 2
      ;;
    --min-cache-free-gib)
      MIN_CACHE_FREE_GIB="${2:-}"
      shift 2
      ;;
    --estimated-hourly-cost-usd)
      ESTIMATED_HOURLY_COST_USD="${2:-}"
      shift 2
      ;;
    --ami-id)
      AMI_ID="${2:-}"
      shift 2
      ;;
    --subnet-id)
      SUBNET_ID="${2:-}"
      shift 2
      ;;
    --expected-cache-volume-id-sha256)
      EXPECTED_CACHE_VOLUME_ID_SHA256="${2:-}"
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
    --follow)
      FOLLOW=true
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

if [[ "$ACTION" == "launch" ]]; then
  RUN=true
elif [[ "$ACTION" == "preflight" ]]; then
  CHECK_ACCOUNT=true
elif [[ "$ACTION" == "run" && "$RUN" != true ]]; then
  echo "run_aws_spot_remote_validation_lane: the run action requires explicit --run" >&2
  exit 2
fi

if [[ ${#INSTANCE_TYPES[@]} -eq 0 ]]; then
  INSTANCE_TYPES=("m7a.2xlarge" "c7a.2xlarge" "c7i.2xlarge")
fi

if [[ -z "$RUN_ID" ]]; then
  RUN_ID="adl-wp-${ISSUE}-aws-spot-$(date -u +%Y%m%d%H%M%S)"
fi

if [[ -z "$PROFILE" ]]; then
  echo "run_aws_spot_remote_validation_lane: --profile must not be empty" >&2
  exit 2
fi

if [[ -z "$CACHE_VOLUME_NAME" ]]; then
  echo "run_aws_spot_remote_validation_lane: cache volume name must not be empty" >&2
  exit 2
fi

if [[ ! "$MIN_CACHE_FREE_GIB" =~ ^[0-9]+$ ]] || [[ "$MIN_CACHE_FREE_GIB" -lt 1 ]]; then
  echo "run_aws_spot_remote_validation_lane: --min-cache-free-gib must be a positive integer" >&2
  exit 2
fi
if [[ ! "$REMOTE_COMMAND_TIMEOUT_SECONDS" =~ ^[0-9]+$ ]] || (( REMOTE_COMMAND_TIMEOUT_SECONDS < 1 || REMOTE_COMMAND_TIMEOUT_SECONDS > 600 )); then
  echo "run_aws_spot_remote_validation_lane: command timeout must be between 1 and 600 seconds" >&2
  exit 2
fi
if [[ -n "$ESTIMATED_HOURLY_COST_USD" ]] && [[ ! "$ESTIMATED_HOURLY_COST_USD" =~ ^[0-9]+([.][0-9]+)?$ ]]; then
  echo "run_aws_spot_remote_validation_lane: --estimated-hourly-cost-usd must be numeric" >&2
  exit 2
fi

if [[ -z "$GIT_REF" ]]; then
  GIT_REF="$(git -C "$ROOT" symbolic-ref --quiet --short HEAD 2>/dev/null || git -C "$ROOT" rev-parse HEAD)"
fi
SOURCE_COMMIT="$(git -C "$ROOT" rev-parse --verify "${GIT_REF}^{commit}" 2>/dev/null || true)"
if [[ ! "$SOURCE_COMMIT" =~ ^[0-9a-f]{40}$ ]]; then
  SOURCE_COMMIT="$(git -C "$ROOT" rev-parse --verify "refs/remotes/origin/${GIT_REF}^{commit}" 2>/dev/null || true)"
fi
if [[ ( "$RUN" == true || "$ACTION" == "preflight" ) && ! "$SOURCE_COMMIT" =~ ^[0-9a-f]{40}$ ]]; then
  echo "run_aws_spot_remote_validation_lane: --git-ref must resolve to a committed source revision" >&2
  exit 2
fi

if [[ -z "$OUT_PATH" ]]; then
  OUT_PATH="$ROOT/.adl/tmp/aws-spot-remote-validation/$RUN_ID/summary.json"
fi

if [[ -z "$ARTIFACT_DIR" ]]; then
  ARTIFACT_DIR="$(dirname "$OUT_PATH")/artifacts"
fi
export ADL_SSH_KNOWN_HOSTS_FILE="$ARTIFACT_DIR/ssh-known-hosts"

if [[ -z "$LANE_BIN" ]]; then
  if [[ -x "${PRIMARY_ROOT:-$ROOT}/.adl/bin/adl-aws-remote-validation" ]]; then
    LANE_BIN="${PRIMARY_ROOT:-$ROOT}/.adl/bin/adl-aws-remote-validation"
  elif [[ -x "$ROOT/tools/aws_remote_validation/target/debug/adl-aws-remote-validation" ]]; then
    LANE_BIN="$ROOT/tools/aws_remote_validation/target/debug/adl-aws-remote-validation"
  elif [[ -x "$ROOT/adl/target/debug/adl-aws-remote-validation" ]]; then
    LANE_BIN="$ROOT/adl/target/debug/adl-aws-remote-validation"
  else
    LANE_BIN="$ROOT/tools/aws_remote_validation/target/debug/adl-aws-remote-validation"
  fi
fi

verify_lane_binary_compatibility() {
  local capabilities_json
  if [[ ! -x "$LANE_BIN" ]]; then
    echo "run_aws_spot_remote_validation_lane: remote validation binary is unavailable: $LANE_BIN" >&2
    return 1
  fi
  if ! capabilities_json="$("$LANE_BIN" capabilities 2>/dev/null)" || \
      ! ADL_AWS_REMOTE_CAPABILITIES="$capabilities_json" python3 - <<'PY' >/dev/null 2>&1
import json
import os

payload = json.loads(os.environ["ADL_AWS_REMOTE_CAPABILITIES"])
assert payload.get("schema") == "adl.aws_remote_validation.capabilities.v1"
assert "embedded_control_bundle_v1" in payload.get("capabilities", [])
assert "spot_only_v1" in payload.get("capabilities", [])
PY
  then
    echo "run_aws_spot_remote_validation_lane: remote validation binary is stale; install the current owner binary or set ADL_AWS_REMOTE_VALIDATION_BIN" >&2
    return 1
  fi
}

check_account() {
  local identity_json
  local aws_profile_args=()
  if [[ "$PROFILE" != "env" && "$PROFILE" != "environment" ]]; then
    aws_profile_args=(--profile "$PROFILE")
  fi
  if ! identity_json="$("$AWS_CLI" sts get-caller-identity ${aws_profile_args[@]+"${aws_profile_args[@]}"} --output json)"; then
    return 1
  fi
  local account_status=0
  ADL_AWS_IDENTITY_JSON="$identity_json" python3 - "$EXPECTED_PROOF" "$PROFILE" <<'PY' || account_status="$?"
import hashlib
import json
import os
import sys

proof_path, profile = sys.argv[1:3]
identity = json.loads(os.environ["ADL_AWS_IDENTITY_JSON"])
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
  return "$account_status"
}

resolve_builder_image() {
  if [[ -n "$BUILDER_IMAGE" ]]; then
    [[ "$BUILDER_IMAGE" =~ @sha256:[0-9a-f]{64}$ ]] || {
      echo "run_aws_spot_remote_validation_lane: --builder-image must use an immutable sha256 digest" >&2
      return 2
    }
    return 0
  fi
  local profile_args=()
  if [[ "$PROFILE" != "env" && "$PROFILE" != "environment" ]]; then
    profile_args=(--profile "$PROFILE")
  fi
  local account digest
  account="$("$AWS_CLI" sts get-caller-identity ${profile_args[@]+"${profile_args[@]}"} --query Account --output text)"
  digest="$("$AWS_CLI" ecr describe-images ${profile_args[@]+"${profile_args[@]}"} --region "$REGION" \
    --repository-name "$BUILDER_IMAGE_REPOSITORY" \
    --image-ids "imageTag=$BUILDER_IMAGE_TAG" \
    --query 'imageDetails[0].imageDigest' --output text)"
  if [[ ! "$account" =~ ^[0-9]{12}$ ]] || [[ ! "$digest" =~ ^sha256:[0-9a-f]{64}$ ]]; then
    echo "run_aws_spot_remote_validation_lane: failed to resolve immutable Agent Logic builder image" >&2
    return 1
  fi
  BUILDER_IMAGE="$account.dkr.ecr.$REGION.amazonaws.com/$BUILDER_IMAGE_REPOSITORY@$digest"
}

resolve_spot_hourly_cost() {
  if [[ -n "$ESTIMATED_HOURLY_COST_USD" ]]; then
    return 0
  fi
  local profile_args=()
  if [[ "$PROFILE" != "env" && "$PROFILE" != "environment" ]]; then
    profile_args=(--profile "$PROFILE")
  fi
  local price_json
  price_json="$("$AWS_CLI" ec2 describe-spot-price-history \
    ${profile_args[@]+"${profile_args[@]}"} --region "$REGION" --instance-types "${INSTANCE_TYPES[@]}" \
    --product-descriptions Linux/UNIX --max-items 20 --output json)"
  ESTIMATED_HOURLY_COST_USD="$(python3 -c 'import json,sys; values=[float(x["SpotPrice"]) for x in json.load(sys.stdin).get("SpotPriceHistory",[])]; print(max(values) if values else "")' <<<"$price_json")"
  if [[ ! "$ESTIMATED_HOURLY_COST_USD" =~ ^[0-9]+([.][0-9]+)?$ ]]; then
    echo "run_aws_spot_remote_validation_lane: failed to resolve Spot hourly price" >&2
    return 1
  fi
}

resolve_and_verify_retained_topology() {
  local proof_topology proof_volume_id proof_subnet_id proof_volume_hash
  proof_topology="$(python3 - "$EXPECTED_PROOF" <<'PY'
import hashlib
import json
import sys

proof = json.load(open(sys.argv[1], encoding="utf-8"))
volume = proof.get("cache_volume") or {}
surface = proof.get("launch_surface") or {}
volume_id = volume.get("volume_id", "")
subnet_id = surface.get("subnet_id", "")
if not volume_id or not subnet_id:
    raise SystemExit("retained proof is missing cache volume or subnet identity")
print(volume_id, subnet_id, hashlib.sha256(volume_id.encode()).hexdigest())
PY
)"
  read -r proof_volume_id proof_subnet_id proof_volume_hash <<<"$proof_topology"
  RETAINED_CACHE_VOLUME_ID="$proof_volume_id"
  if [[ -z "$SUBNET_ID" ]]; then
    SUBNET_ID="$proof_subnet_id"
  fi
  if [[ -z "$EXPECTED_CACHE_VOLUME_ID_SHA256" ]]; then
    EXPECTED_CACHE_VOLUME_ID_SHA256="$proof_volume_hash"
  fi
  [[ "$EXPECTED_CACHE_VOLUME_ID_SHA256" =~ ^[0-9a-f]{64}$ ]] || {
    echo "run_aws_spot_remote_validation_lane: expected cache volume identity hash is invalid" >&2
    return 1
  }

  local profile_args=()
  if [[ "$PROFILE" != "env" && "$PROFILE" != "environment" ]]; then
    profile_args=(--profile "$PROFILE")
  fi
  local volume_state volume_name volume_az subnet_az volume_hash matching_volume_count
  local volume_size volume_type volume_iops volume_throughput
  volume_hash="$(python3 -c 'import hashlib,sys; print(hashlib.sha256(sys.argv[1].encode()).hexdigest())' "$proof_volume_id")"
  [[ "$volume_hash" == "$EXPECTED_CACHE_VOLUME_ID_SHA256" ]] || {
    echo "run_aws_spot_remote_validation_lane: retained proof cache identity mismatch" >&2
    return 1
  }
  volume_state="$("$AWS_CLI" ec2 describe-volumes ${profile_args[@]+"${profile_args[@]}"} --region "$REGION" \
    --volume-ids "$proof_volume_id" --query 'Volumes[0].State' --output text)"
  volume_name="$("$AWS_CLI" ec2 describe-volumes ${profile_args[@]+"${profile_args[@]}"} --region "$REGION" \
    --volume-ids "$proof_volume_id" --query 'Volumes[0].Tags[?Key==`Name`].Value|[0]' --output text)"
  volume_az="$("$AWS_CLI" ec2 describe-volumes ${profile_args[@]+"${profile_args[@]}"} --region "$REGION" \
    --volume-ids "$proof_volume_id" --query 'Volumes[0].AvailabilityZone' --output text)"
  volume_size="$("$AWS_CLI" ec2 describe-volumes ${profile_args[@]+"${profile_args[@]}"} --region "$REGION" \
    --volume-ids "$proof_volume_id" --query 'Volumes[0].Size' --output text)"
  volume_type="$("$AWS_CLI" ec2 describe-volumes ${profile_args[@]+"${profile_args[@]}"} --region "$REGION" \
    --volume-ids "$proof_volume_id" --query 'Volumes[0].VolumeType' --output text)"
  volume_iops="$("$AWS_CLI" ec2 describe-volumes ${profile_args[@]+"${profile_args[@]}"} --region "$REGION" \
    --volume-ids "$proof_volume_id" --query 'Volumes[0].Iops' --output text)"
  volume_throughput="$("$AWS_CLI" ec2 describe-volumes ${profile_args[@]+"${profile_args[@]}"} --region "$REGION" \
    --volume-ids "$proof_volume_id" --query 'Volumes[0].Throughput' --output text)"
  subnet_az="$("$AWS_CLI" ec2 describe-subnets ${profile_args[@]+"${profile_args[@]}"} --region "$REGION" \
    --subnet-ids "$SUBNET_ID" --query 'Subnets[0].AvailabilityZone' --output text)"
  [[ "$volume_state" == "available" ]] || {
    echo "run_aws_spot_remote_validation_lane: retained cache volume is not exclusively available" >&2
    return 1
  }
  [[ "$volume_name" == "$CACHE_VOLUME_NAME" && -n "$volume_az" && "$volume_az" == "$subnet_az" ]] || {
    echo "run_aws_spot_remote_validation_lane: retained cache volume and subnet topology mismatch" >&2
    return 1
  }
  matching_volume_count="$("$AWS_CLI" ec2 describe-volumes ${profile_args[@]+"${profile_args[@]}"} --region "$REGION" \
    --filters "Name=tag:Name,Values=$CACHE_VOLUME_NAME" "Name=availability-zone,Values=$volume_az" \
    --query 'length(Volumes)' --output text)"
  [[ "$matching_volume_count" == "1" ]] || {
    echo "run_aws_spot_remote_validation_lane: retained cache identity is ambiguous in the selected availability zone" >&2
    return 1
  }
  [[ "$volume_size" == "$CACHE_VOLUME_SIZE_GIB" && "$volume_type" == "$CACHE_VOLUME_TYPE" \
      && "$volume_iops" == "$CACHE_VOLUME_IOPS" && "$volume_throughput" == "$CACHE_VOLUME_THROUGHPUT_MBPS" ]] || {
    echo "run_aws_spot_remote_validation_lane: retained cache volume shape mismatch" >&2
    return 1
  }
  if [[ -z "$AMI_ID" ]]; then
    AMI_ID="$("$AWS_CLI" ssm get-parameter ${profile_args[@]+"${profile_args[@]}"} --region "$REGION" \
      --name /aws/service/ami-amazon-linux-latest/al2023-ami-kernel-default-x86_64 \
      --query 'Parameter.Value' --output text)"
  fi
  [[ "$AMI_ID" =~ ^ami-[0-9a-f]{8,17}$ && "$SUBNET_ID" =~ ^subnet-[0-9a-f]{8,17}$ ]] || {
    echo "run_aws_spot_remote_validation_lane: AMI or subnet resolution failed" >&2
    return 1
  }
}

shell_quote() {
  printf '%q' "$1"
}

verify_ssh_recovery_key() {
  local key_mode
  [[ -n "$SSH_KEY_NAME" && -f "$SSH_PRIVATE_KEY_PATH" ]] || {
    echo "run_aws_spot_remote_validation_lane: SSH recovery key is not configured" >&2
    return 1
  }
  key_mode="$(stat -c '%a' "$SSH_PRIVATE_KEY_PATH" 2>/dev/null || stat -f '%Lp' "$SSH_PRIVATE_KEY_PATH")"
  [[ "$key_mode" == "600" || "$key_mode" == "400" ]] || {
    echo "run_aws_spot_remote_validation_lane: SSH private key permissions must be 600 or 400" >&2
    return 1
  }
  ssh-keygen -y -P '' -f "$SSH_PRIVATE_KEY_PATH" >/dev/null 2>&1 || {
    echo "run_aws_spot_remote_validation_lane: SSH private key is not passphraseless" >&2
    return 1
  }
}

redact_stream() {
  sed -E \
    -e 's/[0-9]{12}/<aws-account-id-redacted>/g' \
    -e 's#arn:aws[^[:space:],\"]*#<aws-arn-redacted>#g' \
    -e 's/i-[0-9a-f]{8,17}/<ec2-instance-id-redacted>/g' \
    -e 's/vol-[0-9a-f]{8,17}/<ebs-volume-id-redacted>/g' \
    -e 's/(vpc|subnet|sg|sir)-[0-9a-f]{8,17}/<aws-resource-id-redacted>/g' \
    -e 's/([0-9]{1,3}\.){3}[0-9]{1,3}/<ip-address-redacted>/g'
}

manager_is_active() {
  local pid_file="$ARTIFACT_DIR/manager.pid"
  [[ -f "$pid_file" ]] || return 1
  [[ -x "$PROCESS_BIN" ]] || return 1
  "$PROCESS_BIN" process status --pid-file "$pid_file" --json 2>/dev/null \
    | python3 -c 'import json,sys; data=json.load(sys.stdin); raise SystemExit(0 if data.get("running") or data.get("status") == "running" else 1)'
}

private_command_status_path() {
  if [[ -f "$ARTIFACT_DIR/.private/command-status.log" ]]; then
    printf '%s\n' "$ARTIFACT_DIR/.private/command-status.log"
  elif find "$ARTIFACT_DIR" -maxdepth 2 -path '*/attempt-*/command-status.log' -type f -print -quit 2>/dev/null | grep -q .; then
    find "$ARTIFACT_DIR" -maxdepth 2 -path '*/attempt-*/command-status.log' -type f -print 2>/dev/null | sort | tail -n 1
  else
    printf '%s\n' "$ARTIFACT_DIR/command-status.log"
  fi
}

run_status_action() {
  if [[ -f "$ARTIFACT_DIR/wrapper-final-summary.json" ]]; then
    cat "$ARTIFACT_DIR/wrapper-final-summary.json"
  elif manager_is_active; then
    printf 'status=running run_id=%s\n' "$RUN_ID"
  elif [[ -d "$ARTIFACT_DIR" ]]; then
    printf 'status=incomplete run_id=%s action=inspect_logs_or_cleanup\n' "$RUN_ID"
    return 1
  else
    printf 'status=not_found run_id=%s\n' "$RUN_ID"
    return 1
  fi
}

run_logs_action() {
  local files=()
  for path in "$ARTIFACT_DIR/manager.stderr.log" "$ARTIFACT_DIR/remote-tail.log" "$ARTIFACT_DIR/command-status.log" "$ARTIFACT_DIR/manager.stdout.log"; do
    [[ -f "$path" ]] && files+=("$path")
  done
  while IFS= read -r path; do
    [[ -f "$path" ]] && files+=("$path")
  done < <(find "$ARTIFACT_DIR" -maxdepth 2 -path '*/attempt-*/*' -type f \
    \( -name 'command-status.log' -o -name 'remote-tail.log' \) -print 2>/dev/null | sort)
  if [[ ${#files[@]} -eq 0 ]]; then
    echo "run_aws_spot_remote_validation_lane: no logs found for run id $RUN_ID" >&2
    return 1
  fi
  if [[ "$FOLLOW" == true ]]; then
    tail -n 80 -F "${files[@]}" | redact_stream
  else
    tail -n 120 "${files[@]}" | redact_stream
  fi
}

run_ssh_action() {
  local status_path public_ip
  status_path="$(private_command_status_path)"
  [[ -f "$status_path" ]] || {
    echo "run_aws_spot_remote_validation_lane: SSH control state is not available" >&2
    return 1
  }
  public_ip="$(sed -nE 's/.*public_ip=([0-9.]+).*/\1/p' "$status_path" | tail -n 1)"
  [[ "$public_ip" =~ ^([0-9]{1,3}[.]){3}[0-9]{1,3}$ ]] || {
    echo "run_aws_spot_remote_validation_lane: active SSH endpoint is not available" >&2
    return 1
  }
  verify_ssh_recovery_key
  exec "$SSH_BIN" -o BatchMode=yes -o StrictHostKeyChecking=accept-new -o UserKnownHostsFile="$ADL_SSH_KNOWN_HOSTS_FILE" \
    -o ServerAliveInterval=5 -o ServerAliveCountMax=2 \
    -i "$SSH_PRIVATE_KEY_PATH" "$SSH_USER@$public_ip"
}

run_stop_action() {
  check_account
  local status_path instance_id observed_run_id
  local profile_args=()
  if [[ "$PROFILE" != "env" && "$PROFILE" != "environment" ]]; then
    profile_args=(--profile "$PROFILE")
  fi
  status_path="$(private_command_status_path)"
  instance_id=""
  if [[ -f "$status_path" ]]; then
    instance_id="$(sed -nE 's/.*instance_id=(i-[0-9a-f]+).*/\1/p' "$status_path" | tail -n 1)"
  fi
  if [[ -z "$instance_id" ]]; then
    instance_id="$("$AWS_CLI" ec2 describe-instances ${profile_args[@]+"${profile_args[@]}"} --region "$REGION" \
      --filters "Name=tag:adl:run_id,Values=$RUN_ID" \
        "Name=instance-state-name,Values=pending,running,stopping,stopped" \
      --query 'Reservations[].Instances[].InstanceId' --output text)"
  fi
  if [[ -z "$instance_id" || "$instance_id" == "None" ]]; then
    printf 'status=already_terminated run_id=%s retained_cache_preserved=true\n' "$RUN_ID"
    return 0
  fi
  [[ "$instance_id" =~ ^i-[0-9a-f]{8,17}$ ]] || {
    echo "run_aws_spot_remote_validation_lane: active instance state is invalid or ambiguous" >&2
    return 1
  }
  observed_run_id="$("$AWS_CLI" ec2 describe-instances ${profile_args[@]+"${profile_args[@]}"} --region "$REGION" \
    --instance-ids "$instance_id" --query 'Reservations[0].Instances[0].Tags[?Key==`adl:run_id`].Value|[0]' --output text)"
  [[ "$observed_run_id" == "$RUN_ID" ]] || {
    echo "run_aws_spot_remote_validation_lane: instance run-id tag mismatch; refusing termination" >&2
    return 1
  }
  "$AWS_CLI" ec2 terminate-instances ${profile_args[@]+"${profile_args[@]}"} --region "$REGION" --instance-ids "$instance_id" >/dev/null
  "$AWS_CLI" ec2 wait instance-terminated ${profile_args[@]+"${profile_args[@]}"} --region "$REGION" --instance-ids "$instance_id"
  printf 'status=terminated run_id=%s retained_cache_preserved=true\n' "$RUN_ID"
}

run_cleanup_action() {
  run_stop_action
  local profile_args=()
  if [[ "$PROFILE" != "env" && "$PROFILE" != "environment" ]]; then
    profile_args=(--profile "$PROFILE")
  fi
  check_account
  local volume_state
  volume_state="$("$AWS_CLI" ec2 describe-volumes ${profile_args[@]+"${profile_args[@]}"} --region "$REGION" \
    --filters "Name=tag:Name,Values=$CACHE_VOLUME_NAME" \
    --query 'Volumes[0].State' --output text)"
  [[ "$volume_state" == "available" || "$volume_state" == "in-use" ]] || {
    echo "run_aws_spot_remote_validation_lane: retained cache volume is missing or unhealthy" >&2
    return 1
  }
  printf 'status=clean retained_cache_preserved=true cache_state=%s run_id=%s\n' "$volume_state" "$RUN_ID"
}

case "$ACTION" in
  status) run_status_action; exit $? ;;
  logs) run_logs_action; exit $? ;;
  ssh) run_ssh_action ;;
  stop) run_stop_action; exit $? ;;
  cleanup) run_cleanup_action; exit $? ;;
esac

if [[ "$RUN" == true || "$ACTION" == "preflight" ]]; then
  verify_lane_binary_compatibility
fi

if [[ "$CHECK_ACCOUNT" == true || "$RUN" == true ]]; then
  check_account
fi

if [[ "$RUN" == true || "$ACTION" == "preflight" ]]; then
  resolve_builder_image
  resolve_spot_hourly_cost
  resolve_and_verify_retained_topology
  verify_ssh_recovery_key
fi

if [[ "$ACTION" == "preflight" ]]; then
  python3 - "$BUILDER_IMAGE" "$SOURCE_COMMIT" "$EXPECTED_CACHE_VOLUME_ID_SHA256" "$AMI_ID" "$SUBNET_ID" "$ESTIMATED_HOURLY_COST_USD" <<'PY'
import hashlib
import json
import sys

image, commit, cache_hash, ami, subnet, hourly = sys.argv[1:]
payload = {
    "schema": "adl.aws_spot_preflight.v1",
    "status": "ready",
    "account_matches_retained_proof": True,
    "source_commit": commit,
    "builder_image_digest_sha256": hashlib.sha256(image.rsplit("@", 1)[-1].encode()).hexdigest(),
    "builder_image_immutable": "@sha256:" in image,
    "retained_cache_volume_id_sha256": cache_hash,
    "retained_cache_available": True,
    "ami_id_sha256": hashlib.sha256(ami.encode()).hexdigest(),
    "subnet_id_sha256": hashlib.sha256(subnet.encode()).hexdigest(),
    "ssh_recovery_configured": True,
    "estimated_hourly_cost_usd": float(hourly),
    "aws_resources_created": False,
}
print(json.dumps(payload, indent=2, sort_keys=True))
PY
  exit 0
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
  --spot-only
  --cache-volume-id "$RETAINED_CACHE_VOLUME_ID"
  --cache-volume-name "$CACHE_VOLUME_NAME"
  --cache-volume-size-gib "$CACHE_VOLUME_SIZE_GIB"
  --cache-volume-type "$CACHE_VOLUME_TYPE"
  --cache-volume-iops "$CACHE_VOLUME_IOPS"
  --cache-volume-throughput-mbps "$CACHE_VOLUME_THROUGHPUT_MBPS"
  --cache-volume-device-name "$CACHE_VOLUME_DEVICE_NAME"
  --cache-volume-mount-path "$CACHE_VOLUME_MOUNT_PATH"
  --command-timeout-seconds "$REMOTE_COMMAND_TIMEOUT_SECONDS"
  --ami-id "$AMI_ID"
  --subnet-id "$SUBNET_ID"
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
  if [[ "$RUN" == true ]]; then
    validation_command="$COMMAND"
    case "$validation_command" in
      "bash adl/tools/run_aws_spot_ci_profile.sh "*)
        validation_command='bash "${ADL_SPOT_CONTROL_ROOT:?}/adl/tools/run_aws_spot_ci_profile.sh" '"${validation_command#bash adl/tools/run_aws_spot_ci_profile.sh }"
        ;;
    esac
    remote_command='bash "${ADL_SPOT_CONTROL_ROOT:?}/adl/tools/run_aws_spot_builder_image_validation.sh"'
    remote_command+=" --image $(shell_quote "$BUILDER_IMAGE")"
    remote_command+=" --expected-ref $(shell_quote "$SOURCE_COMMIT")"
    remote_command+=" --expected-architecture $(shell_quote "$EXPECTED_ARCHITECTURE")"
    remote_command+=" --min-cache-free-gib $(shell_quote "$MIN_CACHE_FREE_GIB")"
    remote_command+=" --command $(shell_quote "$validation_command")"
    cmd+=(--command "$remote_command")
  else
    cmd+=(--command "$COMMAND")
  fi
fi

for instance_type in ${INSTANCE_TYPES[@]+"${INSTANCE_TYPES[@]}"}; do
  cmd+=(--instance-type "$instance_type")
done

if [[ "$JSON" == true ]]; then
  cmd+=(--json)
fi

if [[ "$PRINT_COMMAND" == true ]]; then
  printf '%q ' "${cmd[@]}" | redact_stream
  printf '\n'
fi

if [[ "$RUN" != true ]]; then
  echo "DRY-RUN aws_spot_remote_validation profile=$PROFILE region=$REGION git_ref=$GIT_REF source_commit_resolved=$([[ "$SOURCE_COMMIT" =~ ^[0-9a-f]{40}$ ]] && printf true || printf false) out=$OUT_PATH artifact_dir=$ARTIFACT_DIR cache_volume=$CACHE_VOLUME_NAME cache_mount=$CACHE_VOLUME_MOUNT_PATH ssh_tail_enabled=$([[ -n "$SSH_KEY_NAME" ]] && printf true || printf false) builder_image_mode=immutable_digest"
  echo "DRY-RUN no EC2 resources launched; pass --run to execute"
  exit 0
fi

if [[ ! -x "$LANE_BIN" ]]; then
  echo "run_aws_spot_remote_validation_lane: binary not executable: $LANE_BIN" >&2
  exit 2
fi

execute_run() {
  mkdir -p "$(dirname "$OUT_PATH")" "$ARTIFACT_DIR"
  local runner_stdout="$ARTIFACT_DIR/runner.stdout.log"
  local runner_stderr="$ARTIFACT_DIR/runner.stderr.log"
  local runner_status finalize_status wrapper_summary

  set +e
  "${cmd[@]}" >"$runner_stdout" 2>"$runner_stderr"
  runner_status="$?"
  set -e

  wrapper_summary="$ARTIFACT_DIR/wrapper-final-summary.json"
  finalize_status=0
  python3 "$ROOT/adl/tools/aws_spot_artifact_finalize.py" \
    --summary "$OUT_PATH" \
    --artifact-dir "$ARTIFACT_DIR" \
    --wrapper-summary "$wrapper_summary" \
    --expected-source-commit "$SOURCE_COMMIT" \
    --expected-image "$BUILDER_IMAGE" \
    --expected-cache-volume-id-sha256 "$EXPECTED_CACHE_VOLUME_ID_SHA256" \
    --estimated-hourly-cost-usd "$ESTIMATED_HOURLY_COST_USD" \
    --runner-exit-code "$runner_status" \
    >"$ARTIFACT_DIR/finalize.out" 2>"$ARTIFACT_DIR/finalize.err" || finalize_status="$?"

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

  python3 - <<'PY' "$ROOT" "$runner_stderr" >&2
import importlib.util
import sys
from pathlib import Path

root = Path(sys.argv[1])
path = Path(sys.argv[2])
spec = importlib.util.spec_from_file_location(
    "aws_spot_artifact_finalize",
    root / "adl" / "tools" / "aws_spot_artifact_finalize.py",
)
module = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(module)
print(module.redact_text(path.read_text(encoding="utf-8", errors="replace")), end="")
PY
  cat "$ARTIFACT_DIR/finalize.err" >&2
  printf 'aws_spot_remote_validation_wrapper_summary=%s\n' "$wrapper_summary" >&2
  if [[ "$runner_status" -ne 0 ]]; then
    printf '%s\n' "$runner_status" >"$ARTIFACT_DIR/manager.exit-code"
    return "$runner_status"
  fi
  printf '%s\n' "$finalize_status" >"$ARTIFACT_DIR/manager.exit-code"
  return "$finalize_status"
}

if [[ "$ACTION" == "launch" && "${ADL_SPOT_MANAGER_MODE:-0}" != "1" ]]; then
  mkdir -p "$(dirname "$OUT_PATH")" "$ARTIFACT_DIR"
  launch_lock="$ARTIFACT_DIR/.launch-lock"
  if ! mkdir "$launch_lock" 2>/dev/null; then
    echo "run_aws_spot_remote_validation_lane: run id launch lock is already held" >&2
    exit 1
  fi
  trap 'rmdir "$launch_lock" 2>/dev/null || true' EXIT
  if [[ -f "$ARTIFACT_DIR/wrapper-final-summary.json" || -f "$ARTIFACT_DIR/manager.exit-code" ]]; then
    echo "run_aws_spot_remote_validation_lane: run id already has terminal manager state" >&2
    exit 1
  fi
  if manager_is_active; then
    echo "run_aws_spot_remote_validation_lane: run id already has an active manager" >&2
    exit 1
  fi
  if [[ -f "$ARTIFACT_DIR/manager.pid" || -f "$ARTIFACT_DIR/resume-state.json" \
      || -d "$ARTIFACT_DIR/attempt-0" ]]; then
    echo "run_aws_spot_remote_validation_lane: run id has incomplete manager state; inspect or clean up before using a new run id" >&2
    exit 1
  fi
  manager_pid="$(ADL_SPOT_MANAGER_MODE=1 python3 - \
    "$ARTIFACT_DIR/manager.stdout.log" \
    "$ARTIFACT_DIR/manager.stderr.log" \
    "$0" "${ORIGINAL_ARGS[@]}" <<'PY'
import os
import subprocess
import sys

stdout_path, stderr_path, script, *args = sys.argv[1:]
with open(stdout_path, "ab", buffering=0) as stdout, open(stderr_path, "ab", buffering=0) as stderr:
    process = subprocess.Popen(
        ["bash", script, *args],
        stdin=subprocess.DEVNULL,
        stdout=stdout,
        stderr=stderr,
        close_fds=True,
        start_new_session=True,
        env=os.environ.copy(),
    )
print(process.pid)
PY
)"
  printf '%s\n' "$manager_pid" >"$ARTIFACT_DIR/manager.pid"
  rmdir "$launch_lock"
  trap - EXIT
  printf 'status=launched run_id=%s pid=%s\n' "$RUN_ID" "$manager_pid"
  printf 'next_status=bash adl/tools/run_aws_spot_remote_validation_lane.sh status --run-id %q --out %q --artifact-dir %q\n' \
    "$RUN_ID" "$OUT_PATH" "$ARTIFACT_DIR"
  exit 0
fi

execute_run

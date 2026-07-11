#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  run_aws_codefriend_build_lane.sh [--dry-run|--run] --source-version <ref> [options]

Options:
  --dry-run                         Render the CodeBuild request without calling AWS (default).
  --run                             Start the AWS CodeBuild build.
  --check-account                   Verify STS account hash before live AWS work.
  --expected-account-sha256 <hash>  Expected AWS account SHA-256. Defaults to ADL_AWS_CODEFRIEND_ACCOUNT_SHA256,
                                    then the retained Agent Logic #4603 account proof.
  --project-name <name>             AWS CodeBuild project name. Default: adl-codefriend-build.
  --source-version <ref>            Source version/ref passed to CodeBuild.
  --full-nextest                    Run the canonical broad Rust nextest lane.
  --region <region>                 AWS region. Default: ADL_AWS_REGION or us-west-2.
  --profile <profile>               AWS CLI profile for local runs. Default: agent-logic-admin.
                                    Use "env" when GitHub OIDC exports AWS env credentials.
  --env KEY=VALUE                   Environment variable override for CodeBuild. May be repeated.
  --out <path>                      JSON summary path. Default: .adl/local-artifacts/aws-codefriend-build/summary.json.
  --artifact-dir <path>             Directory for request/response artifacts.
  --wait                            Poll CodeBuild until the build reaches a terminal state.
  --live-logs                       Stream redacted CloudWatch build logs while waiting (default).
  --no-live-logs                    Wait without streaming CloudWatch build logs.
  --poll-seconds <n>                Poll interval for --wait. Default: 10.
  --timeout-seconds <n>             Maximum wait time for --wait. Default: 2700.
  --print-command                   Print a redacted command preview.
  -h, --help                        Show this help.

This wrapper is the local/repo-native contract for the GitHub Actions AWS
CodeFriend build lane. Live runs must use the Agent Logic AWS account and must
not print AWS account IDs, ARNs, user IDs, credentials, or token values.
USAGE
}

die() {
  printf 'ERROR: %s\n' "$*" >&2
  exit 2
}

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
AWS_CLI="${ADL_AWS_CLI:-aws}"
MODE="dry-run"
CHECK_ACCOUNT="false"
EXPECTED_ACCOUNT_SHA256="${ADL_AWS_CODEFRIEND_ACCOUNT_SHA256:-}"
PROJECT_NAME="${ADL_AWS_CODEFRIEND_CODEBUILD_PROJECT:-adl-codefriend-build}"
SOURCE_VERSION=""
AWS_REGION="${ADL_AWS_REGION:-us-west-2}"
AWS_PROFILE="${ADL_AWS_PROFILE:-agent-logic-admin}"
OUT_PATH=".adl/local-artifacts/aws-codefriend-build/summary.json"
ARTIFACT_DIR=".adl/local-artifacts/aws-codefriend-build"
PRINT_COMMAND="false"
WAIT_FOR_BUILD="false"
POLL_SECONDS="10"
TIMEOUT_SECONDS="2700"
ENV_OVERRIDES=()
FULL_NEXTEST="false"
LIVE_LOGS="true"
LOG_TAIL_PID=""

stop_log_tail() {
  if [ -n "$LOG_TAIL_PID" ]; then
    kill "$LOG_TAIL_PID" >/dev/null 2>&1 || true
    wait "$LOG_TAIL_PID" 2>/dev/null || true
    LOG_TAIL_PID=""
  fi
}

trap stop_log_tail EXIT

while [ "$#" -gt 0 ]; do
  case "$1" in
    --dry-run)
      MODE="dry-run"
      shift
      ;;
    --run)
      MODE="run"
      shift
      ;;
    --check-account)
      CHECK_ACCOUNT="true"
      shift
      ;;
    --expected-account-sha256)
      [ "$#" -ge 2 ] || die "--expected-account-sha256 requires a value"
      EXPECTED_ACCOUNT_SHA256="$2"
      shift 2
      ;;
    --project-name)
      [ "$#" -ge 2 ] || die "--project-name requires a value"
      PROJECT_NAME="$2"
      shift 2
      ;;
    --source-version)
      [ "$#" -ge 2 ] || die "--source-version requires a value"
      SOURCE_VERSION="$2"
      shift 2
      ;;
    --full-nextest)
      FULL_NEXTEST="true"
      shift
      ;;
    --region)
      [ "$#" -ge 2 ] || die "--region requires a value"
      AWS_REGION="$2"
      shift 2
      ;;
    --profile)
      [ "$#" -ge 2 ] || die "--profile requires a value"
      AWS_PROFILE="$2"
      shift 2
      ;;
    --env)
      [ "$#" -ge 2 ] || die "--env requires KEY=VALUE"
      case "$2" in
        *=*) ENV_OVERRIDES+=("$2") ;;
        *) die "--env requires KEY=VALUE" ;;
      esac
      shift 2
      ;;
    --out)
      [ "$#" -ge 2 ] || die "--out requires a value"
      OUT_PATH="$2"
      shift 2
      ;;
    --artifact-dir)
      [ "$#" -ge 2 ] || die "--artifact-dir requires a value"
      ARTIFACT_DIR="$2"
      shift 2
      ;;
    --wait)
      WAIT_FOR_BUILD="true"
      shift
      ;;
    --live-logs)
      LIVE_LOGS="true"
      shift
      ;;
    --no-live-logs)
      LIVE_LOGS="false"
      shift
      ;;
    --poll-seconds)
      [ "$#" -ge 2 ] || die "--poll-seconds requires a value"
      POLL_SECONDS="$2"
      shift 2
      ;;
    --timeout-seconds)
      [ "$#" -ge 2 ] || die "--timeout-seconds requires a value"
      TIMEOUT_SECONDS="$2"
      shift 2
      ;;
    --print-command)
      PRINT_COMMAND="true"
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      die "unknown argument: $1"
      ;;
  esac
done

if [ "$MODE" = "run" ] && [ "$CHECK_ACCOUNT" != "true" ]; then
  die "--run requires --check-account"
fi
if [ "$MODE" = "run" ]; then
  [ -n "$SOURCE_VERSION" ] || die "--run requires an explicit --source-version branch, tag, or SHA"
  [ "$SOURCE_VERSION" != "HEAD" ] || die "--source-version HEAD is ambiguous"
fi
if [ "$FULL_NEXTEST" = "true" ]; then
  ENV_OVERRIDES+=("ADL_CODEFRIEND_BUILD_COMMAND=cd adl && cargo nextest run --test-threads 8 --status-level all --final-status-level slow")
fi

AWS_PROFILE_ARGS=()
if [ "$AWS_PROFILE" != "env" ] && [ "$AWS_PROFILE" != "environment" ]; then
  AWS_PROFILE_ARGS=(--profile "$AWS_PROFILE")
fi

if [ -z "$EXPECTED_ACCOUNT_SHA256" ]; then
  EXPECTED_ACCOUNT_SHA256="$(
    python3 - <<'PY' "$ROOT/docs/milestones/v0.91.7/review/build_throughput/remote_validation_4603/live_run_summary_retry11_agentlogic_hotcache.json"
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
data = json.loads(path.read_text())
print(data.get("account_identity", {}).get("account_id_sha256", ""))
PY
  )"
fi

[ -n "$EXPECTED_ACCOUNT_SHA256" ] || die "expected account hash is unavailable"

mkdir -p "$ARTIFACT_DIR" "$(dirname "$OUT_PATH")"
REQUEST_PATH="$ARTIFACT_DIR/codebuild-request.json"
RESPONSE_PATH="$ARTIFACT_DIR/codebuild-response.json"
STATUS_PATH="$ARTIFACT_DIR/codebuild-status.json"

ACCOUNT_HASH_MATCHED="not_checked"
if [ "$CHECK_ACCOUNT" = "true" ]; then
  identity_json="$("$AWS_CLI" sts get-caller-identity "${AWS_PROFILE_ARGS[@]+"${AWS_PROFILE_ARGS[@]}"}" --region "$AWS_REGION" --output json)"
  ACCOUNT_HASH_MATCHED="$(
    python3 - <<'PY' "$identity_json" "$EXPECTED_ACCOUNT_SHA256"
import hashlib
import json
import sys

identity = json.loads(sys.argv[1])
expected = sys.argv[2].strip()
account = str(identity.get("Account") or "")
actual = hashlib.sha256(account.encode("utf-8")).hexdigest() if account else ""
print("true" if actual == expected else "false")
PY
  )"
  [ "$ACCOUNT_HASH_MATCHED" = "true" ] || die "AWS profile did not resolve to the approved Agent Logic account hash"
  printf 'PASS account_profile_resolved profile=%s account_matches_retained_proof=true\n' "$AWS_PROFILE"
fi

python3 - <<'PY' "$REQUEST_PATH" "$PROJECT_NAME" "$SOURCE_VERSION" "${ENV_OVERRIDES[@]+"${ENV_OVERRIDES[@]}"}"
import json
import sys
from pathlib import Path

request = {"projectName": sys.argv[2]}
if sys.argv[3]:
    request["sourceVersion"] = sys.argv[3]
env = []
for item in sys.argv[4:]:
    name, value = item.split("=", 1)
    env.append({"name": name, "value": value, "type": "PLAINTEXT"})
if env:
    request["environmentVariablesOverride"] = env
if len(sys.argv[3]) == 40 and all(ch in "0123456789abcdef" for ch in sys.argv[3].lower()):
    request.setdefault("environmentVariablesOverride", []).append({
        "name": "ADL_CODEFRIEND_EXPECTED_SOURCE_SHA",
        "value": sys.argv[3].lower(),
        "type": "PLAINTEXT",
    })
Path(sys.argv[1]).write_text(json.dumps(request, indent=2, sort_keys=True) + "\n")
PY

if [ "$PRINT_COMMAND" = "true" ]; then
  if [ "${#AWS_PROFILE_ARGS[@]}" -gt 0 ]; then
    printf 'aws codebuild start-build --project-name <project> --region %s --profile %s --cli-input-json file://%s\n' "$AWS_REGION" "$AWS_PROFILE" "$REQUEST_PATH"
  else
    printf 'aws codebuild start-build --project-name <project> --region %s --profile <env> --cli-input-json file://%s\n' "$AWS_REGION" "$REQUEST_PATH"
  fi
fi

BUILD_ID=""
BUILD_STATUS=""
BUILD_SUCCEEDED="false"
if [ "$MODE" = "run" ]; then
  "$AWS_CLI" codebuild start-build \
    "${AWS_PROFILE_ARGS[@]+"${AWS_PROFILE_ARGS[@]}"}" \
    --region "$AWS_REGION" \
    --cli-input-json "file://$REQUEST_PATH" \
    --query '{build:{id:build.id,buildStatus:build.buildStatus,currentPhase:build.currentPhase,startTime:build.startTime,logs:{groupName:build.logs.groupName,streamName:build.logs.streamName}}}' \
    --output json >"$RESPONSE_PATH"
  BUILD_ID="$(
    python3 - <<'PY' "$RESPONSE_PATH"
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())
print(data.get("build", {}).get("id", ""))
PY
  )"
  BUILD_STATUS="$(
    python3 - <<'PY' "$RESPONSE_PATH"
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())
print(data.get("build", {}).get("buildStatus", ""))
PY
  )"
  if [ "$WAIT_FOR_BUILD" = "true" ]; then
    [ -n "$BUILD_ID" ] || die "CodeBuild start-build did not return a build id"
    deadline=$(( $(date +%s) + TIMEOUT_SECONDS ))
    while :; do
      "$AWS_CLI" codebuild batch-get-builds \
        "${AWS_PROFILE_ARGS[@]+"${AWS_PROFILE_ARGS[@]}"}" \
        --region "$AWS_REGION" \
        --ids "$BUILD_ID" \
        --query 'builds[0].{id:id,buildStatus:buildStatus,currentPhase:currentPhase,startTime:startTime,endTime:endTime,phases:phases[].{type:phaseType,status:phaseStatus,duration_seconds:durationInSeconds,contexts:contexts},logs:{groupName:logs.groupName,streamName:logs.streamName}}' \
        --output json >"$STATUS_PATH"
      BUILD_STATUS="$(
        python3 - <<'PY' "$STATUS_PATH"
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())
print(data.get("buildStatus", "") if isinstance(data, dict) else "")
PY
      )"
      if [ "$LIVE_LOGS" = "true" ] && [ -z "$LOG_TAIL_PID" ]; then
        read -r log_group log_stream < <(
          python3 - <<'PY' "$STATUS_PATH"
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())
logs = data.get("logs") or {}
print(logs.get("groupName", ""), logs.get("streamName", ""))
PY
        )
        if [ -n "$log_group" ] && [ -n "$log_stream" ]; then
          (
            "$AWS_CLI" logs tail "$log_group" \
              "${AWS_PROFILE_ARGS[@]+"${AWS_PROFILE_ARGS[@]}"}" \
              --region "$AWS_REGION" \
              --log-stream-names "$log_stream" \
              --follow \
              --format short 2>&1 \
              | python3 -u -c '
import re
import sys

for line in sys.stdin:
    line = re.sub(r"(?<![0-9])[0-9]{12}(?![0-9])", "[redacted-account]", line)
    line = re.sub(r"arn:aws[a-zA-Z-]*:[^\\s]+", "[redacted-arn]", line)
    line = re.sub(r"(?i)(authorization|token|secret|credential)([=:]\\s*)[^\\s]+", r"\\1\\2[redacted]", line)
    sys.stderr.write(line)
'
          ) &
          LOG_TAIL_PID="$!"
          printf 'PASS aws_codefriend_live_logs_attached=true\n'
        fi
      fi
      case "$BUILD_STATUS" in
        SUCCEEDED)
          BUILD_SUCCEEDED="true"
          stop_log_tail
          break
          ;;
        FAILED|FAULT|STOPPED|TIMED_OUT)
          BUILD_SUCCEEDED="false"
          stop_log_tail
          break
          ;;
      esac
      if [ "$(date +%s)" -ge "$deadline" ]; then
        "$AWS_CLI" codebuild stop-build \
          "${AWS_PROFILE_ARGS[@]+"${AWS_PROFILE_ARGS[@]}"}" \
          --region "$AWS_REGION" \
          --id "$BUILD_ID" \
          --query '{id:build.id,buildStatus:build.buildStatus,currentPhase:build.currentPhase}' \
          --output json >"$STATUS_PATH.stop-build.json" || true
        die "timed out waiting for CodeBuild build to complete; stop-build requested"
      fi
      sleep "$POLL_SECONDS"
    done
  else
    printf '{"not_waited":true}\n' >"$STATUS_PATH"
  fi
else
  printf '{"dry_run":true,"build":{"id":"","arn":""}}\n' >"$RESPONSE_PATH"
  printf '{"dry_run":true}\n' >"$STATUS_PATH"
fi

python3 - <<'PY' "$OUT_PATH" "$MODE" "$AWS_REGION" "$AWS_PROFILE" "$PROJECT_NAME" "$SOURCE_VERSION" "$CHECK_ACCOUNT" "$ACCOUNT_HASH_MATCHED" "$REQUEST_PATH" "$RESPONSE_PATH" "$STATUS_PATH" "$BUILD_ID" "$WAIT_FOR_BUILD" "$BUILD_STATUS" "$BUILD_SUCCEEDED"
import json
import sys
from pathlib import Path

summary = {
    "schema_version": "adl.aws_codefriend_build_lane.v1",
    "lane": "github_actions_aws_codefriend_codebuild",
    "mode": sys.argv[2],
    "region": sys.argv[3],
    "profile": sys.argv[4],
    "project_name": sys.argv[5],
    "source_version": sys.argv[6],
    "account_check_requested": sys.argv[7] == "true",
    "account_hash_matched": sys.argv[8],
    "request_path": sys.argv[9],
    "response_path": sys.argv[10],
    "status_path": sys.argv[11],
    "build_id_present": bool(sys.argv[12]),
    "wait_requested": sys.argv[13] == "true",
    "build_status": sys.argv[14],
    "build_succeeded": sys.argv[15] == "true",
    "aws_response_redacted": True,
}
status_path = Path(sys.argv[11])
if status_path.exists():
    status = json.loads(status_path.read_text())
    if isinstance(status, dict):
        summary["phase_timings"] = [
            {
                "phase": phase.get("type", ""),
                "status": phase.get("status", ""),
                "duration_seconds": phase.get("duration_seconds"),
                "failure_class": next((
                    context.get("statusCode", "")
                    for context in (phase.get("contexts") or [])
                    if context.get("statusCode")
                ), ""),
            }
            for phase in (status.get("phases") or [])
        ]
Path(sys.argv[1]).write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
PY

printf 'aws_codefriend_build_summary=%s\n' "$OUT_PATH"
if [ "$MODE" = "dry-run" ]; then
  printf 'PASS aws_codefriend_build_dry_run project=%s region=%s profile=%s\n' "$PROJECT_NAME" "$AWS_REGION" "$AWS_PROFILE"
else
  printf 'PASS aws_codefriend_build_started project=%s region=%s profile=%s build_id_present=%s\n' "$PROJECT_NAME" "$AWS_REGION" "$AWS_PROFILE" "$([ -n "$BUILD_ID" ] && printf true || printf false)"
  if [ "$WAIT_FOR_BUILD" = "true" ]; then
    [ "$BUILD_SUCCEEDED" = "true" ] || die "CodeBuild build finished with status $BUILD_STATUS"
    printf 'PASS aws_codefriend_build_completed project=%s region=%s profile=%s status=%s\n' "$PROJECT_NAME" "$AWS_REGION" "$AWS_PROFILE" "$BUILD_STATUS"
  fi
fi

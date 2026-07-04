#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  run_aws_codefriend_build_lane.sh [--dry-run|--run] --project-name <name> [options]

Options:
  --dry-run                         Render the CodeBuild request without calling AWS (default).
  --run                             Start the AWS CodeBuild build.
  --check-account                   Verify STS account hash before live AWS work.
  --expected-account-sha256 <hash>  Expected AWS account SHA-256. Defaults to ADL_AWS_CODEFRIEND_ACCOUNT_SHA256,
                                    then the retained Agent Logic #4603 account proof.
  --project-name <name>             AWS CodeBuild project name.
  --source-version <ref>            Source version/ref passed to CodeBuild.
  --region <region>                 AWS region. Default: ADL_AWS_REGION or us-west-2.
  --profile <profile>               AWS CLI profile for local runs. Default: agent-logic-admin.
                                    Use "env" when GitHub OIDC exports AWS env credentials.
  --env KEY=VALUE                   Environment variable override for CodeBuild. May be repeated.
  --out <path>                      JSON summary path. Default: .adl/tmp/aws-codefriend-build/summary.json.
  --artifact-dir <path>             Directory for request/response artifacts.
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
PROJECT_NAME="${ADL_AWS_CODEFRIEND_CODEBUILD_PROJECT:-}"
SOURCE_VERSION=""
AWS_REGION="${ADL_AWS_REGION:-us-west-2}"
AWS_PROFILE="${ADL_AWS_PROFILE:-agent-logic-admin}"
OUT_PATH=".adl/tmp/aws-codefriend-build/summary.json"
ARTIFACT_DIR=".adl/tmp/aws-codefriend-build"
PRINT_COMMAND="false"
ENV_OVERRIDES=()

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

[ -n "$PROJECT_NAME" ] || die "--project-name or ADL_AWS_CODEFRIEND_CODEBUILD_PROJECT is required"

if [ "$MODE" = "run" ] && [ "$CHECK_ACCOUNT" != "true" ]; then
  die "--run requires --check-account"
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
BUILD_ARN_PRESENT="false"
if [ "$MODE" = "run" ]; then
  "$AWS_CLI" codebuild start-build \
    "${AWS_PROFILE_ARGS[@]+"${AWS_PROFILE_ARGS[@]}"}" \
    --region "$AWS_REGION" \
    --cli-input-json "file://$REQUEST_PATH" \
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
  BUILD_ARN_PRESENT="$(
    python3 - <<'PY' "$RESPONSE_PATH"
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())
print("true" if data.get("build", {}).get("arn") else "false")
PY
  )"
else
  printf '{"dry_run":true,"build":{"id":"","arn":""}}\n' >"$RESPONSE_PATH"
fi

python3 - <<'PY' "$OUT_PATH" "$MODE" "$AWS_REGION" "$AWS_PROFILE" "$PROJECT_NAME" "$SOURCE_VERSION" "$CHECK_ACCOUNT" "$ACCOUNT_HASH_MATCHED" "$REQUEST_PATH" "$RESPONSE_PATH" "$BUILD_ID" "$BUILD_ARN_PRESENT"
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
    "build_id_present": bool(sys.argv[11]),
    "build_arn_present": sys.argv[12] == "true",
}
Path(sys.argv[1]).write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
PY

printf 'aws_codefriend_build_summary=%s\n' "$OUT_PATH"
if [ "$MODE" = "dry-run" ]; then
  printf 'PASS aws_codefriend_build_dry_run project=%s region=%s profile=%s\n' "$PROJECT_NAME" "$AWS_REGION" "$AWS_PROFILE"
else
  printf 'PASS aws_codefriend_build_started project=%s region=%s profile=%s build_id_present=%s\n' "$PROJECT_NAME" "$AWS_REGION" "$AWS_PROFILE" "$([ -n "$BUILD_ID" ] && printf true || printf false)"
fi

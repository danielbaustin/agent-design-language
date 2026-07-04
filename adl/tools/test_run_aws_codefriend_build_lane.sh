#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/run_aws_codefriend_build_lane.sh"
SETUP_SCRIPT="$ROOT/adl/tools/setup_aws_codefriend_build_resources.sh"
WORKFLOW="$ROOT/.github/workflows/aws-codefriend-build.yaml"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

assert_has() {
  local file="$1"
  local needle="$2"
  if ! grep -F -- "$needle" "$file" >/dev/null; then
    echo "expected $file to contain: $needle" >&2
    cat "$file" >&2
    exit 1
  fi
}

assert_not_has() {
  local file="$1"
  local needle="$2"
  if grep -F -- "$needle" "$file" >/dev/null; then
    echo "expected $file not to contain: $needle" >&2
    cat "$file" >&2
    exit 1
  fi
}

cat >"$TMP/aws" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >>"${FAKE_AWS_ARGS_LOG:?}"
if [ "$1" = "sts" ] && [ "$2" = "get-caller-identity" ]; then
  cat <<JSON
{"Account":"000000000000","Arn":"arn:aws:iam::000000000000:user/example","UserId":"AIDAEXAMPLE"}
JSON
  exit 0
fi
if [ "$1" = "codebuild" ] && [ "$2" = "start-build" ]; then
  cat <<JSON
{"build":{"id":"codefriend-build:1234","arn":"arn:aws:codebuild:us-west-2:000000000000:build/codefriend-build:1234"}}
JSON
  exit 0
fi
if [ "$1" = "codebuild" ] && [ "$2" = "batch-get-builds" ]; then
  cat <<JSON
{"id":"codefriend-build:1234","buildStatus":"SUCCEEDED","currentPhase":"COMPLETED","logs":{"groupName":"/aws/codebuild/adl-codefriend-build","streamName":"fixture"}}
JSON
  exit 0
fi
if [ "$1" = "codebuild" ] && [ "$2" = "stop-build" ]; then
  cat <<JSON
{"build":{"id":"codefriend-build:1234","buildStatus":"STOPPED","currentPhase":"COMPLETED"}}
JSON
  exit 0
fi
echo "unexpected fake aws args: $*" >&2
exit 9
EOF
chmod +x "$TMP/aws"

FAKE_AWS_ARGS_LOG="$TMP/aws-args.log" \
ADL_AWS_CLI="$TMP/aws" \
bash "$SCRIPT" \
  --run \
  --check-account \
  --expected-account-sha256 f7b11509f4d675c3c44f0dd37ca830bb02e8cfa58f04c46283c4bfcbdce1ff45 \
  --project-name adl-codefriend-build \
  --source-version refs/heads/codex/example \
  --env ADL_CODEFRIEND_BUILD_COMMAND='bash adl/tools/run_pr_fast_test_lane.sh' \
  --out "$TMP/summary.json" \
  --artifact-dir "$TMP/artifacts" \
  --print-command >"$TMP/run.out"

assert_has "$TMP/run.out" "PASS account_profile_resolved profile=agent-logic-admin account_matches_retained_proof=true"
assert_has "$TMP/run.out" "PASS aws_codefriend_build_started project=adl-codefriend-build region=us-west-2 profile=agent-logic-admin build_id_present=true"
assert_not_has "$TMP/run.out" "000000000000"
assert_not_has "$TMP/run.out" "arn:aws"
assert_not_has "$TMP/run.out" "AIDAEXAMPLE"
assert_has "$TMP/aws-args.log" "sts get-caller-identity --profile agent-logic-admin --region us-west-2 --output json"
assert_has "$TMP/aws-args.log" "codebuild start-build --profile agent-logic-admin --region us-west-2 --cli-input-json file://$TMP/artifacts/codebuild-request.json --query"

FAKE_AWS_ARGS_LOG="$TMP/aws-env-args.log" \
ADL_AWS_CLI="$TMP/aws" \
bash "$SCRIPT" \
  --run \
  --check-account \
  --profile env \
  --expected-account-sha256 f7b11509f4d675c3c44f0dd37ca830bb02e8cfa58f04c46283c4bfcbdce1ff45 \
  --project-name adl-codefriend-build \
  --out "$TMP/env-summary.json" \
  --artifact-dir "$TMP/env-artifacts" >"$TMP/env.out"
assert_has "$TMP/env.out" "PASS account_profile_resolved profile=env account_matches_retained_proof=true"
assert_has "$TMP/aws-env-args.log" "sts get-caller-identity --region us-west-2 --output json"
assert_has "$TMP/aws-env-args.log" "codebuild start-build --region us-west-2 --cli-input-json file://$TMP/env-artifacts/codebuild-request.json --query"
assert_not_has "$TMP/aws-env-args.log" "--profile"

python3 - <<'PY' "$TMP/summary.json" "$TMP/artifacts/codebuild-request.json"
import json
import sys
from pathlib import Path

summary = json.loads(Path(sys.argv[1]).read_text())
request = json.loads(Path(sys.argv[2]).read_text())
assert summary["schema_version"] == "adl.aws_codefriend_build_lane.v1"
assert summary["mode"] == "run"
assert summary["account_hash_matched"] == "true"
assert summary["build_id_present"] is True
assert summary["aws_response_redacted"] is True
assert request["projectName"] == "adl-codefriend-build"
assert request["sourceVersion"] == "refs/heads/codex/example"
assert request["environmentVariablesOverride"][0]["name"] == "ADL_CODEFRIEND_BUILD_COMMAND"
PY

FAKE_AWS_ARGS_LOG="$TMP/aws-wait-args.log" \
ADL_AWS_CLI="$TMP/aws" \
bash "$SCRIPT" \
  --run \
  --check-account \
  --wait \
  --poll-seconds 1 \
  --expected-account-sha256 f7b11509f4d675c3c44f0dd37ca830bb02e8cfa58f04c46283c4bfcbdce1ff45 \
  --project-name adl-codefriend-build \
  --out "$TMP/wait-summary.json" \
  --artifact-dir "$TMP/wait-artifacts" >"$TMP/wait.out"
assert_has "$TMP/wait.out" "PASS aws_codefriend_build_completed project=adl-codefriend-build region=us-west-2 profile=agent-logic-admin status=SUCCEEDED"
assert_has "$TMP/aws-wait-args.log" "codebuild batch-get-builds --profile agent-logic-admin --region us-west-2 --ids codefriend-build:1234 --query"

FAKE_AWS_ARGS_LOG="$TMP/aws-dry-args.log" \
ADL_AWS_CLI="$TMP/aws" \
bash "$SCRIPT" \
  --dry-run \
  --project-name adl-codefriend-build \
  --out "$TMP/dry-summary.json" \
  --artifact-dir "$TMP/dry-artifacts" >"$TMP/dry.out"
assert_has "$TMP/dry.out" "PASS aws_codefriend_build_dry_run"
[ ! -s "$TMP/aws-dry-args.log" ]

if FAKE_AWS_ARGS_LOG="$TMP/aws-no-check.log" \
  ADL_AWS_CLI="$TMP/aws" \
  bash "$SCRIPT" \
    --run \
    --project-name adl-codefriend-build \
    --out "$TMP/no-check.json" \
    --artifact-dir "$TMP/no-check-artifacts" >"$TMP/no-check.out" 2>"$TMP/no-check.err"; then
  echo "expected run without account check to fail" >&2
  exit 1
fi
assert_has "$TMP/no-check.err" "--run requires --check-account"

if FAKE_AWS_ARGS_LOG="$TMP/aws-mismatch.log" \
  ADL_AWS_CLI="$TMP/aws" \
  bash "$SCRIPT" \
    --run \
    --check-account \
    --expected-account-sha256 deadbeef \
    --project-name adl-codefriend-build \
    --out "$TMP/mismatch.json" \
    --artifact-dir "$TMP/mismatch-artifacts" >"$TMP/mismatch.out" 2>"$TMP/mismatch.err"; then
  echo "expected account mismatch to fail" >&2
  exit 1
fi
assert_has "$TMP/mismatch.err" "AWS profile did not resolve to the approved Agent Logic account hash"

[ -f "$WORKFLOW" ]
[ -f "$SETUP_SCRIPT" ]
assert_has "$SETUP_SCRIPT" "--compute-type <type>"
assert_has "$SETUP_SCRIPT" "--cache-bucket <bucket>"
assert_has "$SETUP_SCRIPT" 'CACHE_BUCKET="${ADL_AWS_CODEFRIEND_CACHE_BUCKET:-adl-codefriend-build-cache}"'
assert_has "$SETUP_SCRIPT" 'COMPUTE_TYPE="${ADL_AWS_CODEFRIEND_COMPUTE_TYPE:-BUILD_GENERAL1_LARGE}"'
assert_has "$SETUP_SCRIPT" '"computeType": compute_type'
assert_has "$SETUP_SCRIPT" '"type": "S3"'
assert_has "$SETUP_SCRIPT" 'SCCACHE_VERSION="${SCCACHE_VERSION:-v0.10.0}"'
assert_has "$SETUP_SCRIPT" "https://github.com/mozilla/sccache/releases/download/"
assert_not_has "$SETUP_SCRIPT" "cargo install sccache --locked"
assert_has "$SETUP_SCRIPT" "'/root/.cargo/bin/**/*'"
assert_has "$SETUP_SCRIPT" "'/root/.cache/sccache/**/*'"
assert_not_has "$SETUP_SCRIPT" "'target/**/*'"
assert_has "$SETUP_SCRIPT" 'aws_codefriend_cache_bucket_exists='
assert_has "$SETUP_SCRIPT" 'compute_type=%s'
assert_has "$SETUP_SCRIPT" "codebuild:StopBuild"
assert_has "$SETUP_SCRIPT" "repo:{repo}:ref:refs/heads/main"
assert_has "$SETUP_SCRIPT" "repo:{repo}:ref:refs/heads/codex/*"
assert_has "$SCRIPT" "codebuild stop-build"
assert_has "$SCRIPT" "timed out waiting for CodeBuild build to complete; stop-build requested"
assert_has "$WORKFLOW" "workflow_dispatch:"
assert_has "$WORKFLOW" "id-token: write"
assert_has "$WORKFLOW" "aws-actions/configure-aws-credentials@7474bc4690e29a8392af63c5b98e7449536d5c3a"
assert_has "$WORKFLOW" "AWS_CODEFRIEND_BUILD_ROLE_ARN"
assert_has "$WORKFLOW" "AWS_CODEFRIEND_ACCOUNT_SHA256"
assert_has "$WORKFLOW" "AWS_CODEFRIEND_CODEBUILD_PROJECT"
assert_has "$WORKFLOW" 'ADL_CODEFRIEND_BUILD_COMMAND: ${{ inputs.build_command }}'
assert_has "$WORKFLOW" 'SOURCE_VERSION: ${{ inputs.source_version || github.sha }}'
assert_has "$WORKFLOW" "source_version must be a branch, tag, or SHA; HEAD is ambiguous"
assert_has "$WORKFLOW" '--source-version "$SOURCE_VERSION"'
assert_has "$WORKFLOW" "--wait"
assert_has "$WORKFLOW" "bash adl/tools/run_aws_codefriend_build_lane.sh"
assert_not_has "$WORKFLOW" "pull_request:"
assert_not_has "$WORKFLOW" "push:"
assert_has "$WORKFLOW" "if-no-files-found: error"

printf 'PASS test_run_aws_codefriend_build_lane\n'

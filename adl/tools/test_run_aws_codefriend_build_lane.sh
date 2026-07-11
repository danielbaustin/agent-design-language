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
if [ "$1" = "s3api" ] && [ "$2" = "head-bucket" ]; then
  exit 0
fi
if [ "$1" = "iam" ] && [ "$2" = "list-open-id-connect-providers" ]; then
  printf 'arn:aws:iam::000000000000:oidc-provider/token.actions.githubusercontent.com\n'
  exit 0
fi
if [ "$1" = "iam" ] && [ "$2" = "get-open-id-connect-provider" ]; then
  printf 'token.actions.githubusercontent.com\n'
  exit 0
fi
if [ "$1" = "iam" ] && [ "$2" = "get-role" ]; then
  for arg in "$@"; do
    if [ "$arg" = "Role.Arn" ]; then
      printf 'arn:aws:iam::000000000000:role/fixture-role\n'
      exit 0
    fi
  done
  printf 'fixture-role\n'
  exit 0
fi
if [ "$1" = "iam" ] && { [ "$2" = "update-assume-role-policy" ] || [ "$2" = "put-role-policy" ]; }; then
  printf '{}\n'
  exit 0
fi
if [ "$1" = "codebuild" ] && [ "$2" = "batch-get-projects" ]; then
  printf '1\n'
  exit 0
fi
if [ "$1" = "codebuild" ] && [ "$2" = "update-project" ]; then
  printf 'adl-codefriend-build\n'
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
if [ "$1" = "logs" ] && [ "$2" = "tail" ]; then
  printf 'fixture build log account=000000000000 arn=arn:aws:codebuild:us-west-2:000000000000:build/example\n'
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
  --source-version refs/heads/codex/example \
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
  --source-version refs/heads/codex/example \
  --out "$TMP/wait-summary.json" \
  --artifact-dir "$TMP/wait-artifacts" >"$TMP/wait.out"
assert_has "$TMP/wait.out" "PASS aws_codefriend_build_completed project=adl-codefriend-build region=us-west-2 profile=agent-logic-admin status=SUCCEEDED"
assert_has "$TMP/aws-wait-args.log" "codebuild batch-get-builds --profile agent-logic-admin --region us-west-2 --ids codefriend-build:1234 --query"
assert_has "$TMP/wait-artifacts/codebuild-live.log" "[redacted-account]"
assert_has "$TMP/wait-artifacts/codebuild-live.log" "[redacted-arn]"
assert_not_has "$TMP/wait-artifacts/codebuild-live.log" "000000000000"

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
    --source-version refs/heads/codex/example \
    --out "$TMP/mismatch.json" \
    --artifact-dir "$TMP/mismatch-artifacts" >"$TMP/mismatch.out" 2>"$TMP/mismatch.err"; then
  echo "expected account mismatch to fail" >&2
  exit 1
fi
assert_has "$TMP/mismatch.err" "AWS profile did not resolve to the approved Agent Logic account hash"

[ -f "$WORKFLOW" ]
[ -f "$SETUP_SCRIPT" ]
assert_has "$SETUP_SCRIPT" "--compute-type <type>"
assert_has "$SETUP_SCRIPT" "--image-uri <uri>"
assert_has "$SETUP_SCRIPT" "--cache-bucket <bucket>"
assert_has "$SETUP_SCRIPT" 'CACHE_BUCKET="${ADL_AWS_CODEFRIEND_CACHE_BUCKET:-adl-codefriend-build-cache}"'
assert_has "$SETUP_SCRIPT" 'IMAGE_URI="${ADL_AWS_CODEFRIEND_IMAGE:-adl-builder:v0.91.7-fixed}"'
assert_has "$SETUP_SCRIPT" 'COMPUTE_TYPE="${ADL_AWS_CODEFRIEND_COMPUTE_TYPE:-BUILD_GENERAL1_XLARGE}"'
assert_has "$SETUP_SCRIPT" '"computeType": compute_type'
assert_has "$SETUP_SCRIPT" '"image": image_uri'
assert_has "$SETUP_SCRIPT" '"imagePullCredentialsType": image_pull_credentials_type'
assert_has "$SETUP_SCRIPT" '"type": "LOCAL"'
assert_has "$SETUP_SCRIPT" '"modes": ["LOCAL_SOURCE_CACHE", "LOCAL_CUSTOM_CACHE"]'
assert_has "$SETUP_SCRIPT" 'export PATH="/usr/local/cargo/bin:/usr/local/bin:/usr/bin:/bin"'
assert_has "$SETUP_SCRIPT" 'export NO_PROXY="127.0.0.1,localhost,${NO_PROXY:-}"'
assert_has "$SETUP_SCRIPT" 'export no_proxy="127.0.0.1,localhost,${no_proxy:-}"'
assert_has "$SETUP_SCRIPT" 'for tool in rustc cargo cargo-nextest sccache ld.lld zstd aws git'
assert_has "$SETUP_SCRIPT" 'classification=missing_tool'
assert_has "$SETUP_SCRIPT" 'classification=wrong_image'
assert_has "$SETUP_SCRIPT" 'classification=wrong_ref'
assert_has "$SETUP_SCRIPT" 'classification=cache_configuration'
assert_has "$SETUP_SCRIPT" 'ADL_CODEFRIEND_EXPECTED_IMAGE'
assert_not_has "$SETUP_SCRIPT" "https://github.com/mozilla/sccache/releases/download/"
assert_not_has "$SETUP_SCRIPT" "cargo install sccache --locked"
assert_not_has "$SETUP_SCRIPT" "'/root/.cargo/bin/**/*'"
assert_not_has "$SETUP_SCRIPT" "'/root/.rustup/**/*'"
assert_not_has "$SETUP_SCRIPT" "'/root/.cache/sccache/**/*'"
assert_not_has "$SETUP_SCRIPT" 'ln -sfn "$CODEBUILD_SRC_DIR" /workspace'
assert_has "$SETUP_SCRIPT" "tar -C \"\$CODEBUILD_SRC_DIR\" -cf - . | tar -C /codebuild/adl-source -xf -"
assert_has "$SETUP_SCRIPT" "cd /codebuild/adl-source"
assert_has "$SETUP_SCRIPT" 'export CARGO_TARGET_DIR="/codebuild/adl-target"'
assert_has "$SETUP_SCRIPT" 'export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-18}"'
assert_has "$SETUP_SCRIPT" 'export SCCACHE_BUCKET="__SCCACHE_BUCKET__"'
assert_has "$SETUP_SCRIPT" 'export SCCACHE_REGION="__SCCACHE_REGION__"'
assert_has "$SETUP_SCRIPT" 'export SCCACHE_S3_KEY_PREFIX="__SCCACHE_PREFIX__/sccache/x86_64-unknown-linux-gnu"'
assert_has "$SETUP_SCRIPT" '.replace("__SCCACHE_BUCKET__", cache_bucket)'
assert_has "$SETUP_SCRIPT" 'eval "$(aws configure export-credentials --format env)"'
assert_not_has "$SETUP_SCRIPT" "codebuild-aws-credentials.env"
assert_has "$SETUP_SCRIPT" "export CARGO_INCREMENTAL=0"
assert_not_has "$SETUP_SCRIPT" "apt-get install -y lld clang zstd"
assert_has "$SETUP_SCRIPT" "ld.lld --version"
assert_has "$SETUP_SCRIPT" "zstd --version"
assert_has "$SETUP_SCRIPT" 'export RUSTFLAGS="-C link-arg=-fuse-ld=lld --remap-path-prefix=/codebuild/adl-source=/workspace --remap-path-prefix=/root=/home"'
assert_has "$SETUP_SCRIPT" "bash adl/tools/rust_cache_env.sh write-shell-env /tmp/adl-rust-cache-env.sh"
assert_has "$SETUP_SCRIPT" ". /tmp/adl-rust-cache-env.sh"
assert_has "$SETUP_SCRIPT" 'export ADL_CODEFRIEND_TARGET_CACHE_MODE="${ADL_CODEFRIEND_TARGET_CACHE_MODE:-s3-tar}"'
assert_has "$SETUP_SCRIPT" 'export ADL_CODEFRIEND_TARGET_CACHE_BUCKET="${ADL_CODEFRIEND_TARGET_CACHE_BUCKET:-__SCCACHE_BUCKET__}"'
assert_has "$SETUP_SCRIPT" 'export ADL_CODEFRIEND_TARGET_CACHE_PREFIX="${ADL_CODEFRIEND_TARGET_CACHE_PREFIX:-__SCCACHE_PREFIX__/target/x86_64-unknown-linux-gnu}"'
assert_has "$SETUP_SCRIPT" 'ADL_CODEFRIEND_TARGET_CACHE_URI="s3://${ADL_CODEFRIEND_TARGET_CACHE_BUCKET}/${ADL_CODEFRIEND_TARGET_CACHE_PREFIX}/${ADL_CODEFRIEND_TARGET_CACHE_KEY}.tar.zst"'
assert_has "$SETUP_SCRIPT" "ADL_CODEFRIEND_TARGET_CACHE_RESTORE status=hit"
assert_has "$SETUP_SCRIPT" "ADL_CODEFRIEND_TARGET_CACHE_RESTORE status=miss"
assert_has "$SETUP_SCRIPT" "ADL_CODEFRIEND_TARGET_CACHE_RESTORE status=local-cache"
assert_has "$SETUP_SCRIPT" "ADL_CODEFRIEND_TARGET_CACHE_SAVE status=uploaded"
assert_has "$SETUP_SCRIPT" "ADL_CODEFRIEND_TARGET_CACHE_SAVE status=local-cache"
assert_has "$SETUP_SCRIPT" 'source_key="${CODEBUILD_RESOLVED_SOURCE_VERSION}"'
assert_has "$SETUP_SCRIPT" 'compatibility_hash="$(printf'
assert_has "$SETUP_SCRIPT" 'ADL_CODEFRIEND_TARGET_CACHE_KEY="v2-${source_key}-${lock_hash}-${compatibility_hash}"'
assert_has "$SETUP_SCRIPT" "tar -I 'zstd -d -T0' -xf /tmp/adl-codefriend-target-cache.tar.zst -C /codebuild"
assert_has "$SETUP_SCRIPT" "tar -I 'zstd -T0 -1' -cf /tmp/adl-codefriend-target-cache.tar.zst -C /codebuild adl-target"
assert_has "$SETUP_SCRIPT" 'ADL_CODEFRIEND_TARGET_CACHE_SAVE status=skipped-command-failed'
assert_not_has "$SETUP_SCRIPT" "'/codebuild/adl-target/**/*'"
assert_has "$SETUP_SCRIPT" 'aws_codefriend_cache_bucket_exists='
assert_has "$SETUP_SCRIPT" 'compute_type=%s'
assert_has "$SETUP_SCRIPT" "codebuild:StopBuild"
assert_has "$SETUP_SCRIPT" "repo:{repo}:ref:refs/heads/main"
assert_has "$SETUP_SCRIPT" "repo:{repo}:ref:refs/heads/codex/*"
assert_has "$SCRIPT" "codebuild stop-build"
assert_has "$SCRIPT" "timed out waiting for CodeBuild build to complete; stop-build requested"
assert_has "$SCRIPT" "--live-logs"
assert_has "$SCRIPT" "--no-live-logs"
assert_has "$SCRIPT" "aws_codefriend_live_logs_attached=true"
assert_has "$SCRIPT" '"$AWS_CLI" logs tail "$LOG_GROUP"'
assert_has "$SCRIPT" '"retained_log_path"'
assert_has "$SCRIPT" "[redacted-account]"
assert_has "$SCRIPT" "[redacted-arn]"
assert_has "$WORKFLOW" "workflow_dispatch:"
assert_has "$WORKFLOW" "id-token: write"
assert_has "$WORKFLOW" "aws-actions/configure-aws-credentials@7474bc4690e29a8392af63c5b98e7449536d5c3a"
assert_has "$WORKFLOW" "AWS_CODEFRIEND_BUILD_ROLE_ARN"
assert_has "$WORKFLOW" "AWS_CODEFRIEND_ACCOUNT_SHA256"
assert_has "$WORKFLOW" "AWS_CODEFRIEND_CODEBUILD_PROJECT"
assert_has "$WORKFLOW" 'ADL_CODEFRIEND_BUILD_COMMAND: ${{ inputs.build_command }}'
assert_has "$WORKFLOW" 'SOURCE_VERSION: ${{ inputs.source_version || github.ref_name }}'
assert_not_has "$WORKFLOW" 'SOURCE_VERSION: ${{ inputs.source_version || github.sha }}'
assert_has "$WORKFLOW" "source_version must be a branch, tag, or SHA; HEAD is ambiguous"
assert_has "$WORKFLOW" '--source-version "$SOURCE_VERSION"'
assert_has "$WORKFLOW" "--wait"
assert_has "$WORKFLOW" "bash adl/tools/run_aws_codefriend_build_lane.sh"
assert_has "$SCRIPT" 'PROJECT_NAME="${ADL_AWS_CODEFRIEND_CODEBUILD_PROJECT:-adl-codefriend-build}"'
assert_has "$SCRIPT" "--full-nextest"
assert_has "$SCRIPT" "cd adl && cargo nextest run --test-threads 8 --status-level all --final-status-level slow"
assert_has "$SCRIPT" "--run requires an explicit --source-version"
assert_not_has "$WORKFLOW" "pull_request:"
assert_not_has "$WORKFLOW" "push:"
assert_has "$WORKFLOW" "if-no-files-found: error"

FAKE_AWS_ARGS_LOG="$TMP/aws-setup-args.log" \
ADL_AWS_CLI="$TMP/aws" \
bash "$SETUP_SCRIPT" \
  --apply \
  --profile agent-logic-admin \
  --region us-west-2 \
  --project-name adl-codefriend-build \
  --compute-type BUILD_GENERAL1_XLARGE \
  --image-uri example.invalid/adl-builder:v0.91.7-fixed \
  --cache-bucket adl-codefriend-build-cache \
  --cache-prefix codebuild/cache \
  --artifact-dir "$TMP/setup-artifacts" >"$TMP/setup.out"
assert_has "$TMP/setup.out" "PASS aws_codefriend_resources_ready project=adl-codefriend-build region=us-west-2 profile=agent-logic-admin compute_type=BUILD_GENERAL1_XLARGE cache_bucket=adl-codefriend-build-cache cache_prefix=codebuild/cache"
assert_has "$TMP/aws-setup-args.log" "codebuild update-project --profile agent-logic-admin --region us-west-2 --cli-input-json file://$TMP/setup-artifacts/codebuild-project.json"

python3 - <<'PY' "$TMP/setup-artifacts/codebuild-project.json"
import json
import sys
from pathlib import Path

project = json.loads(Path(sys.argv[1]).read_text())
buildspec = project["source"]["buildspec"]
assert project["environment"]["computeType"] == "BUILD_GENERAL1_XLARGE"
assert project["environment"]["image"] == "example.invalid/adl-builder:v0.91.7-fixed"
assert project["environment"]["imagePullCredentialsType"] == "CODEBUILD"
assert project["cache"]["type"] == "LOCAL"
assert project["cache"]["modes"] == ["LOCAL_SOURCE_CACHE", "LOCAL_CUSTOM_CACHE"]
assert "/root/.cache/sccache" not in buildspec
assert "__SCCACHE_BUCKET__" not in buildspec
assert "__SCCACHE_REGION__" not in buildspec
assert "__SCCACHE_PREFIX__" not in buildspec
assert 'export SCCACHE_BUCKET="adl-codefriend-build-cache"' in buildspec
assert 'export SCCACHE_REGION="us-west-2"' in buildspec
assert 'export SCCACHE_S3_KEY_PREFIX="codebuild/cache/sccache/x86_64-unknown-linux-gnu"' in buildspec
assert 'eval "$(aws configure export-credentials --format env)"' in buildspec
assert "codebuild-aws-credentials.env" not in buildspec
assert 'export CARGO_TARGET_DIR="/codebuild/adl-target"' in buildspec
assert 'export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-18}"' in buildspec
assert 'export ADL_CODEFRIEND_TARGET_CACHE_MODE="${ADL_CODEFRIEND_TARGET_CACHE_MODE:-s3-tar}"' in buildspec
assert "tar -C \"$CODEBUILD_SRC_DIR\" -cf - . | tar -C /codebuild/adl-source -xf -" in buildspec
assert "cd /codebuild/adl-source" in buildspec
assert "apt-get install -y lld clang zstd" not in buildspec
assert "https://sh.rustup.rs" not in buildspec
assert "github.com/mozilla/sccache/releases" not in buildspec
assert 'export PATH="/usr/local/cargo/bin:/usr/local/bin:/usr/bin:/bin"' in buildspec
assert 'export NO_PROXY="127.0.0.1,localhost,${NO_PROXY:-}"' in buildspec
assert 'export no_proxy="127.0.0.1,localhost,${no_proxy:-}"' in buildspec
assert "cargo-nextest" in buildspec
assert "classification=missing_tool" in buildspec
assert "classification=wrong_image" in buildspec
assert "classification=wrong_ref" in buildspec
assert "classification=cache_configuration" in buildspec
assert "ld.lld --version" in buildspec
assert "zstd --version" in buildspec
assert "CARGO_INCREMENTAL=0" in buildspec
assert "-C link-arg=-fuse-ld=lld" in buildspec
assert "--remap-path-prefix=/codebuild/adl-source=/workspace" in buildspec
assert "--remap-path-prefix=/root=/home" in buildspec
assert 'export ADL_CODEFRIEND_TARGET_CACHE_BUCKET="${ADL_CODEFRIEND_TARGET_CACHE_BUCKET:-adl-codefriend-build-cache}"' in buildspec
assert 'export ADL_CODEFRIEND_TARGET_CACHE_PREFIX="${ADL_CODEFRIEND_TARGET_CACHE_PREFIX:-codebuild/cache/target/x86_64-unknown-linux-gnu}"' in buildspec
assert 'ADL_CODEFRIEND_TARGET_CACHE_RESTORE status=hit' in buildspec
assert 'ADL_CODEFRIEND_TARGET_CACHE_RESTORE status=local-cache' in buildspec
assert 'ADL_CODEFRIEND_TARGET_CACHE_SAVE status=uploaded' in buildspec
assert 'ADL_CODEFRIEND_TARGET_CACHE_SAVE status=local-cache' in buildspec
assert 'source_key="${CODEBUILD_RESOLVED_SOURCE_VERSION}"' in buildspec
assert 'ADL_CODEFRIEND_TARGET_CACHE_KEY="v2-${source_key}-${lock_hash}-${compatibility_hash}"' in buildspec
assert "tar -I 'zstd -d -T0' -xf /tmp/adl-codefriend-target-cache.tar.zst -C /codebuild" in buildspec
assert "tar -I 'zstd -T0 -1' -cf /tmp/adl-codefriend-target-cache.tar.zst -C /codebuild adl-target" in buildspec
assert 'ADL_CODEFRIEND_TARGET_CACHE_SAVE status=skipped-command-failed' in buildspec
assert "'/codebuild/adl-target/**/*'" not in buildspec
assert "'/root/.cargo/bin/**/*'" not in buildspec
assert "'/root/.rustup/**/*'" not in buildspec
assert "AWS_SECRET_ACCESS_KEY" not in buildspec
assert "AWS_SESSION_TOKEN" not in buildspec
PY

ruby -rjson -ryaml -e '
project = JSON.parse(File.read(ARGV.fetch(0)))
YAML.safe_load(project.fetch("source").fetch("buildspec"), aliases: false)
' "$TMP/setup-artifacts/codebuild-project.json"

FAKE_AWS_ARGS_LOG="$TMP/aws-setup-ecr-args.log" \
ADL_AWS_CLI="$TMP/aws" \
bash "$SETUP_SCRIPT" \
  --apply \
  --profile agent-logic-admin \
  --region us-west-2 \
  --project-name adl-codefriend-build \
  --compute-type BUILD_GENERAL1_XLARGE \
  --image-uri 000000000000.dkr.ecr.us-west-2.amazonaws.com/adl-builder@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa \
  --cache-bucket adl-codefriend-build-cache \
  --cache-prefix codebuild/cache \
  --artifact-dir "$TMP/setup-ecr-artifacts" >"$TMP/setup-ecr.out"
assert_has "$TMP/setup-ecr.out" "PASS aws_codefriend_resources_ready project=adl-codefriend-build region=us-west-2 profile=agent-logic-admin compute_type=BUILD_GENERAL1_XLARGE cache_bucket=adl-codefriend-build-cache cache_prefix=codebuild/cache"

python3 - <<'PY' "$TMP/setup-ecr-artifacts/codebuild-project.json" "$TMP/setup-ecr-artifacts/codebuild-service-policy.json"
import json
import sys
from pathlib import Path

project = json.loads(Path(sys.argv[1]).read_text())
policy = json.loads(Path(sys.argv[2]).read_text())
assert project["environment"]["image"] == "000000000000.dkr.ecr.us-west-2.amazonaws.com/adl-builder@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
assert project["environment"]["imagePullCredentialsType"] == "SERVICE_ROLE"
actions = {
    action
    for statement in policy["Statement"]
    for action in (
        statement["Action"]
        if isinstance(statement["Action"], list)
        else [statement["Action"]]
    )
}
assert "ecr:GetAuthorizationToken" in actions
assert "ecr:BatchCheckLayerAvailability" in actions
assert "ecr:BatchGetImage" in actions
assert "ecr:GetDownloadUrlForLayer" in actions
resources = [
    statement["Resource"]
    for statement in policy["Statement"]
    if "ecr:BatchGetImage" in (
        statement["Action"]
        if isinstance(statement["Action"], list)
        else [statement["Action"]]
    )
]
assert resources == ["arn:aws:ecr:us-west-2:000000000000:repository/adl-builder"]
PY

printf 'PASS test_run_aws_codefriend_build_lane\n'

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/run_aws_spot_ci_profile.sh"
WORKFLOW="$ROOT/.github/workflows/aws-spot-remote-validation.yaml"
DOCKERFILE="$ROOT/adl/docker/adl-builder/Dockerfile"
SETUP="$ROOT/adl/tools/setup_aws_spot_remote_validation_github_resources.sh"
REDACTION_VERIFY="$ROOT/adl/tools/aws_spot_artifact_redaction_verify.py"

bash -n "$SCRIPT"
if grep -F 'ADL_PR_FAST_ALLOW_FULL_NEXTEST=1' "$SCRIPT" >/dev/null; then
  echo "Spot adl-ci must preserve hosted path-policy selection, not force full nextest" >&2
  exit 1
fi
grep -F 'bash adl/tools/ci_path_policy.sh' "$SCRIPT" >/dev/null
grep -F 'cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings' "$SCRIPT" >/dev/null
if grep -F -- '--all-features' "$SCRIPT" >/dev/null; then
  echo "Spot adl-ci clippy must match the hosted command and must not force all features" >&2
  exit 1
fi
grep -F 'FULL_COVERAGE_REQUIRED="$(policy_value full_coverage_required)"' "$SCRIPT" >/dev/null
grep -F 'if [[ "$RUST_REQUIRED" == true && "$FULL_COVERAGE_REQUIRED" != true ]]' "$SCRIPT" >/dev/null
grep -F 'bash adl/tools/demo_smoke_v07_story.sh' "$SCRIPT" >/dev/null
grep -F 'ADL_COVERAGE_BUILD_ROOT="$CARGO_TARGET_DIR/coverage"' "$SCRIPT" >/dev/null
grep -F 'require_tool cargo-llvm-cov cargo llvm-cov --version' "$SCRIPT" >/dev/null
grep -F 'ADL_SPOT_COVERAGE_SUMMARY_BEGIN' "$SCRIPT" >/dev/null
grep -F 'adl.aws_spot_coverage_summary.v1' "$SCRIPT" >/dev/null
grep -F 'rustup component add rustfmt clippy llvm-tools-preview' "$DOCKERFILE" >/dev/null
grep -F 'cargo llvm-cov --version' "$DOCKERFILE" >/dev/null
grep -F "grep -E '^llvm-tools-'" "$SCRIPT" >/dev/null
grep -F 'ResolveImmutableBuilderImage' "$SETUP" >/dev/null
grep -F 'DescribeAdlBuilderImage' "$SETUP" >/dev/null
grep -F 'ssm:GetParameter' "$SETUP" >/dev/null
grep -F 'al2023-ami-kernel-default-x86_64' "$SETUP" >/dev/null
grep -F 'ADLAwsRemoteValidationRole-*' "$SETUP" >/dev/null
grep -F 'ADLAwsRemoteValidationProfile-*' "$SETUP" >/dev/null
grep -F 'iam:AttachRolePolicy' "$SETUP" >/dev/null
grep -F 'iam:DeleteRolePolicy' "$SETUP" >/dev/null
grep -F 'ec2:AttachVolume' "$SETUP" >/dev/null
grep -F 'ec2:DetachVolume' "$SETUP" >/dev/null

ci_plan="$(bash "$SCRIPT" adl-ci --base HEAD --head HEAD --print-command)"
coverage_plan="$(bash "$SCRIPT" adl-coverage --base HEAD --head HEAD --print-command)"
[[ "$ci_plan" == *run_pr_fast_test_lane.sh* ]]
[[ "$coverage_plan" == *run_authoritative_coverage_lane.sh* ]]

grep -F 'profile:' "$WORKFLOW" >/dev/null
grep -F 'adl-ci' "$WORKFLOW" >/dev/null
grep -F 'adl-coverage' "$WORKFLOW" >/dev/null
grep -F 'validation_command is available only for the custom profile' "$WORKFLOW" >/dev/null
test "$(grep -Fc 'BASE_COMMIT="$(git rev-parse --verify "${BASE_REF}^{commit}")"' "$WORKFLOW")" -eq 2
test "$(grep -Fc -- '--base $BASE_COMMIT --head $HEAD_COMMIT' "$WORKFLOW")" -eq 2
grep -F 'builder_image_tag:' "$WORKFLOW" >/dev/null
grep -F 'issue_number:' "$WORKFLOW" >/dev/null
grep -F -- '--issue "$ISSUE_NUMBER"' "$WORKFLOW" >/dev/null
grep -F -- '--builder-image-tag "$BUILDER_IMAGE_TAG"' "$WORKFLOW" >/dev/null
grep -F 'group: aws-spot-remote-validation-ebs-cache' "$WORKFLOW" >/dev/null
grep -F 'workflow_dispatch:' "$WORKFLOW" >/dev/null
grep -F 'python3 adl/tools/aws_spot_artifact_redaction_verify.py' "$WORKFLOW" >/dev/null
test "$(grep -Fc 'GIT_REF: ${{ github.ref_name }}' "$WORKFLOW")" -eq 2
if grep -F 'GIT_REF: ${{ inputs.git_ref || github.sha }}' "$WORKFLOW" >/dev/null; then
  echo "Spot workflow must clone an advertised branch ref, not a raw commit SHA" >&2
  exit 1
fi

redaction_tmp="$(mktemp -d "${TMPDIR:-/tmp}/adl-spot-redaction.XXXXXX")"
trap 'rm -rf "$redaction_tmp"' EXIT
printf '{"cache_target_preexisting_bytes":123456789012}\n' >"$redaction_tmp/numeric-metric.json"
python3 "$REDACTION_VERIFY" "$redaction_tmp"
printf '{"account_id":"123456789012"}\n' >"$redaction_tmp/aws-identity.json"
if python3 "$REDACTION_VERIFY" "$redaction_tmp" >/dev/null 2>&1; then
  echo "Spot artifact verifier must reject AWS identifiers in JSON strings" >&2
  exit 1
fi

echo "PASS test_run_aws_spot_ci_profile"

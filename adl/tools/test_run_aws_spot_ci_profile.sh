#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/run_aws_spot_ci_profile.sh"
WORKFLOW="$ROOT/.github/workflows/aws-spot-remote-validation.yaml"
DOCKERFILE="$ROOT/adl/docker/adl-builder/Dockerfile"
SETUP="$ROOT/adl/tools/setup_aws_spot_remote_validation_github_resources.sh"

bash -n "$SCRIPT"
grep -F 'ADL_PR_FAST_ALLOW_FULL_NEXTEST=1' "$SCRIPT" >/dev/null
grep -F 'ADL_COVERAGE_BUILD_ROOT="$CARGO_TARGET_DIR/coverage"' "$SCRIPT" >/dev/null
grep -F 'require_tool cargo-llvm-cov cargo llvm-cov --version' "$SCRIPT" >/dev/null
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

ci_plan="$(bash "$SCRIPT" adl-ci --base HEAD --head HEAD --print-command)"
coverage_plan="$(bash "$SCRIPT" adl-coverage --base HEAD --head HEAD --print-command)"
[[ "$ci_plan" == *run_pr_fast_test_lane.sh* ]]
[[ "$coverage_plan" == *run_authoritative_coverage_lane.sh* ]]

grep -F 'profile:' "$WORKFLOW" >/dev/null
grep -F 'adl-ci' "$WORKFLOW" >/dev/null
grep -F 'adl-coverage' "$WORKFLOW" >/dev/null
grep -F 'validation_command is available only for the custom profile' "$WORKFLOW" >/dev/null
grep -F 'builder_image_tag:' "$WORKFLOW" >/dev/null
grep -F -- '--builder-image-tag "$BUILDER_IMAGE_TAG"' "$WORKFLOW" >/dev/null
grep -F 'group: aws-spot-remote-validation-ebs-cache' "$WORKFLOW" >/dev/null
grep -F 'workflow_dispatch:' "$WORKFLOW" >/dev/null

echo "PASS test_run_aws_spot_ci_profile"

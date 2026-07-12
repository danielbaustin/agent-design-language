#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/run_aws_spot_ci_profile.sh"
WORKFLOW="$ROOT/.github/workflows/aws-spot-remote-validation.yaml"
CI_WORKFLOW="$ROOT/.github/workflows/ci.yaml"
DOCKERFILE="$ROOT/adl/docker/adl-builder/Dockerfile"
SETUP="$ROOT/adl/tools/setup_aws_spot_remote_validation_github_resources.sh"
REDACTION_VERIFY="$ROOT/adl/tools/aws_spot_artifact_redaction_verify.py"
VERIFY_ADVERTISED_REF="$ROOT/adl/tools/verify_spot_advertised_ref.sh"
VERIFY_BACKEND_ROUTE="$ROOT/adl/tools/verify_ci_backend_route.py"

bash -n "$SCRIPT"
if grep -F 'ADL_PR_FAST_ALLOW_FULL_NEXTEST=1' "$SCRIPT" >/dev/null; then
  echo "Spot adl-ci must preserve hosted path-policy selection, not force full nextest" >&2
  exit 1
fi
grep -F 'bash adl/tools/ci_path_policy.sh' "$SCRIPT" >/dev/null
grep -F -- '--event-name "$EVENT_NAME"' "$SCRIPT" >/dev/null
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
grep -F 'gh --version' "$DOCKERFILE" >/dev/null
grep -F "for required in rustc cargo cargo-nextest 'gh version' sccache LLD aws-cli" \
  "$ROOT/adl/tools/run_aws_spot_builder_image_validation.sh" >/dev/null
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
coverage_push_plan="$(bash "$SCRIPT" adl-coverage --base HEAD --head HEAD --event-name push --print-command)"
[[ "$ci_plan" == *run_pr_fast_test_lane.sh* ]]
[[ "$coverage_plan" == *run_authoritative_coverage_lane.sh* ]]
[[ "$coverage_push_plan" == *'--event-name push'* ]]

grep -F 'profile:' "$WORKFLOW" >/dev/null
grep -F 'adl-ci' "$WORKFLOW" >/dev/null
grep -F 'adl-coverage' "$WORKFLOW" >/dev/null
grep -F 'validation_command is available only for the custom profile' "$WORKFLOW" >/dev/null
test "$(grep -Fc 'BASE_COMMIT="$(git rev-parse --verify "${BASE_REF}^{commit}")"' "$WORKFLOW")" -eq 2
test "$(grep -Fc -- '--base $BASE_COMMIT --head $HEAD_COMMIT' "$WORKFLOW")" -eq 2
test "$(grep -Fc -- '--event-name $SOURCE_EVENT_NAME' "$WORKFLOW")" -eq 2
grep -F 'builder_image_tag:' "$WORKFLOW" >/dev/null
grep -F 'issue_number:' "$WORKFLOW" >/dev/null
grep -F -- '--issue "$ISSUE_NUMBER"' "$WORKFLOW" >/dev/null
grep -F -- '--builder-image-tag "$BUILDER_IMAGE_TAG"' "$WORKFLOW" >/dev/null
grep -F 'group: aws-spot-remote-validation-ebs-cache' "$WORKFLOW" >/dev/null
grep -F 'workflow_dispatch:' "$WORKFLOW" >/dev/null
grep -F 'workflow_call:' "$WORKFLOW" >/dev/null
grep -F 'environment: adl-spot-ci' "$WORKFLOW" >/dev/null
grep -F 'name: Fetch selected advertised remote ref' "$WORKFLOW" >/dev/null
grep -F 'git check-ref-format --branch "$REMOTE_REF"' "$WORKFLOW" >/dev/null
grep -F '"+refs/heads/$REMOTE_REF:refs/remotes/origin/$REMOTE_REF"' "$WORKFLOW" >/dev/null
grep -F 'bash adl/tools/verify_spot_advertised_ref.sh' "$WORKFLOW" >/dev/null
grep -F 'git update-ref "refs/heads/$REMOTE_REF"' "$WORKFLOW" >/dev/null
grep -F "steps.aws-credentials.outcome == 'success'" "$WORKFLOW" >/dev/null
grep -F 'id: sanitize-artifacts' "$WORKFLOW" >/dev/null
grep -F 'python3 adl/tools/aws_spot_artifact_redaction_verify.py --sanitize' "$WORKFLOW" >/dev/null
grep -F "if: always() && steps.sanitize-artifacts.outcome == 'success'" "$WORKFLOW" >/dev/null
test "$(grep -Fc 'GIT_REF: ${{ inputs.remote_ref || github.ref_name }}' "$WORKFLOW")" -eq 2
if grep -F 'GIT_REF: ${{ inputs.git_ref || github.sha }}' "$WORKFLOW" >/dev/null; then
  echo "Spot workflow must clone an advertised branch ref, not a raw commit SHA" >&2
  exit 1
fi
grep -F "vars.ADL_HEAVY_CI_BACKEND || 'hosted'" "$CI_WORKFLOW" >/dev/null
grep -F 'uses: ./.github/workflows/aws-spot-remote-validation.yaml' "$CI_WORKFLOW" >/dev/null
grep -F 'github.event.pull_request.head.repo.full_name == github.repository' "$CI_WORKFLOW" >/dev/null
grep -F "github.event_name == 'pull_request' && github.event.pull_request.head.repo.full_name == github.repository" "$CI_WORKFLOW" >/dev/null
grep -F "github.event_name != 'pull_request' || github.event.pull_request.head.repo.full_name != github.repository" "$CI_WORKFLOW" >/dev/null
grep -F 'name: adl-coverage-hosted' "$CI_WORKFLOW" >/dev/null
grep -F 'name: Aggregate hosted or Spot coverage lane' "$CI_WORKFLOW" >/dev/null
grep -F 'name: adl-coverage' "$CI_WORKFLOW" >/dev/null
grep -F 'name: adl-ci' "$CI_WORKFLOW" >/dev/null
grep -F '"adl_demo_proof:${{ needs.adl_demo_proof.result }}" \' "$CI_WORKFLOW" >/dev/null
grep -F '"adl_spot_ci:${{ needs.adl_spot_ci.result }}"' "$CI_WORKFLOW" >/dev/null
test "$(grep -Fc 'builder_image_tag: v0.91.7-coverage-5243' "$CI_WORKFLOW")" -eq 2
test "$(grep -Fc 'source_event_name: ${{ github.event_name }}' "$CI_WORKFLOW")" -eq 2
test "$(grep -Fc 'python3 adl/tools/verify_ci_backend_route.py' "$CI_WORKFLOW")" -eq 2

binding_tmp="$(mktemp -d "${TMPDIR:-/tmp}/adl-spot-ref-binding.XXXXXX")"
git -C "$binding_tmp" init -q
git -C "$binding_tmp" config user.name adl-test
git -C "$binding_tmp" config user.email adl-test@example.invalid
printf 'one\n' >"$binding_tmp/value"
git -C "$binding_tmp" add value
git -C "$binding_tmp" commit -qm one
bound_commit="$(git -C "$binding_tmp" rev-parse HEAD)"
git -C "$binding_tmp" update-ref refs/remotes/origin/codex/test "$bound_commit"
git -C "$binding_tmp" -c advice.detachedHead=false checkout -q --detach "$bound_commit"
(
  cd "$binding_tmp"
  bash "$VERIFY_ADVERTISED_REF" codex/test "$bound_commit"
)
printf 'two\n' >>"$binding_tmp/value"
git -C "$binding_tmp" add value
git -C "$binding_tmp" commit -qm two
different_commit="$(git -C "$binding_tmp" rev-parse HEAD)"
if (
  cd "$binding_tmp"
  bash "$VERIFY_ADVERTISED_REF" codex/test "$different_commit"
) >/dev/null 2>&1; then
  echo "advertised-ref verifier accepted a commit different from the branch tip" >&2
  exit 1
fi

python3 "$VERIFY_BACKEND_ROUTE" --surface adl-ci --backend spot \
  --event-name pull_request --same-repo-pr true --work-required true \
  --rust-required true --demo-required true \
  --path-policy-result success --spot-result success \
  --hosted-result rust=skipped >/dev/null
python3 "$VERIFY_BACKEND_ROUTE" --surface adl-ci --backend hosted \
  --event-name pull_request --same-repo-pr true --work-required true \
  --rust-required true --demo-required false \
  --path-policy-result success --spot-result skipped \
  --hosted-result rust-fmt-clippy=success \
  --hosted-result rust-tests=success \
  --hosted-result demo-proof=success >/dev/null
python3 "$VERIFY_BACKEND_ROUTE" --surface adl-coverage --backend spot \
  --event-name pull_request --same-repo-pr true --work-required true \
  --path-policy-result success --spot-result success \
  --hosted-result coverage=skipped >/dev/null
python3 "$VERIFY_BACKEND_ROUTE" --surface adl-coverage --backend spot \
  --event-name pull_request --same-repo-pr true --work-required false \
  --path-policy-result success --spot-result skipped \
  --hosted-result coverage=skipped >/dev/null
for invalid_route in \
  'adl-coverage spot pull_request true true success skipped coverage=skipped' \
  'adl-coverage hosted push false true skipped skipped coverage=success' \
  'adl-coverage hosted push false true success skipped coverage=skipped'
do
  read -r surface backend event same_repo required path_result spot_result hosted_result <<<"$invalid_route"
  if python3 "$VERIFY_BACKEND_ROUTE" --surface "$surface" --backend "$backend" \
      --event-name "$event" --same-repo-pr "$same_repo" --work-required "$required" \
      --path-policy-result "$path_result" --spot-result "$spot_result" \
      --hosted-result "$hosted_result" >/dev/null 2>&1; then
    echo "backend-route verifier accepted invalid route: $invalid_route" >&2
    exit 1
  fi
done
if python3 "$VERIFY_BACKEND_ROUTE" --surface adl-ci --backend spot \
    --event-name pull_request --same-repo-pr true --work-required true \
    --rust-required true --demo-required true \
    --path-policy-result success --spot-result skipped \
    --hosted-result rust-fmt-clippy=skipped --hosted-result rust-tests=skipped \
    --hosted-result demo-proof=success >/dev/null 2>&1; then
  echo "backend-route verifier accepted a skipped selected Spot lane" >&2
  exit 1
fi
if python3 "$VERIFY_BACKEND_ROUTE" --surface adl-ci --backend hosted \
    --event-name pull_request --same-repo-pr true --work-required true \
    --rust-required true --demo-required false \
    --path-policy-result success --spot-result skipped \
    --hosted-result rust-fmt-clippy=skipped --hosted-result rust-tests=skipped \
    --hosted-result demo-proof=success >/dev/null 2>&1; then
  echo "backend-route verifier let demo success mask skipped required Rust lanes" >&2
  exit 1
fi

redaction_tmp="$(mktemp -d "${TMPDIR:-/tmp}/adl-spot-redaction.XXXXXX")"
trap 'rm -rf "$binding_tmp" "$redaction_tmp"' EXIT
printf '{"cache_target_preexisting_bytes":123456789012}\n' >"$redaction_tmp/numeric-metric.json"
python3 "$REDACTION_VERIFY" "$redaction_tmp"
printf '{"account_id":"123456789012"}\n' >"$redaction_tmp/aws-identity.json"
if python3 "$REDACTION_VERIFY" "$redaction_tmp" >/dev/null 2>&1; then
  echo "Spot artifact verifier must reject AWS identifiers in JSON strings" >&2
  exit 1
fi
python3 "$REDACTION_VERIFY" --sanitize "$redaction_tmp"
python3 - "$redaction_tmp" <<'PY'
import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
assert json.loads((root / "numeric-metric.json").read_text())["cache_target_preexisting_bytes"] == 123456789012
assert json.loads((root / "aws-identity.json").read_text())["account_id"] == "<aws-account-id-redacted>"
PY
printf 'cancelled instance=i-0123456789abcdef0 volume=vol-0123456789abcdef0 account=123456789012 temporary_key=ASIAABCDEFGHIJKLMNOP\n{"partial":' \
  >"$redaction_tmp/cancelled-partial.log"
python3 "$REDACTION_VERIFY" --sanitize "$redaction_tmp"
grep -F '<ec2-instance-id-redacted>' "$redaction_tmp/cancelled-partial.log" >/dev/null
grep -F '<ebs-volume-id-redacted>' "$redaction_tmp/cancelled-partial.log" >/dev/null
grep -F '<aws-account-id-redacted>' "$redaction_tmp/cancelled-partial.log" >/dev/null
grep -F '<aws-access-key-redacted>' "$redaction_tmp/cancelled-partial.log" >/dev/null
if grep -F 'ASIAABCDEFGHIJKLMNOP' "$redaction_tmp/cancelled-partial.log" >/dev/null; then
  echo "Spot artifact sanitizer retained an OIDC temporary access-key id" >&2
  exit 1
fi
printf 'ADL_SPOT_BUILDER_PROOF={"cache_free_bytes":123456789012,"cache_target_preexisting_bytes":987654321012,"account_id":"123456789012"}\n' \
  >"$redaction_tmp/embedded-proof.log"
python3 "$REDACTION_VERIFY" --sanitize "$redaction_tmp"
python3 - "$redaction_tmp/embedded-proof.log" <<'PY'
import json
import pathlib
import sys

line = pathlib.Path(sys.argv[1]).read_text().strip()
prefix = "ADL_SPOT_BUILDER_PROOF="
assert line.startswith(prefix), line
proof = json.loads(line[len(prefix):])
assert proof["cache_free_bytes"] == 123456789012, proof
assert proof["cache_target_preexisting_bytes"] == 987654321012, proof
assert proof["account_id"] == "<aws-account-id-redacted>", proof
PY

echo "PASS test_run_aws_spot_ci_profile"

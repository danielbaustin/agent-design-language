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
grep -F 'COVERAGE_AUTHORITY="$(policy_value coverage_authority)"' "$SCRIPT" >/dev/null
grep -F 'run_authoritative_coverage_lane.sh' "$SCRIPT" >/dev/null
grep -F -- '--no-fail-fast' "$SCRIPT" >/dev/null
grep -F 'full coverage policy did not declare coverage_authority' "$SCRIPT" >/dev/null
grep -F 'if [[ "$RUST_REQUIRED" == true && "$FULL_COVERAGE_REQUIRED" != true ]]' "$SCRIPT" >/dev/null
grep -F 'bash adl/tools/demo_smoke_v07_story.sh' "$SCRIPT" >/dev/null
grep -F 'ADL_COVERAGE_BUILD_ROOT="$CARGO_TARGET_DIR/coverage"' "$SCRIPT" >/dev/null
grep -F 'ADL_SPOT_RESET_COVERAGE_CACHE:-1' "$SCRIPT" >/dev/null
grep -F 'ADL_SPOT_CACHE_PRUNE scope=coverage' "$SCRIPT" >/dev/null
grep -F 'preserved=main-target,sccache,cargo-home' "$SCRIPT" >/dev/null
grep -F 'run_pr_fast_coverage_lane.sh' "$SCRIPT" >/dev/null
grep -F 'if [[ "$FULL_COVERAGE_REQUIRED" == true ]]' "$SCRIPT" >/dev/null
grep -F 'ADL_RUST_WARM_CACHE_SOURCE_TARGET="$WARM_SOURCE_TARGET"' "$SCRIPT" >/dev/null
grep -F 'ADL_RUST_WARM_CACHE_DEST_TARGET="$ADL_COVERAGE_BUILD_ROOT/target"' "$SCRIPT" >/dev/null
grep -F 'require_tool cargo-llvm-cov cargo llvm-cov --version' "$SCRIPT" >/dev/null
grep -F 'ADL_SPOT_COVERAGE_SUMMARY_BEGIN' "$SCRIPT" >/dev/null
grep -F 'adl.aws_spot_coverage_summary.v1' "$SCRIPT" >/dev/null
grep -F 'rustup component add rustfmt clippy llvm-tools-preview' "$DOCKERFILE" >/dev/null
grep -F 'cargo llvm-cov --version' "$DOCKERFILE" >/dev/null
grep -F 'gh --version' "$DOCKERFILE" >/dev/null
grep -F "for required in rustc cargo cargo-nextest sccache LLD aws-cli" \
  "$ROOT/adl/tools/run_aws_spot_builder_image_validation.sh" >/dev/null
grep -F "grep -E '^llvm-tools-'" "$SCRIPT" >/dev/null
grep -F 'sts get-caller-identity' "$SETUP" >/dev/null
grep -F 'token.actions.githubusercontent.com' "$SETUP" >/dev/null
grep -F 'AWS_SPOT_REMOTE_VALIDATION_REGION' "$SETUP" >/dev/null
grep -F 'ADL_AWS_REMOTE_VALIDATION_SSH_ALLOWED_CIDR' "$SETUP" >/dev/null
grep -F 'RemoteValidationEc2Lifecycle' "$SETUP" >/dev/null
grep -F 'ec2:RequestSpotInstances' "$SETUP" >/dev/null
grep -F 'RemoteValidationSsmCommands' "$SETUP" >/dev/null
grep -F 'ssm:SendCommand' "$SETUP" >/dev/null
grep -F 'RemoteValidationEphemeralInstanceProfiles' "$SETUP" >/dev/null
grep -F 'iam:PutRolePolicy' "$SETUP" >/dev/null
grep -F 'SSH_ALLOWED_CIDR="${ADL_AWS_REMOTE_VALIDATION_SSH_ALLOWED_CIDR:-}"' "$ROOT/adl/tools/run_aws_spot_remote_validation_lane.sh" >/dev/null
grep -F 'https://checkip.amazonaws.com' "$ROOT/tools/aws_remote_validation/src/aws_remote_validation.rs" >/dev/null
if grep -F 'AWS_SPOT_REMOTE_VALIDATION_SSH_ALLOWED_CIDR' "$WORKFLOW" >/dev/null; then
  echo "hosted Spot workflow must let the runner auto-detect its ephemeral SSH CIDR" >&2
  exit 1
fi
if grep -F -- '--ssh-allowed-cidr' "$WORKFLOW" >/dev/null; then
  echo "hosted Spot workflow must not override runner SSH CIDR auto-detection" >&2
  exit 1
fi

ci_plan="$(bash "$SCRIPT" adl-ci --base HEAD --head HEAD --print-command)"
coverage_plan="$(bash "$SCRIPT" adl-coverage --base HEAD --head HEAD --print-command)"
combined_plan="$(bash "$SCRIPT" adl-ci-and-coverage --base HEAD --head HEAD --print-command)"
coverage_push_plan="$(bash "$SCRIPT" adl-coverage --base HEAD --head HEAD --event-name push --print-command)"
[[ "$ci_plan" == *run_pr_fast_test_lane.sh* ]]
[[ "$coverage_plan" == *'policy-selected: bash adl/tools/run_pr_fast_coverage_lane.sh'* ]]
[[ "$combined_plan" == *'adl-ci:'*'adl-coverage:'* ]]
[[ "$combined_plan" == *run_pr_fast_test_lane.sh* ]]
[[ "$combined_plan" == *'policy-selected: bash adl/tools/run_pr_fast_coverage_lane.sh'* ]]
grep -F 'coverage_command=(cargo llvm-cov nextest --workspace --no-report --no-fail-fast --no-tests pass --test-threads 16 -- --skip real_pr_)' "$SCRIPT" >/dev/null
grep -F 'FULL_COVERAGE_REQUIRED" == true && "$EVENT_NAME" != pull_request' "$SCRIPT" >/dev/null
grep -F 'ADL_SPOT_COVERAGE_PLAN mode=pr-fast-sla full_policy=true' "$SCRIPT" >/dev/null
[[ "$coverage_push_plan" == *'--event-name push'* ]]

# Execute the combined orchestration locally with fake toolchain commands. This
# proves one-container parallel orchestration and evidence emission without
# launching paid AWS compute.
combined_tmp="$(mktemp -d "${TMPDIR:-/tmp}/adl-spot-combined-profile.XXXXXX")"
combined_root="$combined_tmp/repo"
fake_bin="$combined_tmp/bin"
combined_log="$combined_tmp/execution.log"
combined_output_dir="$combined_tmp/profile-output"
mkdir -p "$combined_root/adl/tools" "$combined_root/adl/target" "$fake_bin" "$combined_root/cache/target" "$combined_output_dir"
mkdir -p "$combined_root/cache/target/coverage/target" "$combined_root/cache/sccache" "$combined_root/cache/cargo-home"
printf 'stale coverage artifact\n' >"$combined_root/cache/target/coverage/target/stale.txt"
printf 'warm target artifact\n' >"$combined_root/cache/target/warm.txt"
printf 'sccache artifact\n' >"$combined_root/cache/sccache/warm.bin"
printf 'cargo registry artifact\n' >"$combined_root/cache/cargo-home/warm.crate"
cp "$SCRIPT" "$combined_root/adl/tools/run_aws_spot_ci_profile.sh"
printf '#!/usr/bin/env bash\nset -euo pipefail\nprintf "ci-policy\\n" >>"$ADL_TEST_LOG"\nout=""\nwhile [[ $# -gt 0 ]]; do\n  if [[ "$1" == "--github-output" ]]; then out="$2"; shift 2; continue; fi\n  shift\ndone\nfull=false\nif [[ "${ADL_TEST_FULL_POLICY:-false}" == true ]]; then full=true; fi\nprintf "rust_required=true\\nfull_coverage_required=%%s\\ndemo_smoke_required=false\\nv0913_proof_required=false\\nvalidation_profile_escalation_required=false\\ncoverage_authority=test-policy\\n" "$full" >"$out"\n' >"$combined_root/adl/tools/ci_path_policy.sh"
printf '#!/usr/bin/env bash\nprintf "ci-lane\\n" >>"$ADL_TEST_LOG"\n' >"$combined_root/adl/tools/run_pr_fast_test_lane.sh"
printf '#!/usr/bin/env bash\nprintf "process_status\\n"\n' >"$combined_root/adl/tools/check_coverage_impact.sh"
printf '#!/usr/bin/env bash\nprintf "cargo llvm-cov nextest\\n" >>"$ADL_TEST_LOG"\nprintf "{\\"data\\":[{\\"totals\\":{}}]}\\n" >"$ADL_SPOT_SOURCE_ROOT/adl/target/coverage-impact-summary.json"\n' >"$combined_root/adl/tools/run_pr_fast_coverage_lane.sh"
printf '#!/usr/bin/env bash\nexit 0\n' >"$combined_root/adl/tools/rust_validation_warm_cache.sh"
chmod +x "$combined_root/adl/tools/ci_path_policy.sh" "$combined_root/adl/tools/run_pr_fast_test_lane.sh" "$combined_root/adl/tools/check_coverage_impact.sh" "$combined_root/adl/tools/run_pr_fast_coverage_lane.sh" "$combined_root/adl/tools/rust_validation_warm_cache.sh"
printf '#!/usr/bin/env bash\nprintf "rustc %%s\\n" "$*" >>"$ADL_TEST_LOG"\n' >"$fake_bin/rustc"
printf '#!/usr/bin/env bash\nprintf "cargo %%s\\n" "$*" >>"$ADL_TEST_LOG"\nif [[ "$*" == *"llvm-cov report"* ]]; then printf "{\\\"data\\\":[{\\\"totals\\\":{}}]}\\n" >coverage-summary.json; fi\n' >"$fake_bin/cargo"
printf '#!/usr/bin/env bash\nprintf "sccache %%s\\n" "$*" >>"$ADL_TEST_LOG"\n' >"$fake_bin/sccache"
printf '#!/usr/bin/env bash\nprintf "ld.lld %%s\\n" "$*" >>"$ADL_TEST_LOG"\n' >"$fake_bin/ld.lld"
printf '#!/usr/bin/env bash\nif [[ "$*" == *"component list --installed"* ]]; then printf '\''llvm-tools-preview (installed)\\n'\''; fi\n' >"$fake_bin/rustup"
chmod +x "$fake_bin/rustc" "$fake_bin/cargo" "$fake_bin/sccache" "$fake_bin/ld.lld" "$fake_bin/rustup"
git -C "$combined_root" init -q
git -C "$combined_root" config user.name adl-test
git -C "$combined_root" config user.email adl-test@example.invalid
git -C "$combined_root" add .
git -C "$combined_root" commit -qm combined-profile-test
combined_output="$(ADL_SPOT_SOURCE_ROOT="$combined_root" CARGO_TARGET_DIR="$combined_root/cache/target" ADL_SPOT_RUN_OUTPUT="$combined_output_dir" ADL_TEST_LOG="$combined_log" PATH="$fake_bin:$PATH" bash "$combined_root/adl/tools/run_aws_spot_ci_profile.sh" adl-ci-and-coverage --base HEAD --head HEAD)"
ci_record_line="$(printf '%s\n' "$combined_output" | grep -n 'profile=adl-ci base=' | head -1 | cut -d: -f1)"
coverage_record_line="$(printf '%s\n' "$combined_output" | grep -n 'profile=adl-coverage base=' | head -1 | cut -d: -f1)"
total_record_line="$(printf '%s\n' "$combined_output" | grep -n 'profile=adl-ci-and-coverage base=' | head -1 | cut -d: -f1)"
[[ -n "$ci_record_line" && -n "$coverage_record_line" && -n "$total_record_line" ]]
grep -F 'ci-lane' "$combined_log" >/dev/null
grep -F 'cargo llvm-cov nextest' "$combined_log" >/dev/null
grep -F 'ADL_SPOT_CACHE_PRUNE scope=coverage' <<<"$combined_output" >/dev/null
test ! -e "$combined_root/cache/target/coverage/target/stale.txt"
test -e "$combined_root/cache/target/warm.txt"
test -e "$combined_root/cache/sccache/warm.bin"
test -e "$combined_root/cache/cargo-home/warm.crate"
test -s "$combined_output_dir/adl-ci.log"
test -s "$combined_output_dir/adl-coverage.log"
if printf '%s\n' "$combined_output" | grep -F 'parallel profile failed' >/dev/null; then
  echo "combined profile unexpectedly reported a parallel failure" >&2
  exit 1
fi

# Full-policy pull requests must still use the bounded PR-fast coverage route.
full_policy_log="$combined_tmp/full-policy.log"
full_policy_output="$(ADL_SPOT_SOURCE_ROOT="$combined_root" CARGO_TARGET_DIR="$combined_root/cache/target" ADL_TEST_FULL_POLICY=true ADL_TEST_LOG="$full_policy_log" PATH="$fake_bin:$PATH" bash "$combined_root/adl/tools/run_aws_spot_ci_profile.sh" adl-coverage --base HEAD --head HEAD --event-name pull_request)"
grep -F 'ADL_SPOT_COVERAGE_PLAN mode=pr-fast-sla full_policy=true authority=test-policy' <<<"$full_policy_output" >/dev/null
grep -F 'cargo llvm-cov nextest' "$full_policy_log" >/dev/null
grep -F 'ADL_SPOT_COVERAGE_SUMMARY_BEGIN' <<<"$full_policy_output" >/dev/null
if grep -F 'mode=full-authoritative' <<<"$full_policy_output" >/dev/null; then
  echo "full-policy pull request unexpectedly selected authoritative coverage" >&2
  exit 1
fi
rm -rf "$combined_tmp"

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
grep -F "HEAVY_CI_BACKEND: \${{ vars.ADL_HEAVY_CI_BACKEND || 'hosted' }}" "$CI_WORKFLOW" >/dev/null
grep -F "heavy_ci_backend: \${{ steps.path-policy.outputs.heavy_ci_backend }}" "$CI_WORKFLOW" >/dev/null
grep -F 'echo "heavy_ci_backend=$backend" >> "$GITHUB_OUTPUT"' "$CI_WORKFLOW" >/dev/null
grep -F "BACKEND: \${{ needs.adl_path_policy.outputs.heavy_ci_backend }}" "$CI_WORKFLOW" >/dev/null
grep -F "needs.adl_path_policy.outputs.heavy_ci_backend == 'spot'" "$CI_WORKFLOW" >/dev/null
grep -F "needs.adl_path_policy.outputs.heavy_ci_backend != 'spot'" "$CI_WORKFLOW" >/dev/null
grep -F 'uses: ./.github/workflows/aws-spot-remote-validation.yaml' "$CI_WORKFLOW" >/dev/null
grep -F 'github.event.pull_request.head.repo.full_name == github.repository' "$CI_WORKFLOW" >/dev/null
grep -F "github.event_name == 'pull_request' && github.event.pull_request.head.repo.full_name == github.repository" "$CI_WORKFLOW" >/dev/null
grep -F "github.event_name != 'pull_request' || github.event.pull_request.head.repo.full_name != github.repository" "$CI_WORKFLOW" >/dev/null
grep -F "contains(github.event.pull_request.labels.*.name, 'ci:spot')" "$CI_WORKFLOW" >/dev/null
grep -F -- "--spot-opt-in \"\$SPOT_OPT_IN\"" "$CI_WORKFLOW" >/dev/null
grep -F "if: needs.adl_path_policy.outputs.rust_required == 'true' && needs.adl_path_policy.outputs.runtime_v3_fast_required != 'true' && (needs.adl_path_policy.outputs.heavy_ci_backend != 'spot' || github.event_name != 'pull_request' || github.event.pull_request.head.repo.full_name != github.repository || !contains(github.event.pull_request.labels.*.name, 'ci:spot'))" "$CI_WORKFLOW" >/dev/null
grep -F "if: needs.adl_path_policy.outputs.runtime_v3_fast_required != 'true' && (needs.adl_path_policy.outputs.demo_smoke_required == 'true' || needs.adl_path_policy.outputs.v0913_proof_required == 'true') && (needs.adl_path_policy.outputs.heavy_ci_backend != 'spot' || github.event_name != 'pull_request' || github.event.pull_request.head.repo.full_name != github.repository || !contains(github.event.pull_request.labels.*.name, 'ci:spot'))" "$CI_WORKFLOW" >/dev/null
grep -F "if: needs.adl_path_policy.outputs.heavy_ci_backend == 'spot' && github.event_name == 'pull_request' && github.event.pull_request.head.repo.full_name == github.repository && contains(github.event.pull_request.labels.*.name, 'ci:spot') && (needs.adl_path_policy.outputs.rust_required == 'true' || needs.adl_path_policy.outputs.demo_smoke_required == 'true' || needs.adl_path_policy.outputs.v0913_proof_required == 'true' || needs.adl_path_policy.outputs.coverage_required == 'true')" "$CI_WORKFLOW" >/dev/null
grep -F "if: needs.adl_path_policy.outputs.heavy_ci_backend != 'spot' || github.event_name != 'pull_request' || github.event.pull_request.head.repo.full_name != github.repository || !contains(github.event.pull_request.labels.*.name, 'ci:spot')" "$CI_WORKFLOW" >/dev/null
grep -F 'name: adl-coverage-hosted' "$CI_WORKFLOW" >/dev/null
grep -F 'name: Aggregate hosted or Spot coverage lane' "$CI_WORKFLOW" >/dev/null
grep -F 'name: adl-coverage' "$CI_WORKFLOW" >/dev/null
grep -F 'name: adl-ci' "$CI_WORKFLOW" >/dev/null
grep -F '"adl_demo_proof:${{ needs.adl_demo_proof.result }}" \' "$CI_WORKFLOW" >/dev/null
grep -F '"adl_spot_ci_and_coverage:${{ needs.adl_spot_ci_and_coverage.result }}"' "$CI_WORKFLOW" >/dev/null
test "$(grep -Fc 'builder_image_tag: v0.91.7-coverage-5243' "$CI_WORKFLOW")" -eq 1
test "$(grep -Fc 'source_event_name: ${{ github.event_name }}' "$CI_WORKFLOW")" -eq 1
test "$(grep -Fc 'python3 adl/tools/verify_ci_backend_route.py' "$CI_WORKFLOW")" -eq 2
test "$(grep -Fc 'SPOT_OPT_IN: ${{ github.event_name == '"'"'pull_request'"'"' && contains(github.event.pull_request.labels.*.name, '"'"'ci:spot'"'"') }}' "$CI_WORKFLOW")" -eq 2
test "$(grep -Fc -- '--spot-opt-in "$SPOT_OPT_IN"' "$CI_WORKFLOW")" -eq 2
test "$(grep -Fc 'SPOT_WORK_REQUIRED:' "$CI_WORKFLOW")" -eq 2
test "$(grep -Fc -- '--spot-work-required "$SPOT_WORK_REQUIRED"' "$CI_WORKFLOW")" -eq 2
test "$(grep -Fc 'name: adl-spot-ci-and-coverage' "$CI_WORKFLOW")" -eq 1
grep -F "profile: \${{ needs.adl_path_policy.outputs.coverage_required == 'true' && 'adl-ci-and-coverage' || 'adl-ci' }}" "$CI_WORKFLOW" >/dev/null
if grep -E 'name: adl-spot-(ci|coverage)$' "$CI_WORKFLOW" >/dev/null; then
  echo "Spot CI and coverage must share one lifecycle" >&2
  exit 1
fi

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
  --spot-opt-in true \
  --rust-required true --demo-required true \
  --path-policy-result success --spot-result success \
  --hosted-result rust-fmt-clippy=skipped \
  --hosted-result rust-tests=skipped \
  --hosted-result demo-proof=skipped >/dev/null
python3 "$VERIFY_BACKEND_ROUTE" --surface adl-ci --backend spot \
  --event-name pull_request --same-repo-pr true --work-required true \
  --spot-opt-in false \
  --rust-required true --demo-required false \
  --path-policy-result success --spot-result skipped \
  --hosted-result rust-fmt-clippy=success \
  --hosted-result rust-tests=success \
  --hosted-result demo-proof=success >/dev/null
python3 "$VERIFY_BACKEND_ROUTE" --surface adl-ci --backend hosted \
  --event-name pull_request --same-repo-pr true --work-required true \
  --spot-opt-in false \
  --rust-required true --demo-required false \
  --path-policy-result success --spot-result skipped \
  --hosted-result rust-fmt-clippy=success \
  --hosted-result rust-tests=success \
  --hosted-result demo-proof=success >/dev/null
python3 "$VERIFY_BACKEND_ROUTE" --surface adl-ci --backend hosted \
  --event-name pull_request --same-repo-pr true --work-required false \
  --spot-opt-in false \
  --rust-required false --demo-required false \
  --path-policy-result success --spot-result skipped \
  --hosted-result rust-fmt-clippy=skipped \
  --hosted-result rust-tests=skipped \
  --hosted-result demo-proof=skipped >/dev/null
python3 "$VERIFY_BACKEND_ROUTE" --surface adl-coverage --backend spot \
  --event-name pull_request --same-repo-pr true --work-required true \
  --spot-opt-in true \
  --path-policy-result success --spot-result success \
  --hosted-result coverage=skipped >/dev/null
python3 "$VERIFY_BACKEND_ROUTE" --surface adl-coverage --backend spot \
  --event-name pull_request --same-repo-pr true --work-required true \
  --spot-opt-in false \
  --path-policy-result success --spot-result skipped \
  --hosted-result coverage=success >/dev/null
python3 "$VERIFY_BACKEND_ROUTE" --surface adl-coverage --backend spot \
  --event-name pull_request --same-repo-pr true --work-required false \
  --spot-opt-in true \
  --path-policy-result success --spot-result skipped \
  --hosted-result coverage=skipped >/dev/null
python3 "$VERIFY_BACKEND_ROUTE" --surface adl-ci --backend spot \
  --event-name pull_request --same-repo-pr true --work-required false \
  --spot-work-required true \
  --spot-opt-in true \
  --rust-required false --demo-required false \
  --path-policy-result success --spot-result success \
  --hosted-result rust-fmt-clippy=skipped \
  --hosted-result rust-tests=skipped \
  --hosted-result demo-proof=skipped >/dev/null
for invalid_route in \
  'adl-coverage spot pull_request true true success skipped coverage=skipped' \
  'adl-coverage hosted push false true skipped skipped coverage=success' \
  'adl-coverage hosted push false true success skipped coverage=skipped' \
  'adl-coverage spot pull_request true false success success coverage=success' \
  'adl-coverage hosted pull_request true false success skipped coverage=success' \
  'adl-ci hosted pull_request true false success skipped demo-proof=success'
do
  read -r surface backend event same_repo required path_result spot_result hosted_result <<<"$invalid_route"
  if python3 "$VERIFY_BACKEND_ROUTE" --surface "$surface" --backend "$backend" \
      --event-name "$event" --same-repo-pr "$same_repo" --work-required "$required" \
      --spot-opt-in false \
      --path-policy-result "$path_result" --spot-result "$spot_result" \
      --hosted-result "$hosted_result" >/dev/null 2>&1; then
    echo "backend-route verifier accepted invalid route: $invalid_route" >&2
    exit 1
  fi
done
if python3 "$VERIFY_BACKEND_ROUTE" --surface adl-ci --backend spot \
    --event-name pull_request --same-repo-pr true --work-required true \
    --spot-opt-in true \
    --rust-required true --demo-required true \
    --path-policy-result success --spot-result skipped \
    --hosted-result rust-fmt-clippy=skipped --hosted-result rust-tests=skipped \
    --hosted-result demo-proof=success >/dev/null 2>&1; then
  echo "backend-route verifier accepted a skipped selected Spot lane" >&2
  exit 1
fi
if python3 "$VERIFY_BACKEND_ROUTE" --surface adl-ci --backend spot \
    --event-name pull_request --same-repo-pr true --work-required true \
    --spot-opt-in true \
    --rust-required true --demo-required true \
    --path-policy-result success --spot-result success \
    --hosted-result rust-fmt-clippy=success --hosted-result rust-tests=skipped \
    --hosted-result demo-proof=skipped >/dev/null 2>&1; then
  echo "backend-route verifier accepted a selected Spot route with a hosted lane also running" >&2
  exit 1
fi
if python3 "$VERIFY_BACKEND_ROUTE" --surface adl-ci --backend hosted \
    --event-name pull_request --same-repo-pr true --work-required true \
    --spot-opt-in false \
    --rust-required true --demo-required false \
    --path-policy-result success --spot-result skipped \
    --hosted-result rust-fmt-clippy=skipped --hosted-result rust-tests=skipped \
    --hosted-result demo-proof=success >/dev/null 2>&1; then
  echo "backend-route verifier let demo success mask skipped required Rust lanes" >&2
  exit 1
fi
if python3 "$VERIFY_BACKEND_ROUTE" --surface adl-ci --backend hosted \
    --event-name pull_request --same-repo-pr true --work-required true \
    --spot-opt-in false \
    --rust-required true --demo-required false \
    --path-policy-result success --spot-result success \
    --hosted-result rust-fmt-clippy=success --hosted-result rust-tests=success \
    --hosted-result demo-proof=success >/dev/null 2>&1; then
  echo "backend-route verifier accepted a hosted route with the Spot lane also running" >&2
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

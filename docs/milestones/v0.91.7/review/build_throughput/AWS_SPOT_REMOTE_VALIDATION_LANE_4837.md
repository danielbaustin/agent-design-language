# AWS Spot Remote Validation Lane Integration for `#4837`

Status: `implemented_local_contract_proven_prior_live_proof_consumed_cache_volume_restored_github_trigger_ready`
Issue: `#4837`
Date: 2026-07-04

## Scope

This packet records the v0.91.7 WP-06 follow-up that promotes the already
working AWS Spot EC2 validation runner into a repo-owned operator lane.

This issue proves:

- a stable repo-local wrapper exists at
  `adl/tools/run_aws_spot_remote_validation_lane.sh`
- the wrapper defaults to the approved Agent Logic AWS profile,
  `agent-logic-admin`
- the wrapper verifies the selected AWS profile against retained Agent Logic
  proof before live launch
- the new wrapper account check compares hashes and does not print account ids,
  ARNs, user ids, or credentials in wrapper output
- the wrapper refuses to launch EC2 without `--run`
- live execution requires an explicit remote validation command
- the wrapper forwards issue, run id, profile, region, git ref, instance pool,
  summary path, artifact path, warm EBS cache-volume settings, and JSON mode to
  `adl-aws-remote-validation`
- the current `adl-aws-remote-validation` binary surface again supports the
  retained warm EBS cache volume flags that existed in the earlier Spot work
- an account mismatch fails closed before the runner can launch AWS resources
- profile override through arbitrary lower-level wrapper passthrough is not
  supported, so the checked profile cannot be replaced after preflight
- the wrapper defaults live SSH tail settings to the retained debug key surface
- the GitHub Actions workflow can dry-run or launch the Spot lane through OIDC
- the AWS OIDC role setup helper exists and was applied in the Agent Logic AWS
  account

This issue does not prove:

- automatic migration of ordinary PR validation to AWS Spot
- exact final AWS billing for any run
- that Spot capacity will be available for every requested instance shape
- that the repository secret has been inserted through GitHub; root policy
  requires explicit operator authorization before using `gh secret set`

## Implemented Surfaces

- `adl/tools/run_aws_spot_remote_validation_lane.sh`
- `adl/tools/setup_aws_spot_remote_validation_github_resources.sh`
- `adl/tools/test_run_aws_spot_remote_validation_lane.sh`
- `.github/workflows/aws-spot-remote-validation.yaml`
- `adl/src/aws_remote_validation.rs`
- `adl/src/bin/adl_aws_remote_validation.rs`
- `adl/config/validation_lane_selector.v0.91.6.json`
- `adl/tools/test_select_validation_lanes.sh`
- `adl/tools/run_build_platform_benchmark.sh`
- `docs/tooling/AWS_SPOT_REMOTE_VALIDATION_LANE.md`
- `docs/tooling/BUILD_PLATFORM_BENCHMARKS.md`
- `docs/tooling/README.md`

The active AWS runner is the repo binary at `adl/target/debug/adl-aws-remote-validation`.

## Prior Live AWS Evidence Consumed

The lower-level runner's account-bound live AWS proof was completed in `#4603`
under `AWS_PROFILE=agent-logic-admin`.

The retained `#4603` summary was produced before this wrapper and includes AWS
identity fields as part of that historical proof surface. The `#4837` wrapper
does not add new printed account ids, ARNs, user ids, or credentials to its own
account-check output.

Retained hot-cache proof surfaces:

- summary JSON:
  `docs/milestones/v0.91.7/review/build_throughput/remote_validation_4603/live_run_summary_retry11_agentlogic_hotcache.json`
- canonical summary alias:
  `docs/milestones/v0.91.7/review/build_throughput/remote_validation_4603/live_run_summary.json`
- artifact root:
  `docs/milestones/v0.91.7/review/build_throughput/remote_validation_4603/artifacts_retry11_agentlogic_hotcache/attempt-0`

Retained proof highlights:

- status: `passed`
- profile: `agent-logic-admin`
- instance: Spot `m7a.2xlarge`
- retained EBS cache volume: `adl-aws-remote-validation-cache-volume`
- cache volume state: `attached`
- cache mount: `/mnt/adl-cache`
- total runtime: `248s`
- remote command wall time: `163s`
- focused command time inside host: `113s`
- clean termination recorded
- delayed Cost Explorer evidence recorded with explicit delayed-billing caveat

Earlier retry artifacts that were captured in the wrong AWS account remain
historical diagnostics only and are not used as account-bound proof.

## Local Contract Proof

Focused proving command:

```bash
bash adl/tools/test_run_aws_spot_remote_validation_lane.sh
```

Observed result:

```text
PASS test_run_aws_spot_remote_validation_lane
```

The test fixture runs the wrapper with:

- `ADL_AWS_CLI` pointed at a fake STS command
- `--check-account` against a retained-proof fixture containing only an
  account hash
- `--run` against a fake `adl-aws-remote-validation` binary
- default profile and region
- explicit command, git ref, summary path, artifact dir, instance type, and
  JSON mode
- an account-mismatch fixture that must fail before runner invocation

Assertions covered:

- account check prints only bounded success facts and no account id
- dry-run mode does not launch EC2
- `--extra-arg` is rejected so callers cannot override the checked profile with
  later lower-level parser arguments
- runner invocation includes `--profile agent-logic-admin`
- runner invocation includes `--region us-west-2`
- runner invocation includes `--issue 4837`
- runner invocation includes `--instance-type m7a.2xlarge`
- runner invocation includes the retained cache volume flags:
  `--cache-volume-name adl-aws-remote-validation-cache-volume`,
  `--cache-volume-size-gib 100`, `--cache-volume-type gp3`,
  `--cache-volume-iops 3000`, `--cache-volume-throughput-mbps 125`,
  `--cache-volume-device-name /dev/sdf`, and
  `--cache-volume-mount-path /mnt/adl-cache`
- runner invocation includes `--json`
- runner invocation includes retained SSH tail defaults:
  `--ssh-key-name adl-4603-agentlogic-ssh-debug-20260701`,
  `--ssh-private-key-path $HOME/.ssh/adl-4603-ssh-debug-20260701.pem`, and
  `--ssh-user ec2-user`
- workflow and setup surfaces include the OIDC role secret,
  `aws-actions/configure-aws-credentials`, `--profile env`, account checking on
  live runs, warm EBS cache summary text, and SSH tail summary text
- summary and artifact files are retained
- fixture account id, ARN prefix, and user id do not appear in account-check
  stdout
- mismatched account proof fails closed
- finish publication accepts the selected AWS wrapper validation atom and the
  current validation-manager/Nessus wrapper atom as registered validation
  commands

## Operator Command

Account check:

```bash
bash adl/tools/run_aws_spot_remote_validation_lane.sh \
  --check-account \
  --git-ref <branch-or-ref>
```

Live run:

```bash
bash adl/tools/run_aws_spot_remote_validation_lane.sh \
  --run \
  --command '<focused validation command>' \
  --git-ref <branch-or-ref> \
  --out .adl/tmp/aws-spot-remote-validation/<run-id>/summary.json \
  --artifact-dir .adl/tmp/aws-spot-remote-validation/<run-id>/artifacts \
  --instance-type m7a.2xlarge \
  --json
```

The target ref must be advertised by `origin`; the lower-level runner refuses
dirty local worktrees and unadvertised refs before live AWS execution.

The wrapper defaults to the retained WP-06 EBS cache volume. That volume has a
standing AWS storage cost even when no instance is running. Fresh live proof must
show `cache_volume.attachment_state: "attached"` in the AWS summary before it is
claimed as warm-EBS proof.

GitHub Actions setup:

```bash
bash adl/tools/setup_aws_spot_remote_validation_github_resources.sh \
  --apply \
  --profile agent-logic-admin \
  --region us-west-2
```

Observed setup result on 2026-07-04:

```text
PASS aws_spot_remote_validation_github_resources_ready region=us-west-2 profile=agent-logic-admin role_configured=true
```

The helper generated a local `github-actions-config.env` containing the
repository secret value for `AWS_SPOT_REMOTE_VALIDATION_ROLE_ARN`. The value was
not printed in the proof record. It still needs to be inserted into GitHub by an
operator-approved path before the manual workflow can run live from GitHub.

The workflow uses `.github/workflows/aws-spot-remote-validation.yaml`, has only
`workflow_dispatch`, and uses `--profile env` after OIDC credential setup. The
Rust adapter treats `env` and `environment` as ambient AWS credential mode.

## Fresh Branch Baseline

A fresh `#4837` branch run completed while this follow-up was in progress:

- run id: `adl-wp-4837-build-tool-benchmark-20260704`
- status: `passed`
- instance: Spot `m7a.2xlarge`
- cleanup: instance termination recorded as `terminated`
- total runtime: `734s`
- remote command wall time: `655s`
- benchmark build time: `221s`
- benchmark test time: `190s`
- benchmark total command time: `411s`

This run is a useful no-EBS baseline only. It did not include a `cache_volume`
record in the AWS summary and therefore is not accepted as warm-EBS proof.

## Finish-Gate Proof

Issue `#4837` also registers the AWS wrapper fixture in the repo-native finish
validation-command allowlist so the selector can safely publish branches that
touch the new wrapper. While doing that, it also records the already-selected
`test_run_validation_manager_nessus_lane.sh` atom that appears in the current
validation-manager command sequence.

Focused Rust checks:

```bash
cargo test --manifest-path adl/Cargo.toml finish_validation_profile_classifies_validation_manager_slice -- --nocapture
cargo test --manifest-path adl/Cargo.toml finish_validation_profile_classifies_locked_cargo_fallback_slice -- --nocapture
```

Observed result: both focused tests passed after the allowlist and expectation
updates.

## Residual Risk

This issue consumes retained live AWS proof and adds local wrapper/workflow
contract proof plus an applied AWS OIDC role. A fresh live warm-EBS branch run is
still required before claiming current-branch warm-EBS timings.

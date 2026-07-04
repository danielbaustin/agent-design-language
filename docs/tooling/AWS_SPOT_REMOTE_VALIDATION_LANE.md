# AWS Spot Remote Validation Lane

`adl/tools/run_aws_spot_remote_validation_lane.sh` is the repo-owned wrapper for
the AWS Spot EC2 remote validation lane.

It wraps the lower-level `adl-aws-remote-validation` binary from
`adl/target/debug` and keeps the ADL operator contract small:

- default to the approved Agent Logic AWS profile, `agent-logic-admin`
- verify the live STS account hash against retained Agent Logic proof before a
  live run
- never print the AWS account id, ARN contents, or credentials
- require `--run` before any EC2 resources can be launched
- forward one explicit remote validation command to the AWS runner
- reuse the retained warm EBS cache volume by default
- enable live SSH tail logging by default when the retained debug key is present
- retain the AWS runner's summary JSON and artifact directory

## Account Check

Use this before a live run:

```bash
bash adl/tools/run_aws_spot_remote_validation_lane.sh \
  --check-account \
  --git-ref <branch-or-ref>
```

The wrapper calls STS with `--profile agent-logic-admin` by default, hashes the
resolved account id locally, and compares it to the retained hot-cache proof at:

```text
docs/milestones/v0.91.7/review/build_throughput/remote_validation_4603/live_run_summary_retry11_agentlogic_hotcache.json
```

That comparison proves the configured profile resolves to the same account as
the retained Agent Logic proof without recording a static account id in this
operator guide.

## Live Run

For a pushed branch or advertised remote ref:

```bash
bash adl/tools/run_aws_spot_remote_validation_lane.sh \
  --run \
  --command 'cargo test --manifest-path adl/Cargo.toml --locked --lib provider_communication -- --nocapture' \
  --git-ref <branch-or-ref> \
  --out .adl/tmp/aws-spot-remote-validation/<run-id>/summary.json \
  --artifact-dir .adl/tmp/aws-spot-remote-validation/<run-id>/artifacts \
  --instance-type m7a.2xlarge \
  --json
```

By default the wrapper forwards the retained WP-06 cache volume:

```text
name: adl-aws-remote-validation-cache-volume
size/type: 100 GiB gp3
iops/throughput: 3000 IOPS / 125 MiB/s
device/mount: /dev/sdf -> /mnt/adl-cache
```

This EBS volume is a standing AWS resource and therefore has a standing storage
cost even when no Spot instance is running. Keep it only while the lane is used
often enough to justify the warm cache. Do not change `--cache-volume-name`
unless intentionally creating a separate retained cache.

The remote bootstrap mounts the volume and places shared build state under it:

```text
/mnt/adl-cache/adl-aws-remote-validation/shared/target
/mnt/adl-cache/adl-aws-remote-validation/shared/sccache
/mnt/adl-cache/adl-aws-remote-validation/shared/cargo-home
/mnt/adl-cache/adl-aws-remote-validation/shared/rustup-home
```

The underlying AWS runner still owns launch-surface preparation, Spot-first
selection, on-demand fallback for classified Spot capacity failures, SSM command
dispatch, retained logs, interruption classification, and cleanup truth.

## GitHub Actions Trigger

The manual workflow `.github/workflows/aws-spot-remote-validation.yaml` can
render a dry-run request or start the live Spot lane through GitHub Actions
OIDC. It intentionally has no `push` or `pull_request` trigger.

Create or refresh the AWS OIDC role with:

```bash
bash adl/tools/setup_aws_spot_remote_validation_github_resources.sh \
  --apply \
  --profile agent-logic-admin \
  --region us-west-2
```

The setup helper writes a chmod-600 `github-actions-config.env` under the
selected artifact directory. Configure the repository secret from that file:

```text
AWS_SPOT_REMOTE_VALIDATION_ROLE_ARN
```

The workflow uses `--profile env` after `aws-actions/configure-aws-credentials`
assumes that role. The Rust runner treats `env` and `environment` as ambient AWS
credentials rather than as named local profile names.

Live workflow runs inherit the wrapper's defaults for:

```text
warm EBS cache: adl-aws-remote-validation-cache-volume -> /mnt/adl-cache
SSH tail key: adl-4603-agentlogic-ssh-debug-20260701
SSH user: ec2-user
```

## Benchmark Command

Use the shared benchmark helper when comparing build platforms:

```bash
bash adl/tools/run_aws_spot_remote_validation_lane.sh \
  --run \
  --command 'bash adl/tools/run_build_platform_benchmark.sh --platform aws_spot --cache-posture warm_ebs_cache --out .adl/tmp/build-platform-benchmark/aws-spot-ebs-<date>/summary.json --artifact-dir .adl/tmp/build-platform-benchmark/aws-spot-ebs-<date>' \
  --git-ref <branch-or-ref> \
  --out .adl/tmp/aws-spot-remote-validation/<run-id>/summary.json \
  --artifact-dir .adl/tmp/aws-spot-remote-validation/<run-id>/artifacts \
  --instance-type m7a.2xlarge \
  --json
```

The AWS summary must contain `cache_volume.attachment_state: "attached"` before
the run can be claimed as warm-EBS proof. A benchmark line or cache-posture
string alone is not enough.

## Retained Proof

Issue `#4837` integrates the lane entry point and consumes the earlier live AWS
proof from `#4603`.

Retained account-bound hot-cache proof:

- summary:
  `docs/milestones/v0.91.7/review/build_throughput/remote_validation_4603/live_run_summary_retry11_agentlogic_hotcache.json`
- canonical alias:
  `docs/milestones/v0.91.7/review/build_throughput/remote_validation_4603/live_run_summary.json`
- artifacts:
  `docs/milestones/v0.91.7/review/build_throughput/remote_validation_4603/artifacts_retry11_agentlogic_hotcache/attempt-0`

The retained run used `agent-logic-admin`, launched Spot `m7a.2xlarge`, reused
the retained EBS cache volume, passed, completed in `248s`, recorded `163s`
remote command wall time, recorded `113s` focused command time inside the host,
and recorded clean termination.

Historical retry surfaces captured in the wrong AWS account are not accepted as
current account-bound proof for this lane.

## Failure Posture

The wrapper fails closed when:

- the selected AWS profile does not resolve
- the resolved account hash differs from the retained Agent Logic proof
- `--run` is set without an explicit remote command
- the runner binary is unavailable or not executable
- the underlying AWS runner reports launch, SSM, validation, interruption, or
  cleanup failure

Fresh live AWS execution may incur AWS charges. Keep commands focused, use an
advertised remote ref, and retain summary plus artifact paths in the issue SOR.

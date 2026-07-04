# AWS Spot Remote Validation Lane

`adl/tools/run_aws_spot_remote_validation_lane.sh` is the repo-owned wrapper for
the AWS Spot EC2 remote validation lane.

It wraps the lower-level `adl-aws-remote-validation` binary from
`tools/aws_remote_validation` and keeps the ADL operator contract small:

- default to the approved Agent Logic AWS profile, `agent-logic-admin`
- verify the live STS account hash against retained Agent Logic proof before a
  live run
- never print the AWS account id, ARN contents, or credentials
- require `--run` before any EC2 resources can be launched
- forward one explicit remote validation command to the AWS runner
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

The underlying AWS runner still owns launch-surface preparation, Spot-first
selection, on-demand fallback for classified Spot capacity failures, SSM command
dispatch, retained logs, interruption classification, and cleanup truth.

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

The retained run used `agent-logic-admin`, launched Spot `m7a.2xlarge`, passed,
completed in `248s`, recorded `163s` remote command wall time, recorded `113s`
focused command time inside the host, and recorded clean termination.

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

# AWS Spot Remote Validation Lane

## Metadata

- Feature Name: AWS Spot Remote Validation Lane
- Milestone Target: `v0.91.7`
- Status: implemented
- Owner: ADL maintainers
- Doc Role: retained implementation and proof-update surface
- Feature Types: tooling, build-throughput, aws, validation
- Proof Modes: focused Rust tests, CLI-shape tests, live-gated AWS execution

## Purpose

Provide an ADL-owned disposable EC2 build/test lane that can launch through AWS
Spot when capacity allows, fall back deterministically to on-demand when Spot
is blocked, run one focused ADL validation command over SSM, retain timing plus
`sccache` evidence, and terminate the builder cleanly.

## Implemented Surface

- Standalone binary: `adl-aws-remote-validation`
- Cargo package: `tools/aws_remote_validation/Cargo.toml`
- Source entrypoint: `tools/aws_remote_validation/src/bin/adl_aws_remote_validation.rs`
- Orchestration module: `tools/aws_remote_validation/src/aws_remote_validation.rs`
- AWS SDK crates:
  - `aws-config`
  - `aws-sdk-ec2`
  - `aws-sdk-ssm`
  - `aws-sdk-sts`
  - `aws-sdk-servicequotas`
  - `aws-sdk-costexplorer`
  - `aws-sdk-budgets`

## Current Behavior

The tool:

- resolves caller identity through STS and stores only bounded account evidence;
- discovers EC2 Spot and on-demand quota through Service Quotas;
- auto-prepares a disposable launch surface when explicit AWS ids are omitted:
  default VPC subnet resolution, public Amazon Linux 2023 AMI lookup, and
  temporary security-group plus SSM role/profile creation;
- tries a bounded allowed instance pool in order, preferring Spot first;
- falls back to on-demand only for explicitly classified Spot-capacity failures;
- waits for SSM `Online` readiness before dispatching work;
- runs one remote shell payload over `AWS-RunShellScript`;
- bootstraps `git`, `rustup`, `cargo`, and `sccache` on the disposable host;
- clones ADL, checks out the requested remote ref, and runs a focused ADL command;
- refuses to start a live run when the local worktree is dirty or the requested
  ref is not advertised by `origin`, so the lane cannot silently validate
  `origin/main` while local issue work remains uncommitted;
- captures timing for launch, SSM readiness, remote execution, and teardown;
- records `sccache` evidence in the retained remote summary payload;
- captures AWS-side Spot termination evidence from EC2 and the Spot request
  record so provider reclaim is distinguished from user-initiated teardown;
- queries Cost Explorer and optional Budgets evidence after the run;
- always requests teardown and records cleanup truth plus termination state.

## Operator Boundary

The current implementation can consume reviewed baseline AWS ids or
auto-provision a bounded disposable launch surface for one run. It still keeps
the repo source of truth local and read-only from AWS's perspective.

Required launch inputs:

- one focused validation command
- an advertised remote git ref containing the code to validate

This issue does not claim a persistent EC2 fleet, broad CI migration, or a
mutable remote workspace that can author canonical repo truth.

## Retained Evidence Surfaces

- machine-readable summary JSON written to `--out`
- event log JSONL written under `--artifact-dir/events.jsonl`
- remote stdout/stderr logs written under `--artifact-dir/`
- retained remote summary payload with:
  - launch mode
  - timing fields
  - interruption detection
  - resolved commit
  - toolchain versions
  - `sccache` excerpt
- Cost Explorer snapshot with delayed-billing caveat
- optional Budgets snapshot

## Failure And Debug Posture

The tool is intended to fail closed with enough evidence to debug:

- quota discovery failures
- Spot launch rejection and fallback path selection
- SSM readiness failure
- remote bootstrap failure
- remote validation failure
- Spot interruption during execution
- teardown request or termination-wait failure
- remote-source drift between the bound worktree and the advertised git ref

Human-readable observability remains bounded to retained logs and the summary
artifact. The tool does not claim broad CI migration, per-run final AWS billing
precision, or general Spot savings from this implementation alone.

## Focused Validation

Local focused proof run for this issue:

- `cargo fmt --manifest-path tools/aws_remote_validation/Cargo.toml`
- `cargo build --manifest-path tools/aws_remote_validation/Cargo.toml --bin adl-aws-remote-validation`
- `cargo test --manifest-path tools/aws_remote_validation/Cargo.toml --bin adl-aws-remote-validation -- --nocapture`

Those tests currently prove:

- CLI argument parsing and bounded defaults
- summary extraction and retained artifact behavior
- Spot-first success path
- Spot-capacity fallback to on-demand
- interruption classification
- remote summary script environment handling

## Live-Gated Follow-Through

The retained account-bound proving runs for issue `#4603` completed
successfully on `2026-07-01` under `AWS_PROFILE=agent-logic-admin` in the
Agent Logic AWS account (`713332525889`, user `daniel.austin.admin`) and now
prove:

- bounded launch-surface preparation in the approved AWS account with the
  persistent security group `sg-052e1b4273335e5f7`;
- truthful cold-baseline fallback from Spot to on-demand on `m7a.2xlarge` in
  `us-west-2a` when capacity is unavailable;
- truthful hot-cache execution on Spot `m7a.2xlarge` in `us-west-2a`;
- SSM readiness, SSH tail access, remote command dispatch, cache-volume attach,
  and clean termination;
- remote validation of pushed branch commits rather than `origin/main`,
  including the stable-cache-path fix at
  `afce29350658cb1992d27edbe943f98970fd0c8a`;
- persistent shared-cache reuse on `vol-0934411b84ad4209b` mounted at
  `/mnt/adl-cache`, reducing repeated bootstrap and compile work materially;
- delayed-billing cost evidence capture plus machine-readable cleanup truth.

Retained hot-cache account-bound proof artifacts:

- summary JSON:
  `docs/milestones/v0.91.7/review/build_throughput/remote_validation_4603/live_run_summary_retry11_agentlogic_hotcache.json`
- canonical summary alias:
  `docs/milestones/v0.91.7/review/build_throughput/remote_validation_4603/live_run_summary.json`
- artifact root:
  `docs/milestones/v0.91.7/review/build_throughput/remote_validation_4603/artifacts_retry11_agentlogic_hotcache/attempt-0`

Retained retry-11 hot-cache timing and proof highlights:

- total lane runtime: `248s`
- launch time: `15s`
- SSM readiness: `15s`
- remote command wall time: `163s`
- remote bootstrap time inside the host: `7s`
- focused validation command time inside the host: `113s`
- focused ADL proof:
  - `cargo build --manifest-path adl/Cargo.toml --locked --bin adl-pr-doctor`
    finished in `1m 02s`
  - `cargo test --manifest-path adl/Cargo.toml --locked --lib provider_communication`
    passed `21` tests with `1492` filtered
- `sccache` result:
  - compile requests `3`
  - executed `0`
  - cache hits `0`
  - cache misses `0`
  - non-cacheable calls `3` (`incremental` root-crate work only)
  - degraded: `false`
- sampled Spot price evidence: `0.1728 USD/hour` for `m7a.2xlarge` in
  `us-west-2a`, giving an approximate instance-runtime cost of `0.0119 USD`
  for the `248s` retained hot-cache pass
- delayed Cost Explorer evidence: `0 USD` with explicit delayed-billing caveat

Retained retry-10 cold-baseline comparison artifacts remain available at:

- `docs/milestones/v0.91.7/review/build_throughput/remote_validation_4603/live_run_summary_retry10_agentlogic.json`
- `docs/milestones/v0.91.7/review/build_throughput/remote_validation_4603/artifacts_retry10_agentlogic/attempt-0`

Retry-10 versus retry-11 account-bound comparison:

- total runtime improved from `588s` to `248s` (`340s`, about `57.8%` faster)
- remote command wall time improved from `525s` to `163s` (`362s`, about
  `69.0%` faster)
- in-host focused validation time improved from `280s` to `113s` (`167s`,
  about `59.6%` faster)
- remote bootstrap improved from `199s` to `7s` (`192s`, about `96.5%`
  faster)

The earlier retained `retry5` and `retry6` review surfaces remain historical
artifacts, but they were captured in the wrong AWS account and must not be used
as the account-bound proof for this lane.

## Non-Goals

- No claim that all CI should move to EC2.
- No claim that remote runs include uncommitted local source without a future
  archive/snapshot transport mode.
- No claim that Spot always succeeds for larger instance shapes.
- No claim that delayed AWS billing surfaces provide exact per-run final cost.

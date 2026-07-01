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

Live attempts retained under
`docs/milestones/v0.91.7/review/build_throughput/remote_validation_4603/`
already prove:

- bounded launch-surface preparation in the Agent Logic AWS account;
- SSM readiness and remote command dispatch on disposable EC2 builders;
- truthful distinction between a real Spot-capacity reclaim and
  user-initiated cleanup after command failure;
- delayed-billing cost evidence capture.

This document does not yet claim a successful end-to-end validation of the
current `#4603` issue worktree, because an earlier live run was validating
`origin/main` rather than the bound issue branch. The lane now fails fast on
that condition instead of spending AWS time on the wrong source snapshot.

The proving live run for v0.91.7 should:

- use the Agent Logic AWS account/profile approved for this lane;
- run from a clean issue worktree whose branch is pushed to `origin`;
- execute a real focused ADL validation command, not a shell smoke command;
- retain the generated summary and event log artifacts;
- confirm cleanup state after termination;
- record truthful timing and delayed-billing boundaries.

## Non-Goals

- No claim that all CI should move to EC2.
- No claim that remote runs include uncommitted local source without a future
  archive/snapshot transport mode.
- No claim that Spot always succeeds for larger instance shapes.
- No claim that delayed AWS billing surfaces provide exact per-run final cost.

# Remote Build And Validation Lanes Proof for `#4587`

Status: `nessus_operational_ec2_on_demand_proven_spot_blocked`
Issue: `#4587`
Date: 2026-06-27

## Scope

This packet converges the remote build-lane evidence needed for the v0.91.6
rescue sprint.

It proves:

- ADL has a current non-wuji remote validation lane on `nessus.local`
- the Nessus lane can run a focused ADL Rust validation command without the full
  release test surface
- the retained Nessus runner emits machine-readable evidence and a small log
  bundle
- AWS EC2 can run the same focused Rust validation proof through SSM on a
  short-lived Linux host
- temporary EC2 resources used by the proof were torn down before closeout

It does not prove:

- all ADL validation should run remotely
- provider-credentialed or network-bound validation is remote-safe by default
- GitHub CI has migrated to Nessus or EC2
- Spot savings are currently usable in the Agent Logic AWS account
- EC2 is fast enough without a prebuilt image or persistent cache

## Decision

Use Nessus as the immediate Phase 1 remote validation lane.

Treat EC2 as a Phase 2 reproducibility and burst-capacity lane. The live proof
showed that SSM-driven EC2 validation works, but manual EC2 launch/bootstrap is
too slow and too fragile for routine use. Before repeating EC2 build proofs as a
normal workflow, implement a first-class AWS orchestrator / Spot manager that
handles quota discovery, instance-type fallback, SSM command upload and polling,
log capture, cost accounting, teardown guards, and `sccache` bootstrap.

## Retained Evidence Consumed

| Surface | Status | Evidence |
|---|---|---|
| Local build baseline | retained | `docs/milestones/v0.91.6/review/build_throughput/BUILD_THROUGHPUT_MEASUREMENT_4315.md` |
| CodeBuild/EC2-style cloud evaluation | deferred pilot | `docs/milestones/v0.91.6/review/build_throughput/CODEBUILD_REMOTE_VALIDATION_EVALUATION_4316.md` |
| Nessus WSL runner bootstrap | proven | `docs/milestones/v0.91.6/review/build_throughput/NESSUS_LOCAL_REMOTE_RUST_VALIDATION_RUNNER_4317.md` |
| Nessus validation-manager lane | proven | `docs/milestones/v0.91.6/review/build_throughput/NESSUS_REMOTE_VALIDATION_LANE_4553.md` |
| Fresh Nessus issue proof | proven | `docs/milestones/v0.91.6/review/build_throughput/remote_build_lanes_4587/live_nessus_20260627/summary.json` |
| Fresh EC2 issue proof | proven on-demand after quota blocks | `docs/milestones/v0.91.6/review/build_throughput/remote_build_lanes_4587/ec2_spot_20260627/ec2-build-summary.json` |
| EC2 teardown proof | proven | `docs/milestones/v0.91.6/review/build_throughput/remote_build_lanes_4587/ec2_spot_20260627/teardown-verification.json` |

## Fresh Nessus Proof

The live `#4587` proof ran the repo-native Nessus remote validation wrapper:

```text
bash adl/tools/run_nessus_remote_validation.sh --run-id 20260627-issue-4587 --git-ref origin/main --command 'cargo test --manifest-path adl/Cargo.toml --locked provider_communication -- --nocapture'
```

Observed result:

- status: `passed`
- run id: `20260627-issue-4587`
- resolved commit: `b3c4528fc08b6284aa9ee3c50d209b88d64f59d5`
- elapsed seconds: `92`
- command: `cargo test --manifest-path adl/Cargo.toml --locked provider_communication -- --nocapture`
- remote run root: `/root/adl-remote-runner/logs/20260627-issue-4587`
- retained local summary: `docs/milestones/v0.91.6/review/build_throughput/remote_build_lanes_4587/live_nessus_20260627/summary.json`
- retained local log bundle: `docs/milestones/v0.91.6/review/build_throughput/remote_build_lanes_4587/live_nessus_20260627/run-logs.tar.gz`

The command log in the retained bundle records:

- `21 passed; 0 failed`
- no failed filtered harnesses

Cache interpretation:

- `compile_requests`: `102`
- `compile_requests_executed`: `0`
- `non_cacheable_calls`: `102`
- `cache_hits`: `0`
- `cache_misses`: `0`

The retained `sccache-stats.log` shows the run compiled the ADL test target, but
the requests were non-cacheable incremental/crate-type calls rather than cache
hits or cache misses. The command ran against current `origin/main` and completed
successfully; it does not prove a no-rebuild warm-cache path.

## Nessus Lane Contract

Operational command shape:

```text
bash adl/tools/run_nessus_remote_validation.sh \
  --git-ref origin/main \
  --command '<focused ADL command>'
```

Default host contract:

- SSH host: `nessus.local`
- SSH user: `danie`
- WSL user: `root`
- remote workspace root: `/root/adl-remote-runner`
- repo checkout: `/root/adl-remote-runner/agent-design-language`
- target dir: `/root/adl-remote-runner/cache/target`
- sccache dir: `/root/adl-remote-runner/cache/sccache`
- run logs: `/root/adl-remote-runner/logs/<run-id>`

Lane use policy:

- choose the smallest focused validation command that proves the touched
  surface
- do not run the full release test surface by default
- fail closed on SSH, setup, checkout, build, or validation failure
- retain `summary.json` and a bounded log bundle when the issue requires proof
- do not copy provider credentials, GitHub credentials, AWS credentials, or
  local key material to the remote host as part of this lane

## EC2 Spot And On-Demand Proof

The live `#4587` EC2 proof was executed in the Agent Logic AWS account with
`AWS_PROFILE=agent-logic-admin` and `AWS_REGION=us-west-2`.

Attempted path:

1. Create a temporary SSM role, instance profile, and security group for issue
   `#4587`.
2. Request one short-lived `c7i.4xlarge` Spot instance.
3. Fall back to on-demand only after Spot was rejected by account quota.
4. Run a focused Rust build and validation command through SSM on the smallest
   permitted fallback that was launched.
5. Terminate the instance and delete the temporary role/profile/security group.

Observed AWS constraints:

- Spot launch was blocked by `MaxSpotInstanceCountExceeded`.
- The retained compact packet proves the Spot-blocked path followed by a
  short-lived on-demand `c7i.2xlarge` instance.

Observed successful EC2 result:

- status: `passed`
- purchase option: `on_demand_after_spot_quota_block`
- instance handle: `sha256:f916e7a4ecab11676d2f2505bfa63b53508831b7764779d3fbf2609da5b311ec`
- resource handles: `ADLSpotBuild4587InstanceProfile`,
  `ADLSpotBuild4587SSMRole`, `adl-spot-build-4587`, issue tag `4587`
- instance type: `c7i.2xlarge`
- resolved commit: `b3c4528fc08b6284aa9ee3c50d209b88d64f59d5`
- elapsed seconds: `575`
- binary build seconds: `99`
- focused test seconds: `252`
- retained local summary: `docs/milestones/v0.91.6/review/build_throughput/remote_build_lanes_4587/ec2_spot_20260627/ec2-build-summary.json`
- teardown proof: `docs/milestones/v0.91.6/review/build_throughput/remote_build_lanes_4587/ec2_spot_20260627/teardown-verification.json`

Commands proven on EC2:

```text
cargo build --manifest-path adl/Cargo.toml --locked --bin adl-pr-doctor
cargo test --manifest-path adl/Cargo.toml --locked provider_communication -- --nocapture
```

Teardown state:

- EC2 instance: `terminated`
- temporary security group: deleted
- temporary instance profile: deleted
- temporary SSM role: deleted

Cost interpretation:

- The operator approved up to `$50` for this proof.
- Final AWS billing line items were not available at closeout time.
- The retained packet does not prove final spend.
- The current proof does not establish Spot savings, because Spot capacity was
  quota-blocked before an instance was created.

## Bootstrap Findings

The manual EC2 proof exposed the work the orchestrator must absorb before EC2 is
usable as a routine lane:

- Amazon Linux 2023 package bootstrap hit a `curl` / `curl-minimal` conflict
  when the script installed `curl` unnecessarily.
- SSM command execution did not guarantee `HOME`, so the successful rerun had
  to set `HOME=/root` explicitly.
- Installing `sccache` from source during the proof added about `3m40s` of paid
  bootstrap time.
- The focused cargo test still compiled many binary test harnesses, which
  confirms that validation targeting must keep improving.
- Manual AWS CLI command construction is too error-prone for routine C-SDLC
  validation. The next implementation should be a Rust tool, not shell paste.

## Rust Orchestrator Requirement

The next EC2/Spot lane implementation should use the official AWS SDK for Rust
with async `tokio` execution.

Expected crate families:

- `aws-config` for profile, region, and credential loading
- `aws-sdk-ec2` for instance, Spot, security group, subnet, AMI, and teardown
  operations
- `aws-sdk-ssm` for command execution and polling
- IAM/CloudWatch/Cost Explorer SDK crates as needed for role/profile,
  observability, and cost proof

Do not use Rusoto for new implementation work.

Required product behavior:

- discover Spot and on-demand quotas before launch
- select the smallest useful instance type from an allowed pool
- prefer Spot when quota and capacity allow it
- fall back deterministically when Spot is blocked
- install or attach a prebuilt Rust/sccache environment
- run exactly the requested focused validation command
- preserve stdout/stderr/log artifacts without secrets
- terminate instances and delete temporary resources even on failure
- emit a single machine-readable build-lane summary

## Current Recommendation

For v0.91.6 closeout:

1. Use Nessus for focused heavyweight local-offload proof when a lane is already
   known to be remote-safe.
2. Keep ordinary narrow docs/tooling fixes local when focused shell proof is
   sufficient.
3. Treat EC2 as proven possible but not operationally ready until the AWS
   orchestrator / Spot manager lands.
4. Treat Spot savings as blocked by current Spot quota until quota is raised or
   the orchestrator proves a smaller Spot pool works.

## Non-Claims

- This packet does not claim current AWS credits are available.
- This packet does not claim Spot was successfully used.
- This packet does not claim exact final AWS cost line items.
- This packet does not claim CodeBuild is rejected; `#4316` keeps it as a gated
  later pilot candidate.
- This packet does not claim Nessus is appropriate for provider credentials or
  all runtime proofs.
- This packet does not claim the full ADL test surface ran remotely.

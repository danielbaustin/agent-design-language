# WP-06 Build Throughput And Validation-Cost Reduction Sprint Packet for `#4633`

Status: `sprint_lane_partially_complete_pr_tail_active`
Issue: `#4633`
Date: 2026-07-04

## Scope

This packet records the selected WP-06 sprint lane executed under umbrella
issue `#4633`.

The operator-selected lane covered:

- `#4676` validation manager
- `#4800` fast validation lane plus fanned slow families
- `#4698` long-test fanout reduction
- `#4726` command-owned binary decomposition
- `#4677` CI log archive to S3
- `#4678` Nessus remote validation-lane consumption

This packet does not claim all WP-06 work is finished. In particular, the
`#4679` remote-builder work is already split into two bounded follow-up issues:

- `#4837` finish/integrate the existing AWS Spot EC2 remote lane work
- `#4838` create and test a GitHub Actions plus AWS CodeFriend build lane

Duplicate issues `#4858` and `#4859` were accidentally created during this
rollup, then commented and closed as `not_planned` duplicates of `#4837` and
`#4838`.

## Child Issue State

| Issue | Scope | Current truth | PR/check truth | Closeout truth |
| --- | --- | --- | --- | --- |
| `#4676` | Validation manager | closed/merged | PR `#4828`, `adl-ci` success, `adl-coverage` success, `adl-slow-proof` skipped | local closeout run; worktree pruned |
| `#4800` | Fast validation lane plus fanned slow families | closed/merged | PR `#4832`, `adl-ci` success, `adl-coverage` success, `adl-slow-proof` skipped | local closeout run; worktree already absent |
| `#4698` | Reduce long-test fanout | closed/merged | PR `#4839`, `adl-ci` success, `adl-coverage` success, `adl-slow-proof` skipped | local closeout run; worktree already absent |
| `#4726` | Decompose monolithic `adl` binary into command-owned tools | draft PR open | PR `#4852`, `adl-coverage` success, `adl-slow-proof` skipped, `adl-ci` failed | janitor action required |
| `#4677` | CI log archive to S3 | PR open, not draft | PR `#4856`, `adl-ci` success, `adl-coverage` success, `adl-slow-proof` skipped | waiting for human review/merge |
| `#4678` | Consume Nessus remote validation lane | PR open, not draft | PR `#4857`, `adl-ci` success, `adl-coverage` success, `adl-slow-proof` skipped | waiting for human review/merge |

## Merged Capabilities

The merged child issues establish these integrated paths:

- validation-manager path/profile selection and readiness routing
- fast PR validation selection with fanned slow proof families
- reduced default fanout for long tests

## PR-Tail Candidate Capabilities

The following capabilities are implemented on open PR branches but are not
claimed as merged or fully closeouted yet:

- first pass of command-owned `adl` binary decomposition
- S3-backed CI log archive command with manifest truth and live S3 synthetic
  proof
- validation-manager wrapper that consumes an eligible local lane and routes it
  to Nessus with remote-safe changed-file manifest handling and explicit remote
  git ref support

## Validation Evidence

Retained child proof surfaces include:

- `docs/milestones/v0.91.7/review/pr_finish_release_gate_disposition/PR_FINISH_RELEASE_GATE_DISPOSITION_PROOF_4787.md`
- `docs/milestones/v0.91.7/review/build_throughput/CI_LOG_ARCHIVE_S3_4677.md`
- `docs/milestones/v0.91.7/review/build_throughput/NESSUS_VALIDATION_MANAGER_LANE_4678.md`

Local #4633 rollup checks:

```text
git diff --check
```

Child issue checks were recorded in their own SOR/proof packets. This umbrella
packet intentionally does not restate broad runtime/product proof that belongs
to child issues.

## PR Tail Truth

The current PR tail is mixed:

- `#4726` / PR `#4852` is draft/open and needs janitor because `adl-ci` failed.
- `#4677` / PR `#4856` is open, not draft, and green; it is waiting for human
  review or merge.
- `#4678` / PR `#4857` is open, not draft, and green; it is waiting for human
  review or merge.

The umbrella issue must not close as fully complete until the open PR tail
settles and closeout truth is reconciled, unless the operator explicitly accepts
a partial sprint close with those PR tails still open.

## Remaining WP-06 Work

`#4679` has been split before execution:

1. `#4837` AWS Spot EC2 remote lane integration. This should consume the earlier Spot
   work and prove the integrated lane path, cleanup behavior, and retained
   time/cost evidence.
2. `#4838` GitHub Actions plus AWS CodeFriend build lane. This should prove the build
   lane from GitHub Actions through AWS CodeFriend-style infrastructure with
   credential, cleanup, log, and cost boundaries.

Earlier evidence to reference when creating those issues:

- `docs/milestones/v0.91.6/review/build_throughput/REMOTE_BUILD_LANES_4587.md`
- `docs/milestones/v0.91.7/review/build_throughput/remote_validation_4603/`
- `docs/milestones/v0.91.7/features/AWS_SPOT_REMOTE_VALIDATION_LANE_v0.91.7.md`

## Non-Claims

- This packet does not claim WP-06 is fully done.
- This packet does not claim the open draft PRs have passed `adl-ci`.
- This packet does not claim fresh live SSH Nessus proof for `#4678`; that
  issue proved the wrapper contract locally and referenced prior live Nessus
  evidence.
- This packet does not claim AWS Spot or GitHub Actions plus AWS CodeFriend
  build lanes are integrated; it records that follow-up issues `#4837` and
  `#4838` own those proofs.

# v0.91.2 Coverage and Quality Gate

## Metadata

- Milestone: `v0.91.2`
- Status: `recorded_not_release_ready`
- Canonical gate WP: `WP-18`
- Tracked packet: `docs/milestones/v0.91.2/review/quality_gate/QUALITY_GATE_PACKET_v0.91.2.md`
- Repro command: `bash adl/tools/demo_v0912_quality_gate.sh`

## Purpose

This document is the milestone-level quality gate surface for `v0.91.2`. It
records the current CI, coverage, demo/proof, closeout-truth, and release-tail
posture without overclaiming final release readiness before the remaining
Sprint 4 work lands.

## Current State

- `WP-01` through `WP-22` are closed, and `WP-17A` is also closed as a bounded
  demo follow-on.
- Sprint 1, Sprint 2, and Sprint 3 are closed; Sprint 4 is still open.
- The milestone now has a converged demo/proof map, a release-tail doc set, and
  a passing closed-issue `SOR` truth checker.
- The first `WP-20` internal review packet was too thin for external handoff;
  `WP-20B` is the controlling full internal review packet.
- Accepted `WP-20B` remediation issues are closed, `WP-21` external review is
  closed, and `WP-22` remediation is closed.
- `WP-23` next-milestone planning is in review through PR `#3192`; `WP-24`
  release ceremony remains open and has not approved the release.
- The milestone does not yet have a final ceremony outcome,
  so this gate is not the final release verdict.

## Current Gate Dimensions

| Dimension | Current posture | Evidence | Boundary |
| --- | --- | --- | --- |
| Demo/proof coverage map | `pass` | [DEMO_MATRIX_v0.91.2.md](DEMO_MATRIX_v0.91.2.md), [FEATURE_PROOF_COVERAGE_v0.91.2.md](FEATURE_PROOF_COVERAGE_v0.91.2.md), [DEMO_PROOF_CONVERGENCE_PACKET_v0.91.2.md](review/demo_proof_convergence/DEMO_PROOF_CONVERGENCE_PACKET_v0.91.2.md), and [CODE_FEATURE_DEMO_FOLLOW_ONS_PACKET_v0.91.2.md](review/code_feature_demo_follow_ons/CODE_FEATURE_DEMO_FOLLOW_ONS_PACKET_v0.91.2.md) | Converged proof coverage is present; it is not the same thing as release approval. |
| CI/coverage policy contract | `pass` | `adl/tools/test_ci_path_policy.sh`, `adl/tools/test_ci_runtime_contracts.sh`, `adl/tools/test_run_authoritative_coverage_lane.sh`, and `adl/tools/test_check_coverage_impact.sh` | A green policy/contract surface proves the current CI split is internally consistent, not that full release coverage has already been rerun for this milestone. |
| Closed-issue closeout truth | `partial` | Sprint 1 through Sprint 3 closeout truth is materially cleaner; known retained `#3121` residue remains explicitly deferred out of this issue | This keeps the milestone honest about one deferred closeout-truth gap instead of pretending the whole retained layer is clean. |
| Full authoritative coverage evidence | `pass` | `bash adl/tools/run_authoritative_coverage_lane.sh` completed during WP-18 with `2066` tests passed, `2` skipped, and `coverage-summary.json` emitted | Full release coverage has now been captured explicitly for the current milestone state; this still does not replace later Sprint 4 review/remediation/ceremony work. |
| Release-tail review/remediation/ceremony | `not_ready` | `WP-19` docs review is closed; `WP-20B` is the controlling internal review packet; accepted `WP-20B` remediation issues are closed; `WP-21` external review is closed; `WP-22` remediation is closed; `WP-23` next-milestone planning is in review through PR `#3192` | The milestone cannot proceed to release ceremony until `WP-23` is reviewed/merged and `WP-24` release ceremony completes. |

## Controlling Review Packet Note

For Sprint 4 review and remediation routing, the controlling internal-review
surface is:

- `docs/milestones/v0.91.2/review/internal_review_full/`

The older thin `WP-20` packet under `review/internal_review/` remains
background context only and must not be treated as the controlling
external-review or remediation handoff surface.

## Required Inputs Before Final Pass/Fail Judgment

- demo matrix and feature-proof coverage
- quality-gate packet and command surface
- closeout-truth status, including any explicitly deferred retained-card residue
- review records
- remediation record
- `WP-20B` accepted-finding fixes and re-review outcome
- release evidence and release readiness package
- release ceremony and end-of-milestone report

## Commands

Focused gate contract checks:

```bash
bash adl/tools/test_demo_v0912_quality_gate.sh
bash adl/tools/test_ci_path_policy.sh
bash adl/tools/test_ci_runtime_contracts.sh
bash adl/tools/test_run_authoritative_coverage_lane.sh
bash adl/tools/test_check_coverage_impact.sh
```

Reviewer-facing aggregation surface:

```bash
bash adl/tools/demo_v0912_quality_gate.sh
```

Optional heavy local gate when you want the reviewer-facing quality-gate command
to rerun `fmt`, `clippy`, and the authoritative full coverage lane itself:

```bash
ADL_V0912_QUALITY_GATE_RUN_HEAVY=1 bash adl/tools/demo_v0912_quality_gate.sh
```

## Current Judgment

`NOT_READY`

## Non-Claims

- This gate does not claim `v0.91.2` is release-ready yet.
- This gate does not treat docs-only or contract-only checks as full release
  coverage evidence.
- This gate does not replace the later Sprint 4 review, remediation, release
  evidence, or ceremony surfaces.
- This gate does not allow external review to proceed from the superseded thin
  `WP-20` packet. `WP-21` starts from the refreshed handoff after accepted
  `WP-20B` remediation issues closed.
- This gate does not claim the entire retained local closeout-truth layer is
  clean while `#3121` remains explicitly deferred.

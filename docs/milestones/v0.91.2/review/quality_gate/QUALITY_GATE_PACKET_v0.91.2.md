# Quality Gate Packet - v0.91.2

## Status

Tracked `WP-18` quality-gate packet for Sprint 4 release-tail work.

Current gate judgment: `NOT_READY`.

## Purpose

This packet consolidates the current `v0.91.2` quality posture into one bounded
review surface:

- what the milestone can already prove
- what contract and coverage-governance checks currently pass
- what still blocks final release approval

## Current Gate Dimensions

| Dimension | Status | Strongest evidence now | Why it matters | Boundary |
| --- | --- | --- | --- | --- |
| Demo/proof convergence | PASS | `DEMO_MATRIX_v0.91.2.md`, `FEATURE_PROOF_COVERAGE_v0.91.2.md`, `review/demo_proof_convergence/DEMO_PROOF_CONVERGENCE_PACKET_v0.91.2.md`, and `review/code_feature_demo_follow_ons/CODE_FEATURE_DEMO_FOLLOW_ONS_PACKET_v0.91.2.md` | Shows the milestone now has a legible proof map instead of scattered issue history. | Proof convergence is not itself release approval. |
| CI path-policy and coverage-lane contract | PASS | `bash adl/tools/test_ci_path_policy.sh`, `bash adl/tools/test_ci_runtime_contracts.sh`, `bash adl/tools/test_run_authoritative_coverage_lane.sh`, and `bash adl/tools/test_check_coverage_impact.sh` | Shows the current CI split remains internally coherent after the runtime/coverage recovery work. | Contract checks are not the same thing as a fresh full release coverage run. |
| Retained closeout truth | PARTIAL | Sprint 1 through Sprint 3 closeout truth is materially cleaner; known retained `#3121` residue is explicitly deferred out of this issue | Keeps the packet honest about one remaining deferred retained-card gap. | This is a truth-hygiene gate, not feature proof. |
| Full authoritative coverage evidence | PASS | `bash adl/tools/run_authoritative_coverage_lane.sh` completed during WP-18 with `2066` tests passed, `2` skipped, and `coverage-summary.json` written | Proves the heavyweight authoritative coverage lane still runs cleanly at the current milestone state. | This is strong release-tail evidence, but it still does not by itself approve the milestone for release. |
| Release-tail review, remediation, and ceremony | NOT_READY | `WP-19` docs review is closed; the first `WP-20` packet is superseded for handoff truth by `WP-20B`; accepted `WP-20B` remediation issues have closed; `WP-21` external review starts from the refreshed handoff | Preserves the real remaining path to closeout. | WP-18, the thin WP-20 packet, and the WP-20B remediation pass cannot close the milestone or justify release ceremony by themselves. |

## Command Surfaces

Focused contract/gate checks:

```bash
bash adl/tools/test_demo_v0912_quality_gate.sh
bash adl/tools/test_ci_path_policy.sh
bash adl/tools/test_ci_runtime_contracts.sh
bash adl/tools/test_run_authoritative_coverage_lane.sh
bash adl/tools/test_check_coverage_impact.sh
```

Reviewer-facing gate aggregation:

```bash
bash adl/tools/demo_v0912_quality_gate.sh
```

Optional heavier local gate:

```bash
ADL_V0912_QUALITY_GATE_RUN_HEAVY=1 bash adl/tools/demo_v0912_quality_gate.sh
```

## Why The Gate Is Still Not Ready

The milestone still lacks:

- fixed and rechecked accepted `WP-20B` findings
- clean external-review handoff truth
- final external review records
- remediation truth
- release evidence completion
- ceremony and end-of-milestone closeout
- deferred retained-card cleanup for `#3121`, which remains intentionally out
  of scope for this issue

## Explicit Non-Claims

- This packet does not say `adl-ci` or `adl-coverage` on a docs-only or
  contract-only path are enough for release.
- This packet does not claim `v0.91.2` is ready to ship.
- This packet does not claim the first `WP-20` internal review packet is
  sufficient for external handoff after the corrective `WP-20B` review.
- This packet does not collapse remaining Sprint 4 work into “administrivia.”

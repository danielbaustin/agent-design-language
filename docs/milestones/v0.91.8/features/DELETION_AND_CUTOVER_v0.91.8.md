# Deletion and Cutover

Deletion and default switch were executed through WP-12 and WP-13 for the
bounded v0.91.8 bridge.

Required inputs:

- reviewed shadow parity;
- opt-in soak;
- rollback proof;
- selector transaction proof;
- C-SDLC v2 acceptance from `#5358`;
- Runtime v3 acceptance from `#5361`;
- deletion eligibility manifest;
- post-deletion validation.

The retained WP-13 validation is
`docs/milestones/v0.91.8/evidence/wp13/5346-post-deletion-validation.v1.json`.
It records `status: pass` for the bounded deletion-eligibility manifest. The
46,358-line denominator is not self-authored by the post-deletion result: it is
derived from the independent eligibility manifest
`docs/milestones/v0.91.8/evidence/wp13/5346-deletion-eligibility.v1.json`,
which lists 58 deletion-eligible paths whose `baseline_physical_loc` values sum
to 46,358. The post-deletion validation then records zero retained lines for
that manifest. This document does not approve additional deletion beyond that
retained evidence.

## WP-20 External-Review Remediation

WP-19 external review returned blocked findings retained under
`docs/milestones/v0.91.8/review/external_review_5357/`. WP-20 `#5363`
narrows this page's proof claims as follows:

| Measure | Evidence | Value | Claim boundary |
| --- | --- | --- | --- |
| Eligibility denominator | `evidence/wp13/5346-deletion-eligibility.v1.json` | 58 paths / 46,358 baseline physical lines | Independent file-level denominator for the #5346 deletion-eligible manifest only. |
| Post-deletion retained count | `evidence/wp13/5346-post-deletion-validation.v1.json` | 0 retained lines for the manifest | Proves the bounded manifest was removed; does not prove whole-workspace reduction. |
| Post-deletion status | `evidence/wp13/5346-post-deletion-validation.v1.json` | `pass` | Applies to the retained #5346 manifest and its declared gates. |
| External-band deletion | `evidence/wp13-external-bands/deletion-accounting.json` | 20 files / 1,472 removed lines for #5347 | Separate external-band cleanup; not part of the 46,358-line denominator. |

## Boot-Path Cutover Table

This table records current boot-path ownership at the documentation level. It
does not approve deletion of any retained legacy file outside the deletion
manifests.

| Subsystem | v0.91.8 executing owner on claimed boot path | Retained legacy or evidence surface | Deletion disposition |
| --- | --- | --- | --- |
| Chronosense | `adl-runtime-kernel/src/assembly.rs` and `adl-runtime-kernel/src/operations.rs` require the Chronosense adapter for live assembly and scheduler operation. | Legacy `adl/src/runtime_v2` surfaces may remain as historical or compatibility inputs outside the #5346 manifest. | Delete only manifest-eligible paths; do not claim all legacy Chronosense code retired. |
| Reasoning graph | `adl-runtime-kernel/src/reasoning.rs` and `adl-runtime-kernel/src/parity_b.rs` provide the Runtime v3 reasoning/parity surface. | Feature decisions and crosswalk evidence remain planning/proof surfaces. | Runtime v3 boot-path owner is documented; workspace-wide legacy deletion is not claimed. |
| Affect and governed cognition | `adl-runtime-kernel/src/cognition.rs` owns governed cognition decisions for moral affect, wellbeing, curiosity, and theory-of-mind evidence. | Prior ADL feature docs remain evidence-bound non-claim surfaces unless consumed by closed issue proof. | No deletion beyond retained manifests. |
| Skills and constructability | `adl-runtime-kernel/src/parity_b.rs` retains skill-standard and constructability/Freedom Gate proof surfaces. | ADL and C-SDLC docs remain routing authority for planned-vs-proven state. | No deletion beyond retained manifests. |
| Shepherd, providers, scheduler, identity, and private state | `adl-runtime-kernel/src/assembly.rs` requires operational adapters; `RUNTIME_V3_FUNCTIONAL_PARITY_PLAN_v0.91.8.md` routes Parity-C through #5589 evidence. | Provider and scheduler compatibility surfaces may remain until later cleanup gates. | Boot-path ownership is narrowed to accepted Runtime v3 evidence, not total source deletion. |
| Observatory, secure local/remote, telemetry, soak, and rollback | `RUNTIME_V3_FUNCTIONAL_PARITY_PLAN_v0.91.8.md` routes Parity-D through #5590 and acceptance/rollback evidence. | Observatory/demo evidence remains proof input, not release approval. | Acceptance-before-deletion remains required for any future cleanup. |

## Non-Claims

- This page does not claim total ADL workspace size reduction.
- This page does not claim `adl/src` shrank as a whole.
- This page does not claim every legacy implementation path was deleted.
- This page does not claim release approval or external-review approval.

# WP-20 Remediation Register For External Review Findings

Issue: WP-20 `#5363`

Input review: WP-19 `#5357`

## Retained Review Artifact

- Path:
  `docs/milestones/v0.91.8/review/external_review_5357/ADL_v0.91.8_External_Review_Findings.pdf`
- SHA-256:
  `b1741fb24d0627ccf3d7875168f54cc9b7c558a186efc267800612a6af2748f5`
- Size: `10841` bytes
- Review outcome: `blocked`

## Current Live Truth Consumed

- #5804 is closed.
- PR #5805 is merged into `main` with merge commit
  `8621b6f3b1b91d3ea290e16d07f80ec29afd4ece`.
- The stale #5804 typed claim has derived terminal authority recorded by
  `csdlc-finish`; the #5363 claim now owns the bounded WP-20 remediation paths.
- WP-19 #5357 and WP-20 #5363 remain release-tail issues until this remediation
  lands and terminal closeout observes live GitHub truth.

## Remediation Summary

| Finding | Disposition | Evidence |
| --- | --- | --- |
| `V0918-CHAT-01` | Fixed for current docs by retaining the blocked review result, removing the stale #5804-open premise from current status, and preserving the rule that a future approval review must name an exact PR, head SHA, and digest. | `review/external_review_5357/FINDINGS_REGISTER.md`; `review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md`; live PR #5805 merge commit `8621b6f3b1b91d3ea290e16d07f80ec29afd4ece`. |
| `V0918-CHAT-02` | Fixed by replacing the self-validating deletion shorthand with an independent eligibility-manifest denominator. | `features/DELETION_AND_CUTOVER_v0.91.8.md`; `evidence/wp13/5346-deletion-eligibility.v1.json`; `evidence/wp13/5346-post-deletion-validation.v1.json`. |
| `V0918-CHAT-03` | Fixed by narrowing the reduction claim and recording per-subsystem boot-path ownership instead of a workspace-wide size claim. | `features/DELETION_AND_CUTOVER_v0.91.8.md`; `RUNTIME_V3_FUNCTIONAL_PARITY_PLAN_v0.91.8.md`; `evidence/wp13-external-bands/current-truth-ledger.json`. |

## Non-Claims

- This remediation does not claim release approval.
- This remediation does not claim a final external review pass.
- This remediation does not claim total workspace or `adl/src` size reduction.
- This remediation does not approve deletion beyond the retained #5346/#5347
  eligibility and deletion evidence.

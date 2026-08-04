# v0.91.8 WP-19 External Review Findings Register

Source artifact:
`docs/milestones/v0.91.8/review/external_review_5357/ADL_v0.91.8_External_Review_Findings.pdf`

Source SHA-256:
`b1741fb24d0627ccf3d7875168f54cc9b7c558a186efc267800612a6af2748f5`

## Actionable Findings

| ID | Severity | Finding | WP-20 disposition |
| --- | --- | --- | --- |
| `V0918-CHAT-01` | P1 | The external-review send gate was unsatisfied because the reviewed packet had no PR, head branch, exact SHA, or digest, and it still listed #5804 as open. | Accepted. The result is retained as blocked findings, not release approval. Live #5804 / PR #5805 is now closed and merged at `8621b6f3b1b91d3ea290e16d07f80ec29afd4ece`; final approval review remains required only after a refreshed exact revision and digest are named. |
| `V0918-CHAT-02` | P2 | The deletion validation metric was self-validating because the deleted-line count and pinned denominator were both 46,358. | Accepted. The corrected denominator is derived independently from `docs/milestones/v0.91.8/evidence/wp13/5346-deletion-eligibility.v1.json`: 58 manifest paths totaling 46,358 baseline physical lines. The post-deletion validation then proves zero retained lines for that manifest only. No workspace-wide reduction claim is made. |
| `V0918-CHAT-03` | P2 | The code-reduction claim was not supported at workspace level; `adl/src` grew in the reviewed snapshot and large files remained. | Accepted. WP-20 narrows the claim to the deletion-eligibility manifest and adds a per-subsystem boot-path table in `features/DELETION_AND_CUTOVER_v0.91.8.md`. The docs no longer claim total workspace shrinkage or deletion of all legacy surfaces. |

## Residual Risks

| Risk | Disposition |
| --- | --- |
| Review independence: all review lanes are dispatched and consumed by the system under review. | Retained as informational release risk. Future approval review must preserve exact-revision digest/freeze evidence and operator-visible boundaries. |
| New-crate size regrowth, including large `csdlc-v2` modules. | Retained as v0.92 architectural hygiene input; no automatic issue is opened from this finding register. |
| Doc-to-code drift precedent from v0.91.7 reasoning graph claims. | Retained as a review-quality risk. Future feature docs must remain planned/proven separated. |

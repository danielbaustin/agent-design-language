# v0.91.2 WP-22 Remediation Queue Draft

## Supersession Status

This draft is historical `WP-20` review context. It has been superseded for
current remediation planning by the `WP-20B` full internal review packet under
`docs/milestones/v0.91.2/review/internal_review_full/`.

Controlling upstream packet:
- `docs/milestones/v0.91.2/review/internal_review_full/`

## Accepted Internal Review Items Routed

1. Retained `#3121` closeout residue must be fixed or explicitly deferred with final release-tail owner.
2. WP-21/WP-22 handoff must preserve release non-claims and prevent review/remediation from being mistaken for release approval.
3. Accepted `WP-20B` findings were routed through `#3175` through `#3179`,
   which are now closed before external review entry.

## Grouped Remediation Routing

- Benchmark validity and failure-gate truth:
  - `#3175` closed
- Hosted-provider security and artifact portability:
  - `#3176` closed
- Controlling packet, handoff truth, and release-tail evidence routing:
  - `#3177` closed
- CI pinning and validation reproducibility:
  - `#3178` closed
- Provider native-tool capability reporting:
  - `#3179` closed

## Candidate Follow-On Routing

- If the retained `#3121` residue is too broad for WP-22, open a bounded closeout-truth cleanup issue and reference it from release ceremony records.
- Use `docs/milestones/v0.91.2/review/internal_review_full/FINDING_TO_ISSUE_PLAN.md`
  as the current remediation grouping for benchmark validity, hosted-provider
  portability/security, evidence/handoff truth, tooling hygiene, and provider
  capability reporting.

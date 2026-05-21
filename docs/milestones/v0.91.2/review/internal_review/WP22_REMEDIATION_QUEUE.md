# v0.91.2 WP-22 Remediation Queue Draft

## Supersession Status

This draft is historical `WP-20` review context. It has been superseded for
current remediation planning by the `WP-20B` full internal review packet under
`docs/milestones/v0.91.2/review/internal_review_full/`.

Controlling upstream packet:
- `docs/milestones/v0.91.2/review/internal_review_full/`

## Accepted Internal Review Items To Route

1. Retained `#3121` closeout residue must be fixed or explicitly deferred with final release-tail owner.
2. WP-21/WP-22 handoff must preserve release non-claims and prevent review/remediation from being mistaken for release approval.
3. Accepted `WP-20B` findings must be fixed or explicitly dispositioned and
   rechecked before external review.

## Grouped Remediation Routing

- Benchmark validity and failure-gate truth:
  - `#3175`
- Hosted-provider security and artifact portability:
  - `#3176`
- Controlling packet, handoff truth, and release-tail evidence routing:
  - `#3177`
- CI pinning and validation reproducibility:
  - `#3178`
- Provider native-tool capability reporting:
  - `#3179`

## Candidate Follow-On Routing

- If the retained `#3121` residue is too broad for WP-22, open a bounded closeout-truth cleanup issue and reference it from release ceremony records.
- Use `docs/milestones/v0.91.2/review/internal_review_full/FINDING_TO_ISSUE_PLAN.md`
  as the current remediation grouping for benchmark validity, hosted-provider
  portability/security, evidence/handoff truth, tooling hygiene, and provider
  capability reporting.

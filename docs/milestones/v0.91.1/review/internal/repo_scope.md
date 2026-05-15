# Repo Scope

## Scope Reviewed

This packet is a docs-only internal review root for the tracked `v0.91.1`
release-tail review entry surfaces. Its purpose is to make the milestone review
package legible to packet-quality tooling without reconstructing the historical
local-only multi-role packet.

## Included Paths

- `docs/milestones/v0.91.1/review/internal/README.md`
- `docs/milestones/v0.91.1/RELEASE_READINESS_v0.91.1.md`
- `docs/milestones/v0.91.1/RELEASE_EVIDENCE_v0.91.1.md`
- `docs/milestones/v0.91.1/DEMO_MATRIX_v0.91.1.md`
- `docs/milestones/v0.91.1/FEATURE_PROOF_COVERAGE_v0.91.1.md`
- `docs/milestones/v0.91.1/review/WP22_REMEDIATION_QUEUE.md`
- `docs/milestones/v0.91.1/review/internal/final_report.md`
- `docs/milestones/v0.91.1/review/internal/specialist_reviews/docs.md`

## Excluded Paths

- untracked historical `.adl/docs/reviews/...` packet surfaces
- code, security, test, and architecture specialist artifacts that are not present in tracked `v0.91.1` review docs
- rerun artifacts for the original `WP-20` execution

## Non-Reviewed Surfaces

- code review lane
- security review lane
- test review lane
- architecture review lane
- dependency review lane

## Assumptions

- The tracked milestone docs are the truthful surviving source for a bounded internal-review replay.
- The evaluator is run with required role `docs` for this packet.
- No attempt is made here to recreate missing historical local-only review artifacts.

## Known Limits

- This packet does not prove a full multi-role internal review happened in tracked form.
- This packet is suitable for evaluator compatibility and docs-lane review quality only.
- Any broader release-quality claim would require additional specialist artifacts.

## Next Specialist Lanes

- Add code specialist review artifacts if a broader internal-review replay is desired.
- Add security and test specialist review artifacts before using this root for multi-role packet evaluation.
- Keep packet-quality and publication-decision claims separate.

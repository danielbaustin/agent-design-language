# Review Heuristics And Demos

## Metadata

- Feature Name: Review Heuristics And Demos
- Milestone Target: `v0.91.2`
- Status: implemented
- Planned WP Home: WP-07
- Source Docs: `.adl/docs/TBD/workflow_tooling/ADL_REVIEW_HEURISTICS.md`
- Proof Modes: skill docs, demos, review packet

## Purpose

Make ADL review heuristics repeatable enough to support CodeFriend, internal
review, external review, and review-quality evaluation without inventing source
evidence.

## Scope

In scope:

- Review heuristics docs promoted into review-skill references and bounded demo
  surfaces.
- Demo packets showing bounded review behavior.
- Acceptance checklist for review quality.

Out of scope:

- Automated review approval.
- Findings without source evidence.
- Replacing specialist human judgment.

## Acceptance Criteria

- Heuristics cite or require source evidence.
- Demo outputs are deterministic in fixture mode.
- Review quality can be evaluated against the checklist.

## Proving Surface

- `docs/milestones/v0.91.2/review/review_heuristics_demo/review_heuristics_promotion.md`
- `docs/milestones/v0.91.2/review/review_heuristics_demo/bounded_review_demo_packet.md`
- `docs/milestones/v0.91.2/review/review_heuristics_demo/review_quality_acceptance_checklist.md`
- `docs/milestones/v0.91.2/review/review_heuristics_demo/fixture_docs_review.md`
- `docs/milestones/v0.91.2/review/review_heuristics_demo/fixture_review_synthesis.md`
- `docs/milestones/v0.91.2/review/review_heuristics_demo/fixture_review_quality_evaluation.md`
- `docs/milestones/v0.91.2/review/review_heuristics_demo/fixture_review_quality_evaluation.json`

## Non-Claims

- `WP-07` does not claim that heuristics alone replace bounded specialist
  review.
- `WP-07` does not claim customer publication approval.
- `WP-07` does not claim that every review-quality rule is already fully
  automated.

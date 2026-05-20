# Code / Doc Linkage Report - v0.91.2

## Metadata

- Milestone: `v0.91.2`
- Issue: `#3011`
- Work package: `WP-12 Repo visibility follow-on`
- Slice: reviewer/planner navigation for CodeFriend productization and review
  heuristics
- Status: follow-on report

## Summary

This report connects one bounded `v0.91.2` slice from tracked milestone docs to
the landed review packet surfaces from `WP-06` and `WP-07`.

The selected slice is reviewer/planner navigation because `v0.91.2` already
has real productization and review outputs, but it still benefits from one
stable authority map that tells a reviewer where canonical truth starts and
which supporting packet surfaces prove the claim.

## Authority Model

| Surface | Status | How to read it |
| --- | --- | --- |
| `docs/milestones/v0.91.2/*.md` | canonical tracked milestone truth | Start public milestone review here. |
| `docs/milestones/v0.91.2/features/*.md` | feature-owner and proof-design docs | Use these to understand intended slice meaning and non-claims. |
| `docs/milestones/v0.91.2/review/**` | supporting proof and reviewer-navigation surfaces | Use these after the canonical docs identify the slice. |
| `.adl/docs/TBD/repo_visibility/*.md` | local source material | Background only; not public milestone truth. |

## Canonical Docs For This Slice

| Path | Role | Linkage status |
| --- | --- | --- |
| `docs/milestones/v0.91.2/README.md` | milestone entrypoint | present |
| `docs/milestones/v0.91.2/WBS_v0.91.2.md` | WP map and issue graph | present |
| `docs/milestones/v0.91.2/WP_EXECUTION_READINESS_v0.91.2.md` | execution gates and required outputs | present |
| `docs/milestones/v0.91.2/DEMO_MATRIX_v0.91.2.md` | proof-route summary | present |
| `docs/milestones/v0.91.2/FEATURE_PROOF_COVERAGE_v0.91.2.md` | feature-to-proof map | present |
| `docs/milestones/v0.91.2/features/CODEFRIEND_PRODUCTIZATION.md` | `WP-06` owner doc | present |
| `docs/milestones/v0.91.2/features/REVIEW_HEURISTICS_AND_DEMOS.md` | `WP-07` owner doc | present |
| `docs/milestones/v0.91.2/features/REPO_VISIBILITY_FOLLOW_ON.md` | `WP-12` owner doc | present |

## Linkage Map

### CodeFriend productization

| Surface type | Path | Why it matters |
| --- | --- | --- |
| Owner doc | `docs/milestones/v0.91.2/features/CODEFRIEND_PRODUCTIZATION.md` | Defines the productization slice and its public milestone meaning. |
| Review packet | `docs/milestones/v0.91.2/review/codefriend_productization/review_packet_workflow_package.md` | Shows the packet workflow reviewers should inspect first. |
| Review packet | `docs/milestones/v0.91.2/review/codefriend_productization/product_report_template.md` | Shows the downstream report surface expected from the packet. |
| Review packet | `docs/milestones/v0.91.2/review/codefriend_productization/evidence_requirements.md` | Records the evidence discipline for the slice. |
| Review packet | `docs/milestones/v0.91.2/review/codefriend_productization/skill_demo_alignment.md` | Links productization to concrete skill/demo expectations. |
| Milestone proof docs | `docs/milestones/v0.91.2/DEMO_MATRIX_v0.91.2.md`, `docs/milestones/v0.91.2/FEATURE_PROOF_COVERAGE_v0.91.2.md` | Record how the slice is proven at milestone level. |

### Review heuristics and demos

| Surface type | Path | Why it matters |
| --- | --- | --- |
| Owner doc | `docs/milestones/v0.91.2/features/REVIEW_HEURISTICS_AND_DEMOS.md` | Defines the heuristics/demo slice and its bounded claims. |
| Review packet | `docs/milestones/v0.91.2/review/review_heuristics_demo/review_heuristics_promotion.md` | Records the promotion story for the heuristics lane. |
| Review packet | `docs/milestones/v0.91.2/review/review_heuristics_demo/bounded_review_demo_packet.md` | Shows the bounded demo packet reviewers should inspect. |
| Review packet | `docs/milestones/v0.91.2/review/review_heuristics_demo/review_quality_acceptance_checklist.md` | Gives the acceptance surface for the demo output. |
| Fixture proof | `docs/milestones/v0.91.2/review/review_heuristics_demo/fixture_review_synthesis.md` | Shows the synthetic review output form. |
| Fixture proof | `docs/milestones/v0.91.2/review/review_heuristics_demo/fixture_docs_review.md` | Shows one specialist lane output. |
| Fixture proof | `docs/milestones/v0.91.2/review/review_heuristics_demo/fixture_review_quality_evaluation.md` | Shows the review-quality evaluation lane. |
| Milestone proof docs | `docs/milestones/v0.91.2/DEMO_MATRIX_v0.91.2.md`, `docs/milestones/v0.91.2/FEATURE_PROOF_COVERAGE_v0.91.2.md` | Record how the slice is proven at milestone level. |

## Reviewer Navigation Value

This packet is useful if a reviewer can answer these quickly:

- Which docs define the current `v0.91.2` reviewer/planner slice?
- Which review packet files should be inspected for `WP-06` first?
- Which review packet files should be inspected for `WP-07` first?
- Which milestone-level docs summarize those proof routes?
- Which local repo-visibility notes are only source material and should not be
  treated as public truth?

## Present / Missing / Deferred Surfaces

### Present

- Tracked milestone entrypoints and proof docs.
- Tracked feature-owner docs for `WP-06`, `WP-07`, and `WP-12`.
- Tracked review packet surfaces for CodeFriend productization.
- Tracked review packet and fixture outputs for review heuristics and demos.
- This follow-on manifest, linkage report, and reviewer-navigation packet.

### Expected Pending Work

- `WP-13` should extend the same navigation discipline into the publication
  packet band.
- `WP-17` should preserve these proof links when it does milestone-wide
  convergence and closeout.

### Deferred / Out Of Scope

- Full repo semantic indexing.
- Automatic ownership inference across the entire repository.
- Treating local `.adl/docs/TBD/` notes as canonical milestone truth.
- Expanding this packet into a code-level implementation ownership map for the
  whole repo.

## Validation Notes

Validation for this WP should confirm:

- the packet files exist;
- the referenced tracked docs and review packet surfaces exist;
- the owner-doc to supporting-proof relationships are explicit rather than
  inferred;
- the report does not claim full repo cognition or hidden inference.

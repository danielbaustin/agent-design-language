# Bounded Review Demo Packet

## Purpose

Show one deterministic, fixture-safe review path that demonstrates how ADL
review heuristics produce bounded review artifacts without inventing findings or
authority.

## Demo Mode

- Mode: fixture-safe packet demo
- Live mutation: none
- Customer publication: none
- Repo mutation by the demo itself: none
- Determinism claim: the packet shape, lane order, and acceptance expectations
  are fixed for the named skill surfaces

## Demo Inputs

- review suite contract:
  `adl/tools/skills/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md`
- quality gate:
  `adl/tools/skills/review-quality-evaluator/SKILL.md`
- synthesis contract:
  `adl/tools/skills/repo-review-synthesis/SKILL.md`
- productization handoff from `WP-06`:
  `docs/milestones/v0.91.2/review/codefriend_productization/`

## Fixture Outputs

- `fixture_docs_review.md`
- `fixture_review_synthesis.md`
- `fixture_review_quality_evaluation.md`
- `fixture_review_quality_evaluation.json`

## Bounded Walkthrough

1. Start from a bounded review packet root or bounded changed-path set.
2. In this deterministic fixture, run one docs specialist lane over the `WP-06`
   packet and record a truthful no-finding result instead of inventing defects.
3. Synthesize the specialist artifact without hiding disagreement, missing
   coverage, or residual risk.
4. Run the review-quality gate on the packet/report candidate.
5. Stop before publication, publication-decision claims, remediation, issue creation, or
   repository mutation.

## Deterministic Proving Surface

The proving surface for this demo is the concrete fixture artifact family above,
not a hypothetical output shape and not a live provider execution.

## What The Demo Proves

- review behavior is bounded to explicit lanes and stop boundaries
- findings must remain evidence-backed to survive the quality gate
- missing lanes or missing evidence become visible review-state, not silent
  success
- the workflow package from `WP-06` can host a heuristic-aware review lane

## What The Demo Does Not Prove

- that any single fixture review run is complete by default
- that the review suite is equivalent to human sign-off
- that the heuristics are fully automated
- that customer-facing publication has already been authorized

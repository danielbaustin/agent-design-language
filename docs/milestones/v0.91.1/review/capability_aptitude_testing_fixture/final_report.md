# Executive Summary

This WP-09 slice delivers the first executable capability/aptitude harness artifact for ADL.
It covers contract following, review aptitude, and planning aptitude in deterministic fixture mode and emits a report packet with explicit limitations.

## Task Family

Capability and aptitude fixture families for ADL workflow work.

## Subjects Tested

- fixture_subject.adl_evaluation_slice (`deterministic_fixture` / `wp09-fixture-v1`)

## Test Fixture Summary

- contract_following
- review_aptitude
- planning_aptitude

## Scorecard Table

| Family | Band | Confidence |
| --- | --- | --- |
| contract_following | Strong | fixture_high |
| review_aptitude | Strong | fixture_medium |
| planning_aptitude | Adequate | fixture_medium |

## Findings By Subject

The fixture subject preserves workflow constraints, review structure, and bounded planning shape, but still requires human oversight when workflow wrappers drift or closeout truth diverges.

## Strengths

- preserves issue-local workflow constraints
- supports findings-first review output
- keeps dependencies and non-goals explicit in planning

## Failure Modes

- wrapper hangs can consume sprint time before issue execution starts
- stale card truth reduces review and planning signal quality

## Repair Burden

Repair burden is bounded but real: manual cleanup is still needed when wrapper behavior or sprint-state truth drifts.

## Recommended Use

- internal sprint readiness checks
- pre-PR workflow and review trials

## Discouraged Use

- universal intelligence ranking
- public reputation scoreboard

## Caveats

- fixture mode only
- no live provider/model comparison yet
- WP-10 still owns intelligence-specific metric architecture

## Evidence Appendix

- subject_manifest.json
- test_manifest.json
- scorecard.json
- raw_outputs/*.json

## Publication Boundary

Internal only. This slice must not be presented as a public leaderboard or a universal model score.
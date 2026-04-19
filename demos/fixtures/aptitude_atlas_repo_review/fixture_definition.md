# Fixture Definition: Repo-Review Aptitude Prototype

## Fixture Metadata

- `fixture_id`: `repo-review-v0.90.1-baseline-01`
- `test_family`: `review_aptitude`
- `version`: `v0.90.1`
- `goal`: validate bounded review behavior on one constrained repository slice

## Fixture Summary

This fixture is intentionally small and bounded. It represents one hypothetical
ADL-facing review surface with mixed quality signals:

- one clear real defect
- one tempting false-positive pattern
- one stale docs/release mismatch
- one residual-risk surface for reviewer escalation

## Files and Expected Signal

Use all files under `demos/fixtures/aptitude_atlas_repo_review/` as the target
surface.

### 1) Real Finding (high-confidence)

File: `target_repo_validator.py`

Expect review finding:

- `HIGH`: command injection risk from string-built shell invocation in
  `run_validation`.
- Why real: this is directly exploitable and actionable.

### 2) False-Positive Trap

File: `target_repo_readme.md`

Potentially suspicious phrasing appears, but it is in quoted user education text:

- sentence about “run as root at your own risk”

Reviewers should avoid classifying this as a vulnerability unless it appears in
executable workflow context.

### 3) Docs / release-truth wrinkle

File: `target_repo_deployment.md`

The docs claim feature parity with release notes but intentionally omit a listed
step introduced in the same section. This creates a review-worthy documentation
trust issue, not a code bug.

### 4) Residual risk surface

File: `target_repo_validator.py` comments and deployment notes

Potential risk should be called out as a residual risk rather than false certainty:

- manual review step may miss key pair in runtime config if future subjects do not
  inspect the full fixture context.
- include this as a risk note with low confidence and remediation suggestion.

## Fixture Scoring Rubric Seeds

For each subject run, capture:

- `true_positive_detection` (0-5)
- `false_positive_restraint` (0-5)
- `severity_calibration` (0-5)
- `evidence_quality` (0-5)
- `remediation_usefulness` (0-5)
- `uncertainty_handling` (0-5)
- `repair_burden` (0-5, reverse scale where lower is better)

## Expected Artifact Outputs

For each completed subject run:

- one row in scorecard json (subject + dimension breakdown)
- one evaluator_notes section in `final_report_template.md`
- one run manifest entry with timestamped proof path list


# Reviewer Regression Fixture: Issue 661

This fixture captures a previously reviewed v0.8 card pair for deterministic reviewer regression checks.

## Inputs

- `input_661.md`
- `output_661.md`

## Expected Reviewer Artifact

- `expected_review_output_661.yaml`

## Determinism Expectations

For identical fixture inputs, reviewer output should preserve:

- top-level key ordering per `card_review_output.v1`
- deterministic domain and findings ordering
- correct `evidence_state` usage:
  - `contradicted`
  - `not_evidenced`
  - `not_applicable`
- repo-relative evidence pointers

## Notes

This fixture is for reviewer-surface regression validation only; it is not runtime execution evidence.

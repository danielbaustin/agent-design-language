# Coverage Gate Ergonomics Report

## Scope

This `WP-05` report records the bounded changed-source coverage ergonomics
follow-on for `v0.91.2`.

## Landed Surface

- `adl/tools/check_coverage_impact.sh`
- `adl/tools/test_check_coverage_impact.sh`

## Outcome

The changed-source preflight now emits actionable follow-up guidance instead of
stopping at a bare threshold failure.

For each risky or failing changed Rust source file, the gate now reports:

- the changed file path
- the candidate focused test filter
- the exact focused `cargo llvm-cov` summary command to run next
- the matching preflight rerun command for the current diff mode

The summary-driven failure path now also explains two common failure modes:

- `no coverage row`: the focused summary did not execute the changed file
- `below threshold`: tests need to be added or expanded before the summary is
  refreshed

## Validation

Focused proof for this issue:

- `bash adl/tools/test_check_coverage_impact.sh`
- `git diff --check`

The shell test suite now covers:

- risky changed-file guidance when summary evidence is missing
- low-coverage summary failure guidance
- missing coverage-row guidance
- existing pass behavior for docs-only, test-only, structural barrel, and
  non-executable-module cases

## Non-Claims

- This report does not waive the per-file threshold.
- This report does not replace the authoritative GitHub `adl-coverage` job.
- This report does not claim that every changed file can be solved with one
  filter; it only makes the likely next action explicit.

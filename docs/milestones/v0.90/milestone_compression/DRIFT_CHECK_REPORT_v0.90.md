# Drift Check Report - v0.90

## Metadata

- Milestone: v0.90
- Issue: #2030
- Work package: WP-11 Milestone compression pilot
- Checker: `python3 adl/tools/check_v090_milestone_state.py`
- Status: pilot passing

## Summary

The v0.90 milestone compression pilot now has a minimal canonical state model
and a read-only drift checker. The pilot focuses on high-signal milestone drift:
issue-wave count, issue references in core docs, proof-packet status, and
truthful demo classification.

This is intentionally not a release bot. It makes drift visible; it does not
approve merges, close issues, or rewrite milestone truth.

## Pilot Results

Expected pass classifications:

- The opened v0.90 issue wave records 20 work packages.
- Core milestone docs expose issue references for WP-01 through WP-20.
- The repo visibility proof packet exists and the state model marks it landed.
- The milestone compression proof packet exists and the state model marks it
  landed.
- D1 through D5 match the landed issue evidence now that WP-02 through WP-09
  have landed.

Known mismatch policy:

- If a demo row claims landed work without the expected issue evidence, the
  checker should report a known mismatch rather than silently treat the row as
  release truth.
- If a proof-packet directory exists but the state model still says planned,
  the checker should report a known mismatch.

Fail policy:

- Missing issue-wave work packages are failures.
- Missing issue references in the core milestone docs are failures.
- A state model that marks a proof packet landed while its directory is missing
  is a failure.

## Validation Command

```bash
python3 adl/tools/check_v090_milestone_state.py
```

## Boundaries

- No network calls.
- No GitHub issue mutation.
- No PR or branch mutation.
- No release approval.
- No root-checkout workflow.

## Current Release-Tail Use

WP-13 reran this checker after the implementation WPs and sidecar WPs settled.
WP-18 treats the checker as one pre-third-party readiness input, not as a
replacement for human release ceremony. Final release still requires
third-party review, accepted findings remediation if needed, next-milestone
planning, and ceremony.

## Finish Validation Refinement

The #2053 compression experiment added one important follow-on lesson: the
compression checker helped keep scope and milestone drift bounded, but closeout
still paid the heavy local Rust validation cost during `pr finish`.

Issue #2080 adds `FINISH_VALIDATION_PROFILES_v0.90.md` to separate execution
compression from validation compression. Low-risk docs/static-tooling work may
use focused local validation only when the SOR records that full local
validation was not run and CI remains required before merge.

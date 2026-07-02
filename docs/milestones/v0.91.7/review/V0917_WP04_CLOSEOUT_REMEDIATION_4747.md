# v0.91.7 WP-04 Closeout Review Remediation

Issue: `#4747`

Status: complete, pending PR publication.

## Summary

This packet records the bounded remediation for the WP-04 closeout review
findings after the umbrella issue `#4631` and all known WP-04 child issues were
already closed. The remediation did not reopen WP-04 implementation scope; it
corrected stale lifecycle truth, repaired issue-list metadata drift, and made
prediction readiness semantics explicit.

## Findings Addressed

| Finding | Disposition |
| --- | --- |
| WP-04 umbrella appeared open with bootstrap local SOR/SRP truth | Current ADL issue evidence shows `#4631` is closed. The root-local ignored `#4631` SOR was normalized from pending publication wording to merged/complete closeout truth. |
| WP-04 nested-goal accounting was overclaimed | `V0917_WP04_CLOSEOUT_4631.md` now records that automatic nested per-issue goal capture under an active umbrella goal remains residual. Unknown actual elapsed/token metrics are retained as unknown, not inferred. |
| `#4666` child SOR had stale closeout fields | The root-local ignored `#4666` SOR was normalized from `not_started` / `not_open` / `not_started` to complete, merged, child-closed truth. |
| Prediction packet used an overloaded `prediction_ready` flag | `write_issue_goal_metrics_report.py` now emits `minimal_prediction_ready`, `full_prediction_ready`, and `prediction_readiness`. The legacy `prediction_ready` alias remains as minimal readiness for compatibility. |
| `#4617` lacked the `wp:WP-04` label | The issue metadata was repaired through `adl-issue edit` using the full existing label set plus `wp:WP-04`. |

## Files Updated

- `adl/tools/skills/sprint-conductor/scripts/write_issue_goal_metrics_report.py`
- `adl/tools/test_sprint_conductor_helpers.sh`
- `docs/milestones/v0.91.7/review/V0917_WP04_CODEX_SESSION_TELEMETRY_SAMPLE_4617.md`
- `docs/milestones/v0.91.7/review/V0917_WP04_CODEX_SESSION_TELEMETRY_SAMPLE_4617.prediction.json`
- `docs/milestones/v0.91.7/review/V0917_WP04_CLOSEOUT_4631.md`

The execution prediction sample was regenerated from the updated packet and
remained content-identical because the execution predictor consumes feature
availability and values, not the new readiness labels.

Local ignored lifecycle records repaired:

- `.adl/v0.91.7/tasks/issue-4631__v0-91-7-wp-04-goal-state-nested-goals-and-execution-metrics/sor.md`
- `.adl/v0.91.7/tasks/issue-4666__v0-91-7-wp-04-goals-implement-nested-issue-and-sprint-goal-accounting/sor.md`

## Residual Truth

WP-04 delivered usable metric capture and prediction scaffolding, but full
automatic per-issue goal capture while an umbrella goal remains active is not
yet proven. Future sprint execution should not claim complete per-issue
accounting automation until nested issue goals and umbrella goals can coexist
without refusing child issue goal creation.

## Validation

Validation completed:

- `bash adl/tools/test_sprint_conductor_helpers.sh`: focused
  sprint-conductor helper regression proof for the readiness flags passed.
- `python3 -m json.tool docs/milestones/v0.91.7/review/V0917_WP04_CODEX_SESSION_TELEMETRY_SAMPLE_4617.prediction.json`
  and `python3 -m json.tool docs/milestones/v0.91.7/review/V0917_WP04_EXECUTION_METRICS_PREDICTION_SAMPLE_4743.json`:
  regenerated retained sample packet syntax passed.
- `adl tooling validate-structured-prompt --type sor --phase final` for the
  repaired root-local `#4631` and `#4666` SOR records passed.
- `adl tooling validate-structured-prompt --type sor --phase final` for the
  `#4747` SOR passed.
- `git diff --check`: passed.

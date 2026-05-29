# v0.91.4 Process Drift Regression Report

## Status

Tracked WP-12 regression proof packet.

## Purpose

Record the bounded regression fixtures that keep the C-SDLC lane from drifting
back toward stale cards, unsafe sprint advancement, or machine-local truth.

## Focused Command

```bash
bash adl/tools/test_process_drift_regressions.sh
```

## Fixture Coverage

| Drift mode | Proof surface | Expected result |
| --- | --- | --- |
| legacy `SRP` policy wording in new bundles | `cargo test -p adl structured_prompt_srp_rejects_legacy_review_policy_artifact_type` | validator fails closed |
| stale `SRP` review truth | `cargo test -p adl structured_prompt_srp_completed_card_status_requires_final_review_truth` | validator fails closed |
| stale `SOR` closeout truth | `cargo test -p adl structured_prompt_sor_completed_card_status_requires_full_closeout_truth` | validator fails closed |
| machine-local absolute host-path leakage in durable prompt cards | `cargo test -p adl structured_prompt_srp_validator_rejects_absolute_host_path_leakage` | validator fails closed |
| skipped sprint child closeout / stale sprint state | `bash adl/tools/test_sprint_conductor_helpers.sh` | sprint helper fixtures fail closed and matched fixtures pass |

## Notes

- The regression command is intentionally compositional rather than broad: it
  reuses the focused sprint-helper and structured-prompt fixtures already owned
  by the repo.
- WP-12 does not claim to prove every future process failure is impossible.
  It adds a bounded, reviewer-visible safety net for the drift modes that
  repeatedly surfaced during v0.91.4 dogfooding.

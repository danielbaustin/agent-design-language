# `#3925` Publication Friction And Remediation Capture

## Summary

This note records the concrete problems encountered while implementing and
publishing `#3925` so they remain available for follow-on tooling remediation.
It separates issues that were fixed in-branch from issues that still deserve a
tracked follow-up.

## Problems Encountered

| Problem | Impact | Status | Remediation |
| --- | --- | --- | --- |
| `pr finish` did not classify `adl/tools/check_repo_quality_staleness.py`, `adl/tools/test_check_repo_quality_staleness.sh`, or `adl/tools/README.md` into an allowed validation lane. | Normal publication failed closed even though the issue-local proof was already green. | Fixed in `#3925`. | Added focused finish-lane routing in `adl/src/cli/pr_cmd/finish_support.rs` plus regression coverage in `adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs`. |
| `pr finish` failed when `--output-card` pointed at the ignored cards-root path `.adl/cards/3925/output_3925.md`. | Finish completed validation, then failed while trying to re-stage an ignored output card. | Not fixed in `#3925`; worked around. | Use the task-bundle `SOR` path `.adl/v0.91.6/tasks/issue-3925__v0-91-6-quality-add-repo-quality-and-documentation-staleness-checks/sor.md` for publication until finish learns to skip restaging ignored cards-root outputs. |
| Including worktree task-bundle `SRP`/`SOR` paths in `--paths` caused `git add` pathspec failures. | First finish attempt failed before validation because local bundle files are not tracked repo publication paths. | Not fixed in `#3925`; operator/workflow knowledge workaround only. | Improve `pr finish` diagnostics so it explicitly warns that task-bundle cards are local truth surfaces and must not be passed in `--paths`. |
| The branch became one commit behind `origin/main` before publication. | Finish stopped before validation and required a rebase. | Healthy guard, not a defect. | Keep the stale-base guard; improve operator/runbook guidance or watcher automation so rebases happen before the final finish pass. |
| The broader owner-lane attempt `bash adl/tools/run_owner_validation_lane.sh csdlc` exposed an unrelated failure in `adl/tools/test_control_plane_observability.sh`. | Broader owner-lane proof was not trustworthy for this issue and had to be excluded from issue proof. | Unresolved outside `#3925`. | Route separately as owner-lane/observability remediation; do not treat it as a blocker for the bounded repo-quality checker issue. |
| Local PR URL opening failed after publication with `kLSExecutableIncorrectFormat`. | PR publication succeeded, but local auto-open did not. | Environment-specific and non-blocking. | Low-priority workstation/tool-launcher follow-up only if repeated. |

## Most Important Follow-Ups

1. Make `pr finish` tolerant of ignored cards-root output cards, or standardize
   the command to always publish against the task-bundle `SOR`.
2. Improve `pr finish` error text for accidental task-bundle paths in
   `--paths`.
3. Route the unrelated `adl/tools/test_control_plane_observability.sh` owner-lane
   failure into its own remediation issue so broader C-SDLC owner-lane trust
   does not depend on operator memory.

## Non-Claims

- This note does not claim every tooling rough edge in the repo was exercised by
  `#3925`.
- This note does not treat the stale-base guard as a bug; it is recorded here
  because it affected cycle time during publication.

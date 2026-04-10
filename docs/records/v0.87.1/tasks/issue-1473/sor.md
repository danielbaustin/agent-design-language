# v0-87-1-tools-clean-up-local-adl-runs-directory-layout-and-retention

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Task ID: issue-1473
Run ID: issue-1473
Version: v0.87.1
Title: [v0.87.1][tools] Clean up local .adl/runs directory layout and retention
Branch: codex/1473-v0-87-1-tools-clean-up-local-adl-runs-directory-layout-and-retention
Status: DONE

Execution:
- Actor: Codex
- Model: gpt-5.4
- Provider: chatgpt
- Start Time: 2026-04-10T02:14:00Z
- End Time: 2026-04-10T02:24:59Z

## Summary

Implemented the remaining #1473 cleanup policy by extending the run archive helper with an explicit archive-first active-run pruning mode. The default helper still copies and inventories only; the new `--apply --prune-active-runs` mode preserves top-level `.adl/runs/<run-id>` source directories under `.adl/trace-archive/source-roots/<timestamp>/repo-adl-runs-flat/` after archiving, then leaves `.adl/runs/README.md` as the active/new-run surface.

Applied the new mode to the primary local runtime data after validation. The primary `.adl/runs` surface now contains only `README.md` and `_shared/`; `_shared/obsmem_store.v1.json` is shared runtime state rather than a run directory and was intentionally retained.

## Artifacts produced

- `adl/tools/archive_run_artifacts.sh`: added `--prune-active-runs`, guarded so pruning requires `--apply`.
- `adl/tools/test_archive_run_artifacts.sh`: added coverage for archive-first pruning, preserved source roots, README recreation, and refusal to prune without `--apply`.
- `adl/tools/README.md`: documented the retention/cleanup command.
- `.adl/trace-archive/MANIFEST.tsv`: regenerated local untracked archive manifest for the primary checkout.
- `.adl/trace-archive/README.md`: regenerated local untracked archive summary showing active-run pruning.
- `.adl/trace-archive/source-roots/<timestamp>/repo-adl-runs-flat/`: preserved moved source directories from the primary `.adl/runs` surface.

## Actions taken

- Bound #1473 with the repo-native `pr run` flow into an issue worktree.
- Added a preservation-only cleanup mode to the archive helper instead of deleting source data.
- Verified the helper keeps dry-run behavior unchanged and only moves active runs when `--apply --prune-active-runs` is explicit.
- Ran the helper against the primary checkout local `.adl` data to archive first and clear the active run surface.
- Confirmed `.adl/runs` was reduced from 60 top-level run directories to no top-level run directories, with only `README.md` and `_shared/` remaining.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked repository paths from this issue are present on main via merged PR #1539.
- Worktree-only paths remaining: none for required tracked artifacts; local ignored `.adl/trace-archive` and `.adl/runs` cleanup artifacts remain untracked operator data by design.
- Integration state: merged
- Verification scope: worktree
- Integration method used: issue branch/worktree changes were published and merged via PR #1539. Local untracked runtime cleanup data was intentionally not committed.
- Verification performed:
  - `git status --short --branch` in the issue worktree confirmed only the three intended tracked files changed before finish.
  - `find .adl/runs -mindepth 1 -maxdepth 1 -type d | wc -l` in the primary checkout confirmed the active run directory count before cleanup.
  - `find .adl/runs -mindepth 1 -maxdepth 1 -print | sort` in the primary checkout confirmed only `README.md` and `_shared/` remain afterward.
- Result: PASS

## Validation

- Validation commands and their purpose:
  - `bash -n adl/tools/archive_run_artifacts.sh adl/tools/test_archive_run_artifacts.sh`: verified edited shell scripts parse.
  - `bash adl/tools/test_archive_run_artifacts.sh`: verified dry-run inventory, apply-mode copying, manifest-based milestone inference, active-run pruning, preserved source roots, README recreation, and the `--apply` guard.
  - `adl/tools/archive_run_artifacts.sh --repo-root <primary-checkout> --archive-root <primary-checkout>/.adl/trace-archive --include-worktrees --apply --prune-active-runs`: archived current local run artifacts and moved active `.adl/runs` source directories into the local trace archive.
  - `find .adl/trace-archive/milestones -mindepth 3 -maxdepth 3 -type d | wc -l`: confirmed the local archive now contains 326 milestone-organized run directories.
  - `du -sh .adl/runs .adl/trace-archive`: confirmed `.adl/runs` is reduced to a small active surface and `.adl/trace-archive` contains the preserved data.
- Results: PASS

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "bash -n adl/tools/archive_run_artifacts.sh adl/tools/test_archive_run_artifacts.sh"
      - "bash adl/tools/test_archive_run_artifacts.sh"
      - "adl/tools/archive_run_artifacts.sh --repo-root <primary-checkout> --archive-root <primary-checkout>/.adl/trace-archive --include-worktrees --apply --prune-active-runs"
  determinism:
    status: PASS
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: PASS
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: PASS
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence

- Determinism tests executed: `bash adl/tools/test_archive_run_artifacts.sh`
- Fixtures or scripts used: the archive regression fixture creates deterministic root `.adl/runs`, report runs, and artifact runtime run roots.
- Replay verification (same inputs -> same artifacts/order): the helper sorts discovered run directories and records duplicate `milestone + run-id` entries deterministically.
- Ordering guarantees (sorting / tie-break rules used): source roots retain priority order, and run directories are sorted before manifest generation and pruning.
- Artifact stability notes: pruning uses a timestamped source-root preservation directory by design, but archive selection and manifest status remain deterministic for the same input roots.

## Security / Privacy Checks

- Secret leakage scan performed: no secrets are introduced; the helper records run IDs, relative source roots, milestone labels, and artifact presence only.
- Prompt / tool argument redaction verified: the SOR records `<primary-checkout>` placeholders rather than absolute host paths.
- Absolute path leakage check: output record uses repository-relative paths and placeholders; local archive README generated by the helper records relative paths.
- Sandbox / policy invariants preserved: helper refuses `--prune-active-runs` without `--apply` and refuses archive roots inside `.adl/runs`.

## Replay Artifacts

- Trace bundle path(s): not applicable; this issue changes run-retention tooling rather than producing a replay bundle.
- Run artifact root: `.adl/trace-archive/milestones/`
- Replay command used for verification: `bash adl/tools/test_archive_run_artifacts.sh`
- Replay result: PASS

## Artifact Verification

- Primary proof surface: `.adl/trace-archive/README.md` and `.adl/trace-archive/MANIFEST.tsv`
- Required artifacts present: yes; tracked helper, regression test, and tool README updates are present in the issue worktree, while local archive artifacts are present under the primary checkout `.adl/trace-archive`.
- Artifact schema/version checks: no tracked schema change.
- Hash/byte-stability checks: not applicable; the generated local archive summary includes run counts and a timestamped preservation root.
- Missing/optional artifacts and rationale: no standalone demo was required because proof comes from the focused tool regression and primary local cleanup.

## Decisions / Deviations

- The cleanup mode moves active `.adl/runs` source directories only after archive apply succeeds; it does not delete run data.
- `_shared/` remains in `.adl/runs` because it contains shared runtime state, not a top-level run artifact directory.
- The primary checkout had unrelated local branch/state outside #1473; tracked changes were kept in the issue worktree.

## Follow-ups / Deferred work

- No follow-up is required for #1473. If automatic age/count retention is desired later, it should be a separate issue because this change deliberately implements an explicit operator command rather than background deletion.

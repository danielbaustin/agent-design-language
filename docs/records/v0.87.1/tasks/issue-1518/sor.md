# v0-87-1-tools-inventory-and-consolidate-scattered-run-artifacts

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1518
Run ID: issue-1518
Version: v0.87.1
Title: [v0.87.1][tools] Inventory and consolidate scattered run artifacts
Branch: codex/1518-v0-87-1-tools-inventory-and-consolidate-scattered-run-artifacts
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5.4
- Provider: ChatGPT
- Start Time: 2026-04-10T00:47:00Z
- End Time: 2026-04-10T01:09:42Z

## Summary

Added a safe run-artifact archive helper that inventories local ADL run roots, classifies runs by inferred milestone, records duplicate sources, and copies only the first discovered run for each `milestone + run-id` pair into a canonical local archive. Ran it against the primary checkout with worktrees included; the archive now preserves the unique discovered local runs under `.adl/trace-archive/milestones/` without deleting source data.

## Artifacts produced

- `adl/tools/archive_run_artifacts.sh`
- `adl/tools/test_archive_run_artifacts.sh`
- `adl/tools/README.md`
- `.adl/trace-archive/README.md` in the primary checkout local runtime area
- `.adl/trace-archive/MANIFEST.tsv` in the primary checkout local runtime area
- `.adl/trace-archive/milestones/<milestone>/runs/<run-id>/` copied local run artifacts in the primary checkout local runtime area

## Actions taken

- Implemented `archive_run_artifacts.sh` with dry-run/apply modes, `--repo-root`, `--archive-root`, `--include-worktrees`, and idempotent duplicate handling.
- Added focused regression coverage in `test_archive_run_artifacts.sh`.
- Documented the new helper in `adl/tools/README.md`.
- Ran a dry-run against the primary checkout local data with worktrees included.
- Ran apply against the primary checkout local data with worktrees included.
- Preserved 282 unique run artifacts under `.adl/trace-archive/milestones/` and recorded 6,503 duplicate source entries as `duplicate-skipped` in the manifest.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked repository paths from this issue are present on main via merged PR #1524.
- Worktree-only paths remaining: none for required tracked artifacts; the local runtime archive under `.adl/trace-archive/` remains intentionally untracked operator data.
- Integration state: merged
- Verification scope: worktree
- Integration method used: issue branch/worktree changes were published and merged via PR #1524. Local archive data was intentionally left untracked.
- Verification performed:
  - `git status --short` confirmed the tracked worktree change set is limited to the new tool, test, and README update.
  - `find .adl/trace-archive/milestones -mindepth 3 -maxdepth 3 -type d | wc -l` from the primary checkout confirmed the local archive contains 281 run directories after the first apply pass.
  - `sed -n '1,100p' .adl/trace-archive/README.md` from the primary checkout confirmed the local archive summary was generated.
- Result: PASS
- Result notes: tracked changes are ready for PR; local run data is preserved in the primary checkout archive without deleting original sources.

## Validation

- Validation commands and their purpose:
  - `bash adl/tools/test_archive_run_artifacts.sh`: verifies dry-run manifest generation, milestone inference, apply-mode copying, and source metadata creation.
  - `bash -n adl/tools/archive_run_artifacts.sh adl/tools/test_archive_run_artifacts.sh`: verifies shell syntax for the new tool and test.
  - `bash adl/tools/archive_run_artifacts.sh --repo-root <primary-checkout> --archive-root <primary-checkout>/.adl/trace-archive --include-worktrees`: verifies full dry-run discovery across the primary checkout and worktrees.
  - `bash adl/tools/archive_run_artifacts.sh --repo-root <primary-checkout> --archive-root <primary-checkout>/.adl/trace-archive --include-worktrees --apply`: copies the unique discovered run artifacts into the canonical local archive.
- Results: all commands completed successfully.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "bash adl/tools/test_archive_run_artifacts.sh"
      - "bash -n adl/tools/archive_run_artifacts.sh adl/tools/test_archive_run_artifacts.sh"
      - "archive dry-run against the primary checkout with worktrees included"
      - "archive apply against the primary checkout with worktrees included"
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
- Fixtures or scripts used: the test creates a temporary fixture repo with root `.adl/runs`, `.adl/reports/*/runs`, and `artifacts/v0871/.../runtime/runs` inputs.
- Replay verification (same inputs -> same artifacts/order): the test reruns dry-run and apply paths against deterministic fixture names and checks expected archive paths.
- Ordering guarantees (sorting / tie-break rules used): run directories are sorted inside each root, and root discovery preserves priority order so repo-local `.adl/runs` wins over duplicate worktree copies.
- Artifact stability notes: duplicate source entries remain in `MANIFEST.tsv`, but only the first discovered `milestone + run-id` pair is copied.

## Security / Privacy Checks

- Secret leakage scan performed: no secret-bearing content was added; the script copies existing local run artifacts only when explicitly invoked.
- Prompt / tool argument redaction verified: the manifest records source roots, run ids, milestones, archive paths, status, and artifact-presence flags; it does not inspect or print prompt payloads.
- Absolute path leakage check: tracked docs and output card use repo-relative paths or `<primary-checkout>` placeholders for local-only archive commands.
- Sandbox / policy invariants preserved: the tool defaults to dry-run, copies only with `--apply`, and never deletes or prunes source directories.

## Replay Artifacts

- Trace bundle path(s): not applicable; this task preserves run artifacts rather than producing an ADL runtime trace bundle.
- Run artifact root: `.adl/trace-archive/milestones/`
- Replay command used for verification: `bash adl/tools/test_archive_run_artifacts.sh`
- Replay result: PASS

## Artifact Verification

- Primary proof surface: `.adl/trace-archive/README.md`
- Required artifacts present: yes; `.adl/trace-archive/MANIFEST.tsv` and milestone-organized run copies were generated locally.
- Artifact schema/version checks: not applicable; this issue adds a TSV manifest/report, not a new runtime schema.
- Hash/byte-stability checks: not applicable; source artifact bytes are copied without transformation, and full content hashing is deferred to future trace provenance work.
- Missing/optional artifacts and rationale: v0.87/v0.87.1 provider-demo traces were not found in the current obvious roots; that recovery/capture gap is tracked in #1520.

## Decisions / Deviations

- Chose copy-only archive behavior instead of moving or deleting source run directories so cleanup remains reversible.
- Chose first-discovered duplicate handling to avoid copying thousands of repeated worktree `.adl/runs` directories.
- Kept the archive under `.adl/trace-archive` because `.adl` is already the local untracked runtime area and this issue is about local operational data preservation.

## Follow-ups / Deferred work

- #1473 should use this archive as the preservation baseline before cleaning the visible `.adl/runs` surface.
- #1519 should add first-class trace manifest/provenance for future runs.
- #1520 should make provider/demo scripts preserve traces into the canonical archive automatically.
- #1521 should make export/discovery commands archive-aware.

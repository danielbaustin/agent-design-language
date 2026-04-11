# migrate-and-ban-legacy-tracked-adl-issue-record-residue

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1631
Run ID: issue-1631
Version: v0.87.1
Title: [v0.87.1][tools] Migrate and ban legacy tracked .adl issue-record residue
Branch: codex/1631-migrate-and-ban-legacy-tracked-adl-issue-record-residue
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5-codex
- Provider: OpenAI Codex desktop
- Start Time: 2026-04-11T23:19:00Z
- End Time: 2026-04-11T23:35:56Z

## Summary

Migrated all tracked legacy `.adl` issue prompt/task-card residue into a historical archive under `docs/records/v0.87.1/legacy-issue-records/`, documented the tracked-vs-local boundary, and added a hard guard plus regression test so new tracked `.adl/.../bodies/issue-*` and `.adl/.../tasks/issue-*` residue cannot slip back in unnoticed.

## Artifacts produced
- Migrated tracked historical issue-record files from `.adl/v0.87*` and `.adl/v0.87.1` into `docs/records/v0.87.1/legacy-issue-records/`, preserving original relative path structure for provenance.
- Added `docs/records/v0.87.1/README.md` and `docs/records/v0.87.1/legacy-issue-records/README.md` to explain the archive and the local-only canonical workflow model.
- Added `adl/tools/check_no_tracked_adl_issue_record_residue.sh` to fail when tracked `.adl` issue bodies or STP/SIP/SOR task cards reappear.
- Added `adl/tools/test_check_no_tracked_adl_issue_record_residue.sh` as regression coverage for passing and failing residue cases.
- Updated `adl/src/cli/pr_cmd.rs` and PR-command test fixtures so Rust batched checks include the new guard and fixture repos seed the helper script correctly.
- Updated `docs/records/README.md` and `docs/tooling/issue-prompts/README.md` to document the canonical tracked-vs-local boundary.

## Actions taken
- Identified the full tracked residue set with `git ls-files '.adl/*/bodies/issue-*.md' '.adl/*/tasks/issue-*/*.md'`, including older `v0.87` records and newer `v0.87.1` records that should have remained local-only.
- Moved the tracked residue into `docs/records/v0.87.1/legacy-issue-records/` with `git mv` so git history still preserves provenance.
- Added a portable shell guard that scans tracked paths via `git ls-files` and fails with a bounded remediation message when forbidden tracked `.adl` issue-record surfaces are present.
- Added a focused shell regression test covering an allowed historical archive case and a forbidden tracked `.adl` residue case.
- Wired the guard into `run_batched_checks_rust(...)` and `adl/tools/batched_checks.sh` so publication and batched local checks both enforce the boundary.
- Patched PR-command fixture helpers and the finish helper test so temporary repos seed the new guard script and the full cargo suite remains truthful.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `docs/records/README.md`, `docs/records/v0.87.1/README.md`, `docs/records/v0.87.1/legacy-issue-records/`, `docs/tooling/issue-prompts/README.md`, `adl/tools/check_no_tracked_adl_issue_record_residue.sh`, `adl/tools/test_check_no_tracked_adl_issue_record_residue.sh`, `adl/tools/batched_checks.sh`, `adl/src/cli/pr_cmd.rs`, `adl/src/cli/tests/pr_cmd_inline/mod.rs`, `adl/src/cli/tests/pr_cmd_inline/finish.rs`
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: managed issue worktree with tracked historical archive migration plus guard/test updates published through `pr finish`
- Verification performed:
  - `git ls-files '.adl/*/bodies/issue-*.md' '.adl/*/tasks/issue-*/*.md'`
    - verified the branch no longer tracks forbidden `.adl` issue-record residue after migration.
  - `find docs/records/v0.87.1/legacy-issue-records -maxdepth 4 -type f | sort`
    - verified the historical archive contains the migrated tracked provenance surfaces.
  - `git diff --check`
    - verified the patch is whitespace-clean before publication.
- Result: PASS

Rules:
- Final artifacts must exist in the main repository, not only in a worktree.
- Do not leave docs, code, or generated artifacts only under a `adl-wp-*` worktree.
- Prefer git-aware transfer into the main repo (`git checkout <branch> -- <path>` or commit + cherry-pick).
- If artifacts exist only in the worktree, the task is NOT complete.
- `Integration state` describes lifecycle state of the integrated artifact set, not where verification happened.
- `Verification scope` describes where the verification commands were run.
- `worktree_only` means at least one required path still exists only outside the main repository path.
- `pr_open` should pair with truthful `Worktree-only paths remaining` content; list those paths when they still exist only in the worktree or say `none` only when the branch contents are fully represented in the main repository path.
- If `Integration state` is `pr_open`, verify the actual proof artifacts rather than only the containing directory or card path.
- If `Integration method used` is `direct write in main repo`, `Verification scope` should normally be `main_repo` unless the deviation is explained.
- If `Verification scope` and `Integration method used` differ in a non-obvious way, explain the difference in one line.
- Completed output records must not leave `Status` as `NOT_STARTED`.
- By `pr finish`, `Status` should normally be `DONE` (or `FAILED` if the run failed and the record is documenting that failure).

## Validation
- Validation commands and their purpose:
  - `bash adl/tools/check_no_tracked_adl_issue_record_residue.sh`
    - verified the migrated branch contains no tracked `.adl` issue body or task-card residue.
  - `bash adl/tools/test_check_no_tracked_adl_issue_record_residue.sh`
    - verified the guard allows historical archive placement and rejects tracked `.adl` residue in a controlled failing fixture.
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check`
    - verified the Rust changes are correctly formatted.
  - `cargo test --manifest-path adl/Cargo.toml --quiet`
    - verified the full CLI/tooling/test suite still passes with the new residue guard wired into Rust batched checks and finish fixtures.
  - `git diff --check`
    - verified the final patch is whitespace-clean.
- Results: PASS

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

Rules:
- Replace the example values below with one actual final value per field.
- Do not leave pipe-delimited enum menus or placeholder text in a finished record.

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh"
      - "bash adl/tools/test_check_no_tracked_adl_issue_record_residue.sh"
      - "cargo fmt --manifest-path adl/Cargo.toml --all --check"
      - "cargo test --manifest-path adl/Cargo.toml --quiet"
      - "git diff --check"
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
- Determinism tests executed: the residue guard regression script plus the full cargo suite, including finish-path fixture repos that now seed the guard script deterministically.
- Fixtures or scripts used: `adl/tools/test_check_no_tracked_adl_issue_record_residue.sh` and the Rust PR-command fixture repos under `adl/src/cli/tests/pr_cmd_inline/`.
- Replay verification (same inputs -> same artifacts/order): rerunning the guard on the same repository state yields the same pass/fail classification because it inspects tracked git paths rather than mutable untracked residue.
- Ordering guarantees (sorting / tie-break rules used): the archive verification and git path scans use stable path sorting, so the migrated residue set is reported in deterministic order for identical repository state.
- Artifact stability notes: the archive preserves original tracked file contents verbatim; only their tracked location changes from `.adl/...` into `docs/records/v0.87.1/legacy-issue-records/...`.

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: manual review plus the new path-only guard confirmed this issue only moves tracked repository files and does not widen any secret-bearing surface.
- Prompt / tool argument redaction verified: the new guard reports only tracked repository-relative paths and remediation guidance.
- Absolute path leakage check: recorded commands and artifact references in this card stay repository-relative.
- Sandbox / policy invariants preserved: the change stays inside repository-local migration, documentation, and test/tooling guard surfaces.

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not_applicable; this tooling issue is verified through repository-local migration state and regression checks rather than runtime trace bundles.
- Run artifact root: `docs/records/v0.87.1/legacy-issue-records/` plus temporary repos created by `adl/tools/test_check_no_tracked_adl_issue_record_residue.sh`.
- Replay command used for verification: `bash adl/tools/test_check_no_tracked_adl_issue_record_residue.sh`
- Replay result: PASS

## Artifact Verification
- Primary proof surface: `docs/records/v0.87.1/legacy-issue-records/`, `adl/tools/check_no_tracked_adl_issue_record_residue.sh`, and the Rust PR-command wiring in `adl/src/cli/pr_cmd.rs`.
- Required artifacts present: yes; the migrated archive, boundary docs, guard script, regression script, and Rust/test updates all exist on the branch.
- Artifact schema/version checks: no schema changes; this issue hardens placement and publication rules rather than changing prompt schemas.
- Hash/byte-stability checks: not_applicable; proof is repository path migration plus deterministic guard behavior, not a byte-stable generated artifact.
- Missing/optional artifacts and rationale: no standalone demo artifact is required because this is a tooling/boundary hardening issue.

## Decisions / Deviations
- Preserved original tracked file contents and original relative path shapes under `legacy-issue-records/` rather than rewriting the archived artifacts, so provenance remains reviewable.
- Archived the tracked `#1634` root task cards as residue too, because once they were tracked they became part of the same forbidden class this issue is supposed to eliminate.

## Follow-ups / Deferred work
- `#1630` remains the finish/closeout-time SOR truth gate.
- `#1632` remains the automatic post-merge normalization follow-on.
- `#1633` remains the milestone stale-record gate follow-on.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.

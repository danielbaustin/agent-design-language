# v0-87-maintainability-create-large-file-watchlist-from-v0-86-review

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1289
Run ID: issue-1289
Version: v0.87
Title: [v0.87][maintainability] Create large-file watchlist from v0.86 review
Branch: codex/1289-v0-87-maintainability-create-large-file-watchlist-from-v0-86-review
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5
- Provider: OpenAI Codex desktop
- Start Time: 2026-04-06 11:15:05 PDT
- End Time: 2026-04-06 11:17:32 PDT

## Summary
Refreshed the canonical Rust module watch list in `docs/tooling/rust_module_watch_list.md` so it reflects the current repository state and explicitly captures the deferred large-file follow-up from the external `v0.86` review. The update preserves the existing non-blocking governance posture while naming the current largest modules, refreshing measured LoC bands, and recording explicit future split boundaries for the three files the review called out.

## Artifacts produced
- `docs/tooling/rust_module_watch_list.md`
- updated final execution record at `.adl/v0.87/tasks/issue-1289__v0-87-maintainability-create-large-file-watchlist-from-v0-86-review/sor.md`

## Actions taken
- rebased the `1289` worktree branch onto current `origin/main` before making documentation changes
- measured the current large-module set with `bash adl/tools/report_large_rust_modules.sh --format tsv`
- refreshed the canonical watch-list table to match current module sizes and watch levels
- added a `v0.86 External Review Follow-up` section naming `adl/src/cli/pr_cmd.rs`, `adl/src/demo.rs`, and `adl/src/remote_exec.rs`
- recorded explicit deferral/split guidance for the `Rationale` band rather than forcing immediate refactors

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `docs/tooling/rust_module_watch_list.md` and `.adl/v0.87/tasks/issue-1289__v0-87-maintainability-create-large-file-watchlist-from-v0-86-review/sor.md` on branch `codex/1289-v0-87-maintainability-create-large-file-watchlist-from-v0-86-review`
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: direct edit in the `adl-wp-1289` worktree branch followed by commit, push, and PR publication
- Verification performed:
  - `git status --short` confirmed the branch was clean after publication before janitor follow-up edits
  - `gh pr view 1331 --json url,state,headRefName,baseRefName` verified an open PR from the branch to `main`
  - `sed -n '1,260p' docs/tooling/rust_module_watch_list.md` confirmed the canonical proof surface contains the refreshed table and `v0.86` follow-up section
- Result: the branch is published, PR `#1331` is open, and no required artifacts remain worktree-only

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
  - `bash adl/tools/report_large_rust_modules.sh --format tsv` to measure the current Rust module watch bands from repository source
  - `git diff -- docs/tooling/rust_module_watch_list.md` to verify the documentation delta matches the intended governance update
- Results:
  - the report script succeeded and identified `adl/src/cli/pr_cmd.rs`, `adl/src/demo.rs`, and `adl/src/remote_exec.rs` as current `RATIONALE` entries
  - the diff shows the watch-list table, external-review follow-up section, and required deferral rule updates with no unrelated file edits

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
      - "bash adl/tools/report_large_rust_modules.sh --format tsv"
      - "git diff -- docs/tooling/rust_module_watch_list.md"
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
- Determinism tests executed: reran `bash adl/tools/report_large_rust_modules.sh --format tsv` as the canonical measurement surface
- Fixtures or scripts used: `adl/tools/report_large_rust_modules.sh`
- Replay verification (same inputs -> same artifacts/order): the same repository tree produces the same ordered TSV watch list on rerun because the script remeasures the same file set and emits a stable banded list
- Ordering guarantees (sorting / tie-break rules used): the watch-list update records the script output order, which is stable for a fixed tree
- Artifact stability notes: this is a documentation refresh driven by deterministic repository measurements, not a generated binary artifact

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: manual review of the updated doc and execution record confirmed no credentials or tokens were introduced
- Prompt / tool argument redaction verified: not applicable; this task updated maintainability documentation only and did not record prompt transcripts or tool arguments beyond safe shell commands
- Absolute path leakage check: reviewed the edited doc and record to keep artifact references repository-relative; no unjustified absolute host paths were added
- Sandbox / policy invariants preserved: yes; the task modified documentation only and did not change runtime policy, sandboxing, or execution behavior

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable; no runtime trace artifacts were produced for this docs-only task
- Run artifact root: not applicable; no run-artifact directory was created
- Replay command used for verification: `bash adl/tools/report_large_rust_modules.sh --format tsv`
- Replay result: the measurement command succeeded and provided the canonical current watch-band data used in the updated doc

## Artifact Verification
- Primary proof surface: `docs/tooling/rust_module_watch_list.md`
- Required artifacts present: yes; the canonical watch-list doc now records the current large-module table and the external `v0.86` review follow-up
- Artifact schema/version checks: not applicable; this is a Markdown governance document with no external schema version
- Hash/byte-stability checks: not applicable; human-reviewed documentation diff rather than hashed build output
- Missing/optional artifacts and rationale: no additional artifacts were required beyond the canonical watch-list doc and execution records

## Decisions / Deviations
- kept the issue scoped to documentation/governance rather than opportunistic Rust refactors, matching the issue body and external review posture
- used the existing canonical watch-list document in `docs/tooling/` instead of introducing a second maintainability surface

## Follow-ups / Deferred work
- when a future PR materially changes any `Rationale`-band module, it should either improve structure or record an explicit deferral note in the output card
- the watch list should be refreshed again when later milestone work significantly changes module boundaries or introduces new large modules

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.

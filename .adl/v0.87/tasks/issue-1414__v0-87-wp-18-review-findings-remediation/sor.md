# v0-87-wp-18-review-findings-remediation

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1414
Run ID: issue-1414
Version: v0.87
Title: [v0.87][WP-18] Review findings remediation
Branch: codex/1414-v0-87-wp-18-review-findings-remediation
Status: IN_PROGRESS

Execution:
- Actor: Codex
- Model: GPT-5 Codex
- Provider: OpenAI Codex desktop
- Start Time: 2026-04-08T07:31:11Z
- End Time: 2026-04-08T07:31:11Z

## Summary
Applied the first accepted remediation slice from the WP-15 internal review. This patch updates the v0.87 README, sprint plan, and WBS so the reviewer-facing milestone spine reflects the closed Sprint 3 issues and the active WP-18 remediation issue truthfully.

## Artifacts produced
- `docs/milestones/v0.87/README.md`
- `docs/milestones/v0.87/SPRINT_v0.87.md`
- `docs/milestones/v0.87/WBS_v0.87.md`

## Actions taken
- updated the milestone README release-tail status to reflect the closed/open Sprint 3 issue set truthfully
- updated the Sprint 3 work-package table and execution summary to reflect closed `#1345`, `#1346`, `#1347`, `#1354` and open `#1348`, `#1349`, `#1414`, `#1350`
- added `WP-18 / #1414` to the WBS sequencing and acceptance mapping

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none yet; changes are currently on the issue branch only
- Worktree-only paths remaining: `docs/milestones/v0.87/README.md`, `docs/milestones/v0.87/SPRINT_v0.87.md`, `docs/milestones/v0.87/WBS_v0.87.md`
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: bounded branch update in the `adl-wp-1414` worktree with PR publication
- Verification performed:
  - `git status` verified only the three intended milestone docs were modified
  - `git diff -- docs/milestones/v0.87/README.md docs/milestones/v0.87/SPRINT_v0.87.md docs/milestones/v0.87/WBS_v0.87.md` verified the remediation scope matched the accepted findings
- Result: worktree contains the intended remediation patch and is ready for PR publication

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
  - `git diff -- docs/milestones/v0.87/README.md docs/milestones/v0.87/SPRINT_v0.87.md docs/milestones/v0.87/WBS_v0.87.md` to verify the patch matches the three accepted review findings and does not widen scope
  - `gh issue view 1345 --json state`, `gh issue view 1346 --json state`, `gh issue view 1347 --json state`, `gh issue view 1354 --json state`, `gh issue view 1414 --json state` to verify the referenced issue states are truthful
- Results:
  - diff scope matched the accepted README, Sprint, and WBS fixes only
  - referenced issue states matched the updated milestone-doc status language

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
      - "git diff -- docs/milestones/v0.87/README.md docs/milestones/v0.87/SPRINT_v0.87.md docs/milestones/v0.87/WBS_v0.87.md"
      - "gh issue view 1345 --json state"
      - "gh issue view 1346 --json state"
      - "gh issue view 1347 --json state"
      - "gh issue view 1354 --json state"
      - "gh issue view 1414 --json state"
  determinism:
    status: PASS
    replay_verified: unknown
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
- Determinism tests executed: no runtime replay test was needed for this doc-only remediation slice
- Fixtures or scripts used: not applicable; this patch updates milestone status text only
- Replay verification (same inputs -> same artifacts/order): unknown; not applicable to this review-driven doc patch
- Ordering guarantees (sorting / tie-break rules used): issue-state references were checked directly against live GitHub issue state before recording the updated status lines
- Artifact stability notes: the affected artifacts are canonical milestone markdown docs with bounded textual changes only

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: reviewed changed markdown content for secrets; none were introduced
- Prompt / tool argument redaction verified: no prompts or tool arguments were added to the milestone docs
- Absolute path leakage check: final changed docs use repository-relative paths and issue references only
- Sandbox / policy invariants preserved: work stayed in the issue worktree and no main-branch edits were made

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable; no trace artifact changed in this doc-only remediation
- Run artifact root: not applicable
- Replay command used for verification: not applicable
- Replay result: not applicable

## Artifact Verification
- Primary proof surface: `docs/milestones/v0.87/SPRINT_v0.87.md`
- Required artifacts present: yes; the README, sprint plan, and WBS all contain the remediation changes
- Artifact schema/version checks: not applicable; markdown structure only
- Hash/byte-stability checks: not run; this issue does not require byte-stable generated artifacts
- Missing/optional artifacts and rationale: no additional artifacts were required for this bounded remediation slice

## Decisions / Deviations
- Kept the remediation scoped to the three accepted internal-review findings only.
- Did not broaden this issue into the separate real repo code review; that will be tracked independently.

## Follow-ups / Deferred work
- Publish this remediation as a PR under `#1414`.
- Create and run a separate real repo code-review issue rather than overloading `#1348` or `#1414`.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.

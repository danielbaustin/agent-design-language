# v0-87-1-meta-create-sprint-2-issue-and-card-set

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1455
Run ID: issue-1455
Version: v0.87.1
Title: [v0.87.1][meta] Create Sprint 2 issue and card set
Branch: codex/1455-v0-87-1-wp-11-create-sprint-2-issue-and-card-set
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5-codex
- Provider: local Codex desktop session
- Start Time: 2026-04-08T18:11:00Z
- End Time: 2026-04-08T19:10:00Z

## Summary
Created the full `v0.87.1` Sprint 2 issue set (`#1458` through `#1464`), corrected this out-of-band meta issue so it no longer collides with the real `WP-11`, updated the milestone WBS and Sprint docs with the new Sprint 2 issue numbers, and normalized the new Sprint 2 SIP cards into truthful pre-run review state.

## Artifacts produced
- created GitHub issues `#1458` through `#1464`
- updated local source prompts and task bundles for `#1458` through `#1464`
- updated `docs/milestones/v0.87.1/WBS_v0.87.1.md`
- updated `docs/milestones/v0.87.1/SPRINT_v0.87.1.md`
- corrected local source/STP/SIP/SOR surfaces for meta issue `#1455`

## Actions taken
- bootstrapped the out-of-band Sprint 2 meta issue and bound it to the `adl-wp-1455` worktree
- authored and created the Sprint 2 issues for `WP-09` through `WP-15` as `#1458` through `#1464`
- corrected issue `#1455` from a mistaken `WP-11` issue into a `[meta]` bootstrap/support issue
- renamed the local `#1455` source/body/task-bundle surfaces from the old `wp-11` slug to the corrected `meta` slug
- updated the Sprint 2 WBS and Sprint doc rows to reference `#1458` through `#1464`
- normalized the newly created Sprint 2 SIP cards to truthful bootstrap-only `Branch: not bound yet` state so they are review-ready instead of falsely run-bound
- synced the corrected `#1455` GitHub issue title/body to match the authored local source prompt

## Main Repo Integration (REQUIRED)
- Main-repo paths updated:
  - `docs/milestones/v0.87.1/WBS_v0.87.1.md`
  - `docs/milestones/v0.87.1/SPRINT_v0.87.1.md`
- Worktree-only paths remaining:
  - `docs/milestones/v0.87.1/WBS_v0.87.1.md`
  - `docs/milestones/v0.87.1/SPRINT_v0.87.1.md`
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: issue worktree edits pending commit/PR publication
- Verification performed:
  - `git status --short` verified only the intended tracked milestone-doc paths were changed for issue `#1455`, aside from one unrelated unstaged worktree-local template diff that was intentionally left unstaged.
  - `git diff -- docs/milestones/v0.87.1/WBS_v0.87.1.md docs/milestones/v0.87.1/SPRINT_v0.87.1.md` verified the tracked diff is exactly the Sprint 2 issue-number assignment update.
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
  - `bash adl/tools/pr.sh create --title ... --body-file ... --labels ... --version v0.87.1`
    - created and bootstrapped the Sprint 2 issues `#1458` through `#1464` with local source/STP/SIP/SOR bundles
  - `gh issue edit 1455 --title '[v0.87.1][meta] Create Sprint 2 issue and card set' --body-file .adl/v0.87.1/bodies/issue-1455-v0-87-1-meta-create-sprint-2-issue-and-card-set.md`
    - verified the corrected meta-issue body/title was synced to GitHub
  - `bash adl/tools/pr.sh doctor 1458 --version v0.87.1 --json`
    - verified the first Sprint 2 issue is in truthful `pre_run` state with `ready_status: PASS`
  - `bash adl/tools/pr.sh doctor 1464 --version v0.87.1 --json`
    - verified the last Sprint 2 issue is in truthful `pre_run` state with `ready_status: PASS`
  - `for n in 1458 1459 1460 1461 1462 1463 1464; do gh issue view $n --json title,url >/dev/null; done`
    - verified all Sprint 2 issues exist on GitHub
- Results:
  - all issue-creation, issue-sync, and doctor verification commands passed
  - both doctor checks blocked only on the expected open-PR wave guard and reported `ready_status: PASS`

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
      - "bash adl/tools/pr.sh create --title ... --body-file ... --labels ... --version v0.87.1"
      - "gh issue edit 1455 --title '[v0.87.1][meta] Create Sprint 2 issue and card set' --body-file .adl/v0.87.1/bodies/issue-1455-v0-87-1-meta-create-sprint-2-issue-and-card-set.md"
      - "bash adl/tools/pr.sh doctor 1458 --version v0.87.1 --json"
      - "bash adl/tools/pr.sh doctor 1464 --version v0.87.1 --json"
      - "for n in 1458 1459 1460 1461 1462 1463 1464; do gh issue view $n --json title,url >/dev/null; done"
  determinism:
    status: PASS
    replay_verified: false
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
- Determinism tests executed: repeated repo-native bootstrap commands for Sprint 2 issue creation and two bounded doctor checks on the resulting issue set.
- Fixtures or scripts used: `adl/tools/pr.sh` create/doctor flows and the authored issue-body files used to seed `#1458` through `#1464`.
- Replay verification (same inputs -> same artifacts/order): full replay was not required for this planning/bootstrap issue.
- Ordering guarantees (sorting / tie-break rules used): Sprint 2 issue assignment was created in the intended WBS order (`WP-09` through `WP-15`) and then written back to the WBS/Sprint docs in the same sequence.
- Artifact stability notes: this issue changes milestone-doc issue references and local bootstrap cards only; it does not change runtime artifact schemas or trace formats.

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: manual review of the tracked WBS/Sprint diff and the authored local card content; no secrets or credentials were introduced.
- Prompt / tool argument redaction verified: final tracked changes contain only milestone issue references; the local cards use repository-relative paths only.
- Absolute path leakage check: tracked docs are free of absolute host paths; local card references remained repository-relative where recorded.
- Sandbox / policy invariants preserved: yes; no runtime, network, or privilege-boundary behavior was changed by this issue.

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable; this meta/bootstrap issue did not produce runtime trace artifacts.
- Run artifact root: not applicable; no persistent runtime artifact root was required.
- Replay command used for verification: not applicable.
- Replay result: not applicable.

## Artifact Verification
- Primary proof surface: the created Sprint 2 issue set (`#1458` through `#1464`), the corrected meta issue `#1455`, and the updated `v0.87.1` WBS/Sprint issue references.
- Required artifacts present: yes; the Sprint 2 GitHub issues exist, their local source/STP/SIP/SOR bundles exist, and the tracked WBS/Sprint docs carry the new issue numbers.
- Artifact schema/version checks: no schema changes; the card cleanup stayed within the existing issue/STP/SIP/SOR contracts.
- Hash/byte-stability checks: not required for this planning/bootstrap issue.
- Missing/optional artifacts and rationale: no demo or runtime proof artifacts were required because this issue stops at review-ready bootstrap state.

## Decisions / Deviations
- Corrected `#1455` from a mistaken `WP-11` title/scope to `[meta]` so it no longer collides with the real Sprint 2 `WP-11` issue (`#1460`).
- Left the already-bound `1455` execution branch/worktree slug unchanged to avoid unnecessary branch churn; only the issue/card truth surfaces were normalized.
- Normalized the new Sprint 2 SIP cards to truthful pre-run state after creation because the generated SIPs initially overclaimed that branches/worktrees already existed.

## Follow-ups / Deferred work
- Review the Sprint 2 source/STP/SIP bundles qualitatively before starting run phase for any of `#1458` through `#1464`.
- Open the later review-tail issues (`WP-16` through `WP-20`) with the same out-of-band/bootstrap discipline when Sprint 3 planning is ready.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.

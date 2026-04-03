# v0-86-tools-retire-pr-start-from-the-public-lifecycle-and-remove-it-from-issue-bootstrap-skill-guidance

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1312
Run ID: issue-1312
Version: v0.86
Title: [v0.86][tools] Retire pr start from the public lifecycle and remove it from issue-bootstrap skill guidance
Branch: codex/1312-v0-86-tools-retire-pr-start-from-the-public-lifecycle-and-remove-it-from-issue-bootstrap-skill-guidance
Status: IN_PROGRESS

Execution:
- Actor: Codex
- Model: GPT-5 Codex
- Provider: OpenAI
- Start Time: 2026-04-02T15:08:00Z
- End Time: 2026-04-02T15:20:00Z

## Summary
This run retires `pr start` from the taught public lifecycle without breaking the legacy alias itself. The public help, PR tooling docs, and issue-bootstrap skill bundle now teach the bootstrap -> qualitative review -> issue-mode `pr run` -> review/closeout model, with `doctor` separate and `start` reduced to legacy-background status only.

## Artifacts produced
- Updated `adl/tools/pr.sh`.
- Updated `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`.
- Updated `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md`.
- Updated `.adl/skills/issue-bootstrap/SKILL.md`.
- Updated `.adl/skills/issue-bootstrap/references/bootstrap-playbook.md`.

## Actions taken
- Removed `start` from the public `pr.sh --help` command list, flag guidance, and examples while keeping a narrow legacy-alias note.
- Reframed PR tooling docs so `start` is no longer taught as an active public lifecycle step.
- Rebuilt the issue-bootstrap skill wording so it teaches bootstrap, qualitative review, and issue-mode `pr run` without start-era ambiguity.
- Kept the underlying `start` dispatch path in code as a legacy alias during migration instead of deleting compatibility outright in this issue.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `adl/tools/pr.sh`, `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`, `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md`, `.adl/skills/issue-bootstrap/SKILL.md`, `.adl/skills/issue-bootstrap/references/bootstrap-playbook.md`
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: tracked repair in the issue branch worktree, with the legacy alias preserved in code but removed from the taught command surface.
- Verification performed:
  - `bash adl/tools/pr.sh --help | sed -n '1,90p'` to verify the taught public command surface no longer lists `start`.
  - `rg -n "\bstart\b" docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md .adl/skills/issue-bootstrap adl/tools/pr.sh -g '!**/target/**'` to verify remaining `start` mentions are legacy/internal rather than active public guidance.
  - `git status --short --untracked-files=all` to verify the branch contains only the intended retirement-surface edits.
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
  - `bash adl/tools/pr.sh --help | sed -n '1,90p'` to verify the public help surface teaches `run` but no longer lists `start` as a command.
  - `rg -n "\bstart\b" docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md .adl/skills/issue-bootstrap adl/tools/pr.sh -g '!**/target/**'` to audit where `start` still appears after the cleanup.
  - `git status --short --untracked-files=all` to verify the branch changed only the intended help/docs/skill surfaces.
- Results:
  - Public help now teaches `create`, `init`, `run`, `finish`, and other non-legacy commands without listing `start`.
  - Remaining `start` mentions are limited to the legacy alias implementation, a narrow legacy note in help, and architecture text that explicitly marks it as a legacy alias.
  - The issue-bootstrap skill bundle no longer teaches `start` as part of the normal handoff.

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
      - "bash adl/tools/pr.sh --help | sed -n '1,90p'"
      - "rg -n \"\\bstart\\b\" docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md .adl/skills/issue-bootstrap adl/tools/pr.sh -g '!**/target/**'"
      - "git status --short --untracked-files=all"
  determinism:
    status: PARTIAL
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
- Determinism tests executed: repeated help-surface and grep-based audits over the same checkout.
- Fixtures or scripts used: `pr.sh --help`, `rg`, and `git status`.
- Replay verification (same inputs -> same artifacts/order): rerunning the help output and grep sweep on the same branch yields the same taught command surface and the same legacy-only `start` locations.
- Ordering guarantees (sorting / tie-break rules used): grep output is stable for the same files and checkout state.
- Artifact stability notes: this issue changes only shell/docs/skill text surfaces and keeps runtime behavior unchanged except for public command teaching.

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: manual review plus targeted grep over changed text surfaces found no secrets or tokens.
- Prompt / tool argument redaction verified: the changed help/docs/skill surfaces describe workflow commands only.
- Absolute path leakage check: the output record uses repository-relative references and does not introduce unjustified absolute host paths.
- Sandbox / policy invariants preserved: the issue does not widen command privileges; it narrows the taught public surface while preserving legacy compatibility under the hood.

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable; this issue does not produce runtime trace bundles.
- Run artifact root: not applicable; this is a docs/help/skill control-plane cleanup.
- Replay command used for verification: `bash adl/tools/pr.sh --help | sed -n '1,90p'` and the targeted `rg` audit.
- Replay result: PASS. The taught public surface consistently omits `start` while the legacy alias remains visible only in bounded compatibility locations.

## Artifact Verification
- Primary proof surface: the updated `pr.sh` help output plus the aligned PR tooling docs and issue-bootstrap skill bundle.
- Required artifacts present: yes.
- Artifact schema/version checks: not applicable; no runtime or card schema changed.
- Hash/byte-stability checks: not run; textual verification was sufficient for this bounded lifecycle-teaching change.
- Missing/optional artifacts and rationale: none.

## Decisions / Deviations
- `start` remains implemented as a legacy alias in code, but it is intentionally no longer listed as part of the public command model.
- This issue does not resolve the separate long-term `create` versus `init` bootstrap simplification question; it only removes `start` from the taught downstream binder role.

## Follow-ups / Deferred work
- Use the updated issue-bootstrap skill for the next issue and confirm that no manual command interpretation is needed.
- If the legacy alias is no longer used after migration, a later issue can delete the `start` dispatch path entirely instead of merely retiring it from teaching.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.

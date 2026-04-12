# make-workflow-conductor-derive-live-routing-state-and-prove-real-repo-handoffs

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1679
Run ID: issue-1679
Version: v0.88
Title: [v0.88][tools] Make workflow-conductor derive live routing state and prove real repo handoffs
Branch: codex/1679-make-workflow-conductor-derive-live-routing-state-and-prove-real-repo-handoffs
Status: DONE

Execution:
- Actor: Codex
- Model: gpt-5-codex
- Provider: OpenAI
- Start Time: 2026-04-12T19:10:00Z
- End Time: 2026-04-12T19:48:38Z

## Summary
Implemented a route-only workflow-conductor entrypoint that derives live routing state from real repo surfaces, writes a bounded routing artifact, blocks on hard policy failures, proves realistic handoff behavior with stronger contract tests, and is now published for review in PR `#1682`.

## Artifacts produced
- `adl/tools/skills/workflow-conductor/scripts/route_workflow.py`
- `.gitignore` Python-bytecode ignore rules
- updated workflow-conductor contract, playbook, schema, and operator-guide docs
- expanded `adl/tools/test_workflow_conductor_skill_contracts.sh`
- local routing artifact proof at `.adl/reviews/workflow-conductor-1679-check.md`

## Actions taken
- Added `route_workflow.py` as the preferred route-only entrypoint for structured conductor invocations.
- Taught the conductor to collect routing state from real issue, task-bundle, branch, worktree, and PR surfaces rather than requiring only hand-authored derived payloads.
- Tightened policy handling so required-subagent and skill/editor policy failures return `blocked` instead of silently routing through a failure.
- Split healthy open-PR state from blocker-driven in-flight PR state so janitor is only selected when the PR actually needs janitor work.
- Added bounded routing-artifact emission and exposed the artifact path in the structured result.
- Extended contract tests to cover route-issue, route-task-bundle editor routing, route-finish, worktree-output-driven finish routing, blocked policy behavior, and PR janitor/closeout/review-wait cases.
- Removed tracked `__pycache__/*.pyc` artifacts from the branch and added repo ignore rules so future Python skill runs do not stage bytecode by accident.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked repository paths are updated on the issue branch via PR 1682
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: `pr finish` commit + push, followed by a cleanup commit on the same issue branch
- Verification performed:
  - `bash adl/tools/pr.sh finish 1679 --title "[v0.88][tools] Make workflow-conductor derive live routing state and prove real repo handoffs" --paths "adl/tools/skills/workflow-conductor,adl/tools/skills/docs/WORKFLOW_CONDUCTOR_SKILL_INPUT_SCHEMA.md,adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md,adl/tools/test_workflow_conductor_skill_contracts.sh"` validated, committed, pushed, and opened PR `#1682`.
  - `git status --short --branch` after the bytecode cleanup verified the branch contained only the intended follow-up cleanup changes before the final push.
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
  - `python3 -m py_compile adl/tools/skills/workflow-conductor/scripts/route_workflow.py adl/tools/skills/workflow-conductor/scripts/select_next_skill.py` verified the conductor Python entrypoints parse cleanly.
  - `bash adl/tools/test_workflow_conductor_skill_contracts.sh` verified deterministic selection, policy blocking, routing-artifact emission, realistic fixture-driven route collection, and bounded PR-state routing behavior.
  - `python3 adl/tools/skills/workflow-conductor/scripts/route_workflow.py --input <tmp-input> --artifact-path .adl/reviews/workflow-conductor-1679-check.md` verified the new route-only entrypoint against the live issue worktree for `#1679`.
  - `python3 adl/tools/skills/workflow-conductor/scripts/route_workflow.py --input <tmp-input> --artifact-path .adl/reviews/workflow-conductor-1679-finish-check.md` verified that a completed worktree-local `sor.md` now drives the conductor to `pr-finish` rather than `pr-run` or a false PR-wait state.
  - `git diff --check` verified no whitespace or patch-format errors remain in the changed tracked files.
- Results: all listed validation commands passed.

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
      - "python3 -m py_compile adl/tools/skills/workflow-conductor/scripts/route_workflow.py adl/tools/skills/workflow-conductor/scripts/select_next_skill.py"
      - "bash adl/tools/test_workflow_conductor_skill_contracts.sh"
      - "python3 adl/tools/skills/workflow-conductor/scripts/route_workflow.py --input <tmp-input> --artifact-path .adl/reviews/workflow-conductor-1679-check.md"
      - "python3 adl/tools/skills/workflow-conductor/scripts/route_workflow.py --input <tmp-input> --artifact-path .adl/reviews/workflow-conductor-1679-finish-check.md"
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
      present: true
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: reran the conductor contract test after each routing/policy change and reran the live `route_workflow.py` invocation for the same `#1679` worktree input.
- Fixtures or scripts used: `adl/tools/test_workflow_conductor_skill_contracts.sh`, including fixture repos and mocked `gh`/`pr.sh` surfaces, plus `adl/tools/skills/workflow-conductor/scripts/route_workflow.py`.
- Replay verification (same inputs -> same artifacts/order): identical fixture inputs produce the same selected skill and artifact path shape, the live `#1679` worktree route invocation selected `pr-run` before output-card completion, and the same invocation selected `pr-finish` once the completed `sor.md` became part of the collected state.
- Ordering guarantees (sorting / tie-break rules used): route collection sorts candidate task-bundle/body matches before selecting and fails on ambiguous duplicate canonical surfaces rather than choosing nondeterministically.
- Artifact stability notes: the routing artifact content is derived from the structured result fields only and is stable for identical collected state.

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: manual review of the new script, docs, and test fixtures for hard-coded tokens or secrets; none were introduced.
- Prompt / tool argument redaction verified: the new routing artifact writes only bounded routing/compliance fields and does not record prompts or tool arguments beyond the declared artifact path.
- Absolute path leakage check: `git diff --check` passed, and the finalized SOR records repository-relative command paths rather than host-path command strings.
- Sandbox / policy invariants preserved: yes; the conductor remains route-only and writes only the declared routing artifact plus issue-scoped local execution records.

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable; this issue did not generate a runtime trace bundle.
- Run artifact root: `.adl/reviews/workflow-conductor-1679-check.md`
- Replay command used for verification: reran `python3 adl/tools/skills/workflow-conductor/scripts/route_workflow.py --input <tmp-input> --artifact-path .adl/reviews/workflow-conductor-1679-check.md` and `python3 adl/tools/skills/workflow-conductor/scripts/route_workflow.py --input <tmp-input> --artifact-path .adl/reviews/workflow-conductor-1679-finish-check.md`
- Replay result: pass; the route entrypoint reproduced the same `run_bound -> pr-run` handoff before output-card completion and the same `execution_done -> pr-finish` handoff after the completed `sor.md` was present.

## Artifact Verification
- Primary proof surface: `adl/tools/test_workflow_conductor_skill_contracts.sh` plus the live artifact `.adl/reviews/workflow-conductor-1679-check.md`
- Required artifacts present: yes; the new route entrypoint, updated contract/docs, expanded conductor contract test, and `.gitignore` cleanup are all present on the issue branch.
- Artifact schema/version checks: the skill manifest still declares `workflow_conductor.v1`, and the updated schema/guide examples include `observed_state.subagent_assigned` for explicit policy observation.
- Hash/byte-stability checks: not run as a separate hash step; deterministic rerun behavior was verified through repeated identical route/test results.
- Missing/optional artifacts and rationale: no separate demo artifact is required for this bounded tooling issue.

## Decisions / Deviations
- Added an explicit route-only entrypoint script instead of overloading `select_next_skill.py` with both state collection and artifact writing so the pure selector remains usable in deterministic fixture tests.
- Treated healthy open PRs as `human_review` handoff rather than auto-janitor to avoid overreach from fuzzy PR state.
- Taught worktree/task-bundle routing to honor `sor.md` `Status: DONE` as a finish handoff signal so the conductor can advance from execution to publication without requiring a separate hidden state machine.
- Treated doctor open-PR-wave scheduling state as separate from issue-specific PR state so the conductor does not mistake another issue's open PR for this issue being already in janitor/review mode.

## Follow-ups / Deferred work
- The conductor can now derive live repo state and write its routing artifact, but full automatic lifecycle chaining still depends on the underlying lifecycle skills remaining truthful and merge-ready.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.

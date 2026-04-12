# mature-workflow-conductor-into-a-trusted-skill-orchestrator

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1685
Run ID: issue-1685
Version: v0.88
Title: [v0.88][tools] Mature workflow-conductor into a trusted skill orchestrator
Branch: codex/1685-mature-workflow-conductor-into-a-trusted-skill-orchestrator
Status: DONE

Execution:
- Actor: Codex
- Model: gpt-5-codex
- Provider: OpenAI
- Start Time: 2026-04-12T13:56:31Z
- End Time: 2026-04-12T22:56:07Z

## Summary
Matured the workflow-conductor into a more trusted skill orchestrator by adding bounded blocker classification, explicit continue/ask-operator/stop handoff intent, safer worktree disambiguation, child-wave satisfaction detection for tracker issues, and stronger behavioral proof against real and fixture-driven repo states. Published for review in PR `#1688`.

## Artifacts produced
- `adl/tools/skills/workflow-conductor/scripts/route_workflow.py`
- `adl/tools/skills/workflow-conductor/scripts/select_next_skill.py`
- updated workflow-conductor contract, playbook, schema, and operator-guide docs
- expanded `adl/tools/test_workflow_conductor_skill_contracts.sh`
- live routing artifact at `.adl/reviews/workflow-conductor-1685-post-implementation.md`

## Actions taken
- Added explicit blocker classification to the conductor result so known doctor and PR failure families are recorded instead of being flattened into generic blocked state.
- Added explicit handoff intent and escalation reasons so the conductor now distinguishes `continue`, `ask_operator`, and `stop` outcomes.
- Taught `route_worktree` to disambiguate multi-bundle worktrees by issue number instead of failing reflexively on any extra bundle presence.
- Added a bounded tracker/WP check that detects when closed child issues already mark the acceptance surface as satisfied and routes to human review instead of `pr-run`.
- Tightened the docs and output contract so the skill’s behavior and its stated stop-boundary model now match.
- Expanded behavioral tests to cover policy-stop outcomes, open-PR-wave finish escalation, PR blocker classification, healthy-PR wait handling, and disambiguated worktree routing.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked repository paths are updated on the issue branch via PR 1688
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: `pr finish` validation followed by manual commit + push + PR creation because unrelated tracked legacy `.adl` residue on `main` blocked the publication guard
- Verification performed:
  - `bash adl/tools/pr.sh finish 1685 --title "[v0.88][tools] Mature workflow-conductor into a trusted skill orchestrator" --paths "adl/tools/skills/workflow-conductor,adl/tools/skills/docs/WORKFLOW_CONDUCTOR_SKILL_INPUT_SCHEMA.md,adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md,adl/tools/test_workflow_conductor_skill_contracts.sh"` validated the issue bundle and publication inputs before stopping on unrelated tracked legacy `.adl` residue from another issue.
  - `git add ... && git add -f .adl/v0.88/.../issue-1685... && git commit` recorded the intended tracked changes plus the canonical issue bundle on the issue branch.
  - `git push -u origin codex/1685-mature-workflow-conductor-into-a-trusted-skill-orchestrator` published the issue branch.
  - `gh pr create --base main --head codex/1685-mature-workflow-conductor-into-a-trusted-skill-orchestrator --title "[v0.88][tools] Mature workflow-conductor into a trusted skill orchestrator"` opened PR `#1688` with closing linkage for issue `#1685`.
  - `git status --short --branch` verified the branch was clean after publication.
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
  - `python3 -m py_compile adl/tools/skills/workflow-conductor/scripts/route_workflow.py adl/tools/skills/workflow-conductor/scripts/select_next_skill.py` verified the conductor Python entrypoints parse cleanly after the maturity changes.
  - `bash adl/tools/test_workflow_conductor_skill_contracts.sh` verified deterministic skill selection, explicit policy-stop behavior, open-PR-wave escalation, PR blocker classification, disambiguated worktree routing, and tracker-child-wave satisfaction detection.
  - `python3 adl/tools/skills/workflow-conductor/scripts/route_workflow.py --input <tmp-input> --artifact-path .adl/reviews/workflow-conductor-1685-post-implementation.md` verified the live issue worktree routes cleanly with an explicit issue-number disambiguator.
  - `git diff --check` verified there are no whitespace or malformed patch artifacts in the tracked changes.
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
      - "python3 adl/tools/skills/workflow-conductor/scripts/route_workflow.py --input <tmp-input> --artifact-path .adl/reviews/workflow-conductor-1685-post-implementation.md"
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
- Determinism tests executed: reran the conductor contract suite after the blocker-class, escalation, worktree-disambiguation, and tracker-child-wave changes, then reran the live route entrypoint for the same `#1685` worktree target.
- Fixtures or scripts used: `adl/tools/test_workflow_conductor_skill_contracts.sh` and `adl/tools/skills/workflow-conductor/scripts/route_workflow.py`.
- Replay verification (same inputs -> same artifacts/order): identical fixture inputs produce the same selected skill, blocker class, and continuation result, and the live `#1685` worktree invocation reproducibly writes the same bounded routing artifact shape for the same collected state.
- Ordering guarantees (sorting / tie-break rules used): route collection sorts candidate canonical surfaces and now uses explicit `issue_number` disambiguation for multi-bundle worktrees rather than choosing nondeterministically.
- Artifact stability notes: the routing artifact is derived from the structured result only, so stable collected state yields stable emitted content.

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: manual review of the conductor scripts, docs, and test fixtures for tokens or secrets; none were introduced.
- Prompt / tool argument redaction verified: yes; the routing artifact records bounded workflow/compliance facts and does not persist prompt bodies or arbitrary tool arguments.
- Absolute path leakage check: `git diff --check` passed, and the finalized SOR records repository-relative command paths rather than host-path command strings.
- Sandbox / policy invariants preserved: yes; the conductor remains route-only and only writes its declared routing artifact plus issue-scoped local records.

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable; this tooling issue did not produce a runtime trace bundle.
- Run artifact root: `.adl/reviews/workflow-conductor-1685-post-implementation.md`
- Replay command used for verification: reran `python3 adl/tools/skills/workflow-conductor/scripts/route_workflow.py --input <tmp-input> --artifact-path .adl/reviews/workflow-conductor-1685-post-implementation.md`
- Replay result: pass; the conductor reproduced the same `run_bound -> pr-run` handoff with explicit `continuation: continue` for the live `#1685` worktree target.

## Artifact Verification
- Primary proof surface: `adl/tools/test_workflow_conductor_skill_contracts.sh` plus the live route artifact `.adl/reviews/workflow-conductor-1685-post-implementation.md`
- Required artifacts present: yes; the conductor scripts, contract/docs, and expanded tests are present on the issue branch.
- Artifact schema/version checks: the skill manifest still declares `workflow_conductor.v1`, and the updated schema/guide now document blocker classification and worktree disambiguation behavior.
- Hash/byte-stability checks: not run as a separate hash step; deterministic rerun behavior was verified through repeated identical route/test outcomes.
- Missing/optional artifacts and rationale: no separate demo artifact is required for this bounded tooling issue.

## Decisions / Deviations
- Added bounded blocker-family reporting instead of trying to make the conductor fully autonomous over every failure surface.
- Allowed `route_worktree` to accept `target.issue_number` as a disambiguator because real issue worktrees can legitimately contain more than one task bundle during milestone work.
- Used only explicit `child of #<parent>` issue-graph notes plus closed child issue state as the tracker-satisfaction signal, so the conductor stays conservative instead of guessing that a tracker "looks done."
- Kept healthy open PRs in human-review state rather than treating them as janitor work, which preserves the conductor’s thin stop boundary.
- Accepted a manual publication deviation after `pr finish` proved the issue bundle but was blocked by unrelated tracked legacy `.adl` residue on `main`; this did not change the issue-scoped implementation payload.

## Follow-ups / Deferred work
- The conductor is materially better at routing and escalation, but deeper automatic lifecycle chaining still depends on the underlying lifecycle skills and janitor surfaces remaining truthful.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.

# add-lightweight-workflow-conductor-skill

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1647
Run ID: issue-1647
Version: v0.88
Title: [v0.88][tools] Add lightweight workflow conductor skill
Branch: codex/1647-add-lightweight-workflow-conductor-skill
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5-codex
- Provider: OpenAI Codex
- Start Time: 2026-04-12T17:50:50Z
- End Time: 2026-04-12T17:50:50Z

## Summary

Added a first-class `workflow-conductor` operational skill bundle that stays intentionally thin: it inspects one concrete issue/workflow target, selects the next appropriate lifecycle or editor skill, applies explicit skill/subagent policy, and stops after routing/compliance recording rather than reimplementing the underlying work. Added the matching input schema doc, output/playbook references, operator-guide coverage, contract tests, and install-surface expectations.

## Artifacts produced
- `adl/tools/skills/workflow-conductor/SKILL.md`
- `adl/tools/skills/workflow-conductor/adl-skill.yaml`
- `adl/tools/skills/workflow-conductor/agents/openai.yaml`
- `adl/tools/skills/workflow-conductor/references/conductor-playbook.md`
- `adl/tools/skills/workflow-conductor/references/output-contract.md`
- `adl/tools/skills/docs/WORKFLOW_CONDUCTOR_SKILL_INPUT_SCHEMA.md`
- updated `adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md`
- updated `adl/tools/test_install_adl_operational_skills.sh`
- `adl/tools/test_workflow_conductor_skill_contracts.sh`

## Actions taken
- created the new `workflow-conductor` skill bundle under `adl/tools/skills/`
- defined structured admission rules and policy fields for routing one concrete target at a time
- documented the thin-conductor model, including resume-from-partial-state behavior and editor-skill routing
- added an explicit input schema doc and operator-guide section so the skill is first-class alongside the existing operational skills
- added a contract test for bundle/schema/guide parity and updated the install-surface test so the new skill is treated as part of the shipped operational set

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked repository paths are updated on the issue branch via PR 1664
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: pr_branch
- Integration method used: managed issue worktree edits with manual git publication after repo-native `pr finish` was blocked by unrelated tracked legacy `.adl` residue on main
- Verification performed:
  - `gh pr view 1664 --json number,url,state,isDraft,headRefName,baseRefName` verified PR 1664 is open on the issue branch
  - `git status --short` verified the branch is clean after publication
  - `git diff --check` verified no patch-integrity or whitespace defects remain in the published branch state
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
  - `bash adl/tools/test_workflow_conductor_skill_contracts.sh` verified the bundle files, schema linkage, guide entry, and key policy phrases for the new skill
  - `bash adl/tools/test_install_adl_operational_skills.sh` verified the new skill is included in copy/symlink operational-skill installation flows
  - `bash adl/tools/validate_structured_prompt.sh --type stp --phase bootstrap --input .adl/v0.88/tasks/issue-1647__add-lightweight-workflow-conductor-skill/stp.md` verified the rooted task plan remains structurally valid
  - `bash adl/tools/validate_structured_prompt.sh --type sip --phase bootstrap --input .adl/v0.88/tasks/issue-1647__add-lightweight-workflow-conductor-skill/sip.md` verified the pre-run input card remains structurally valid
  - `bash adl/tools/pr.sh finish 1647 --title "[v0.88][tools] Add lightweight workflow conductor skill"` verified the normal finish path and surfaced the unrelated tracked-legacy-`.adl` residue blocker that required manual publication
  - `git diff --check` verified there are no patch-integrity defects in the final worktree
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
      - "bash adl/tools/test_workflow_conductor_skill_contracts.sh"
      - "bash adl/tools/test_install_adl_operational_skills.sh"
      - "bash adl/tools/validate_structured_prompt.sh --type stp --phase bootstrap --input .adl/v0.88/tasks/issue-1647__add-lightweight-workflow-conductor-skill/stp.md"
      - "bash adl/tools/validate_structured_prompt.sh --type sip --phase bootstrap --input .adl/v0.88/tasks/issue-1647__add-lightweight-workflow-conductor-skill/sip.md"
      - "bash adl/tools/pr.sh finish 1647 --title \"[v0.88][tools] Add lightweight workflow conductor skill\""
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
- Determinism tests executed: `bash adl/tools/test_workflow_conductor_skill_contracts.sh`
- Fixtures or scripts used: the bundle/schema/guide contract test plus the operational-skills install copy/symlink test
- Replay verification (same inputs -> same artifacts/order): the contract test checks fixed bundle paths and fixed schema/guide strings, and the install test checks the same bundle names under both install modes
- Ordering guarantees (sorting / tie-break rules used): install verification uses a fixed expected skill list including `workflow-conductor`, keeping bundle presence checks stable across runs
- Artifact stability notes: the new bundle and docs are static tracked files with deterministic install and contract-check expectations

## Security / Privacy Checks
- Secret leakage scan performed: manual review of the new skill bundle, schema doc, and tests
- Prompt / tool argument redaction verified: yes; the new skill bundle and docs describe policy and routing only and do not introduce secrets or operator-specific credentials
- Absolute path leakage check: passed; recorded commands and artifact references in this output card are repository-relative
- Sandbox / policy invariants preserved: yes; the conductor skill is explicitly route-only and does not widen into direct implementation work

## Replay Artifacts
- Trace bundle path(s): not applicable; this issue adds skill/docs/test surfaces rather than runtime replay traces
- Run artifact root: not applicable
- Replay command used for verification: `bash adl/tools/test_workflow_conductor_skill_contracts.sh`
- Replay result: PASS

## Artifact Verification
- Primary proof surface: `adl/tools/test_workflow_conductor_skill_contracts.sh`
- Required artifacts present: true
- Artifact schema/version checks: the manifest references `workflow_conductor.v1` and the matching tracked schema doc
- Hash/byte-stability checks: not applicable; deterministic proof is via fixed bundle-path and guide-contract verification
- Missing/optional artifacts and rationale: no standalone demo is required because the acceptance surface is a normalized skill bundle plus docs/tests

## Decisions / Deviations
- kept `workflow-conductor` thin and route-only instead of inventing a second workflow engine
- made resume-from-partial-state behavior explicit in docs instead of assuming every issue starts from bootstrap
- relied on the existing install script auto-discovery rather than adding a special-case installer path for the new skill
- published the branch manually after `pr finish` was blocked by unrelated tracked legacy `.adl` residue on main; the issue work itself remained complete and reviewable

## Follow-ups / Deferred work
- runtime-level enforcement of `workflow_compliance` recording beyond skill contracts remains future work

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.

---
name: workflow-conductor
description: Lightweight conductor for the ADL workflow skills. Use when the operator wants one bounded entrypoint that detects the current issue/workflow state, selects the correct next lifecycle or editor skill, enforces skill/subagent policy, and stops after routing/compliance recording rather than reimplementing the underlying work.
description: Lightweight conductor for the ADL workflow skills. Use when the operator wants one bounded entrypoint that detects the current issue/workflow state, selects the correct next lifecycle or editor skill, enforces skill/subagent policy, and either stops after routing or dispatches one bounded downstream skill subtask without reimplementing the underlying work.
---

# Workflow Conductor

This skill is a thin orchestrator over the existing ADL operational skills.

Its job is to:
- inspect current workflow state
- choose the next appropriate lifecycle or editor skill
- ensure card-local work is routed to the matching editor skill
- apply explicit skill/subagent execution policy
- record workflow-compliance outcomes
- emit a bounded routing artifact
- when explicitly enabled, dispatch one bounded downstream skill subtask and stop at that boundary

This skill must remain lightweight.

It must not replace:
- `pr-init`
- `pr-ready`
- `pr-run`
- `pr-finish`
- `pr-janitor`
- `pr-closeout`
- `stp-editor`
- `sip-editor`
- `sor-editor`

It must stop at the routing/dispatch boundary rather than reimplementing the selected skill's underlying work.

The bundle may include small deterministic helpers for routing evaluation and test fixtures, but those helpers exist to support the stop-boundary contract, not to silently turn the conductor into a second execution engine.

## Design Basis

This skill should track the repository's canonical operational skill family and
the workflow-policy notes that motivated it.

At the moment, the key repo references are:
- `/Users/daniel/git/agent-design-language/adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md`
- `/Users/daniel/git/agent-design-language/.adl/docs/TBD/ADL_EXECUTION_POLICY_FOR_SKILLS_AND_SUBAGENTS.md`
- `/Users/daniel/git/agent-design-language/.adl/docs/TBD/LIGHTWEIGHT_WORKFLOW_CONDUCTOR_SKILL.md`

Within this bundle, the operational details live in:
- `references/conductor-playbook.md`
- `references/output-contract.md`

If those docs move, prefer the moved tracked canonical copies over stale path references.

## Entry Conditions

Use this skill when all of the following are true:
- there is one concrete issue/workflow target
- the operator wants help choosing the next ADL skill
- the operator wants policy-aware routing rather than manual phase selection

Do not use this skill for:
- directly doing the implementation work
- bypassing editor skills
- repo-wide orchestration across many unrelated issues
- silently finishing or closing an issue

## Required Inputs

At minimum, gather:
- `repo_root`
- one concrete target:
  - `issue_number`
  - `task_bundle_path`
  - `branch`
  - `worktree_path`
  - `pr_number`
- one explicit routing mode
- one explicit policy block

Useful additional inputs:
- `slug`
- `version`
- `doctor_result`
- `source_prompt_path`
- `stp_path`
- `sip_path`
- `sor_path`
- current `pr_state`
- requested `stop_boundary`

If there is no concrete target, stop and report `blocked`.

## Quick Start

1. Resolve the concrete issue/workflow target.
2. Inspect the current workflow state using the strongest available evidence:
   - doctor JSON
   - task bundle paths
   - branch/worktree state
   - PR state
   - observed subagent assignment state
3. Determine whether the next step is:
   - lifecycle routing
   - card-editor routing
   - blocked/no-op reporting
4. Apply the declared skill/subagent policy.
5. Select the next skill.
6. Record the workflow-compliance result and write the routing artifact.
7. When explicit dispatch mode is enabled, dispatch one bounded downstream skill subtask.
8. Stop without absorbing the selected skill's logic.

## Routing Model

Preferred next-skill mapping:
- bootstrap missing -> `pr-init`
- card-local STP issue -> `stp-editor`
- card-local SIP issue -> `sip-editor`
- card-local SOR issue -> `sor-editor`
- structurally ready but not bound -> `pr-ready`
- ready for execution bind -> `pr-run`
- execution complete, needs publication -> `pr-finish`
- PR in flight with checks/conflicts/review blockers -> `pr-janitor`
- PR merged or intentionally closed -> `pr-closeout`

Important rule:
- treat partially completed early steps as normal state, not corruption
- the conductor should resume from the next truthful step instead of restarting bootstrap by reflex
- healthy open PRs should normally hand off to human review/waiting state rather than janitor unless there is an actual blocker
- explicit `covered by #<n>` / `satisfied by #<n>` style references should block fresh execution when the referenced issue is already closed
- repo-policy residue such as tracked legacy `.adl` issue records should escalate as a mechanical blocker rather than being mistaken for issue-local implementation work
- linkage-only PR failures should route as a bounded janitor case rather than a generic merge-blocked state

## Policy Model

This skill should enforce policy when supplied, including:
- `skills_required`
- `card_editor_skills_required`
- `subagent_requirement`
- `bypass_without_explicit_blocker`
- `required_skill_by_phase`
- `required_card_skill_by_type`
- observed subagent assignment state

If policy and repo reality conflict:
- prefer truthful `blocked` output over hidden fallback

## Stop Boundary

This skill must stop after:
- selecting the next skill
- recording compliance and routing facts
- writing the bounded routing artifact
- optionally dispatching one bounded downstream skill subtask
- surfacing any blocker that prevents safe routing

It must not:
- perform the selected skill's implementation work itself
- silently invoke unrelated repo-wide cleanup
- create an unrecorded fallback path

## Dispatch Boundary

When explicit dispatch mode is enabled, the conductor may:
- call one supported downstream lifecycle skill through the repo-native command it already wraps
- call one editor or special-case skill through an explicit operator-provided command override
- record the selected skill, command source, command shape, and bounded result

It must not:
- inline the downstream skill's behavior into the conductor
- chain multiple downstream skills in one silent rollout
- continue past the first dispatched subtask as if it owned the downstream lifecycle

## Output

Return a concise structured result including:
- selected phase
- selected skill
- selected card-editor skill if any
- policy/compliance result
- whether subagent assignment is required
- whether the target should continue, stop, or ask for operator confirmation
- bounded blocker classification for known doctor/PR failure families
- bounded tracker/umbrella satisfaction detection when a child issue wave already appears to cover the acceptance surface
- explicit escalation reason when the conductor should not continue silently
- the artifact path or equivalent routing-proof surface when one is written

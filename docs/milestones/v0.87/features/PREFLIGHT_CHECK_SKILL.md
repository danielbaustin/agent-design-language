

---
name: preflight-check
description: Verify issue, task-bundle, worktree, branch, and card readiness before implementation begins. Use when starting issue work, when execution readiness is uncertain, or when the workflow state may be broken or incomplete.
---

# Preflight Check

## Purpose

This skill verifies that an ADL issue is actually ready for execution before implementation work begins.

It should be treated as a **bounded operational skill**:
- inspect the current workflow state
- identify missing or broken prerequisites
- make only the smallest safe repairs when clearly allowed
- emit a structured readiness result
- stop before broad repository surgery or issue redesign

This is not a general repo-repair skill.
It is a workflow readiness skill.

## Skill Class

This is a **procedural execution skill**.

Default mode:
- `auto-apply` for very small, clearly bounded readiness repairs
- otherwise `findings-only`

## Invocation

### Trigger description

Use this skill when:
- a new issue/worktree/task-bundle session is starting
- the issue process may be partially bootstrapped or inconsistent
- cards, STP, SIP, or output surfaces may be missing or malformed
- branch/worktree state may not match the intended issue
- the user asks whether a task is ready to begin

### Entry conditions

This skill is appropriate when there is a concrete issue or task context to inspect, such as:
- an issue number
- an issue-specific task bundle
- a known branch/worktree
- a known input card / STP / SIP surface

If there is no concrete workflow target, stop and request the missing target.

### Required inputs

At minimum, identify or infer:
- target issue or task id
- expected branch or worktree context if available
- expected task-bundle location if available

### Optional inputs

Useful additional inputs:
- paired issue body path
- paired input card path
- paired STP path
- paired SIP path
- paired SOR path
- expected PR state
- expected demo/proof requirement state

### Conflicting skills

Potentially overlapping skills:
- `issue-bootstrap`
- `card-review`
- `pr-janitor`
- `worktree-hygiene`

### Preferred-over rules

- Prefer `preflight-check` over broader review skills when the immediate question is readiness.
- Prefer `issue-bootstrap` when the task bundle does not exist at all and full bootstrap is required.
- Prefer `card-review` when the workflow exists but content quality/truthfulness is the main question.
- Prefer `worktree-hygiene` when the main problem is repository/worktree cleanup rather than issue readiness.

### Auto-run policy

Allow small bounded repairs only when they are mechanical and low-risk, for example:
- filling in clearly missing branch traceability where it is unambiguous
- fixing obviously broken issue/task-bundle path references when the intended target is clear
- normalizing trivial readiness metadata drift

Do not auto-apply if the repair would:
- change milestone scope
- rewrite issue intent
- invent missing semantics
- broadly edit multiple workflow documents without clear authority

### Human-review threshold

Stop for review when:
- multiple readiness surfaces disagree materially
- issue intent is ambiguous
- branch/worktree ownership is unclear
- fixing readiness would require inventing new content rather than repairing known structure

### Failure-to-admit behavior

If the skill cannot determine the target issue/task context with confidence, emit a readiness result of `blocked` and identify the missing prerequisites.

## What It Should Check

At minimum, inspect the following where applicable:

### 1. Issue / task-bundle targeting

Verify:
- target issue id is known
- task-bundle directory exists if expected
- local files point at the same issue and slug
- branch/worktree naming matches the target issue

### 2. Core workflow surfaces

Verify the presence and basic integrity of:
- issue body
- input card
- STP
- SIP
- SOR/output record target

Where relevant, verify the surfaces point to each other correctly.

### 3. Branch and worktree readiness

Verify:
- not on `main` when issue implementation work is expected to happen elsewhere
- intended branch exists and matches the issue slug if already created
- worktree state is not obviously broken
- repository state is clean enough for issue work to begin

### 4. Traceability surfaces

Verify:
- issue id is consistent across issue body, input card, STP, SIP
- branch traceability is consistent
- source issue prompt paths are coherent
- paired output surface exists or is correctly referenced

### 5. Execution readiness

Verify:
- required outcome type is clear enough to begin
- demo/proof requirement state is explicit where needed
- no obviously blocking placeholder/bootstrap text remains in critical execution surfaces

## Allowed Actions

This skill may:
- inspect files and repo state
- compare issue/task-bundle surfaces for consistency
- propose bounded repairs
- apply very small structural readiness repairs when clearly safe
- emit a structured readiness status

This skill must not:
- invent missing issue semantics
- silently rewrite major workflow documents
- perform broad repo cleanup beyond readiness scope
- bootstrap an entirely missing task bundle unless explicitly delegated
- continue into implementation work

## Process Steps

1. Identify the target issue/task context.
2. Locate the relevant workflow surfaces.
3. Check issue/task/branch/worktree consistency.
4. Check core readiness surfaces for presence and obvious structural integrity.
5. Distinguish:
   - ready
   - ready with small bounded repairs
   - blocked
6. Apply only clearly safe mechanical fixes if permitted.
7. Emit a structured readiness result.
8. Stop at the handoff boundary.

## Outputs

Where practical, emit a structured output with fields such as:

- `status` (`ready`, `ready_with_repairs`, `blocked`)
- `findings`
- `actions_taken`
- `actions_recommended`
- `files_touched`
- `validation_performed`
- `handoff_state`
- `follow_up_required`

Suggested readiness-specific fields:

- `target_issue`
- `expected_branch`
- `actual_branch`
- `task_bundle_present`
- `core_surfaces_present`
- `blocking_gaps`
- `safe_repairs_applied`

## Non-goals

This skill is not responsible for:
- full content review of milestone truth
- deep doc reconciliation
- PR cleanup after implementation
- worktree garbage collection across the repo
- implementation of the issue itself

## Failure Modes

Common failure modes:
- wrong issue/worktree targeted
- mixed issue ids across STP/SIP/cards
- bootstrap placeholder text still present in execution surfaces
- branch traceability drift
- output record path missing or wrong
- repo appears clean but readiness semantics are actually incomplete

## Escalation / Handoff

Handoff targets may include:
- `issue-bootstrap` when structure is missing entirely
- `card-review` when content quality/truthfulness is the main problem
- `worktree-hygiene` when the repo/worktree state is the real blocker
- a human reviewer when ambiguity is material

## Proof Surface

Useful proof surfaces for this skill include:
- one example ready issue/task-bundle state
- one blocked issue/task-bundle state
- one example structured readiness output
- one regression fixture for malformed workflow-state combinations

## Version / Owner

- Version: `v0.1`
- Owner: `ADL / Daniel Austin`

## Compatibility Note

This skill is intentionally compatible with simple Codex-style discovery:
- `name`
- `description`

But the real operational behavior is defined here as an ADL skill contract.

That means:
- simple systems can still discover the skill
- ADL can apply stronger admission, execution-mode, and handoff rules

## Notes

Keep this skill lean.

If it grows large, move:
- detailed repair procedures
- example fixtures
- reference checklists

into adjacent repo resources rather than bloating the main skill contract.
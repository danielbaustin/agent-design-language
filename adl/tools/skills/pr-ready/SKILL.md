---
name: pr-ready
description: Diagnose PR workflow readiness and drift for an issue, task bundle, branch, or worktree. Use when the user wants a doctor-style readiness result, wants to know whether workflow state is broken or incomplete, or wants small bounded mechanical repairs applied automatically without widening into implementation work.
---

# PR Ready

This skill owns the cross-cutting diagnostic and bounded-repair surface for the PR workflow.

Its job is to:
- inspect workflow readiness and drift
- classify the target's doctor/readiness result as `ready`, `ready_with_repairs`, or `blocked`
- apply only very small, clearly safe mechanical repairs when allowed
- emit a structured readiness result
- stop before qualitative review, implementation, or broad repository repair

This is a procedural execution skill.

## Design Basis

This skill should track the repository's canonical PR tooling docs.

At the moment, the canonical repo docs are:
- `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`
- `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md`
- `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PREFLIGHT_CHECK_SKILL.md`

Within this skill bundle, the operational details live in:
- `references/ready-playbook.md`
- `references/output-contract.md`

If those docs move, prefer the moved tracked canonical copies over stale path references. Do not silently invent a new doctor/readiness model from memory when the repo docs have changed.

## Current Compatibility Model

The intended workflow model treats `doctor` as the canonical automation surface, with `ready` and `preflight` as compatibility aliases.

Current repo truth:
1. `doctor --json` is the canonical structured readiness surface
2. compatibility surfaces may still include `pr ready` and `pr preflight`
3. doctor reports lifecycle-aware readiness:
   - pre-run issues may be ready before a worktree exists
   - run-bound issues must still validate the bound worktree/task context
4. preflight-compatible checks still report milestone/open-PR blocking state
5. the skill may combine doctor output, compatibility aliases, and direct inspection to produce one readiness result

Do not collapse preflight into the main status. It is a separate gate input to the doctor/readiness result.

## Entry Conditions

Run this skill when all of the following are true:
- the user wants to know whether workflow state is ready, blocked, or drifting
- there is a concrete target to inspect
- the task should stop after diagnosis and any allowed bounded repairs

Concrete targets may include:
- an issue number
- a task-bundle path
- a branch
- a worktree path
- a known source prompt / STP / SIP / SOR surface

Do not use this skill for:
- full `pr init` when the structure is entirely missing
- qualitative STP/SIP review
- implementation work
- broad repo cleanup beyond the immediate readiness problem

## Required Inputs

At minimum, gather:
- repository root
- one concrete target:
  - issue number
  - task_bundle_path
  - branch
  - worktree_path

Useful additional inputs:
- slug
- version
- source_prompt_path
- stp_path
- sip_path
- sor_path
- expected_pr_state
- repair_mode

If there is no concrete target, stop and report `blocked` with the missing target information.

## Quick Start

1. Resolve the concrete target context.
2. Prefer the canonical doctor path first:
   - `adl/tools/pr.sh doctor --json`
   - `adl pr doctor --json`
3. Use compatibility aliases only when the canonical doctor surface is unavailable:
   - `adl/tools/pr.sh ready`
   - `adl pr ready`
   - `adl/tools/pr.sh preflight`
   - `adl pr preflight`
4. Use direct inspection only as a last resort.
5. Inspect the relevant workflow surfaces:
   - issue/task identity
   - source prompt, STP, SIP, SOR
   - branch and worktree state
   - milestone/open-PR preflight state if relevant
6. Distinguish:
   - `ready`
   - `ready_with_repairs`
   - `blocked`
7. Report preflight or scheduling gates separately from execution readiness when the issue structure itself is sound.
8. Treat missing worktree before `pr-run` as expected pre-run state when the root bundle is authored and execution has not yet been bound.
9. Apply only clearly safe bounded repairs if permitted.
10. Emit a structured readiness result and stop.

## Workflow

### 1. Resolve Target Context

Identify the target issue/task context using the most concrete available input.

Prefer this order:
1. explicit issue number plus slug/version if provided
2. task-bundle path
3. worktree path
4. branch

If multiple surfaces disagree materially on issue identity, report `blocked` rather than guessing.

### 2. Inspect Readiness Surfaces

At minimum, inspect where applicable:
- source issue prompt
- root STP
- root SIP
- root SOR target
- worktree-local STP/SIP/SOR if execution has already been bound
- branch naming and branch-to-issue traceability
- worktree presence and worktree branch match
- milestone/open-PR blocking state when a preflight-style check is relevant

### 3. Validate Core Readiness

Validation should confirm:
- issue/task identity is coherent across the inspected surfaces
- critical surfaces exist where they are expected
- execution surfaces do not obviously contain bootstrap placeholders when execution readiness is being checked
- branch/worktree state matches the intended issue when execution context exists
- milestone/open-PR wave or scheduling gates are reported truthfully when preflight-style gating applies

Execution-readiness classification must answer:
- is this issue structurally ready to execute once scheduled?

Preflight classification must answer separately:
- may this issue begin right now under the current wave/open-PR policy?

Do not collapse those two questions into one unless the preflight condition also proves the issue itself is structurally unready.

### 4. Apply Only Safe Bounded Repairs

Allowed bounded repairs include only small mechanical actions such as:
- correcting an unambiguous local path/reference drift
- normalizing trivial readiness metadata drift
- reporting a deterministic canonical path when the wrong one was referenced

Do not auto-apply if the repair would:
- invent missing semantics
- rewrite issue intent
- broadly edit multiple workflow documents
- create bootstrap surfaces that should be delegated to `pr-init`
- create or change implementation state

### 5. Stop Boundary

This skill must stop after diagnosis and any permitted bounded repair.

It must not:
- perform qualitative STP/SIP rewriting
- create a branch or worktree as part of execution
- continue into implementation
- silently expand into repo-wide hygiene work

The normal handoff is to one of:
- `pr-init`
- qualitative card review
- issue-mode `pr run`
- a human reviewer

## Parallelism

This skill is fully automatable and can run in parallel across distinct targets when write sets do not overlap.

Safe parallel examples:
- diagnosing issue A and issue B at the same time
- checking a root bundle target and a different issue's worktree target

Unsafe parallel examples:
- diagnosing and repairing the same issue target from multiple agents
- overlapping doctor and bootstrap execution on the same issue

## Preferred Commands

Canonical machine surface:
- `adl/tools/pr.sh doctor --json`
- `adl pr doctor --json`

Compatibility aliases:
- `adl/tools/pr.sh ready`
- `adl/tools/pr.sh preflight`
- `adl pr ready`
- `adl pr preflight`

Command-order rule:
- prefer `doctor --json` first when the surface exists
- if the shell compatibility surface exists, prefer `adl/tools/pr.sh ready` before falling back to direct inspection
- do not skip to manual inspection merely because a built `adl` binary is absent
- use direct inspection only when the repo-native doctor/readiness/preflight paths are unavailable or fail to produce a usable result

It is acceptable for this skill to combine:
- repo-native doctor/readiness/preflight commands
- direct file inspection
- direct git/worktree inspection

to produce one doctor-style readiness result.

## Output

Return status in a concise structured shape.

When writing an artifact for ADL, use the contract in `references/output-contract.md`.

Default result should make these explicit:
- target issue or target surface
- expected branch/worktree if known
- actual branch/worktree if known
- execution readiness status
- preflight or scheduling status if checked
- blocking gaps
- safe repairs applied
- files touched
- recommended handoff

## Failure Modes

Common failure modes:
- wrong issue/worktree targeted
- mixed issue ids across source prompt/STP/SIP/cards
- missing source prompt or task-bundle surfaces
- worktree missing or on the wrong branch
- branch traceability drift
- placeholder/bootstrap text still present in execution-critical surfaces
- open milestone PR wave blocking execution

If the target cannot be determined confidently, report `blocked`.

## Boundaries

This skill may:
- inspect repo state
- inspect workflow documents and compatibility cards
- inspect branch/worktree state
- run bounded readiness and preflight commands
- apply very small mechanical repairs when clearly safe
- emit a structured readiness result

This skill must not:
- invent missing workflow semantics
- silently rewrite major workflow documents
- perform full bootstrap when the structure is missing entirely
- implement the issue
- claim readiness if blocking gaps remain

## ADL Compatibility

This skill is Codex-compatible through frontmatter discovery.

For stricter ADL execution, also use:
- `adl-skill.yaml`
- `references/ready-playbook.md`
- `references/output-contract.md`

## Resources

- Playbook: `references/ready-playbook.md`
- Output contract: `references/output-contract.md`
- PR tooling feature doc: `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`
- PR tooling architecture doc: `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md`

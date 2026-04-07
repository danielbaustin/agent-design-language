---
name: pr-janitor
description: Watch a PR after draft creation, diagnose failed checks or merge conflicts, recommend or apply bounded fixes when clearly justified, and stop before unreviewed scope expansion. Use when a PR is in flight and the user wants help monitoring CI, conflicts, review state, or merge-readiness.
---

# PR Janitor

This skill owns the PR-in-flight monitoring and bounded intervention surface.

Its job is to:
- inspect a PR's current progress
- detect failed checks, merge conflicts, or blocked review state
- distinguish actionable fixes from issues requiring human judgment
- apply only bounded fixes when clearly justified and authorized
- emit a structured PR progress result
- stop before unreviewed scope expansion or silent closeout

This is a judgment-heavy operational skill.

Prefer a stronger model for this skill. In this environment, default to `gpt-5.4` rather than a mini model when available because the work often requires synthesizing CI state, PR context, and remediation tradeoffs safely.

## Design Basis

This skill should track the repository's canonical PR tooling docs.

At the moment, the canonical repo docs are:
- `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`
- `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md`
- `/Users/daniel/git/agent-design-language/.adl/docs/v0.87planning/promoted/PR_TOOLING_SKILLS.md`

Within this skill bundle, the operational details live in:
- `references/janitor-playbook.md`
- `references/output-contract.md`

If those docs move, prefer the moved tracked canonical copies over stale path references. Do not silently invent a new PR closeout model from memory when the repo docs have changed.

## Current Compatibility Model

The intended workflow model treats review/closeout as Step 4 and keeps monitoring/repair activity process-driven rather than fully collapsed into a single command.

Current repo truth:
1. a PR may already exist after issue-mode `pr run`
2. `finish` may still exist during compatibility and closeout transition
3. CI failures, conflicts, and review findings may require additional diagnosis and bounded remediation
4. this skill should monitor and intervene without pretending merge/closure is automatic

This skill should not teach silent PR completion. Human review and truthful closeout remain part of the workflow.

## Entry Conditions

Run this skill when all of the following are true:
- there is a concrete PR or branch-under-review target
- the user wants to monitor progress, diagnose failures, or react to merge blockers
- the task should stop after diagnosis or bounded remediation

Concrete targets may include:
- a PR number or URL
- a branch associated with an open PR
- a known issue whose PR is in flight

Do not use this skill for:
- initial issue bootstrap
- qualitative STP/SIP authoring before execution
- implementation from scratch when no PR exists yet
- silent merge or closeout without explicit user direction

## Required Inputs

At minimum, gather:
- repository root
- one concrete PR-progress target:
  - `pr_number`
  - `pr_url`
  - `branch`

Useful additional inputs:
- `issue_number`
- `expected_checks`
- `expected_pr_state`
- `repair_mode`
- `review_standard`

If there is no concrete PR-progress target, stop and report `blocked`.

## Quick Start

1. Resolve the concrete PR-progress target.
2. Inspect:
   - PR state
   - check status
   - mergeability / conflict state
   - review findings or requested changes if available
3. Distinguish:
   - `healthy`
   - `action_required`
   - `blocked`
4. Apply only bounded fixes if policy and evidence support them.
5. Emit a structured progress result and stop.

## Workflow

### 1. Resolve PR Target

Identify the target using the most concrete available input.

Prefer this order:
1. explicit PR number
2. explicit PR URL
3. explicit branch
4. explicit issue number if it maps unambiguously to an open PR

If multiple PRs or branches match ambiguously, report `blocked`.

### 2. Inspect PR Progress

At minimum, inspect where applicable:
- PR open/draft/ready state
- mergeability or conflict state
- CI / check-run outcomes
- review status and blocking review findings
- whether the branch is behind main or otherwise needs refresh

### 3. Classify the Problem

Distinguish among:
- CI failure or flaky check risk
- merge conflict or branch drift
- requested review changes
- blocked merge readiness with no obvious automated fix
- no blocker currently present

### 4. Apply Only Bounded Fixes

Allowed bounded interventions may include:
- rerunning or re-verifying the smallest relevant local checks
- preparing a focused fix for a clear CI failure
- refreshing branch state or conflict remediation when the intended resolution is unambiguous
- updating truthful PR progress notes or result output

Do not auto-apply if the intervention would:
- widen issue scope materially
- rewrite large areas without a clear blocker-driven reason
- override substantive reviewer judgment silently
- merge or close the PR without explicit user direction

### 5. Stop Boundary

This skill must stop after diagnosis and any permitted bounded remediation.

It must not:
- silently merge the PR
- silently close the issue
- convert broad new implementation work into “janitoring”
- ignore active review findings to optimize for green checks

Normal handoff targets include:
- `repo-code-review`
- `pr-ready`
- a human reviewer
- a focused implementation/fix task

## Parallelism

This skill is automatable, but use more caution than with bootstrap or doctor.

Safe parallel examples:
- monitor two unrelated PRs in separate branches
- inspect one PR while another agent works on a different issue

Unsafe parallel examples:
- two janitor runs applying fixes to the same PR branch
- janitoring a PR while another agent is rebasing or resolving conflicts on that same branch

## Preferred Commands

Prefer repo-native and PR-aware surfaces such as:
- GitHub or `gh` PR metadata and checks inspection
- local branch and mergeability inspection
- bounded local test commands relevant to the failing check
- existing review and closeout workflow artifacts such as SOR when helpful

Use the smallest relevant inspection and validation surface first. Do not turn routine monitoring into a full repo review unless the PR signal actually warrants it.

## Output

Return status in a concise structured shape.

When writing an artifact for ADL, use the contract in `references/output-contract.md`.

Default result should make these explicit:
- target PR
- branch
- current PR state
- checks summary
- conflict status
- actions taken
- actions recommended
- whether human review is still required

## Failure Modes

Common failure modes:
- wrong PR targeted
- stale or incomplete check interpretation
- merge conflict diagnosis without clear safe resolution
- silently treating substantive review feedback as mechanical
- over-fixing beyond the actual blocker

If the target cannot be determined confidently, report `blocked`.

## Boundaries

This skill may:
- inspect PR/check/review state
- inspect local branch state
- run bounded local validation commands
- apply small blocker-driven fixes when clearly justified
- emit a structured PR progress result

This skill must not:
- silently merge or close
- overrule substantive reviewer feedback without surfacing it
- expand into unrelated product work
- claim the PR is healthy if blocking checks or conflicts remain

## ADL Compatibility

This skill is Codex-compatible through frontmatter discovery.

For stricter ADL execution, also use:
- `adl-skill.yaml`
- `references/janitor-playbook.md`
- `references/output-contract.md`

## Resources

- Playbook: `references/janitor-playbook.md`
- Output contract: `references/output-contract.md`
- PR tooling feature doc: `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`
- PR tooling architecture doc: `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md`

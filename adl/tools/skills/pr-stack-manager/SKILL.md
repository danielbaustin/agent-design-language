---
name: pr-stack-manager
description: Analyze and manage PR stack topology, dependency order, and base alignment for ADL issue workflows in a bounded, reviewable way.
---

# PR Stack Manager

This skill helps operators and automation reason about stacked PR execution safely.

Its job is to:

- inspect current PR/dependency topology for ADL issue workflows,
- detect base drift and merge-order risks,
- surface concrete recommendations for restack/rebase workflows only within the bounded target,
- emit machine-readable state with optional, bounded mutation actions when explicitly requested.

It is not a replacement for other lifecycle skills. It supports stack governance
and truth recording around `pr-init`, `pr-run`, `pr-ready`, `pr-janitor`,
and `pr-finish`.

## Entry Conditions

Use this skill when all of the following are true:

- the target is an issue or bounded worktree with a concrete stack-relevant context,
- the operator needs a deterministic stack assessment or bounded stack action plan,
- the requested action is limited to PR ordering/dependency truth and bounded remediation.

Use `pr-stack-manager` when:

- dependent branches are expected but dependency edges are uncertain,
- PR base branches drift from expected parent branches,
- topological merge order appears to violate dependency intent,
- worktree/branch/PR metadata in task artifacts and local git are out of sync.

Do not use this skill for:

- general workflow arbitration that should stay with lifecycle routing,
- open-ended branch rewrites without explicit stack scope,
- unrelated repo-wide PR cleanup not bound to an issue/worktree,
- speculative dependency graph construction without evidence.

## Required Inputs

At minimum, gather:

- `repo_root`
- one concrete target:
  - `issue_number`
  - `task_bundle_path`
  - `branch`
  - `worktree_path`

Useful additional inputs:

- `slug`
- `version`
- explicit PR/task surfaces (`source_prompt_path`, `stp_path`, `sip_path`, `sor_path`)
- `policy` (`mode`, `base_alignment`, `allow_mutation`, `dry_run`, `max_stack_depth`)

## Workflow

### 1. Resolve and Normalize the Target

Use one primary target mode and bound metadata:

1. `issue`/`task bundle`/`branch`/`worktree`
2. issue artifacts (`source_prompt`, `stp`, `sip`, `sor`)
3. local git branch and worktree topology
4. GitHub PR metadata for stack-relevant issue branches

### 2. Analyze Stack Topology

Surface:

- issue-to-branch binding and expected stack ancestry,
- PR base relationships and whether bases still match expected dependency ordering,
- dependent issue overlap (directly and transitively),
- worktree-only vs open-pr state mismatch,
- unresolved blockers (e.g., open dependency PRs, stale bases, missing proofs).

### 3. Classify Risk and Action

Use four risk levels:

- `info`: low-risk visibility gaps
- `warning`: non-blocking stack friction
- `blocking`: incorrect execution/finish dependency truth
- `error`: analysis or safety precondition failure

Classify each action by:

- deterministic and bounded mutation
- ambiguous/human-review-needed
- safe planning only

### 4. Optional Bounded Mutation

Mutation is allowed only if requested and bounded:

- generate a dry-run plan,
- reorder a documented dependency action sequence,
- emit explicit commands to reopen/rebase/rebase-base where evidence is unambiguous,
- never perform branch surgery without explicit safe mutation policy.

### 5. Emit Contracted Output

Write structured findings and next-step recommendations to a bounded artifact.

## Quick Start

1. Resolve target identity and stack context.
2. Run topology+base analysis.
   - For fixture-backed analysis, use
     `python3 adl/tools/skills/pr-stack-manager/scripts/analyze_pr_stack.py <stack-packet>`.
3. Report `blocking` stack risks first.
4. If `allow_mutation` is enabled and risks are unambiguous, apply bounded actions.
5. Write residual recommendations and handoff state.

## Workflow Boundaries

Allowed writes are mechanical and bounded to the addressed issue surface.

Stop before:

- wide PR strategy changes,
- milestone-level workflow redesign,
- unrelated branch operations outside the target stack,
- merge decisions outside scoped input.

## Output

Use `references/output-contract.md` and emit deterministic structured records that are
machine-readable and reviewable before any merge-facing publication.

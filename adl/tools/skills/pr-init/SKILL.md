---
name: pr-init
description: Initialize a tracked issue for the PR lifecycle. Use when the user wants to create or resolve a GitHub issue, generate the canonical local source issue prompt plus initial STP/SIP/SOR bundle in the correct locations, validate the init surfaces, and stop before any branch or worktree creation.
---

# PR Init

This skill owns the bounded `pr init` phase for the PR tooling workflow.

Its job is to:
- create or locate the GitHub issue
- ensure the canonical local source issue prompt exists
- seed the initial root task bundle surfaces
- validate that the issue is ready for the next lifecycle step
- stop at the mechanical bootstrap boundary before branch creation, worktree creation, or implementation work

This is an execution skill, not a review-only skill.
Keep mechanical work separate from qualitative review.

## Design Basis

This skill should track the repository's canonical PR tooling planning docs.

At the moment, the canonical repo docs are:
- `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`
- `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md`

Within this skill bundle, the operational details live in:
- `references/init-playbook.md`
- `references/output-contract.md`

The canonical caller-facing invocation template lives at:
- `/Users/daniel/git/agent-design-language/docs/templates/PR_INIT_INVOCATION_TEMPLATE.md`

If those docs move, prefer the moved tracked canonical copies over stale path references. Do not silently invent a new workflow model from memory when the repo docs have changed.

## Current Compatibility Model

The intended workflow model treats `pr init` as Step 1.

Current repo truth:
1. new issues are created and bootstrapped with `pr create`
2. existing issues are bootstrapped with `pr init`
3. qualitative STP/SIP review is a separate step
4. `pr run` is the later execution-time binder
5. `doctor` remains the diagnostic and drift-review surface

Skill-model boundary:
- this skill covers only Step 1: `pr init`
- later qualitative STP/SIP editing is a separate skill or human-review phase
- later execution-time branch/worktree creation belongs to the run skill

Treat `create` and `init` as two command shapes for the same bounded bootstrap phase:
- `create` when a new GitHub issue must be created
- `init` when the issue already exists

Do not teach `create` as a later workflow step. Teach qualitative review first, then issue-mode `run` as the binder.

## Entry Conditions

Run this skill when all of the following are true:
- the user wants to create or initialize a tracked issue
- the target repository is known
- the task should stop after issue creation, root bundle generation, and bootstrap validation

Do not use this skill for:
- qualitative rewriting of the STP or SIP
- branch or worktree creation
- implementation work
- PR publication
- post-bootstrap readiness/doctor checks for an existing worktree

## Required Inputs

At minimum, gather:
- repository root
- one of:
  - existing issue number to bootstrap
  - title for a new issue

For new issue creation, prefer:
- title
- slug, if explicitly supplied
- version or milestone scope, if known
- labels matching the repo-standard tracked issue set
- issue body or body file, if the user provides one

For deterministic ADL execution, also prefer an explicit issue-metadata policy:
- how version is chosen
- how labels are chosen or normalized
- whether body content is user-authored or bootstrap-generated

When the caller can provide structured input, prefer the tracked invocation
template in:
- `/Users/daniel/git/agent-design-language/docs/templates/PR_INIT_INVOCATION_TEMPLATE.md`

If no slug is given, derive one from the title using the repo's normal slug rules.

## Quick Start

1. Resolve whether the request is:
   - `bootstrap-existing-issue`
   - `create-and-bootstrap-new-issue`
2. If using structured invocation, start from the canonical tracked template
   rather than writing the payload from scratch.
3. Prefer the Rust-owned path when available.
4. For new issues:
   - create the GitHub issue correctly
   - pass explicit repo-standard labels rather than relying on label inference
   - verify the created issue actually has the expected labels before continuing
   - ensure the canonical local source issue prompt and root bundle exist
5. For existing issues:
   - run the bootstrap/init phase
   - seed the task-bundle `stp.md`
   - seed the initial `sip.md`
   - seed the initial `sor.md`
   - ensure canonical compatibility links exist when the repo expects them
6. Validate the resulting surfaces mechanically.
7. Emit a structured readiness result for qualitative card review and stop.

## Workflow

### 1. Determine Mode

Use one of these modes:

- `create_and_bootstrap`
  - use when the user gives a title or asks to create a new issue
- `bootstrap_existing_issue`
  - use when the user already has an issue number and only wants the bundle/bootstrap step

### 2. Create or Resolve the GitHub Issue

For `create_and_bootstrap`:
- prefer the repository's standard issue-creation path
- require explicit labels for tracked issue creation
- create the GitHub issue with the correct title, labels, version, and body inputs
- verify the created issue carries the requested labels before treating bootstrap as successful
- capture the resulting issue number and URL
- ensure the canonical local source prompt is generated or written in the expected location

For `bootstrap_existing_issue`:
- resolve the issue title and scope using the standard repo flow
- infer or confirm slug/version using the repo's existing rules

### 3. Seed Canonical Bootstrap Surfaces

Ensure the repo's canonical bootstrap surfaces exist in the right places:
- source issue prompt
- root task-bundle `stp.md`
- root task-bundle `sip.md`
- root task-bundle `sor.md`
- any required compatibility links such as canonical `.adl/cards/...` pointers if the workflow still uses them

Prefer the repo's existing templates and control-plane logic over hand-written file generation.

Do not qualitatively rewrite STP or SIP content in this step beyond the mechanical bootstrap required by the repo's standard control-plane behavior.

### 4. Validate and Review the Bootstrap Result

Validation must confirm:
- the GitHub issue exists or was created successfully
- new tracked issues have the expected labels after creation
- the canonical source issue prompt exists
- the expected task-bundle directory exists
- `stp.md`, `sip.md`, and `sor.md` exist in the bundle
- compatibility links exist when the repo expects them
- the bootstrap step did not create a branch or worktree
- the surfaces are mechanically complete and ready for the qualitative card-review step

Review the result for obvious bootstrap defects such as:
- empty or missing prompt surfaces
- missing source prompt linkage
- wrong version/scope placement
- slug/path mismatches
- missing bundle files
- compatibility links missing or pointing at the wrong target

If the compatibility path created obviously placeholder or contradictory bootstrap output that is not ready for the next step, emit a blocked result instead of pretending the issue is ready.

This step may confirm bootstrap completeness, but it must not perform the deeper qualitative review that turns STP/SIP into execution-ready instructions.

### 5. Stop Boundary

This skill must stop after bootstrap creation and validation of the root bundle.

It must not:
- create or switch branches
- create worktrees
- start implementation
- run `pr run`
- run `pr finish`

The immediate handoff is to qualitative card review.
Only after that does issue-mode `run` bind branch/worktree execution context.

## Parallelism

This skill is a good candidate for parallel execution across distinct issues when write targets do not overlap.

Parallel execution is allowed only when each invocation has a disjoint target such as:
- different issue numbers
- different slugs
- different task-bundle directories

Within one `pr init` run, keep the operations serialized.

## Preferred Commands

Prefer repo-native control-plane commands such as:
- `adl/tools/pr.sh create`
- `adl/tools/pr.sh init`
- `adl pr create`
- `adl pr init`

For caller payload shape, prefer the canonical tracked template:
- `/Users/daniel/git/agent-design-language/docs/templates/PR_INIT_INVOCATION_TEMPLATE.md`

Use existing templates, validation helpers, and path logic from the repository. Do not recreate bundle contents manually unless the repo-native path is unavailable and the user explicitly wants a fallback.

## Output

Return findings/status in a concise structured shape.

When writing an artifact for ADL, use the contract in `references/output-contract.md`.

Default success result should make these explicit:
- issue number
- issue URL
- title
- slug
- version/scope
- source prompt path
- bundle directory
- `stp.md` path
- `sip.md` path
- `sor.md` path
- validation status
- next step: qualitative card review
- later-step handoff after review: issue-mode `pr run`

## Failure Modes

Common failure modes:
- `gh` unavailable or unauthenticated
- title/slug resolution fails
- version/scope inference is missing or inconsistent
- source issue prompt cannot be created or found
- templates or contracts are missing
- bundle files were not seeded correctly
- validation fails

If the issue or bundle is only partially created, report exactly which surfaces exist and which do not.

## Boundaries

This skill may:
- inspect repo state
- call the repo's issue creation/bootstrap commands
- write the canonical bootstrap files for the issue
- validate the resulting prompt and bundle surfaces
- emit a readiness decision for the qualitative review step

This skill must not:
- qualitatively rewrite STP or SIP beyond normal bootstrap generation
- implement the issue
- mutate unrelated issues
- create branches or worktrees
- silently skip validation
- claim readiness if required bootstrap surfaces are missing

## ADL Compatibility

This skill is Codex-compatible through frontmatter discovery.

For stricter ADL execution, also use:
- `adl-skill.yaml`
- `references/init-playbook.md`
- `references/output-contract.md`

## Resources

- Playbook: `references/init-playbook.md`
- Output contract: `references/output-contract.md`
- PR tooling feature doc: `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`
- PR tooling architecture doc: `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md`

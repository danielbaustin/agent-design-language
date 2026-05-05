# v0.91 SPP Readiness

## Purpose

This record captures the first structured planning prompt readiness slice for
v0.91. The goal is to test the SPP template on a small number of real work
packages before generating the rest of the milestone wave.

The local SPP files remain workflow records under `.adl/` and are not published
as tracked repository artifacts. This tracked record exists so reviewers can see
what was tested, what was intentionally deferred, and what remains blocked on
the planned SPP editor skill.

## Scope

- Template: `docs/templates/STRUCTURED_PLAN_PROMPT_TEMPLATE.md`.
- Hand-authored sample SPPs: WP-01 through WP-03.
- Local records:
  - `.adl/v0.91/tasks/issue-2735__v0-91-wp-01-docs-design-pass-milestone-docs-planning/spp.md`
  - `.adl/v0.91/tasks/issue-2736__v0-91-wp-02-docs-moral-event-contract/spp.md`
  - `.adl/v0.91/tasks/issue-2737__v0-91-wp-03-tools-moral-event-validation/spp.md`

## Template Contract

The template introduces an SPP as a read-only planning artifact created after
STP, SIP, and SOR records exist and before execution is bound.

It is intentionally compatible with Codex plan mode through a `codex_plan`
frontmatter list. Each plan item carries:

- `step`: one concise execution step.
- `status`: one of `pending`, `in_progress`, or `completed`.

For pre-execution SPPs, implementation steps should remain `pending` until work
actually happens.

## Sample Findings

The first three SPPs are useful enough to validate the shape, but they also show
why the rest of the wave should not be mass-generated without an editor skill:

- WP-01 needs a planning-readiness SPP that avoids claiming all SPPs are done.
- WP-02 needs contract-specific non-claims so moral evidence does not become a
  scoreboard or production moral-agency claim.
- WP-03 needs dependency-aware stop conditions so it consumes WP-02 instead of
  silently redefining the contract.

## Deferred Work

The remaining v0.91 SPPs are intentionally deferred until the SPP editor skill
exists. Follow-on issue #2766 should create a bounded editor skill that can
normalize SPPs the same way STP, SIP, and SOR cards are edited today.

The editor skill should preserve:

- concrete issue dependencies and source references
- Codex-compatible `codex_plan` status values
- truthful pre-execution state
- stop conditions and non-goals
- review hooks
- no implementation or branch-binding claims before execution

## Non-Claims

- This pass does not create all v0.91 SPPs.
- This pass does not mark any v0.91 feature issue as implemented.
- This pass does not bind per-issue branches or worktrees.
- This pass does not replace STP, SIP, or SOR cards.
- This pass does not publish local `.adl/` issue records as tracked repository
  artifacts.

# ADL Agent Guidelines

This file is the repository-local operating contract for coding agents working
in ADL.

It follows the OpenAI `AGENTS.md` pattern of keeping one predictable,
high-signal instruction surface at the repo root, then adapts that pattern to
ADL's real workflow and review discipline.

## Core Principles

These are the four behavioral principles at the center of this file.

1. Think before coding.
   - Understand the goal, the acceptance surface, and the smallest safe change
     before editing.
2. Simplicity first.
   - Prefer the simplest truthful solution over cleverness, abstraction churn,
     or framework theater.
3. Make surgical changes.
   - Change only the files and behavior needed for the issue you are working
     on.
4. Stay goal-driven.
   - Keep work tied to the issue outcome, not to adjacent cleanup or tempting
     side quests.

## Workflow Rules

These rules are mandatory for ADL issue work.

1. Use `workflow-conductor` for every issue and lifecycle stage.
   - Do not bypass the conductor for issue execution, review routing,
     publication, janitor work, or closeout.
2. Edit cards only with editor skills.
   - Use `sip-editor`, `stp-editor`, `spp-editor`, `srp-editor`, `sor-editor`,
     or other issue-card editor skills when card surfaces need normalization.
   - Do not hand-edit cards opportunistically.
3. Always work in a bound worktree on a specific branch.
   - Never do tracked issue work on `main`.
   - Use the repo-native issue-mode `pr run` flow to bind execution context.
4. Always review work with a subagent before opening the PR.
   - Run a bounded review subagent over the changed work product.
   - Fix all actionable findings immediately before publication.
5. Always perform closeout after the issue is closed.
   - Use the normal closeout path so issue truth, cards, artifacts, and GitHub
     state all agree.

## Repository-Specific Working Style

- ADL is deterministic by design. Do not introduce hidden state, undeclared
  side effects, or review-hostile magic.
- Treat model/tool output as governed work, not free authority.
- Keep milestone claims, proof claims, and review claims evidence-bound.
- Prefer repo-relative paths in artifacts and records.
- Do not silently widen issue scope.
- Preserve the canonical card lifecycle: `SIP -> STP -> SPP -> SRP -> SOR`.
  `SRP` is the Structured Review Prompt and review-result surface; `SOR` is the
  truthful execution and integration record.

## Where To Start

For a normal tracked issue:

1. read the source issue prompt and current task bundle
2. route through `workflow-conductor`
3. follow the conductor-selected lifecycle step
4. if the issue is ready for execution binding, use `adl/tools/pr.sh run <issue>`
5. make the bounded change in the issue worktree
6. run the smallest meaningful validation for the touched surface
7. run a pre-PR subagent review and fix findings
8. verify PR base/stack topology, then publish through the normal PR workflow
9. perform closeout after merge/closure

## Validation Expectations

- Run the smallest proving validation that matches the issue's outcome type.
- Do not skip required proof just because the change is small.
- Do not run broad validation reflexively when focused proof is enough.
- Keep review records and output cards truthful about what was and was not run.

## Review And Publication Rules

- No PR should open before the work has had bounded subagent review.
- Verify the intended base branch before publication and verify the actual PR
  base immediately after creation, especially for stacked issue work.
- Findings come before summary.
- Fixes should stay within the issue's scope unless the operator explicitly
  widens it.
- If review uncovers a separate problem, open or route a follow-on issue
  instead of hiding new scope inside the current one.

## Non-Goals For This File

This root `AGENTS.md` is intentionally compact.

It is not:

- the full milestone manual
- a replacement for skill docs
- a substitute for issue cards
- the final word on nested package-specific agent guidance

## Source Baseline Used

Last reviewed: 2026-05-19.

This file was shaped from the OpenAI/source baselines named by `#2986`, plus
ADL-specific workflow policy:

- issue-named OpenAI `agents.md` GitHub baseline:
  `https://github.com/openai/agents.md`
- official OpenAI guide for `AGENTS.md` in Codex:
  `https://developers.openai.com/codex/guides/agents-md`
- practical OpenAI repository example:
  `https://github.com/openai/openai-cookbook/blob/main/AGENTS.md`
- broader open-format companion reference:
  `https://agents.md/`
- ADL's conductor, worktree, review, and closeout discipline

The issue named the GitHub `openai/agents.md` baseline explicitly. That
repository now routes into the broader `agents.md` effort, so this file keeps
both the issue-named GitHub source and the live `agents.md` reference visible
for traceability.

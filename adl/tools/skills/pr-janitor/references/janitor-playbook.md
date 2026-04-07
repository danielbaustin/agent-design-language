# PR Janitor Playbook

Use this file after the main skill triggers and you are ready to monitor or intervene on a PR in flight.

Planning basis:
- `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`
- `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md`

If the repo relocates those docs, follow the relocated canonical copies instead of these exact paths.

## Purpose

Watch a PR's progress, detect blockers, and respond proportionally.

This skill is not a full implementation skill and not a silent merge bot.

It may:
- inspect PR metadata and checks
- inspect local branch state
- classify blockers
- apply only small blocker-driven fixes when clearly justified
- stop after reporting status or bounded remediation

It must not:
- silently merge or close
- treat substantive reviewer judgment as purely mechanical
- turn monitoring into broad product development

## Target Resolution

Resolve the most concrete available target in this order:
1. explicit PR number
2. explicit PR URL
3. explicit branch
4. explicit issue number if it maps unambiguously to one open PR

If multiple PRs match, report `blocked`.

## What To Inspect

Inspect where applicable:
- PR open/draft/ready state
- merge conflict status
- failed/pending/passing checks
- requested changes and review blockers
- branch drift relative to main
- whether a bounded local reproduction or fix path exists

## Status Mapping

Use:
- `healthy`
  - no blocker currently requires intervention
- `action_required`
  - a concrete blocker exists and either was fixed or needs a focused next step
- `blocked`
  - the target is ambiguous or the blocker cannot be addressed safely in-bounds

## Allowed Bounded Actions

Allowed:
- rerun or locally reproduce the smallest relevant failing check
- prepare a focused blocker-driven fix
- resolve an unambiguous conflict when the intended resolution is clear
- record truthful PR progress output

Not allowed:
- widening implementation scope materially
- ignoring or overwriting substantive review feedback silently
- merging or closing the PR without explicit direction

## Model Guidance

Prefer a stronger model for this skill because it often requires synthesis and judgment rather than only mechanical validation.

Default:
- `gpt-5.4`

Use a smaller model only when the task is clearly limited to simple status inspection.

## Failure Handling

If the process fails or remains blocked:
- report the target PR
- report the current checks/conflict/review state that was actually observed
- report any action attempted
- record whether no repair was attempted, a repair was attempted and succeeded, or a repair was attempted and did not resolve the blocker
- report the exact next handoff needed
- stop without widening scope

# Multi-Agent C-SDLC Operation v0.91.5

## Metadata

- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Date: `2026-05-29`
- Owner: ADL maintainers
- Status: `active_wp_01_opening`
- Related issues: `#3415`, `#3501`, `#3503`, `#3504`
- Prior satisfied evidence: `#3484`

## Template Rules

This is a planning feature doc, not implementation evidence.

## Purpose

Define the bounded multi-agent operating surface that must work before v0.91.5
closes.

## Context

Single-threaded sprints are useful but too slow for the long-term C-SDLC. The
multi-agent lane must prove governed parallelism without hiding state or
weakening review.

## Coverage / Ownership

This feature owns role, shard, coordination, review, and closeout expectations
for multi-agent C-SDLC execution.

## Overview

The target workcell includes planner, worker, reviewer, janitor/closeout, and
watcher roles. Roles may use different models based on aptitude, but all work
must remain bound to issues, cards, branches, reviews, and PR truth.

## Design

- Each role records provider/model identity.
- Shards have explicit ownership and interface boundaries.
- Review and merge/closeout gates remain serialized where needed.
- Multi-agent benefit is measured against single-agent overhead.

## Execution Flow

1. Build or verify role/shard planning.
2. Select models from the provider/model matrix.
3. Execute a bounded workcell proof.
4. Review usefulness and overhead.
5. Record completion or blocker truth.

## Determinism and Constraints

No agent gets hidden authority. No role bypasses card, review, branch, or
closeout truth.

## Integration Points

- [../SPRINT_v0.91.5.md](../SPRINT_v0.91.5.md)
- [../DEMO_MATRIX_v0.91.5.md](../DEMO_MATRIX_v0.91.5.md)
- [PROVIDER_MODEL_MATRIX_v0.91.5.md](PROVIDER_MODEL_MATRIX_v0.91.5.md)

## Validation

Validation should include a bounded proof packet, role records, shard records,
timing/overhead comparison, and reviewer checklist.

## Acceptance Criteria

- Multi-agent C-SDLC executes a bounded issue/sprint slice or blocks truthfully.
- Role and model identity are visible.
- Review and closeout truth are preserved.

## Risks

- Multi-agent overhead may exceed benefit on small tasks.
- Local models may be weak for some roles.

## Future Work

Future milestones can expand from bounded workcells to richer Software
Development Polis operation.

## Notes

This feature does not claim unbounded autonomous development.

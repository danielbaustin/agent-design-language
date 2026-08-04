# Structured Task Prompt

Template: 1.0.0

Issue: 5007

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Preparation only for #5007: prepare reviewed, issue-specific C-SDLC v2 cards/design/diagram and focused validation for a future proof-gated Memory Palace ADR acceptance execution session.

## Deliverables

- Six refreshed #5007 cards under `.csdlc/issues/5007/cards/`.
- Reviewed preparation design `.csdlc/prepared/issues/5007/design.md` and Mermaid diagram `.csdlc/prepared/issues/5007/diagram.mmd`.
- Bounded preparation review `.csdlc/evidence/5007/preparation/gpt-5.5-preparation-review.md` with actionable fixes reflected in the packet.
- Focused preparation validation evidence under `.csdlc/evidence/5007/preparation/`.
- Clean commit and push on `codex/5007-v0918-wp14-preparation` with `origin/main` 51bc5ae51b57c19dbab693af1c5a45142995f4e5 integrated.

## Acceptance

1. AC1: `origin/main` is integrated at exact SHA 51bc5ae51b57c19dbab693af1c5a45142995f4e5 on branch codex/5007-v0918-wp14-preparation in `/Volumes/FastWork/adl-wp-5007`.
2. AC2: All six #5007 cards are issue-specific and name the preparation-only boundary, actual #4760 execution gate, exact dependencies, COTS limits, intended paths, LoC/time budgets, PVF lanes, rollback, and no-deferral policy.
3. AC3: Reviewed design and Mermaid diagram describe the Memory Palace ADR acceptance flow without drafting or accepting the ADR.
4. AC4: A bounded GPT-5.5 preparation review is retained, all actionable preparation findings are fixed, and review truth does not claim ADR execution readiness.
5. AC5: Focused preparation validation runs from the named worktree with temp/build outputs under `/Volumes/FastWork/adl-wp-5007`, and stale claim/closeout reconciliation is recorded as execution-time truth rather than a preparation blocker.

## Dependencies

- #4760 Memory Palace context handoff implementation/proof: OPEN at preparation time and the hard execution gate.
- #4765 Chronosense integration: CLOSED, proof must be consumed directly during execution.
- #4768 temporal query index for ObsMem and Memory Palace: CLOSED, proof must be consumed directly during execution.
- #4771 long-running context continuity proof: CLOSED, proof must be consumed directly during execution.
- ADR 0051 deferred Chronosense and Memory Palace disposition.
- WP-21 parent #5362 and v0.91.8 issue-wave routing for #5007.
- `origin/main` exact SHA 51bc5ae51b57c19dbab693af1c5a45142995f4e5.

## Inputs

- https://github.com/danielbaustin/agent-design-language/issues/5007
- https://github.com/danielbaustin/agent-design-language/issues/4760
- https://github.com/danielbaustin/agent-design-language/issues/4765
- https://github.com/danielbaustin/agent-design-language/issues/4768
- https://github.com/danielbaustin/agent-design-language/issues/4771
- docs/adr/0051-chronosense-and-memory-palace-adr-disposition.md
- docs/adr/README.md
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/WBS_v0.91.8.md
- docs/milestones/v0.91.8/ADR_PLAN_v0.91.8.md
- docs/milestones/v0.91.8/NEXT_MILESTONE_HANDOFF_v0.91.8.md
- docs/milestones/v0.91.8/features/V092_HANDOFF_v0.91.8.md

## Non Goals

- Do not draft or commit `docs/adr/0058-memory-palace-context-handoff-architecture.md` during preparation.
- Do not implement Memory Palace, change runtime/source code, add dependencies, run AWS/providers, or consume credentials.
- Do not open a PR, publish lifecycle state, merge, close #5007, or close #4760.
- Do not claim #5007 execution readiness until #4760 has actual completed implementation proof and a fresh execution-time claim is acquired.
- Do not defer any future ADR acceptance criterion; missing proof must block execution.

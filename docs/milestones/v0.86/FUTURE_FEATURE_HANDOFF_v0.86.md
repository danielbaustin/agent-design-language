# Future Feature Handoff — v0.86

## Purpose

Preserve important architectural content from the promoted `v0.86` feature docs without forcing the milestone WBS to claim full implementation of every future-facing concept mentioned there.

This document distinguishes:
- what `v0.86` must make real now in bounded form
- what later milestones must carry forward explicitly

The goal is to avoid losing important content while keeping the milestone contract aligned with the actual WBS.

## v0.86 Bounded Commitment Rule

Promoted feature docs under `docs/milestones/v0.86/features/` are allowed to contain:
- bounded `v0.86` implementation commitments
- future-facing architectural context

For `v0.86`, only the bounded commitments reflected in the tracked milestone package and WBS are normative.

Future-facing concepts in the promoted feature docs are preserved as design guidance and must be implemented by the milestone owners listed below rather than silently treated as `v0.86` commitments.

## Handoff Map

| Theme | Present in promoted feature docs | `v0.86` bounded commitment | Future milestone owner |
|---|---|---|---|
| Richer instinct / agency expansion | `AGENCY_AND_AGENTS.md` | bounded candidate selection / early agency only | `v0.88` |
| Richer convergence behavior | `COGNITIVE_LOOP_MODEL.md`, `CONCEPT_PLANNING_FOR_v0.86.md` | bounded execution (`AEE-lite`) only | `v0.89` |
| Stronger long-term learning / routing improvement | `COGNITIVE_ARBITRATION.md`, `FAST_SLOW_THINKING_MODEL.md` | bounded routing behavior only | `v0.89` |
| Reasoning graphs / trace-heavy reasoning structure | `COGNITIVE_STACK.md`, `FAST_SLOW_THINKING_MODEL.md` | none beyond bounded local reasoning control | `v0.90` |
| Richer affect systems | `COGNITIVE_LOOP_MODEL.md`, `CONCEPT_PLANNING_FOR_v0.86.md` | affect as bounded signal input only | `v0.91` |
| Identity / narrative continuity | `AGENCY_AND_AGENTS.md` | none beyond bounded local continuity assumptions | `v0.92` |
| Governance / constitutional / signed-trace expansion | `COGNITIVE_ARBITRATION.md`, `COGNITIVE_STACK.md`, `FREEDOM_GATE.md` | Freedom Gate allow/defer/refuse only | `v0.93` and `v0.90` for signed trace |
| Multi-agent / broad platform cognition | `COGNITIVE_STACK.md`, `FAST_SLOW_THINKING_MODEL.md` | none | `v0.95+` |

## Usage

- Use the milestone docs and WBS for `v0.86` implementation commitments.
- Use the promoted feature docs for bounded feature detail plus preserved architectural context.
- Use this handoff map when a promoted feature doc mentions a concept that exceeds the current milestone WBS.

If a concept is not explicitly committed in the `v0.86` milestone docs or WBS, treat it as future-facing guidance unless and until a tracked milestone doc says otherwise.

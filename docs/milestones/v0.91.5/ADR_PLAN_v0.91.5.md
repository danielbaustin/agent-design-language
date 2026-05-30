# v0.91.5 ADR Plan

## Status

Candidate ADR planning surface.

## Purpose

Record architecture decisions that may need ADR treatment during or after
v0.91.5.

## Candidate ADRs

| Candidate | Decision surface | Source docs | Timing |
| --- | --- | --- | --- |
| Multi-agent C-SDLC operating boundary | Bounded roles, shard ownership, review/closeout serialization, and no unbounded autonomy. | [MULTI_AGENT_CSDL_OPERATION_v0.91.5.md](features/MULTI_AGENT_CSDL_OPERATION_v0.91.5.md) | Author if v0.91.5 changes the accepted multi-agent boundary. |
| Provider/model matrix and OpenRouter substrate | Provider identity, model-role aptitude, and OpenRouter as a test substrate. | [PROVIDER_MODEL_MATRIX_v0.91.5.md](features/PROVIDER_MODEL_MATRIX_v0.91.5.md) | Author if provider architecture changes beyond planning. |
| Public prompt records and local-state archive boundary | Public prompt packets versus local `.adl` execution cache. | [PUBLIC_PROMPT_RECORDS_v0.91.5.md](features/PUBLIC_PROMPT_RECORDS_v0.91.5.md) | Author if public prompt records become canonical system objects. |
| v0.92 activation readiness boundary | What v0.91.5 must prove before v0.92 opens. | [V092_ACTIVATION_READINESS_v0.91.5.md](features/V092_ACTIVATION_READINESS_v0.91.5.md) | Usually a planning/release decision, unless architecture boundaries change. |

## Non-Claims

This plan does not accept ADRs. It only identifies candidate decision surfaces.

## Exit Criteria

- Any ADR-worthy implementation decision is authored, deferred, or explicitly
  marked unnecessary before release.


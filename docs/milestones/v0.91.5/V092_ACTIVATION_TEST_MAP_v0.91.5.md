# v0.92 Activation Test Map

## Metadata

- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Date: `2026-05-29`
- Owner: ADL maintainers
- Related issues: `#3377`, `#3502`

## Purpose

This map records the features and substrate work that were developed before
v0.92 but need explicit activation testing before or during the first-birthday
milestone.

## Activation Surfaces

| Surface | Why it matters for v0.92 | Required v0.91.5 action |
| --- | --- | --- |
| Memory v2 / ObsMem handoff | First birthday needs witnessed, redacted memory grounding. | Confirm available evidence, blockers, and required v0.92 tests. |
| ACP / cognitive profiles | Birthday records need bounded cognitive-profile context without free-floating labels. | Map profile fixtures and privacy constraints into v0.92 WP-07. |
| Aptitude and capability selector | Multi-agent and birth capability envelopes need model/role suitability evidence. | Connect model-role matrix to v0.92 capability-envelope planning. |
| Identity and continuity | First birthday depends on stable identity, name, continuity head, and cycle evidence. | Verify prior identity/continuity surfaces and negative cases. |
| Affect and happiness surfaces | Earlier affect/wellbeing work may become visible in birthday evidence. | Identify safe tests and non-claims for affect, humor, happiness, wellbeing. |
| Gödel mechanics | v0.92 is the first true Gödel-agent birthday. | Map experiment, hypothesis, mutation, evaluation, and promotion mechanics into birthday proof. |
| Economics context | Earlier economics work may inform capability boundaries but must not dominate v0.92. | Record whether economics is context-only or requires explicit tests. |
| Observatory | v0.92 should make the birthday reviewable and visible. | Define which Observatory surfaces are demo proof versus future UI. |
| Unity demo readiness | Celestial Rescue may double as Unity Observatory preparation. | Decide whether it is proof, rehearsal, or deferred demo substrate. |
| ACIP / provider communications | Birthday evidence may use communication envelopes and schema transport. | Verify public schema, JSON projection, and mock carrier readiness. |
| Provider/model matrix | Multi-agent roles and capability envelopes need actual model identity. | Test hosted, local Ollama, remote Ollama, and OpenRouter lanes. |
| Public prompt records | v0.92 should consume durable prompt records, not local chat memory. | Export/redact/index prompt packets needed by v0.92 WP-01. |

## Acceptance Criteria

- Every row has an owner issue, v0.92 candidate WP, and test/proof posture
  before v0.91.5 closeout.
- Rows that cannot be activated safely are marked blocked or deferred before
  v0.92 WP-01 opens.
- The final `#3377` readiness packet consumes this map.


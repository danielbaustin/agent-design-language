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

| Surface | Why it matters for v0.92 | Owner issue | v0.92 candidate WP | Test / proof posture | Required v0.91.5 action |
| --- | --- | --- | --- | --- | --- |
| AEE completion tranche | First birthday and multi-agent operation need governed adaptation, not scattered retry/convergence claims. | `#3526`, `#3528`, `#3534`, `#3377` | WP-01 / WP-14 candidate | Closure criteria and proof expectations for steering, queue/wake/handoff, distributed boundary, policy stops, trace/replay, and end-to-end demo. | Feed the AEE closure tranche into `#3377`; v0.92 WP-01 must seed or route concrete AEE proof work before claiming birthday readiness. |
| Memory v2 / ObsMem handoff | First birthday needs witnessed, redacted memory grounding. | `#3502`, `#3377` | WP-03 / WP-04 candidate | Inventory plus activation-test plan; no live-memory claim until proof exists. | Confirm available evidence, blockers, and required v0.92 tests. |
| ACP / cognitive profiles | Birthday records need bounded cognitive-profile context without free-floating labels. | `#3502`, `#3377` | WP-07 candidate | Fixture and privacy-boundary review before birthday use. | Map profile fixtures and privacy constraints into v0.92 WP-07. |
| Aptitude and capability selector | Multi-agent and birth capability envelopes need model/role suitability evidence. | `#3501`, `#3505`, `#3377` | WP-06 candidate | Provider/model role matrix plus explicit suitability caveats. | Connect model-role matrix to v0.92 capability-envelope planning. |
| Identity and continuity | First birthday depends on stable identity, name, continuity head, and cycle evidence. | `#3377` | WP-01 / WP-02 candidate | Continuity proof checklist with negative-case review. | Verify prior identity/continuity surfaces and negative cases. |
| Affect and happiness surfaces | Earlier affect/wellbeing work may become visible in birthday evidence. | `#3502`, `#3377` | WP-08 candidate | Safe-test plan and non-claims before public birthday evidence. | Identify safe tests and non-claims for affect, humor, happiness, wellbeing. |
| Gödel mechanics | v0.92 is the first true Gödel-agent birthday. | `#3502`, `#3377` | WP-09 candidate | Experiment/hypothesis/mutation/evaluation proof map. | Map experiment, hypothesis, mutation, evaluation, and promotion mechanics into birthday proof. |
| Economics context | Earlier economics work may inform capability boundaries but must not dominate v0.92. | `#3502`, `#3377` | WP-10 candidate | Context-only or explicit-test decision recorded before use. | Record whether economics is context-only or requires explicit tests. |
| Observatory | v0.92 should make the birthday reviewable and visible. | `#3455`, `#3460`, `#3461`, `#3377` | WP-11 candidate | Demo-readiness proof separated from future UI claims. | Define which Observatory surfaces are demo proof versus future UI. |
| Unity demo readiness | Celestial Rescue may double as Unity Observatory preparation. | `#3460`, `#3461`, `#3377` | WP-12 candidate | Runnable demo, rehearsal, or deferred substrate disposition. | Decide whether it is proof, rehearsal, or deferred demo substrate. |
| ACIP / provider communications | Birthday evidence may use communication envelopes and schema transport. | `#3501`, `#3505`, `#3377` | WP-13 candidate | Schema/transport readiness check with mock-carrier fallback. | Verify public schema, JSON projection, and mock carrier readiness. |
| Provider/model matrix | Multi-agent roles and capability envelopes need actual model identity. | `#3501`, `#3505` | WP-06 candidate | Hosted, local Ollama, remote Ollama, and OpenRouter lane evidence. | Test hosted, local Ollama, remote Ollama, and OpenRouter lanes. |
| Public prompt records | v0.92 should consume durable prompt records, not local chat memory. | `#3472`-`#3476`, `#3377` | WP-01 candidate | Exported, redacted, indexed prompt packet proof. | Export/redact/index prompt packets needed by v0.92 WP-01. |

## Acceptance Criteria

- Every row has an owner issue, v0.92 candidate WP, and test/proof posture
  before v0.91.5 closeout.
- Rows that cannot be activated safely are marked blocked or deferred before
  v0.92 WP-01 opens.
- The final `#3377` readiness packet consumes this map.

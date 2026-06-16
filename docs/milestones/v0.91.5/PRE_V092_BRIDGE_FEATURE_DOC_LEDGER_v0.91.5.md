# Pre-v0.92 Bridge Feature-Doc Ledger

## Status

Tracked bridge ledger for issue `#3778`.

This document is a planning and routing surface. It does not implement runtime
features and does not claim `v0.92` activation readiness. Its job is to ensure
that every named pre-`v0.92` bridge surface has a feature-doc route,
disposition, and review gate before `v0.92` activation work begins.

## Purpose

The `v0.92` birthday and activation milestone depends on several bridge
surfaces that cannot remain in local notes or chat memory. This ledger routes
those surfaces into two planned bridge tranches:

- `v0.91.6` issue `#3800`: first bridge tranche for resilience, logging/tooling proof-loop
  fixes, public prompt records, provider/model reliability, and first
  ACIP/A2A/security decisions.
- `v0.91.7` issue `#3801`: second bridge tranche for Curiosity, Constructability, reasoning
  graph / loop / `adl.skill.v1`, residual security readiness, and residual
  ACIP/A2A/protobuf decisions.

If `v0.91.6` pulls a `v0.91.7` surface forward, the feature doc must say so
explicitly and must not weaken the resilience, logging/tooling, public-record,
provider/model, ACIP/A2A, or security proof surfaces.

## Source Evidence

Tracked sources:

- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/planning/FEATURE_DOC_PRODUCTION_MINI_SPRINT_v0.91.5.md`
- `docs/milestones/v0.91.5/V092_ACTIVATION_TEST_MAP_v0.91.5.md`
- `docs/milestones/v0.91.5/features/V092_ACTIVATION_READINESS_v0.91.5.md`
- `docs/milestones/v0.91.5/CONTROL_PLANE_OBSERVABILITY_CONTRACT_3609.md`
- `docs/milestones/v0.91.5/features/AEE_COMPLETION_TRANCHE_v0.91.5.md`

Local authoring sources used as inputs:

- local resilience planning
- local security and Continuous Adversarial Verification planning
- local ACIP/A2A/provider-communications planning
- local reasoning-graph, loop, and skill-standard planning
- local Curiosity and Constructability planning
- local public prompt-record and multi-agent/provider reliability planning

The local authoring sources are evidence inputs only. This ledger does not
promote or move those local files.

## Bridge Surface Ledger

| Surface | Disposition before v0.92 | Target tranche | Feature-doc or bridge-record requirement | Review gate |
|---|---|---|---|---|
| AEE completion | Routed | `v0.91.6` / `v0.92` handoff | AEE completion feature doc or bridge record must distinguish completed boundary, residual runtime/provider action work, and what `v0.92` may consume. | Do not let `v0.92` consume AEE as complete without named evidence or explicit residual routing. |
| Memory/ObsMem handoff | Routed | `v0.92` with pre-`v0.92` accounting | Bridge record must distinguish ObsMem handoff, memory grounding, Memory Palace planning, working set, and context cache. | `v0.92` docs must consume this as a handoff, not rediscover it. |
| ACP/cognitive profiles | Routed | `v0.92` with pre-`v0.92` accounting | Feature doc must define profile scope, capability envelope, privacy boundary, update rules, and relation to provider/model matrix. | No activation claim until profile evidence and limits are explicit. |
| Aptitude and capability selector | Routed | `v0.91.6` issue `#3800` / `v0.92` handoff | Bridge record must connect capability evidence to model/role suitability without starting the later Aptitude Atlas baseline. | `v0.92` may consume capability-testing evidence only; Aptitude Atlas construction is deferred beyond MVP scope. |
| Identity and continuity | Routed | `v0.91.6` issue `#3800` / `v0.92` handoff | Bridge record must account for stable identity, continuity head, cycle evidence, negative cases, and relation to resilience/citizen persistence. | No birthday identity claim without continuity proof or explicit residual routing. |
| Affect and happiness surfaces | Routed | `v0.91.7` issue `#3801` / `v0.92` handoff | Bridge record must define safe tests, non-claims, and public-evidence limits for affect, humor, happiness, and wellbeing surfaces. | Public birthday evidence must not imply unproved affect or wellbeing claims. |
| Gödel mechanics | Routed | `v0.91.7` issue `#3801` / `v0.92` handoff | Bridge record must map experiment, hypothesis, mutation, evaluation, promotion, and proof boundaries for Gödel-agent birthday use. | `v0.92` must consume a reviewed mechanics map or mark the surface blocked/deferred. |
| Economics context | Routed | `v0.91.7` issue `#3801` / `v0.92` handoff | Bridge record must decide whether economics is context-only or requires explicit activation tests. | Economics must not dominate `v0.92` scope without an explicit test/proof decision. |
| Provider/model matrix and multi-agent readiness | Routed | `v0.91.6` issue `#3800` | Feature doc or bridge record must state hosted/local/remote/OpenRouter/Gemma expectations, role suitability, known failure modes, and multi-agent reliability proof limits. | Reliability proof must be separated from ANRM/training/product claims. |
| Observatory/Unity readiness | Routed | `v0.91.6` / `v0.92` handoff | Bridge record must classify each Observatory/Unity surface as proof, rehearsal, substrate, blocked, or deferred. | No birthday demo can rely on an unclassified Observatory/Unity surface. |
| ACIP/A2A/provider communications | Routed | `v0.91.6` issue `#3800` with possible `v0.91.7` issue `#3801` residuals | Feature doc must cover schema catalog, message access rules, A2A/external-agent posture, provider communications, WebSocket boundary, deterministic JSON projection, and protobuf decision point. | Security and constructability boundaries must be included before activation. |
| Public prompt records export/redaction/indexing | Routed | `v0.91.6` issue `#3800` | Feature doc must preserve local editable authoring while defining public export, redaction, validation, indexing, evidence, and security review boundaries. | No public-record publication claim without redaction/security/index posture. |
| Logging/tooling proof-loop fixes | Routed | `v0.91.6` issue `#3800` | Feature doc or bridge record must cover validation architecture split, CI runtime-budget observability, logging/Otel consumption, and bounded PR proof-loop reliability. | Must improve bounded PR flow without weakening broad release confidence. |
| Security bridge readiness and CAV | Routed | `v0.91.6` issue `#3800` with possible `v0.91.7` issue `#3801` residuals | Feature doc must cover threat-model refresh, Continuous Adversarial Verification, provider/model trust, public-record redaction/security, ACIP access/security, and adversarial/malformed-output expectations. | Security cannot be deferred out of the activation path silently. |
| Resilience, citizen persistence, and sleep/wake | Routed | `v0.91.6` issue `#3800` | Feature doc must cover retry/fault classification, provider/tool/workflow resilience, health persistence, checkpoint/restore, sleep/wake, hibernation, simulation, in-transit custody, migration, replay, and continuity proof. | Do not close as half-work; sleep/wake, migration, and continuity proof need explicit disposition. |
| Curiosity Engine / Discovery Substrate | Routed | `v0.91.7` issue `#3801` unless pulled forward | Feature doc must cover curiosity artifacts, detection hooks, hypotheses, experiment plans, discovery budget, governance, Freedom Gate integration, ObsMem/reasoning-graph update, and proof. | At least one governed discovery-cycle proof is required before `v0.92` activation consumes it. |
| Constructability Gate | Routed | `v0.91.7` issue `#3801` unless pulled forward | Feature doc must cover construction-event schema, external-anchor schema, admissibility validator, shared-reality boundary, and proof path. | Must distinguish provisional cognition from authoritative shared reality. |
| Reasoning graph, loop runtime, and `adl.skill.v1` bridge | Routed | `v0.91.7` issue `#3801` with deeper `v0.94` convergence | Feature doc must define the pre-`v0.92` bridge among prompts, skills, loops, trace, ObsMem, PVF, AEE, Runtime v2, UTS, ACC, and `adl.skill.v1`. | Deeper reasoning-graph convergence remains `v0.94`; do not hide it in pre-`v0.92` bridge docs. |

## v0.91.6 First-Tranche Packet

Issue `#3800` should produce feature docs or bridge records for:

- resilience, citizen persistence, sleep/wake, and continuity proof
- logging/tooling proof-loop fixes and observability consumption
- public prompt records export, redaction, validation, and indexing
- provider/model reliability and multi-agent readiness
- first ACIP/A2A/provider-communications decisions
- first security bridge readiness and CAV decisions
- identity/continuity and capability-selector bridge accounting

Exit condition: `v0.91.6` has enough reviewed bridge truth that `v0.91.7`
contains only explicitly routed second-tranche work, not vague spillover.

## v0.91.7 Second-Tranche Packet

Issue `#3801` should produce feature docs or bridge records for:

- Curiosity Engine / Discovery Substrate
- Constructability Gate
- reasoning graph, loop runtime, and `adl.skill.v1` bridge
- residual security readiness
- residual ACIP/A2A/protobuf/JSON projection decisions
- affect/happiness, Gödel mechanics, and economics-context bridge accounting

Exit condition: `v0.92` can consume complete, deferred, blocked, or routed
truth for every activation surface without reconstructing the plan from local
notes.

## v0.92 Consumption Contract

`v0.92` activation and birthday docs may consume this bridge only after each
surface above has one of these states:

- `complete`: feature doc and proof/review evidence are present.
- `deferred`: explicitly not required for `v0.92`, with risk accepted.
- `blocked`: cannot proceed without named missing evidence or operator
  decision.
- `routed`: owned by a named issue/tranche with a clear exit condition.

The bridge ledger itself is not activation evidence. It is the route to the
evidence.

## Validation Plan

When this ledger is updated or consumed:

- run `git diff --check`
- scan added tracked public-doc lines for host-local paths, obvious secret
  markers, and local planning-workspace links
- verify every surface in the bridge surface ledger appears in the `v0.91.6`,
  `v0.91.7`, or `v0.92` consumption sections
- run bounded pre-PR review focused on missing bridge surfaces, overclaiming
  readiness, and accidental implementation scope

## Non-Goals

- Do not implement bridge features in this ledger.
- Do not create `v0.91.6` or `v0.91.7` milestone packages here; issues
  `#3800` and `#3801` own that work.
- Do not claim `v0.92` activation readiness.
- Do not move or delete local planning files.
- Do not collapse security, ACIP/A2A, or constructability into generic
  follow-up language.

## Current Verdict

Every named pre-`v0.92` activation surface is accounted for, but none is marked
complete by this ledger. The next work is to execute the `v0.91.6` and
`v0.91.7` feature-doc packets so `v0.92` can consume reviewed bridge truth.

# v0.92 Activation Bridge Ledger

## Status

Tracked activation bridge ledger for issue `#3780`.

This document refreshes the `v0.92` activation and first-birthday planning
surface after the pre-`v0.92` bridge route was created. It is not activation
evidence, release evidence, or a claim that `v0.92` is ready to execute.

Current verdict: `v0.92` activation remains blocked until the `v0.91.6` and
`v0.91.7` bridge tranches produce reviewed feature docs, decision records, or
explicit blocked/deferred dispositions for every surface below.

## Purpose

The `v0.92` birthday milestone should not reconstruct bridge requirements from
chat history, local notes, or scattered feature docs. This ledger states what
`v0.92` may consume, what must remain outside the birthday claim, and which
upstream tranche owns the missing proof or decision.

## Source Evidence

Tracked sources:

- `docs/milestones/v0.91.5/PRE_V092_BRIDGE_FEATURE_DOC_LEDGER_v0.91.5.md`
- `docs/milestones/v0.91.5/V092_ACTIVATION_TEST_MAP_v0.91.5.md`
- `docs/milestones/v0.91.5/features/V092_ACTIVATION_READINESS_v0.91.5.md`
- `docs/milestones/v0.91.6/`
- `docs/milestones/v0.91.7/`
- `docs/milestones/v0.92/README.md`
- `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`
- `docs/milestones/v0.92/features/`
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/planning/FEATURE_DOC_PRODUCTION_MINI_SPRINT_v0.91.5.md`

When this ledger consumes `v0.91.6` bridge state, the current issue-truth
surfaces are:

- `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_RETAINED_EVIDENCE_MATRIX_4251.md`
  for closed bridge umbrellas and retained evidence posture
- `docs/milestones/v0.91.6/CLOSEOUT_TAIL_SPRINT_v0.91.6.md` for the ordered
  open release-tail issue wave

Use `docs/milestones/v0.91.6/review/V0916_RELEASE_AND_BRIDGE_DOC_TRUTH_CONSUMPTION_REVIEW_4522.md`
as the bounded audit of this consumption rule and its remaining manual
boundary, not as a third current-state ledger.

For pre-`v0.92` activation and C-SDLC carryforward specifically, the tracked
v0.91.6 truth now includes:

- closed adoption sprint `#4433`-`#4438`
- closed observability/docs follow-ons `#4520`-`#4522`

Those issues are closed retained inputs, not still-open activation blockers.
What remains blocking for `v0.92` is any surface that still lacks reviewed
runtime/product evidence or an explicit blocked/deferred/routed disposition.

## Consumption States

- `complete`: reviewed feature doc and proof/review evidence are present.
- `deferred`: explicitly not required for `v0.92`, with risk accepted.
- `blocked`: cannot proceed without named evidence or operator decision.
- `routed`: owned by a named issue, milestone tranche, or `v0.92` work package
  with a clear exit condition.

Planning packages alone are not `complete` evidence. The `v0.91.6` and
`v0.91.7` packages currently provide routes, not completed bridge proof.

## Activation Surface Ledger

| Surface | Current state for v0.92 | Owner before activation | v0.92 consumption rule |
| --- | --- | --- | --- |
| Activation contract and bridge evidence | Routed | `v0.91.6`, `v0.91.7`, then `v0.92` WP-01 | `v0.92` may define the birthday contract now, but may not mark activation ready until every bridge surface has a complete, deferred, blocked, or routed disposition with evidence. Closed process/docs fixes from `#4433`-`#4438` and `#4520`-`#4522` count as retained bridge inputs rather than open blockers by themselves. |
| Birthday and first-run behavior | Routed | `v0.92` birthday feature docs and `#3377` readiness packet | Birthday must remain evidence-bound. Startup, wake, restore, admission, copied state, and ordinary process launch are negative cases until the birthday packet proves otherwise. |
| Identity and continuity | Routed | `v0.91.6` identity/continuity bridge, then `v0.92` identity feature docs | Stable name, identity root, continuity head, cycle evidence, and negative cases must be reviewable before the birthday claim can pass. |
| AEE completion | Routed | `v0.91.6` AEE bridge accounting and residual runtime/provider action work | `v0.92` may consume only named AEE completion evidence and must preserve residual action/provider boundaries. |
| Memory/ObsMem handoff | Routed | `v0.91.6` AEE/Memory/ACP accounting and `v0.92` memory grounding docs | `v0.92` must distinguish ObsMem handoff, memory grounding, working set, context cache, and Memory Palace planning. |
| Memory Palace | Routed | `v0.91.6`/`v0.92` handoff; detailed implementation remains under development | Memory Palace is a planned solution direction for long-running context, not a completed runtime surface for this refresh. Birthday docs may reference it only as planned or under development. |
| ACP/cognitive profiles | Routed | `v0.91.6` bridge accounting and `v0.92` ACP feature docs | Profiles must state scope, privacy boundary, update rules, capability-envelope relation, and provider/model relation before activation consumes them. |
| Capability evidence and selector | Routed | `v0.91.6` identity/capability bridge; later Aptitude Atlas deferred beyond MVP | `v0.92` may consume capability-testing evidence for envelopes and role suitability, but must not start or imply a complete Aptitude Atlas baseline. |
| Provider/model matrix and multi-agent readiness | Routed | `v0.91.6` provider/model reliability feature doc | Hosted, local, remote, OpenRouter, Gemma, and multi-agent lanes need role suitability, known failure modes, and proof limits before birthday demos rely on them. |
| Observatory/Unity readiness | Routed | `v0.91.6` Observatory/Unity consumption classification, then `v0.92` demo planning | Each surface must be classified as proof, rehearsal, substrate, blocked, or deferred before a birthday demo depends on it. |
| ACIP/provider communications | Routed | `v0.91.6` ACIP/A2A/provider communications and `v0.91.7` residual decisions | Schema catalog, message access rules, provider communications, JSON projection, protobuf decision, and WebSocket boundary must be explicit before activation consumes the channel. |
| Public prompt records | Routed | `v0.91.6` public prompt records export feature doc | `v0.92` may consume public prompt records only after local authoring, export, redaction, validation, indexing, evidence, and security review boundaries are documented. |
| Logging/tooling proof-loop reliability | Routed | `v0.91.6` tooling proof-loop reliability feature doc | `v0.92` may rely on C-SDLC proof-loop outputs only after validation split, CI runtime-budget observability, OTel/logging consumption, and bounded PR reliability residuals are complete or explicitly routed. |
| Security and Continuous Adversarial Verification | Routed | `v0.91.6` security bridge and CAV, with possible `v0.91.7` residuals | Activation cannot silently defer threat-model, adversarial-output, provider-trust, public-record security, or ACIP security requirements. |
| Resilience, citizen persistence, and sleep/wake | Routed | `v0.91.6` resilience/persistence/sleep-wake feature doc | Transient fault handling, checkpoint/restore, sleep/wake, hibernation, simulation, in-transit custody, migration, replay, and continuity proof must be complete, blocked, deferred, or routed. |
| Curiosity Engine / Discovery Substrate | Routed | `v0.91.7` Curiosity feature doc unless pulled forward | Curiosity is required before `v0.92` activation consumes governed discovery behavior; absent proof must be marked blocked, deferred, or routed. |
| Constructability Gate | Routed | `v0.91.7` Constructability feature doc unless pulled forward | Birthday evidence must distinguish provisional cognition from authoritative shared reality. |
| Reasoning graph, loop runtime, and `adl.skill.v1` | Routed | `v0.91.7` reasoning graph/loop/skill-standard bridge | Pre-`v0.92` bridge must connect prompts, skills, loops, trace, ObsMem, PVF, AEE, Runtime v2, UTS, ACC, and `adl.skill.v1`; deeper convergence remains later. |
| Affect, happiness, humor, and wellbeing claims | Routed | `v0.91.7` affect/happiness bridge | Public birthday evidence must not imply unproved affect or wellbeing claims. |
| Godel mechanics | Routed | `v0.91.7` Godel mechanics bridge and `v0.92` birthday docs | Experiment, hypothesis, mutation, evaluation, promotion, and proof boundaries must be reviewable before the first true Godel-agent birthday claim relies on them. |
| Economics context | Routed | `v0.91.7` economics context decision | Economics is context-only unless a reviewed bridge decision says explicit activation tests are required. |

## Birthday Contract Refresh

For `v0.92`, the first birthday remains a deterministic review event over
evidence, not a ceremony or a process-start marker. The minimum birthday
packet must include:

- stable name and identity root
- continuity record and continuity head
- memory grounding through redaction-safe references
- capability envelope with provider, model, tool, skill, authority, and limit
  context
- ACP/cognitive profile evidence and privacy boundary
- inherited moral/governance context
- witness set and citizen-facing receipt
- activation trace and review packet

The negative-case set must include ordinary startup, wake, restore, snapshot,
copied state, admission, simulation, in-transit custody, shutdown,
forced-suspension, and missing-evidence cases.

## Memory And Context Boundary

`v0.92` may use Memory/ObsMem handoff evidence for birthday grounding, but the
handoff must not be confused with a completed Memory Palace runtime.

Memory Palace is planned as a major solution direction for long-running
context. Until its design and proof surface are reviewed, the `v0.92` feature
set should treat it as:

- a named planning dependency
- a continuity and context-management direction
- a non-claim for raw private-state exposure
- a non-claim for completed runtime behavior

## Demo And Review Expectations

The first-birthday demo/review packet should prove both positive and negative
behavior:

- valid birthday packet assembles all required evidence surfaces
- missing identity, continuity, memory, capability, witness, receipt, profile,
  or inherited governance evidence fails closed
- startup, wake, restore, snapshot, copied state, and admission fixtures are
  rejected as birth
- Observatory/Unity surfaces used by the demo are classified as proof,
  rehearsal, substrate, blocked, or deferred
- public prompt records used by review are exported, redacted, validated, and
  indexed under the documented public-record boundary
- provider/model lanes used by the demo are named with reliability limits

## Upstream Tranche Gate

`v0.91.6` must resolve or route:

- resilience, citizen persistence, sleep/wake, and continuity proof
- logging/tooling proof-loop reliability and observability consumption
- public prompt records export, redaction, validation, and indexing
- provider/model reliability and multi-agent readiness
- first ACIP/A2A/provider-communications decisions
- first security bridge readiness and CAV decisions
- identity/continuity and capability-selector bridge accounting
- AEE completion, Memory/ObsMem handoff, ACP/cognitive profile accounting
- Observatory/Unity consumption classification

`v0.92` should consume current `v0.91.6` bridge closure truth from the retained
evidence matrix and current open release-tail truth from the closeout-tail
sprint surface rather than reconstructing state from individual issue histories.

`v0.91.7` must resolve or route:

- Curiosity Engine / Discovery Substrate
- Constructability Gate
- reasoning graph, loop runtime, and `adl.skill.v1`
- residual security readiness
- residual ACIP/A2A/protobuf/JSON projection decisions
- affect/happiness bridge
- Godel mechanics bridge
- economics-context decision

## Non-Goals

- Do not claim `v0.92` activation readiness in this ledger.
- Do not implement Memory Palace, ACIP transport, ACP profiles, resilience, or
  runtime behavior here.
- Do not absorb `v0.93` governance, `v0.94` secure execution/trust/time, or
  `v0.95` MVP convergence work.
- Do not treat planning routes as completed proof.
- Do not publish or migrate local authoring notes from this issue.

## Validation Plan

When this ledger is updated:

- run `git diff --check`
- verify the `v0.92` README links this ledger
- scan added public-doc lines for host-local paths, secret markers, and local
  authoring-workspace links
- scan for every required activation surface named by issue `#3780`
- run bounded pre-PR review focused on missing surfaces, readiness overclaims,
  and accidental implementation scope

## Current Verdict

Every named activation surface is accounted for, but all are currently routed
rather than complete. `v0.92` can use this ledger as a consumption map only
after the upstream bridge tranches produce reviewed evidence or explicit
blocked/deferred decisions.

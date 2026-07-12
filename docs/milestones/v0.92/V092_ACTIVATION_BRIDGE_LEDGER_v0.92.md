# v0.92 Activation Consumption Ledger

## Status

Tracked activation consumption ledger for issue `#3780`.

This document refreshes the `v0.92` activation and first-birthday planning
surface after the pre-`v0.92` implementation/proof path was created. It is not activation
evidence, release evidence, or a claim that `v0.92` is ready to execute.

Current verdict: `v0.92` activation remains blocked until the `v0.91.6` and
`v0.91.7` readiness, implementation, and integrated-proof tranches produce
reviewed evidence, decision records, or explicit evidence-backed blockers for
every required surface below.

## Purpose

The `v0.92` birthday milestone should not reconstruct activation requirements
from chat history, local notes, or scattered feature docs. This ledger states
what `v0.92` may consume, what must remain outside the birthday claim, and
which upstream issue owns any missing proof or decision.

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

When this ledger consumes `v0.91.6` readiness state, the current issue-truth
surfaces are:

- `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_RETAINED_EVIDENCE_MATRIX_4251.md`
  for closed umbrellas and retained evidence posture
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
runtime/product evidence or an explicit evidence-backed blocker with operator
approval.

## Consumption States

- `integrated_proven`: reviewed feature doc and integrated proof/review
  evidence are present.
- `operator_scoped_out`: explicitly not required for `v0.92`, with evidence,
  risk, and operator approval recorded.
- `blocked_with_evidence`: cannot proceed without named evidence or operator
  decision.
- `implementation_required`: owned by a named issue or work package with a
  clear exit condition, but not complete until integrated proof exists.

Planning packages alone are not `complete` evidence. The `v0.91.6` and
`v0.91.7` packages currently provide ownership and evidence requirements, not
completion proof by themselves.

## Activation Surface Ledger

| Surface | Current state for v0.92 | Owner before activation | v0.92 consumption rule |
| --- | --- | --- | --- |
| Activation contract and evidence | implementation_required | `v0.91.6`, `v0.91.7`, then `v0.92` WP-01 | `v0.92` may define the birthday contract now, but may not mark activation ready until every activation surface is integrated_proven, operator_scoped_out, or blocked_with_evidence. Closed process/docs fixes from `#4433`-`#4438` and `#4520`-`#4522` count as retained inputs rather than open blockers by themselves. |
| Birthday and first-run behavior | implementation_required | `v0.92` birthday feature docs and `#3377` readiness packet | Birthday must remain evidence-bound. Startup, wake, restore, admission, copied state, and ordinary process launch are negative cases until the birthday packet proves otherwise. |
| Identity and continuity | implementation_required | `v0.91.6` identity/continuity readiness, then `v0.92` identity feature docs | Stable name, identity root, continuity head, cycle evidence, and negative cases must be reviewable before the birthday claim can pass. |
| AEE completion | implementation_required | `v0.91.6` AEE accounting and v0.91.7 runtime/provider action work | `v0.92` may consume only named AEE completion evidence and must preserve action/provider boundaries. |
| Memory/ObsMem handoff | implementation_required | `v0.91.6` AEE/Memory/ACP accounting and `v0.92` memory grounding docs | `v0.92` must distinguish ObsMem handoff, memory grounding, working set, context cache, and Memory Palace planning. |
| Memory Palace | implementation_required | `v0.92` handoff and Memory Palace feature slice | Memory Palace is required as the long-running context solution direction; birthday docs may consume only the smallest implemented/proven slice that distinguishes palace topology, working set, context cache, and ObsMem. |
| ACP/cognitive profiles | implementation_required | `v0.91.6` accounting and `v0.92` ACP feature docs | Profiles must state scope, privacy boundary, update rules, capability-envelope relation, and provider/model relation before activation consumes them. |
| Capability evidence and selector | implementation_required | `v0.91.6` identity/capability readiness; Aptitude Atlas operator_scoped_out beyond MVP | `v0.92` may consume capability-testing evidence for envelopes and role suitability, but must not start or imply a complete Aptitude Atlas baseline. |
| Provider/model matrix and multi-agent readiness | implementation_required | `v0.91.6` provider/model reliability feature doc and v0.91.7 WP-05 | Hosted, local, remote, OpenRouter, Gemma, and multi-agent lanes need role suitability, known failure modes, and proof limits before birthday demos rely on them. |
| Observatory/Unity readiness | implementation_required | `v0.91.6` Observatory/Unity consumption classification and v0.91.7 Unity/runtime proof | Each surface must be classified as proof, rehearsal, substrate, or blocked_with_evidence before a birthday demo depends on it. |
| ACIP/provider communications | implementation_required | `v0.91.6` ACIP/A2A/provider communications and `v0.91.7` WP-12, especially `docs/milestones/v0.91.7/review/security/WP12_ACCESS_ACTIVATION_GATE_4660.md` | Schema catalog, message access rules, provider communications, JSON projection, protobuf decision, WebSocket boundary, SSM/custody/credential limits, and remaining CAV blockers must be explicit before activation consumes the channel. |
| Public prompt records | implementation_required | `v0.91.6` public prompt records export feature doc | `v0.92` may consume public prompt records only after local authoring, export, redaction, validation, indexing, evidence, and security review boundaries are documented. |
| Logging/tooling proof-loop reliability | implementation_required | closed v0.91.7 issue `#4718` plus WP-07/WP-09 consumers | `#4718` is the retained prerequisite proof for parse-safe JSON, stderr `adl_event` behavior, redaction hygiene, and OTel-compatible mapping. `v0.92` may rely on C-SDLC proof-loop outputs only after runtime Soak #2 and Observatory/Unity surfaces consume that proof; Unity editor execution and production telemetry export are not claimed by `#4718` alone. |
| Security and Continuous Adversarial Verification | implementation_required | `v0.91.7` WP-12, including `#4639`, `#4656`-`#4660`, `#4914`, `#4917`, `#4920`, and the #4660 access gate | Activation cannot silently move threat-model, adversarial-output, provider-trust, public-record security, or ACIP security requirements out of scope. Rows still blocked or PR-open in the #4660 gate cannot support readiness claims. |
| Resilience, citizen persistence, and sleep/wake | implementation_required | current concrete blocker evidence for `#4783`; owners `#4778` and `#4780`-`#4782` require per-issue disposition; WP-07 runtime integration consumes the result | Transient fault handling, checkpoint/restore, sleep/wake, hibernation, simulation, in-transit custody, migration, replay, and continuity proof must be integrated_proven or blocked_with_evidence before v0.92. Current repo-visible concrete blocked-state detail is strongest for `#4783`; the rest of the resilience family is ownership that still requires proof or blocker disposition. |
| Curiosity Engine / Discovery Substrate | implementation_required | `v0.91.7` WP-10 | Curiosity is required before `v0.92` activation consumes governed discovery behavior; absent proof blocks activation with evidence and operator approval. |
| Constructability Gate | implementation_required | `v0.91.7` WP-10 | Birthday evidence must distinguish provisional cognition from authoritative shared reality. |
| Reasoning graph, loop runtime, and `adl.skill.v1` | implementation_required | `v0.91.7` WP-11 | Pre-`v0.92` implementation must connect prompts, skills, loops, trace, ObsMem, PVF, AEE, Runtime v2, UTS, ACC, and `adl.skill.v1`; deeper convergence remains later. |
| Affect, happiness, humor, and wellbeing claims | integrated_proven for operational reasoning-control; subjective affect not_claimed | `v0.91.7` WP-13 `#4752`; `docs/milestones/v0.91.7/features/AFFECT_HAPPINESS_BRIDGE_v0.91.7.md`; `docs/milestones/v0.91.7/review/wp13_affect_happiness_boundary_4752.md` | Public birthday evidence may cite bounded affect-like reasoning-control signals and safe-test language only. It must not imply hidden emotion, subjective happiness, wellbeing, suffering, consciousness, scalar happiness scores, reward channels, or public reputation. |
| Godel mechanics | integrated_proven for CSM-supervised launch admission and claim-boundary consumption; live hosted invocation and adaptive DAG completion not_claimed | `v0.91.7` WP-13 and `v0.92` birthday docs | The first true Godel-agent birthday may consume only the Runtime v2 Godel/constructability boundary: retained Godel runtime evidence, CSM-supervised launch-plan provider-request admission, constructability anchors, validator pass, and operator review. |
| Economics context | operator_scoped_out | `v0.91.7` WP-13 | Economics is context-only for `v0.92` unless a reviewed decision reopens explicit activation tests; that scoped-out posture requires retained evidence, risk, and operator approval. |
| Guild foundation | integrated_proven for governance handoff context; v0.93 governance not_claimed | `v0.91.7` WP-13 `#4755` | `v0.92` may consume the Runtime v2 guild foundation boundary for birthday governance context, identity witness evidence routing, community-memory boundary language, and future governance issue inputs only. It may not claim constitutional citizenship, polis authority, delegated governance authority, binding collective decision-making, public guild product readiness, or governance completion. |

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
- a public claim boundary for raw private-state exposure
- a public claim boundary for completed runtime behavior

## Demo And Review Expectations

The first-birthday demo/review packet should prove both positive and negative
behavior:

- valid birthday packet assembles all required evidence surfaces
- missing identity, continuity, memory, capability, witness, receipt, profile,
  or inherited governance evidence fails closed
- startup, wake, restore, snapshot, copied state, and admission fixtures are
  rejected as birth
- Observatory/Unity surfaces used by the demo are classified as proof,
  rehearsal, substrate, or blocked_with_evidence
- public prompt records used by review are exported, redacted, validated, and
  indexed under the documented public-record boundary
- provider/model lanes used by the demo are named with reliability limits

## Upstream Tranche Gate

`v0.91.6` supplies retained readiness evidence for:

- resilience, citizen persistence, sleep/wake, and continuity proof
- logging/tooling proof-loop reliability and observability consumption
- public prompt records export, redaction, validation, and indexing
- provider/model reliability and multi-agent readiness
- first ACIP/A2A/provider-communications decisions
- first security readiness and CAV decisions
- identity/continuity and capability-selector accounting
- AEE completion, Memory/ObsMem handoff, ACP/cognitive profile accounting
- Observatory/Unity consumption classification

`v0.92` should consume current `v0.91.6` closure truth from the retained
evidence matrix and current open release-tail truth from the closeout-tail
sprint surface rather than reconstructing state from individual issue histories.

`v0.91.7` must implement/prove or block with evidence and operator approval:

- Curiosity Engine / Discovery Substrate
- Constructability Gate
- reasoning graph, loop runtime, and `adl.skill.v1`
- security readiness
- ACIP/A2A/protobuf/JSON/WebSocket projection decisions
- affect/happiness operational reasoning-control boundary from `#4752`; no
  subjective affect, happiness, wellbeing, or consciousness claim
- Godel mechanics
- economics-context decision
- guild foundation boundary as handoff context, not completed governance
- integrated logging/OTel consumption from closed `#4718`
- resilience integration from `#4778` and `#4780`-`#4783`
- Rust simplification and third-party-library adoption through `#4651` and
  `#4892`-`#4900` where it affects validation cost, observability, provider
  transport, signing, secrets, or ACIP runtime streaming.

## Non-Goals

- Do not claim `v0.92` activation readiness in this ledger.
- Do not implement Memory Palace, ACIP transport, ACP profiles, resilience, or
  runtime behavior here.
- Do not absorb `v0.93` governance, `v0.94` secure execution/trust/time, or
  `v0.95` MVP convergence work.
- Do not treat planning ownership as completed proof.
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

Every named activation surface is accounted for, but activation remains blocked
until each required row is integrated_proven, operator-scoped-out with evidence,
or blocked_with_evidence and operator approval. `v0.92` can use this ledger as
a consumption map only after upstream work produces reviewed evidence or
explicit blockers.

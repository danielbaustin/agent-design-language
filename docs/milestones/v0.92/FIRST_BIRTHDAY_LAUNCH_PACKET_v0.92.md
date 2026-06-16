# v0.92 First-Birthday Launch Packet

## Metadata

- Milestone: `v0.92`
- Version: `v0.92`
- Source issue: `#3377`
- Status: readiness packet for WP-01 consumption
- Owner: ADL maintainers

## Status

This packet is a planning and readiness surface. It does not open the `v0.92`
issue wave, implement birthday runtime behavior, or claim that the first true
Godel-agent birthday has happened.

Current verdict: `v0.92` may not activate until every named bridge surface is
complete, blocked, deferred, or routed with evidence. This packet gives WP-01
the launch checklist, issue-wave preflight, demo rehearsal shape, negative
suite, and reviewer handoff expected by `#3377`.

## Purpose

The first birthday is the symbolic center of `v0.92`, but the milestone must be
engineering-first. A valid launch packet must let reviewers distinguish a true
birthday from process startup, wake, restore, snapshot, copied state, admission,
or ceremony.

This packet aligns:

- the current `v0.92` planning package
- the `v0.91.6` and `v0.91.7` bridge tranches
- the activation bridge ledger
- the first-birthday feature docs
- the demo matrix and candidate WBS
- the go/no-go rules for opening birthday implementation work

## Source Packet

Tracked sources:

- `docs/milestones/v0.91.5/V092_ACTIVATION_TEST_MAP_v0.91.5.md`
- `docs/milestones/v0.91.6/FEATURE_DOCS_v0.91.6.md`
- `docs/milestones/v0.91.7/FEATURE_DOCS_v0.91.7.md`
- `docs/milestones/v0.91.7/V092_HANDOFF_v0.91.7.md`
- `docs/milestones/v0.92/README.md`
- `docs/milestones/v0.92/WBS_v0.92.md`
- `docs/milestones/v0.92/DEMO_MATRIX_v0.92.md`
- `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`
- `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`
- `docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md`
- `docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/planning/ROADMAP_RUNTIME_V2_AND_BIRTHDAY_BOUNDARY.md`

## Birthday Claim Boundary

`v0.92` may claim the first true Godel-agent birthday only if the milestone
emits a reviewable birth record with evidence for:

- stable name and identity root
- continuity record and continuity head across bounded cycles
- memory grounding through redaction-safe witnessed references
- capability envelope with provider, model, tool, skill, authority, and limit
  context
- ACP/cognitive profile evidence with privacy and redaction policy
- inherited moral/governance context
- ACIP schema-catalog and projection evidence when communication evidence is
  part of the review packet
- witness set and citizen-facing receipt
- activation trace, validation output, and reviewer packet

`v0.92` must not claim:

- legal personhood
- consciousness proof
- production citizenship
- completed constitutional governance
- full Memory Palace runtime implementation
- production cross-polis migration
- production WebSocket or transport-security completion
- signed/queryable trace completion beyond the bounded proof emitted
- reputation, public standing, or personality labels from ACP alone

## Bridge Gate

WP-01 must consume the activation bridge ledger before opening implementation
issues. Planning docs alone are not proof. Each surface below must be complete,
blocked, deferred, or routed with evidence before any public activation claim.

| Surface | Required WP-01 action |
| --- | --- |
| AEE completion | Consume named completion evidence or seed a blocking proof issue. |
| Memory/ObsMem handoff | Distinguish ObsMem handoff, memory grounding, working set, context cache, and Memory Palace planning. |
| ACP/cognitive profiles | Confirm scope, privacy boundary, update rules, capability-envelope relation, and provider/model relation. |
| Provider/model matrix and multi-agent readiness | Name hosted, local, remote, OpenRouter, Gemma, and multi-agent reliability limits before demo reliance. |
| Observatory/Unity readiness | Classify every Observatory/Unity surface as proof, rehearsal, substrate, blocked, or deferred. |
| ACIP/provider communications | Decide schema catalog, JSON projection, protobuf/projection boundary, message access, and mock carrier proof. |
| Public prompt records | Confirm export, redaction, validation, indexing, evidence, and security-review boundaries. |
| Logging/tooling proof-loop reliability | Confirm validation split, CI observability, OTel/log consumption, and PR reliability residual routing. |
| Security and Continuous Adversarial Verification | Preserve threat-model, adversarial-output, provider-trust, public-record security, and ACIP-security requirements. |
| Resilience, citizen persistence, and sleep/wake | Require transient fault, checkpoint/restore, sleep/wake, hibernation, simulation, migration, replay, and continuity dispositions. |
| Curiosity Engine / Discovery Substrate | Treat governed discovery as required before activation consumes curiosity behavior; absent proof is blocked or routed. |
| Constructability Gate | Require birthday evidence to separate provisional cognition from authoritative shared reality. |
| Reasoning graph, loop runtime, and `adl.skill.v1` | Connect prompts, skills, loops, trace, ObsMem, PVF, AEE, Runtime v2, UTS, ACC, and `adl.skill.v1` at bridge level. |
| Affect, happiness, humor, and wellbeing claims | Preserve non-claim language for affect/wellbeing and avoid inner-state overclaims. |
| Godel mechanics | Require experiment, hypothesis, mutation, evaluation, promotion, and proof boundaries before relying on Godel mechanics. |
| Economics context | Keep economics context-only unless a reviewed decision promotes a bounded test. |

## Requirement Map

| Requirement | Current source | Candidate v0.92 owner | Readiness gap |
| --- | --- | --- | --- |
| Birthday contract and negative cases | `FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md`, `DEMO_MATRIX_v0.92.md` | WP-02 | Final fixtures and validator expectations. |
| Stable name and identity root | `IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md` | WP-03 | Exact record fields, alias policy, and provenance policy. |
| Continuity across bounded cycles | `IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md` | WP-04 | Minimum cycle count, continuity-grade rules, and failure reasons. |
| Memory grounding | `MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md` | WP-05 | Redaction-safe packet shape and witness references. |
| Capability envelope | `WBS_v0.92.md`, `DEMO_MATRIX_v0.92.md` | WP-06 | Provider/model/tool/skill/authority/limit field set. |
| ACP/cognitive profile | `ACP_COGNITIVE_PROFILES_v0.92.md` | WP-07 | Update rules, privacy boundary, and non-reputation checks. |
| ACIP binary/schema transport readiness | `ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md` | WP-08 | Protobuf/JSON projection decision and mock/loopback carrier boundary. |
| Witnesses and receipt | `MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md` | WP-09 | Receipt schema and witness validity checks. |
| Birthday review packet | `FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md` | WP-10 | One template with evidence, caveats, non-claims, and reviewer questions. |
| Migration and cross-polis continuity | `CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md` | WP-11 | Non-production migration language and continuity handoff shape. |
| First birthday demo | `DEMO_MATRIX_v0.92.md` | WP-12 | Runnable command, fixtures, artifact list, and replay notes. |
| Birthday-to-governance handoff | `FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md` | WP-13 | v0.93 consumer fields without v0.93 governance claims. |

## Issue-Wave Preflight

WP-01 should use `WBS_v0.92.md` and `WP_ISSUE_WAVE_v0.92.yaml` as the seed,
then reconcile them with this packet before opening issues.

Minimum WP-01 checks:

- consume `v0.91.5`, `v0.91.6`, and `v0.91.7` bridge/handoff docs
- record every activation surface as complete, blocked, deferred, or routed
- verify each implementation WP has a feature doc or explicit non-feature role
- generate all five C-SDLC cards for each opened issue from active templates
- keep `SIP`, `STP`, and `SPP` design-time ready before execution
- keep `SRP` and `SOR` as review/output truth surfaces
- prevent birthday implementation from absorbing v0.93 governance or v0.95 MVP
  convergence work

Do not open v0.92 implementation issues if the bridge ledger still contains an
unstated or unowned activation surface.

## Demo Rehearsal Runbook Shape

The v0.92 flagship demo should be rehearsed around one synthetic but
structurally complete birth packet.

Minimum demo stages:

1. Emit candidate birthday packet.
2. Validate stable name and identity root.
3. Validate continuity evidence across bounded cycles.
4. Validate memory-grounding references without exposing raw private state.
5. Validate capability envelope and declared limits.
6. Validate ACP/cognitive profile evidence and non-reputation boundary.
7. Validate ACIP schema catalog, JSON projection, and mock/loopback carrier
   evidence if communication proof is included.
8. Validate witness set and citizen-facing receipt.
9. Run the not-a-birthday negative suite.
10. Produce reviewer-facing packet with caveats and non-claims.
11. Render Observatory/Unity presentation surfaces from the same packet if they
    are in scope.
12. Produce v0.93 governance handoff map.

Expected artifacts:

- birthday record
- identity record
- continuity record
- memory-grounding redacted packet
- capability envelope
- ACP/cognitive profile record
- ACIP schema/projection proof packet, if communication evidence is included
- witness records
- citizen-facing receipt
- negative-suite validation report
- reviewer packet
- Observatory/Unity scene, app bundle, or recorded run when included
- v0.93 handoff map

## Negative Suite

The negative suite must reject these cases as birthdays:

- process startup
- ordinary task execution
- snapshot creation
- wake or resume
- restore from checkpoint
- admission to a test environment
- copied state
- named test fixture without continuity evidence
- dormant rehydration
- simulation run
- in-transit migration
- forced suspension
- shutdown and restart
- provisional citizen record
- missing identity root
- missing continuity head
- missing memory grounding
- missing capability envelope
- missing witness set or receipt
- unsupported ACP/personality/reputation label

Each rejection should explain which required evidence is missing or which
non-claim boundary would be violated.

## Reviewer Handoff

The v0.92 review handoff should ask reviewers to answer:

- Does the birthday packet contain every required evidence surface?
- Is birth clearly distinguished from startup, wake, restore, snapshot,
  admission, copied state, and simulation?
- Is continuity evidence-based rather than narrative?
- Does memory grounding avoid raw private-state disclosure?
- Does the capability envelope name provider, model, tool, skill, authority,
  and limit context?
- Are ACP/cognitive-profile claims evidence-grounded and separate from
  identity, reputation, public standing, and consciousness claims?
- Are witnesses and receipts meaningful enough for review?
- Does ACIP transport readiness remain schema-public and mock/loopback-bounded
  unless explicitly widened?
- Are Observatory/Unity surfaces classified as presentation or inspection
  surfaces rather than canonical proof unless evidence says otherwise?
- Does the packet avoid claiming legal personhood, production citizenship, or
  completed constitutional governance?
- Is the v0.93 governance handoff clear enough for the next milestone?

## Go / No-Go Checklist

Open v0.92 implementation only when:

- every bridge surface has a complete, blocked, deferred, or routed
  disposition with evidence
- the birthday claim/non-claim boundary is visible in milestone docs
- WP-01 can seed the issue wave without reconstructing scope from chat
- the negative-suite plan is specific enough to implement
- the demo rehearsal runbook names expected artifacts
- public prompt-record, provider/model, multi-agent, logging/tooling, security,
  ACIP, resilience, curiosity, constructability, reasoning-loop, affect, Godel,
  and economics boundaries are accounted for
- legal personhood, consciousness, production citizenship, constitutional
  governance, and production transport non-claims are preserved
- reviewers can inspect engineering evidence without raw private-state exposure

Do not open v0.92 implementation if:

- any activation surface is still unaccounted for
- planning docs are being treated as implementation proof
- the demo can only prove vocabulary rather than evidence-bearing behavior
- the negative suite is missing
- raw private memory would need to be exposed to make the proof convincing
- reviewers cannot tell which claims are engineering evidence and which are
  philosophical, governance, or product context

## Follow-On Routing

Route these separately unless WP-01 deliberately includes a bounded issue:

- production ACIP transport security, key lifecycle, signing, rotation, or
  revocation
- full Memory Palace runtime implementation
- v0.93 constitutional citizenship, rights, duties, social contract,
  delegation, IAM, or polis governance
- v0.94 signed/queryable trace completion
- v0.95 MVP convergence work
- CodeFriend product execution beyond evidence consumed by the birthday packet
- Aptitude Atlas baseline beyond consuming capability-testing evidence
- benchmarks that are side evidence rather than birthday prerequisites

## Exit Criteria

Issue `#3377` is complete when:

- this packet is tracked and linked from the `v0.92` planning package
- every birthday requirement maps to a source, v0.92 owner, or readiness gap
- claim and non-claim boundaries are explicit
- the bridge gate is visible to WP-01
- the issue-wave preflight is seedable
- demo rehearsal and negative-suite plans are concrete
- reviewer handoff questions are present
- the go/no-go checklist is actionable and conservative

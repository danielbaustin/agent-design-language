# Identity, Continuity, And Capability Selector Bridge

## Metadata

- Feature Name: Identity, Continuity, And Capability Selector Bridge
- Milestone Target: `v0.91.6`
- Status: wp_08_closeout_packet_authored
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: architecture, policy
- Proof Modes: review, replay

## Purpose

Connect capability evidence, identity continuity, resilience, and negative
cases before `v0.92` consumes birthday or activation identity claims.

## Scope

In scope:

- capability evidence consumption;
- identity continuity boundaries;
- negative cases and invalid continuity claims;
- resilience and persistence dependencies;
- Aptitude Atlas boundary.

Out of scope:

- Aptitude Atlas productization;
- full identity runtime implementation;
- Memory Palace implementation.

## Required Decisions

- Which capability evidence may feed `v0.92`?
- Which identity continuity claims require replay or witness proof?
- Which negative cases invalidate continuity?
- Which surfaces route to Memory Palace or `v0.91.7`?

## Dependencies

- Resilience, persistence, and sleep/wake feature doc.
- Provider/model reliability feature doc.
- `v0.92` identity and birthday docs.

## Source Boundary Inputs

This bridge consumes the currently tracked `v0.91.6` boundary inputs rather
than inventing a new provider or identity catalog:

- `docs/milestones/v0.91.6/features/PROVIDER_MODEL_RELIABILITY_v0.91.6.md`
- `docs/milestones/v0.91.6/review/provider/PROVIDER_CAPABILITY_PROFILE_CATALOG_4007.md`
- `docs/milestones/v0.91.6/review/provider/PROVIDER_PROFILES_V2_RECONCILIATION_4111.md`
- `docs/milestones/v0.91.6/features/ACIP_A2A_PROVIDER_COMMUNICATIONS_v0.91.6.md`
- `docs/milestones/v0.91.6/features/RESILIENCE_PERSISTENCE_SLEEP_WAKE_v0.91.6.md`

The authored issue input named `ADL_PROFILES_PROVIDERS_V2.MD`, but that path is
not the live tracked source of truth in the current repository. WP-08 consumes
the tracked provider/profile and ACIP feature surfaces above instead.

## I-00 Boundary Decision

`#4025` establishes the first WP-08 boundary vocabulary that downstream
identity, capability-evidence, continuity, and selector work must preserve.

| Surface | Owns | Must remain distinct from |
| --- | --- | --- |
| Provider profile | infrastructure/service identity, locality, endpoint class, substrate defaults, operator-managed auth expectations | capability need, civil identity, citizen record, institution record, guild policy |
| Capability profile | provider-independent behavioral capability descriptors, interaction modes, determinism/safety posture, tool-orchestration posture | vendor identity, endpoint configuration, civil identity, institution record |
| Identity profile vocabulary | planning-only higher-layer identity attributes that may later help describe continuity or selector-relevant identity posture | provider substrate, model identity, role authority, or a settled first-class runtime/schema contract in `v0.91.6` |
| Citizen record | governance/continuity-facing record that may later constrain what routes or actions are legitimate | provider profile, raw capability evidence, guild membership alone |
| Guild | MVP-scope policy/grouping input that may influence capability discovery or route narrowing | runtime execution authority, transport substrate, provider identity |
| Institution | higher-layer organizational identity/governance surface | provider substrate, capability profile, guild policy by itself |

The non-collapse rule for WP-08 is:

- provider profiles remain substrate descriptors;
- capability profiles remain behavioral descriptors;
- identity/citizen/institution surfaces remain governance/continuity surfaces;
- guilds remain MVP policy/grouping inputs;
- no single profile object becomes a disguised union of provider, capability,
  identity, citizen, and institution state.

## Selector Inputs And Non-Inputs

The capability selector MVP may consume only bounded higher-layer inputs.

Allowed selector inputs:

- requested capability need;
- capability evidence and freshness posture;
- provider/model suitability evidence from WP-05;
- guild-policy inputs where the route policy explicitly allows them;
- identity/citizen continuity posture only when a later issue proves the
  relevant continuity constraint and non-claim boundary.

Selector non-inputs for `v0.91.6`:

- raw provider identity as the primary routing primitive;
- civil personhood or unproved continuity assertions;
- Aptitude Atlas productized scoring/badging;
- institutional authority inferred from provider or capability metadata alone;
- guild presence treated as runtime execution authority by itself.

## Capability Evidence Inputs

Capability evidence may inform selector behavior only when it is sourced from the bounded `v0.91.6` provider/capability reliability surfaces explicitly consumed by this bridge. Identity continuity must consume that evidence as advisory input rather than as actor identity, civil truth, or institutional authority.

| Evidence surface | Allowed selector use | Not allowed to imply |
| --- | --- | --- |
| `PROVIDER_MODEL_RELIABILITY_v0.91.6.md` | Role-scoped provider/model suitability summaries such as `supported_with_limits`, `useful_with_limits`, `candidate`, or `blocked` | Personal identity, civil standing, or authority to act |
| `PROVIDER_PROFILES_V2_RECONCILIATION_4111.md` and merged WP-05 proof surfaces | Current tracked provider/capability boundary truth and lane-specific reliability evidence that already landed in `v0.91.6` | Broad competence, merge authority, or autonomous workflow approval |
| Provider proof packets and closeout matrices cited by WP-05 | Lane-specific reliability or failure-mode evidence when the cited packet still supports the claim | Permanent trust, provider-family superiority, or actor continuity |

## Evidence Ingestion Boundary

The selector may ingest only normalized capability evidence that answers bounded questions such as:

- which advisory role lanes are currently evidenced, blocked, or candidate-only;
- which runtime surface produced the evidence;
- which failure class or limit note must stay attached to the lane;
- which evidence packet is the strongest currently tracked proof.

The selector must not ingest raw provider configuration, credentials, citizen records, guild membership, institution membership, or identity continuity state as if they were capability evidence. Provider profiles remain infrastructure descriptors, and capability evidence remains role-scoped behavioral evidence.

### Normalized evidence fields

The minimal identity-side evidence tuple is:

| Field | Meaning |
| --- | --- |
| `provider_profile_ref` | Deterministic provider or route identifier from the cited provider surface |
| `role_profile` | Advisory role being discussed, such as watcher, reviewer, planner, implementer, or tester |
| `evidence_status` | Bounded status such as `supported_with_limits`, `useful_with_limits`, `candidate`, `blocked`, or panel-level `pass_with_limits` |
| `proof_ref` | Strongest tracked packet or feature-doc reference supporting the row |
| `limit_notes` | Required non-claims, failure notes, or authority limits that must survive selection |
| `observed_at` | Timestamp or milestone-time marker for freshness checks |

## Negative Cases And Stale-Evidence Rules

The selector must treat capability evidence as unusable when any of the following is true:

- the source packet is explicitly historical, superseded, or weaker than a newer tracked proof;
- the evidence does not preserve its authority limits or failure notes;
- the lane is recorded as `candidate`, `blocked`, `skipped_blocked`, `fail_truth`, `fail_authority`, or `timeout_or_empty`;
- the evidence depends on credentials, runtimes, or local-model availability that are not currently present;
- the evidence row cannot be tied back to a durable proof reference.

Freshness rule:

- capability evidence should be treated as advisory only within the milestone and proof context that produced it;
- if a newer suitability packet, provider closeout packet, or milestone review contradicts an older row, the newer tracked evidence wins;
- if no current proof exists, the selector must downgrade the lane to unknown or candidate rather than carrying forward stale confidence.

## Aptitude Atlas Non-Claim

This milestone still does not productize Aptitude Atlas as a general capability-ingestion or universal aptitude-ranking system. The identity bridge may later consume bounded capability evidence, but `v0.91.6` does not claim:

- a first-class Aptitude Atlas runtime service;
- universal cross-provider aptitude scoring;
- identity continuity derived from provider/model performance;
- selector authority to promote a lane beyond what the cited proof explicitly supports.

## Citizen And Guild Planning Hooks

WP-08 keeps citizen and guild concepts visible without pretending the full
runtime or governance layer already exists.

Citizen planning hooks:

- continuity-sensitive selector narrowing may later consume citizen-record
  posture;
- continuity claims remain evidence-bound and must not be inferred from labels
  alone;
- citizen state depends on later resilience and persistence proof, and any
  privacy-sensitive publication or display claims must remain separately
  reviewed rather than inferred here.

Guild planning hooks:

- guilds may later act as policy/grouping inputs for capability discovery or
  route narrowing;
- guilds are not yet first-class runtime authority objects;
- guild membership must not bypass higher-layer authority or security review.

## Aptitude Atlas Boundary

WP-08 may consume bounded capability-testing evidence, but `v0.91.6` still does
not claim:

- an Aptitude Atlas product baseline;
- a shipped capability-market runtime;
- selector decisions driven by productized aptitude scores;
- continuity proof derived from capability-testing output.

## Identity Continuity Positive Cases

Identity continuity in `v0.91.6` is only meaningful when the claim can be tied
back to concrete resilience and security prerequisites rather than labels or
chat-memory alone.

| Case | What must be true | Why it is positive but still bounded |
| --- | --- | --- |
| Reviewed capability-selection continuity | The selector decision is tied to current `v0.91.6` capability evidence with proof references, freshness posture, and preserved limit notes | Shows that a later identity-sensitive route can reuse reviewed evidence without claiming civil or personhood continuity |
| Checkpoint-aware continuity planning | The continuity claim names checkpoint/restore and replay as prerequisites from `RESILIENCE_PERSISTENCE_SLEEP_WAKE_v0.91.6.md` rather than silently assuming durable state | Keeps continuity connected to the resilience substrate instead of a label-only identity story |
| Sleep/wake guarded continuity | A future sleep/wake or wake-after-pause claim stays explicitly blocked until replayable transition proof exists | Preserves an honest bridge from present planning vocabulary to later runtime proof |
| Privacy-reviewed identity-sensitive publication | Any public prompt, Observatory, or inhabitant-facing use names the WP-07 security bridge as the publication/privacy guard instead of assuming identity-safe display by default | Keeps continuity-related publication claims subordinate to reviewed security surfaces |

## Identity Continuity Negative Cases

The bridge must treat continuity as invalid, downgraded, or blocked when any of
the following conditions hold:

| Negative case | Why continuity fails or downgrades | Required handling |
| --- | --- | --- |
| Label-only continuity | The system has a stable name, label, or role tag but no checkpoint, replay, or reviewed evidence chain | Reject the continuity claim as non-proving |
| Interrupted evidence custody | Capability evidence, resilience artifacts, or route-selection rationale cannot be linked to durable proof references | Downgrade to unknown or candidate-only continuity posture |
| Replay gap | The claimed same-entity or same-session continuation depends on sleep/wake, restore, or migration behavior that WP-02 still marks residual | Block the continuity claim pending replay proof |
| Provider-performance inference | The claim tries to infer continuity from provider/model reliability or capability results alone | Reject as category error; provider evidence is advisory input only |
| Unreviewed publication or display | Identity-sensitive continuity is exposed through public records, Observatory, or cross-agent messages without the relevant WP-07 security review path | Treat the publication/display claim as blocked even if internal planning vocabulary exists |
| Guild or citizen overreach | Guild presence or citizen posture is used as if it were autonomous runtime authority or proven durable identity | Reject the authority upgrade and route it back to planned governance surfaces |

## Resilience And Provider Dependency Links

Identity continuity in this bridge depends on two different but cooperating
proof families:

- resilience proof from `RESILIENCE_PERSISTENCE_SLEEP_WAKE_v0.91.6.md` for
  checkpoint shape, restore expectations, sleep/wake replay boundaries, and
  in-transit custody expectations;
- provider and capability proof from
  `PROVIDER_MODEL_RELIABILITY_v0.91.6.md` for bounded advisory suitability
  evidence, failure classes, and freshness-limited route selection.

Those dependencies must remain distinct:

- provider/model reliability can help explain which advisory lane was chosen;
- resilience can help explain whether a continuity-sensitive claim has replay or
  custody support;
- neither proof family by itself is enough to establish durable identity
  continuity.

## Privacy And Security Links

Continuity-sensitive identity claims must also preserve the WP-07 security
boundaries documented in `SECURITY_BRIDGE_AND_CAV_v0.91.6.md`. In particular:

- public prompt records and publication surfaces are not identity-safe by
  default;
- ACIP/A2A or provider-message transport does not grant trusted identity
  continuity by itself;
- Observatory or inhabitant-facing consumption remains blocked from silent
  identity upgrades while WP-08, WP-09, and WP-10 stay open;
- malformed-output, prompt-injection, or provider-trust failures must be
  treated as potential continuity-breaking events rather than cosmetic errors.

A continuity claim is therefore only eligible for later `v0.92` consumption
when all three boundaries line up:

1. capability evidence is current and reviewable;
2. resilience replay/custody prerequisites are satisfied or explicitly blocked;
3. privacy/publication and transport security reviews do not leave the claim in
   an unreviewed exposure path.

## Capability Selector MVP Contract

The `v0.91.6` selector MVP is capability-first. It does not begin from a
provider name, model id, or identity label. It begins from a requested
capability need plus explicit constraints and then narrows the candidate set
through the reviewed bridge layers.

### Selector input contract

The minimum selector request must be expressible as:

| Input field | Meaning |
| --- | --- |
| `requested_capability` | The named capability or capability family being sought |
| `proof_posture` | Whether the caller needs reviewed evidence, stronger safety posture, replay sensitivity, or a refusal when proof is absent |
| `constraints` | Latency, cost, tool-use, safety, locality, or other bounded route constraints |
| `allowed_candidate_classes` | Which implementation-facing classes may remain eligible after policy narrowing: capability-policy, role-provider, provider-profile, or model candidates |
| `governance_context` | Optional guild, citizen, member, workflow-role, or institutional policy posture that may narrow but not redefine the request |
| `refusal_posture` | What to do when no safe candidate exists: refuse, escalate, or return blocked/routed status |

The selector request must not require:

- a provider name as the primary routing key;
- a model identifier as the primary routing key;
- an endpoint or credential surface;
- a collapsed record that mixes provider substrate, capability need, and
  identity authority into one object.

### Selector output contract

The selector output for `v0.91.6` is still an advisory bridge record, not a
full runtime execution order. The minimum output should declare:

| Output field | Meaning |
| --- | --- |
| `selected_candidate_class` | Which non-governance candidate layer satisfied the decision posture after policy narrowing |
| `candidate_set_summary` | The narrowed candidate families still in scope after policy checks |
| `provider_or_model_candidates` | Optional lower-layer provider/model candidates when the capability-first checks passed |
| `continuity_posture` | Whether the route is continuity-safe, blocked, downgraded, or requires later replay/security proof |
| `proof_refs` | Evidence packets and bridge sections supporting the decision |
| `limit_notes` | Non-claims, refusal notes, or residual routes that must stay attached |
| `decision_status` | `candidate`, `supported_with_limits`, `blocked`, `deferred`, or similar bounded posture |

The selector output must not claim:

- final autonomous execution authority;
- personhood or durable identity continuity;
- guild or citizen legitimacy from provider suitability alone;
- a single universally best provider field.

## Candidate Source Ordering

WP-08 adopts the same capability-first discovery ordering established by the
ACIP delegation contract:

1. interpret the requested capability and explicit constraints;
2. determine the allowed candidate classes for that request;
3. narrow the set through guild, citizen, member, workflow-role, or
   institutional policy only when those higher-layer inputs are explicitly
   allowed;
4. consult capability evidence and role-provider policy before resolving any
   concrete provider or model candidate;
5. resolve provider-profile and model candidates only after the higher-layer
   capability and governance checks pass;
6. fail closed, defer, or escalate when no candidate satisfies the requested
   capability and security posture.

This ordering preserves the non-collapse rule: provider or model selection may
implement the route, but it is not the meaning of the request.

## Provider, Citizen, And Guild Boundary In The Selector

The selector MVP must preserve three distinct responsibilities:

| Surface | What it may do in the selector | What it must not do |
| --- | --- | --- |
| Provider/model candidate | Offer a lower-layer implementation candidate after capability and policy narrowing | Define the meaning of the request, prove continuity, or authorize itself |
| Citizen posture | Narrow or block continuity-sensitive routes when later governance or continuity constraints apply | Replace capability semantics or masquerade as provider selection |
| Guild posture | Narrow discovery and route policy for an allowed request | Act as direct runtime authority or proof of durable continuity |

Institutional or workflow-role context may also narrow routes, but the same
rule applies: governance context may constrain a request, not replace the
capability-first contract. Governance is therefore a narrowing or blocking
filter, not a satisfying candidate class in the selector output.

## Selector Validation And Proof Expectations

A selector claim is only acceptable for `v0.91.6` bridge consumption when it
shows all of the following:

- the request can be expressed by capability need first rather than provider
  identity first;
- provider/model names appear as candidates or implementation options, not as
  the only primitive;
- guild/citizen/institution context is clearly marked as narrowing policy input
  rather than substrate authority;
- continuity-sensitive routes preserve the positive/negative case and
  resilience/security gates added by `#4027`;
- blocked, deferred, or residual states remain explicit when the bridge lacks
  runtime proof or governance closure.

## Selector MVP Acceptance Surface

Issue `#4028` should only be treated as satisfied when the bridge leaves a
reviewable acceptance surface that proves all of the following:

| Acceptance item | Required proof surface in `v0.91.6` |
| --- | --- |
| Capability-first request shape exists | This bridge doc names the minimum selector input record and explicitly rejects provider-first or model-first routing keys |
| Advisory selector output shape exists | This bridge doc names the minimum selector output record, including candidate summary, continuity posture, proof refs, and explicit decision status |
| Governance is narrowing-only | This bridge doc states that guild, citizen, institutional, and workflow-role context may narrow or block routes but may not become the satisfying candidate class |
| Candidate ordering is deterministic at the policy level | This bridge doc records the ordered discovery posture from capability interpretation through lower-layer provider/model candidate selection |
| Blocked and deferred states stay visible | This bridge doc preserves blocked, deferred, residual, and non-claim routing rather than implying a full runtime selector exists |

The proving artifact for this issue is the reviewed bridge contract itself plus
the issue-local SRP/SOR truth that records bounded docs review and focused
validation. `#4028` does not claim a shipped runtime selector, executable
fixture, or end-to-end provider-routing implementation in `v0.91.6`.

## Bridge Completion And Evidence Classification

This bridge is not complete merely because the vocabulary exists. `v0.92`
consumption should classify the bridge state using the following evidence rules:

| State | Meaning | Required evidence |
| --- | --- | --- |
| `complete_for_bridge_consumption` | The bridge vocabulary, continuity cases, negative cases, and dependency links are all present and have reviewed issue-truth surfaces | Merged WP-08 bridge issues, bounded review packets, and explicit residual non-claims still preserved |
| `blocked_on_replay_or_custody_proof` | A continuity-sensitive claim depends on checkpoint, restore, sleep/wake, migration, or custody proof that WP-02 still routes as residual | Named replay/custody gap tied back to `RESILIENCE_PERSISTENCE_SLEEP_WAKE_v0.91.6.md` and the affected continuity claim downgraded or blocked |
| `blocked_on_security_exposure_review` | A publication, Observatory, inhabitant, or transport-facing continuity claim has not yet cleared the relevant WP-07 or downstream security review boundary | Named WP-07/WP-09/WP-10 review dependency and no silent upgrade to safe publication |
| `deferred_to_future_runtime_work` | The bridge names a valid future continuity or selector behavior but runtime implementation or proof is intentionally not part of `v0.91.6` | Explicit route to later milestone/runtime work with no present-tense claim |
| `routed_non_claim` | The bridge explicitly rules a tempting claim out of scope, such as personhood, productized Aptitude Atlas, or provider-derived continuity | The non-claim remains visible in the bridge and any dependent proof packet |

Bridge completion for this issue wave requires at least:

- reviewed positive and negative continuity cases;
- explicit distinction between advisory capability evidence and durable continuity proof;
- named resilience prerequisites for replay, custody, sleep/wake, or restore-sensitive claims;
- named privacy/security gates for publication, transport, or inhabitant-facing exposure;
- explicit downgrade or block behavior when those prerequisites are absent.

Anything less should be recorded as blocked, deferred, or routed rather than
quietly treated as complete continuity support.

## WP-08 Completion Matrix

WP-08 closeout is only valid if the identity/capability bridge records what was
actually completed in each child issue and which claims remain routed or
bounded.

| Child issue | Delivered surface | Bridge truth in this closeout packet |
| --- | --- | --- |
| `#4025` `I-00` | Provider/capability/citizen/guild/institution boundary vocabulary | Complete as the non-collapse baseline for later WP-08 slices |
| `#4026` `I-01` | Capability evidence ingestion boundary and stale-evidence rules | Complete as the advisory capability-evidence intake posture for `v0.91.6` |
| `#4027` `I-02` | Positive and negative continuity cases plus resilience/security dependency links | Complete as the continuity-claim guardrail layer; replay and publication residuals remain explicit |
| `#4028` `I-03` | Capability-selector MVP bridge contract, candidate ordering, and governance narrowing rules | Complete as a capability-first advisory selector contract; no runtime selector implementation is implied |
| `#4029` `I-04` | Final closeout matrix, validation/review link surface, and residual routing | This closeout packet records the final reviewer-facing bridge truth and becomes terminal when merged |

Once `#4029` merges, WP-08 should therefore be treated as:

- `complete_for_bridge_consumption` for the reviewed boundary vocabulary and
  advisory bridge rules documented here;
- `blocked_on_replay_or_custody_proof` for any durable continuity claim that
  needs replay, restore, migration, or custody proof beyond the current bridge;
- `blocked_on_security_exposure_review` for any publication, Observatory, or
  transport-facing identity claim that exceeds the named WP-07/WP-09/WP-10
  review boundaries;
- `deferred_to_future_runtime_work` for any runtime selector, durable identity,
  or Aptitude Atlas product behavior not explicitly proved in `v0.91.6`.

## Validation And Review Links

The closeout-ready bridge depends on the merged and reviewed issue wave rather
than on one prose block alone. The minimum proving surfaces are:

- `#4025` boundary vocabulary merged through PR `#4194`;
- `#4026` capability-evidence ingestion merged through PR `#4195`;
- `#4027` continuity positive/negative cases merged through PR `#4197`;
- `#4028` selector MVP bridge merged through PR `#4198`;
- issue-local SRP/SOR truth for each child issue capturing bounded review and
  focused validation;
- this bridge doc as the consolidated reviewer-facing WP-08 surface.

The proving bar for WP-08 is documentation and review truth, not runtime-demo
theater. No section of this bridge should be read as proof of a shipped identity
runtime, durable citizen substrate, or provider-executing selector.

## Residual List And Owners

| Residual area | Current owner / route | Why it remains residual |
| --- | --- | --- |
| durable continuity replay and custody proof | WP-02 resilience substrate and later continuity work | `v0.91.6` names prerequisites but does not prove a durable same-entity runtime loop |
| publication and inhabitant-safe identity exposure | WP-07 security bridge plus WP-09 and WP-10 open implementation owners | Identity-sensitive display and ingestion are still blocked from silent promotion |
| runtime capability-selector implementation | future runtime / activation work after the bridge | WP-08 proves the contract shape, not an executable selector service |
| guild/citizen/institution governance realization | later governance and identity milestone work | `v0.91.6` keeps these as narrowing or policy surfaces, not first-class runtime authority |
| Aptitude Atlas productization | post-`v0.95` route only | WP-08 consumes bounded evidence but does not create an Atlas product baseline |

## v0.92 Bridge Status

For `v0.92`, this bridge now provides:

- reviewed provider/capability/identity boundary vocabulary;
- reviewed capability-evidence ingestion rules;
- reviewed continuity positive/negative case language;
- a reviewed capability-first selector MVP contract;
- explicit blocked/deferred/residual routing for what is still not true.

What is not true after WP-08 closeout:

- `v0.92` does not inherit a shipped runtime selector or durable continuity
  engine from this bridge;
- `v0.92` does not inherit publication-safe identity exposure by default;
- `v0.92` does not inherit personhood, citizen legitimacy, or institutional
  authority from provider or capability evidence;
- `v0.92` does not inherit Aptitude Atlas productization.

## Validation And Review

- Review identity claims against evidence and non-goals.
- Require negative-case language for continuity boundaries.
- Ensure capability evidence is consumed without Aptitude Atlas product claims.

## v0.92 Consumption

`v0.92` may consume the WP-08 boundary vocabulary and capability-evidence
posture established here. It must not:

- collapse provider, capability, identity, citizen, guild, and institution
  layers into one selector object;
- treat capability testing as Aptitude Atlas productization;
- treat continuity-sensitive identity claims as already proved;
- treat guild or citizen inputs as shipped runtime authority objects.

## Non-Goals

- No productized Aptitude Atlas baseline.
- No unproved personhood, continuity, or memory claims.
- No hidden Memory Palace implementation.

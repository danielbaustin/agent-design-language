# AEE, Memory/ObsMem, And ACP Bridge Accounting

## Metadata

- Feature Name: AEE, Memory/ObsMem, And ACP Bridge Accounting
- Milestone Target: `v0.91.6`
- Status: planned_with_aee_obsmem_memory_palace_and_acp_routing_state_explicit
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: architecture, policy
- Proof Modes: review, replay

## Purpose

Account for AEE completion, Memory/ObsMem handoff, and ACP/cognitive profile
scope before `v0.92` consumes activation or birthday claims.

## Scope

In scope:

- AEE completion boundary;
- residual runtime/provider action routing;
- ObsMem handoff;
- Memory Palace planning state;
- ACP/cognitive profile scope and privacy boundary;
- `v0.92` consumption limits.

Out of scope:

- Memory Palace implementation;
- full ACP runtime;
- provider action implementation.

## Required Decisions

- Which AEE boundaries are complete enough to consume?
- Which Memory/ObsMem handoff artifacts are required?
- Which cognitive profile fields are private, public, or blocked?
- Which provider/runtime action residuals block activation?

## Dependencies

- Identity/continuity bridge feature doc.
- Resilience and persistence feature doc.
- Security bridge feature doc.
- `v0.92` ACP and memory grounding docs.

## WP-10 Completion Ledger

WP-10 cannot be closed from broad milestone narrative alone. Each memory-facing
surface must carry a terminal or explicitly routed state before `v0.92`
consumes it.

| Surface | Current `v0.91.6` owner | What counts as complete in this ledger | Current ledger state |
| --- | --- | --- | --- |
| AEE completion and readiness | `#4037` | AEE is classified as complete, blocked, or explicitly routed with proof limits and `v0.92` status | routed with named `v0.92` proof owners |
| Memory/ObsMem handoff | `#4038` | Memory/ObsMem boundary is distinct, evidence-backed, and privacy-constrained | routed with explicit handoff boundary and `v0.92` consumption limits |
| Memory Palace long-context bridge | `#4039` | Long-context solution path is explicit with proof or explicitly accepted residual routing | routed with explicit long-context boundary and first proof shape |
| ACP/cognitive profile scope and privacy boundary | `#4040` | ACP and cognitive-profile boundaries stay distinct from provider/capability/identity surfaces and preserve privacy/security rules | routed with explicit ACP boundary, privacy rules, and non-collapse constraints |
| WP-10 feature closeout matrix | `#4041` | All child surfaces have terminal status and the final closeout packet preserves residual truth | planned closeout issue |

## Feature Surface Inventory

The WP-10 ledger must keep the following surfaces separate:

| Surface family | What it is | What it is not |
| --- | --- | --- |
| AEE | Completion and readiness accounting for the AEE program and its `v0.92` consumption limits | Proof that every future runtime or identity feature is already done |
| Memory/ObsMem handoff | The boundary where memory-facing surfaces hand off to ObsMem with privacy and interface constraints | The whole Memory Palace or long-context solution |
| Working set and context cache | Additional memory-handling surfaces that `v0.92` must distinguish explicitly from ObsMem handoff and Memory Palace planning | Proof that long-running memory/runtime completion is already done here |
| Memory Palace | The long-context architecture/proof route under active development | A silently deferred or forgotten future bucket |
| ACP/cognitive profiles | Cognitive-profile scope, access, privacy, and relationship to other profile families | Provider profiles, capability profiles, or public identity records by default |
| Privacy/security consumption | Bounded security review inputs from WP-07 and later lanes | Automatic closure of open WP-10 implementation work |

## AEE Completion And Readiness Status

Current truthful classification:

- `routed_with_named_v0_92_proof_requirements`

AEE has a real baseline and a real closure definition, but this issue does not
prove subsystem completion in `v0.91.6`.

Evidence consumed for this status:

- `docs/milestones/v0.91.5/features/AEE_COMPLETION_TRANCHE_v0.91.5.md`
- `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`
- this `v0.91.6` bridge ledger

What is already explicit enough to consume as routed planning truth:

| AEE surface | Current evidence-backed truth | Why it is not full completion |
| --- | --- | --- |
| Baseline existence | AEE already has bounded retry, policy hooks, convergence docs, and runtime-adjacent control surfaces | Baseline existence is not closure proof |
| Closure criteria | The closure components are explicit: steering, queue/wake/handoff, distributed boundary, control-path truth, policy/budget stops, trace/replay, and bounded end-to-end proof | Criteria definition is not proof that every component is done |
| Milestone routing | `v0.91.5` routes the remaining AEE closure lane into `v0.92`, and `v0.92` activation may consume only named AEE completion evidence | Routing still leaves implementation/proof work open |
| Consumption boundary | `v0.92` may consume named AEE evidence only and must preserve residual action/provider boundaries | Consumption rules are not implementation evidence |

Named open closure components that remain outside `v0.91.6` completion proof:

| Closure component | Current routed owner | Required proof before AEE can be called complete |
| --- | --- | --- |
| Steering semantics | `v0.92` AEE implementation/proof wave | Reviewable decision artifacts plus focused control-path proof |
| Queue / wake / handoff semantics | `v0.92` AEE implementation/proof wave | Deterministic state-transition evidence that does not depend on chat-only state |
| Distributed execution boundary | `v0.92` proof, informed by local/hosted multi-agent evidence | Evidence that delegation preserves authority and truth |
| Control-path truth | `v0.92` proof wave | Inspectable control-path artifact and validation packet |
| Policy and budget stops | `v0.92` proof wave | Negative cases showing blocked/deferred/refused fail-closed behavior |
| Trace / replay proof | `v0.92` proof wave | Durable replay/inspection command with no private-state leakage |
| End-to-end AEE proof/demo | `v0.92` proof/demo wave | Bounded proving packet or demo with explicit non-claims |

Current non-claims:

- `v0.91.6` does not prove AEE subsystem completion.
- `v0.91.6` does not prove all steering, queue/wake/handoff, or distributed
  execution semantics are finished.
- `v0.91.6` does not authorize `v0.95` to rediscover AEE closure implicitly.
- `v0.92` must still block or defer activation if the named AEE proof surfaces
  do not land.
- `v0.91.6` does not prove working-set or context-cache completion as part of
  the memory bridge.

## v0.92 AEE Consumption Rule

`v0.92` may consume this issue as proof that:

- AEE completion is a named subsystem closure lane;
- the remaining closure components are explicit;
- activation must not mark AEE complete without named proof for those
  components.

`v0.92` may not consume this issue as proof that:

- AEE is already complete;
- distributed execution, queue/wake/handoff, or trace/replay proof is already
  done;
- later milestones may treat AEE as settled without further evidence.

## Memory / ObsMem Handoff Status

Current truthful classification:

- `routed_with_explicit_handoff_boundary_and_memory_non_claims`

The Memory/ObsMem boundary is real enough to route `v0.92` consumption, but
this issue does not prove a completed memory runtime or Memory Palace
implementation.

Evidence consumed for this status:

- `docs/architecture/adr/0034-c-sdlc-evidence-convergence-signed-trace-and-obsmem-handoff.md`
- `docs/adr/0007-obsmem-external-boundary.md`
- `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`
- `docs/milestones/v0.91.6/review/security/PUBLIC_RECORD_MEMORY_PROFILE_SECURITY_REVIEW_4022.md`
- this `v0.91.6` bridge ledger

What is already evidence-backed enough to consume:

| Handoff surface | Current evidence-backed truth | Why it is not full memory completion |
| --- | --- | --- |
| ObsMem architectural boundary | ObsMem remains a distinct boundary and external subsystem contract, not an unbounded in-runtime memory blur | Contract existence is not full memory-runtime proof |
| Durable evidence handoff rule | C-SDLC evidence convergence and signed-trace-backed durable records are the expected feed into ObsMem-style memory inputs | Durable evidence handoff does not prove every downstream memory behavior |
| Activation routing | `v0.92` already requires the handoff to stay distinct from memory grounding, working set, context cache, and Memory Palace planning | Routing keeps the distinction visible but leaves implementation/proof work open |
| Privacy/publication floor | WP-07 `#4022` keeps memory/profile publication and privacy as explicit open security boundaries | Security-floor consumption is not publication-safe completion |

Current handoff boundary:

| Surface | Current handoff rule | Non-claim preserved here |
| --- | --- | --- |
| Tracked evidence and signed trace | Durable tracked evidence is the allowed feed surface into ObsMem-style memory inputs | Local-only lore or untracked session state is not valid durable memory authority |
| ObsMem handoff | ObsMem is a distinct ingestion/query boundary for evidence-derived memory surfaces | ObsMem is not the whole Memory Palace solution |
| Memory grounding | `v0.92` birthday work may consume handoff evidence for grounding only through redaction-safe references | Grounding is not implied complete in `v0.91.6` |
| Working set | Must remain distinct from ObsMem handoff and palace topology | This issue does not prove working-set runtime behavior |
| Context cache | Must remain distinct from ObsMem handoff and palace topology | This issue does not prove context-cache runtime behavior |
| Memory Palace | Planned long-context topology remains its own future bridge/implementation lane | This issue does not prove Memory Palace completion |

Current privacy and security constraints:

- Memory/ObsMem handoff must consume tracked evidence rather than local-only
  memory lore.
- Private memory, cognitive-profile, or identity-adjacent material is not
  publication-safe merely because a bridge or planning doc exists.
- The handoff boundary must remain explicit enough that `v0.92` can distinguish
  memory grounding, working set, context cache, and Memory Palace planning.

## v0.92 Memory / ObsMem Consumption Rule

`v0.92` may consume this issue as proof that:

- Memory/ObsMem handoff is a named, distinct bridge surface;
- the allowed durable input posture is tracked evidence plus explicit handoff
  boundaries;
- activation must keep ObsMem handoff, memory grounding, working set, context
  cache, and Memory Palace planning separate.

`v0.92` may not consume this issue as proof that:

- memory grounding is fully implemented;
- working set or context cache runtime behavior is complete;
- Memory Palace is complete;
- memory/profile publication or privacy closure is already finished.

## Memory Palace Long-Context Bridge Status

Current truthful classification:

- `routed_with_explicit_long_context_boundary_and_first_proof_shape`

Memory Palace is now an explicit long-context solution direction with a named
first proof shape, but this issue does not prove runtime completion,
publication/privacy closure, or durable continuity by itself.

Evidence consumed for this status:

- `.adl/docs/TBD/ADL_MEMORY_PALACE_CONTEXT_PROBLEM.md`
- `.adl/docs/TBD/memory_identity/ADL_MEMORY_PALACE_ARCHITECTURE.md`
- `docs/milestones/v0.92/features/MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md`
- `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`
- `docs/milestones/v0.91.6/features/RESILIENCE_PERSISTENCE_SLEEP_WAKE_v0.91.6.md`
- `docs/milestones/v0.91.6/features/IDENTITY_CONTINUITY_CAPABILITY_SELECTOR_BRIDGE_v0.91.6.md`
- `docs/milestones/v0.91.6/review/security/PUBLIC_RECORD_MEMORY_PROFILE_SECURITY_REVIEW_4022.md`
- this `v0.91.6` bridge ledger

What is already evidence-backed enough to consume:

| Memory Palace surface | Current evidence-backed truth | Why it is not full completion |
| --- | --- | --- |
| Context problem statement | Long-lived agents cannot solve context loss by increasing context windows alone; they need navigable structure over bounded working context | The source remains an operator/TBD planning input, not a stable milestone proof artifact |
| Architectural role | Memory Palace is the navigational layer between ObsMem, working set, and context window rather than a duplicate storage system | The architecture note remains source material, not implemented runtime behavior |
| Boundary vocabulary | The distinction among ObsMem, Memory Palace, working set, and context cache is explicit and already required by `v0.92` routing | Boundary clarity is not execution proof |
| First proof shape | The first reviewable slice is no longer a vague future promise; topology, working-set, and stale-context proof surfaces are named as the intended shape | Only the topology route is concretely anchored today; the other proof surfaces still need explicit `v0.92` owners |
| Integration posture | Memory Palace must remain aligned with resilience replay/custody prerequisites and identity/continuity boundaries | Dependency naming is not continuity proof |

Current long-context boundary:

| Surface | Current bridge rule | Non-claim preserved here |
| --- | --- | --- |
| ObsMem | Durable storage and evidence retrieval input | ObsMem is not the navigational solution to the context problem |
| Memory Palace | Navigational topology over durable context anchors, rooms, portals, and traversal hints | Memory Palace is not a finished runtime memory system here |
| Working set | Bounded active cognition materialized for the current task | This issue does not prove working-set runtime behavior |
| Context cache | Execution-time cache of the current working set | Context cache is not memory and is not long-term continuity proof |
| Identity continuity | May later consume palace navigation as part of continuity posture when replay/custody/security proof exists | No durable same-entity continuity claim is proved here |
| Publication/privacy | Memory Palace planning and bridge docs remain security-sensitive surfaces | This issue does not approve publication of private memory or profile material |

Current first proof / fixture plan:

| Planned proof surface | Why it matters | Current owner / route |
| --- | --- | --- |
| Memory Palace topology packet | Makes the navigational structure reviewable instead of metaphor-only | `docs/milestones/v0.92/features/MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md` and later `v0.92` issue work |
| Context working-set packet | Shows how bounded active cognition is materialized from durable references | planned `v0.92` memory/context implementation slice; concrete issue owner still required before downstream proof consumption |
| Stale-context validation report | Proves the system can detect outdated or misleading retrieved context | planned `v0.92` validation slice; concrete issue owner still required before downstream proof consumption |

Current non-claims:

- `v0.91.6` does not prove a full Memory Palace runtime implementation.
- `v0.91.6` does not prove replayable long-lived continuity or same-entity
  persistence through migration, wake, or restore.
- `v0.91.6` does not prove private memory material is publication-safe.
- `v0.91.6` does not replace ObsMem, working set, or context cache with
  Memory Palace.
- `v0.92` must still require bounded implementation and proof before using
  Memory Palace as more than a planned long-context route.

## v0.92 Memory Palace Consumption Rule

`v0.92` may consume this issue as proof that:

- Memory Palace is a named long-context solution direction rather than a vague
  deferral;
- the boundary among ObsMem, Memory Palace, working set, and context cache is
  explicit;
- the first proof shape is at least explicit enough to route, with a concrete
  topology artifact already named and the remaining proof surfaces still
  requiring explicit `v0.92` owners;
- Memory Palace must stay aligned with resilience and identity-continuity
  prerequisites.

`v0.92` may not consume this issue as proof that:

- the Memory Palace runtime is already complete;
- the `TBD` context-problem or architecture notes are themselves stable
  milestone-proof artifacts;
- replay, custody, wake/restore, or durable identity continuity proof already
  exists;
- the context working-set packet or stale-context validation report already has
  an opened implementation/proof owner;
- publication/privacy review is already closed for memory-adjacent material;
- activation or birthday readiness may rely on Memory Palace as shipped
  runtime behavior.

## ACP / Cognitive Profile Scope And Privacy Boundary Status

Current truthful classification:

- `routed_with_explicit_acp_scope_privacy_and_non_collapse_rules`

ACP / cognitive profiles are now an explicit routed bridge surface with named
privacy, schema, and non-collapse rules, but this issue does not prove a
completed ACP runtime or authorize publication of private profile material.

Evidence consumed for this status:

- `docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md`
- `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`
- `docs/milestones/v0.91.6/review/provider/PROVIDER_PROFILES_V2_RECONCILIATION_4111.md`
- `docs/milestones/v0.91.6/review/security/PUBLIC_RECORD_MEMORY_PROFILE_SECURITY_REVIEW_4022.md`
- `docs/milestones/v0.91.6/features/IDENTITY_CONTINUITY_CAPABILITY_SELECTOR_BRIDGE_v0.91.6.md`
- this `v0.91.6` bridge ledger

Authoring/input drift retained for traceability:

- the issue body named `ADL_PROFILES_PROVIDERS_V2.MD`
- that path is not the live tracked ACP/provider-profile authority in the
  current repository state
- the current tracked authority for the provider/profile boundary is the
  reconciliation packet `PROVIDER_PROFILES_V2_RECONCILIATION_4111.md` plus the
  cited bridge surfaces above

What is already explicit enough to consume as routed planning truth:

| ACP/profile surface | Current routed truth | Why it is not full completion |
| --- | --- | --- |
| ACP/profile role | ACP/cognitive profiles are bounded runtime-visible profile records, not reputation, public standing, rights, or identity itself | Role definition is not runtime implementation proof |
| Provider/capability separation | Provider profiles remain low-level substrate records, and capability profiles remain provider-independent behavioral descriptors | Separation rules do not mean ACP profiles are already implemented |
| Identity/citizen truth boundary | Identity, citizen, institution, guild, and continuity surfaces remain outside the provider lane and must not be collapsed into ACP records | Boundary truth is not durable identity or citizenship proof |
| Privacy/redaction posture | Profile use must preserve private-state, redaction, and publication boundaries consumed from WP-07 | Security-floor consumption is not publication-safe completion |
| v0.92 route | `v0.92` owns the first bounded ACP/profile contract, fixtures, update rules, and review packet posture | Routing still leaves implementation/proof work open |

Current ACP/profile boundary:

| Surface | Current bridge rule | Non-claim preserved here |
| --- | --- | --- |
| ACP / cognitive profile | Evidence-grounded runtime-visible profile record describing what evidence is available, private, or unsupported | ACP is not identity, personhood, rights, or reputation |
| Provider profile | Low-level infrastructure and transport record | Provider profile is not ACP, capability, citizen, or public identity |
| Capability profile | Provider-independent behavioral descriptor | Capability profile is not ACP/cognitive profile or civil identity |
| Identity / citizen / institution / guild | Governance and continuity-facing surfaces that may later constrain profile use | These records are not provider or ACP profile fields by default |
| Public/reviewer projection | Redacted reviewer/public surface only when the relevant security boundary is satisfied | This issue does not approve raw private profile publication |

Current ACP field/privacy classification:

| Field family | Current privacy class | Current rule |
| --- | --- | --- |
| Profile identifier, schema version, update reason, update actor, non-claims summary | `public_or_reviewer_visible_if_redacted` | These fields may appear in reviewer/public projections only as bounded metadata and must not imply reputation, personhood, or rights |
| Source-evidence links and evidence-status summaries | `reviewer_visible_redacted_public_blocked_by_default` | Reviewer packets may cite bounded evidence references; public projection must stay redacted and must not expose private state or raw memory/profile material |
| Memory-, continuity-, learning-, or intelligence-derived profile content | `private` | This issue does not authorize broad publication; such content remains inside the bounded runtime/reviewer path until later proof explicitly clears a narrower projection |
| Unsupported or speculative reputation / standing / rights claims | `blocked` | These claims must not appear in ACP/profile records |
| Citizen, institution, guild, and continuity records themselves | `blocked_from_acp_field_collapse` | These records may constrain later profile use but are not ACP fields in this bridge |

Current ACP access posture:

| Access class | What may be seen | Current posture |
| --- | --- | --- |
| Runtime-local bounded owner path | Full bounded ACP/profile record once later implementation exists | planned route only; not implemented by this issue |
| Internal reviewer packet | Redacted metadata, bounded evidence references, privacy policy, and non-claims | allowed as the intended review surface once later proof lands |
| Public/reviewer-facing projection | Redacted metadata and explicit non-claims only | blocked by default unless the relevant WP-07/WP-10 privacy proof clears the narrower projection |
| Provider/capability/profile routing layers | No direct ACP/profile field access by default | must consume only the boundary rules, not private ACP content |
| Identity/citizen/governance surfaces | May later constrain profile use, but not read ACP as authority by default | deferred and out of scope for this bridge |

Current ACP/privacy and access rules:

- ACP/profile records must stay evidence-grounded and cite allowed source
  references rather than free-floating labels.
- Access posture is bounded: reviewer-facing use is the intended first visible
  path, public projection is blocked by default, and runtime-local full access
  remains future implementation work.
- Unsupported reputation, personhood, standing, or rights claims must remain
  rejected.
- Public or reviewer-facing profile material must respect WP-07 redaction and
  publication boundaries.
- ACP/profile scope must remain distinct from provider profiles, capability
  profiles, and identity/citizen records.

Current first ACP proof / fixture route:

| Planned ACP proof surface | Why it matters | Current owner / route |
| --- | --- | --- |
| ACP/profile schema and fixtures | Makes the runtime-visible profile contract reviewable rather than narrative-only | `docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md` and later `v0.92` issue work |
| Update-rationale and evidence references | Shows profile changes are grounded in evidence and not label drift | planned `v0.92` ACP implementation slice; concrete issue owner still required before downstream proof consumption |
| Redacted reviewer packet and validation report | Proves privacy policy, non-claims, and rejection behavior | planned `v0.92` review/validation slice; concrete issue owner still required before downstream proof consumption |

Current non-claims:

- `v0.91.6` does not prove a full ACP/profile runtime implementation.
- `v0.91.6` does not prove profile schemas or fixtures are already landed.
- `v0.91.6` does not authorize ACP/profile publication by default.
- `v0.91.6` does not collapse provider, capability, identity, citizen, or
  institution surfaces into ACP records.
- `v0.92` must still require bounded implementation and proof before using ACP
  profiles as more than a routed bridge surface.

## v0.92 ACP / Cognitive Profile Consumption Rule

`v0.92` may consume this issue as proof that:

- ACP / cognitive profiles are a named bounded runtime-profile direction rather
  than a vague future bucket;
- ACP/profile scope must remain distinct from provider profiles, capability
  profiles, and identity/citizen records;
- privacy, redaction, and non-reputation rules are explicit;
- the first ACP proof shape is at least explicit enough to route, with schema,
  evidence/update-rationale, and redacted reviewer-packet surfaces named for
  later ownership.

`v0.92` may not consume this issue as proof that:

- ACP/profile schema and fixtures are already implemented;
- ACP/profile publication is already safe by default;
- provider-profile reconciliation has turned ACP into a provider or capability
  surface;
- identity, citizen, institution, guild, or continuity truth can be inferred
  from ACP alone;
- activation or birthday readiness may rely on ACP as a fully shipped runtime
  contract today.

## Dependency And Proof Expectations

WP-10 depends on the following reviewed bridges and must preserve their role
separation:

- WP-08 identity continuity and selector bridge for identity/governance
  boundaries;
- WP-02 resilience bridge for replay, custody, and continuity-sensitive
  prerequisites;
- WP-07 security bridge and packet `#4022` for bounded publication/privacy
  review;
- later child issues `#4037` through `#4041` for surface-specific completion
  proof.

Proof expectations for this setup ledger are intentionally modest:

- the ledger must name every required WP-10 surface;
- each surface must have a current owner and a completion condition;
- the ledger must distinguish implementation, documentation, proof, and
  residual routing;
- the ledger must not let any one surface claim completion by borrowing proof
  from another layer.

## Residual Routing Rules

- If a child issue cannot prove implementation completion, it must record a
  blocked, deferred, or residual route instead of vague partial success.
- Memory Palace may route to later work only if the residual is explicit and
  accepted in the relevant child issue and final closeout matrix.
- WP-07 publication/privacy review may be consumed as a security input, but it
  must not be rewritten as full WP-10 completion.
- `v0.92` may consume only surfaces classified as evidence-backed in the final
  WP-10 closeout matrix.

## Validation And Review

- Review AEE completion claims against concrete artifacts.
- Separate Memory/ObsMem handoff from Memory Palace future work.
- Treat ACP profile privacy as a security boundary.
- Consume WP-07 `#4022` for bounded publication/privacy review without
  converting that review into false completion of open WP-10 work.

## v0.92 Consumption

`v0.92` may consume only named completion and handoff surfaces. It must not
rediscover AEE, Memory/ObsMem, or ACP scope during activation.

## Non-Goals

- No Memory Palace completion claim.
- No unreviewed cognitive-profile publication.
- No runtime/provider action completion claim.

## Current Security Consumption

- WP-07 packet
  `docs/milestones/v0.91.6/review/security/PUBLIC_RECORD_MEMORY_PROFILE_SECURITY_REVIEW_4022.md`
  may be consumed as the current bounded publication/privacy review for this
  bridge.
- That packet does not close WP-10. It records that memory/profile publication
  remains an open dependency until the WP-10 issue set, including the privacy
  boundary lane `#4040` and closeout lane `#4041`, lands its own reviewed
  privacy and closeout surfaces.

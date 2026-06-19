# AEE, Memory/ObsMem, And ACP Bridge Accounting

## Metadata

- Feature Name: AEE, Memory/ObsMem, And ACP Bridge Accounting
- Milestone Target: `v0.91.6`
- Status: planned_with_aee_routing_state_explicit
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
| Memory/ObsMem handoff | `#4038` | Memory/ObsMem boundary is distinct, evidence-backed, and privacy-constrained | planned child issue |
| Memory Palace long-context bridge | `#4039` | Long-context solution path is explicit with proof or explicitly accepted residual routing | planned child issue |
| ACP/cognitive profile scope and privacy boundary | `#4040` | ACP and cognitive-profile boundaries stay distinct from provider/capability/identity surfaces and preserve privacy/security rules | planned child issue |
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

What is already evidence-backed enough to consume:

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

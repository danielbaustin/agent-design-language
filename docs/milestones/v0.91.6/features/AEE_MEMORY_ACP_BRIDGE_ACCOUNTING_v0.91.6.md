# AEE, Memory/ObsMem, And ACP Bridge Accounting

## Metadata

- Feature Name: AEE, Memory/ObsMem, And ACP Bridge Accounting
- Milestone Target: `v0.91.6`
- Status: planned_with_completion_ledger_authored
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
| AEE completion and readiness | `#4037` | AEE is classified as complete, blocked, or explicitly routed with proof limits and `v0.92` status | planned child issue |
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
| Memory Palace | The long-context architecture/proof route under active development | A silently deferred or forgotten future bucket |
| ACP/cognitive profiles | Cognitive-profile scope, access, privacy, and relationship to other profile families | Provider profiles, capability profiles, or public identity records by default |
| Privacy/security consumption | Bounded security review inputs from WP-07 and later lanes | Automatic closure of open WP-10 implementation work |

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

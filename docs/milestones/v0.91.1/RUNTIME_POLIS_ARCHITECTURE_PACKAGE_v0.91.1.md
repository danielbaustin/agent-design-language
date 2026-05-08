# Runtime And Polis Architecture Package v0.91.1

## Status

Tracked `WP-02` architecture package for `v0.91.1`.

## Purpose

This package aligns the `v0.91.1` runtime/polis story with the code and proof
surfaces that actually exist in the repository at the start of the inhabited
runtime wave.

It is intentionally narrower than the older Runtime v2 source corpus. The goal
here is not to restate the full aspirational "Gödel Agent Land" thesis. The
goal is to document what later `v0.91.1` work packages may safely assume about:

- Runtime v2 substrate shape
- manifold and snapshot contracts
- citizen lifecycle and standing surfaces
- control-plane and observatory touchpoints
- what "polis" currently means in tracked repo truth

## Source Basis

Primary current-code evidence:

- `adl/src/runtime_v2/mod.rs`
- `adl/src/runtime_v2/manifold.rs`
- `adl/src/runtime_v2/kernel_loop.rs`
- `adl/src/runtime_v2/citizen.rs`
- `adl/src/runtime_v2/snapshot.rs`
- `adl/src/runtime_v2/boot_admission.rs`
- `adl/src/runtime_v2/standing.rs`
- `adl/src/runtime_v2/access_control.rs`
- `adl/src/runtime_v2/transition_authority.rs`
- `adl/src/runtime_v2/contract_lifecycle_state.rs`
- `adl/src/runtime_v2/observatory.rs`
- `adl/src/runtime_v2/operator.rs`
- `adl/src/runtime_v2/quarantine.rs`
- `adl/src/runtime_v2/recovery.rs`
- `adl/src/runtime_v2/security.rs`
- `adl/src/runtime_v2/csm_run.rs`
- `adl/src/runtime_v2/integrated_csm_run.rs`
- `adl/src/control_plane.rs`

Primary older source docs reviewed for drift:

- `.adl/docs/TBD/runtime_v2/ADL_RUNTIME_v2.md`
- `.adl/docs/TBD/runtime_v2/POLIS_SPEC.md`
- `.adl/docs/TBD/runtime_v2/KERNEL_SERVICES_AND_CONTROL_PLANE.md`
- `.adl/docs/TBD/runtime_v2/MANIFOLD_AND_SNAPSHOT_SPEC.md`
- `.adl/docs/TBD/runtime_v2/CITIZEN_LIFECYCLE_AND_STATE_MACHINE.md`

## Executive Summary

The current tracked Runtime v2 surface is best understood as a reviewable
artifact-contract substrate, not yet as a continuously inhabited world kernel.

What exists now is real and downstream-usable:

- manifold-root contracts
- kernel-loop and service-registry artifacts
- provisional citizen records and indices
- snapshot and rehydration manifests
- boot/admission, standing, access-control, and transition-authority packets
- observatory/operator projection artifacts
- integrated proof packets that link these surfaces together

What does not yet exist as repo truth is equally important:

- no always-on resident staff runtime
- no serialized polis constitution/governance stack as a single canonical
  module
- no live cross-polis transport or migration substrate
- no birthday/identity-completion claims

For `v0.91.1`, "polis" should therefore be read as the governed runtime-facing
social and authority boundary distributed across standing, access, transition,
admission, and observability artifacts, rather than as a single separate
runtime implementation layer.

## Runtime v2 Current-State Model

### 1. Runtime Root

`adl/src/runtime_v2/mod.rs` is the public integration surface. It re-exports a
large set of proof-bearing modules rather than one monolithic resident runtime.
That module tree is the current top-level substrate boundary for later WPs.

### 2. Manifold

`adl/src/runtime_v2/manifold.rs` defines the current manifold root as a durable
artifact contract with:

- clock anchor
- citizen registry refs
- kernel service refs
- trace root
- snapshot root
- invariant policy refs
- review-surface hooks

This means the manifold is currently represented as a structured artifact model
that binds the rest of the runtime evidence together. It is not yet a live
always-on execution host with independent scheduling semantics.

### 3. Kernel

`adl/src/runtime_v2/kernel_loop.rs` defines a bounded kernel service registry,
service state, and ordered loop events. The kernel is currently modeled as a
deterministic service-loop artifact surface with contiguous event sequencing and
explicit registry/state alignment.

That is enough for reviewable kernel claims, but it is still narrower than the
older source-doc picture of a permanently running world kernel with richer
orchestration behavior.

### 4. Citizens

`adl/src/runtime_v2/citizen.rs` defines provisional citizen records, plus active
and pending registry indices. Citizens already have:

- `citizen_id`
- lifecycle state
- memory/identity refs
- policy boundary refs
- rehydration and termination hooks

This is a real runtime-facing citizen substrate, but it remains explicitly
provisional and bounded.

### 5. Snapshot And Rehydration

`adl/src/runtime_v2/snapshot.rs` gives the current serialization and
rehydration shape: a snapshot manifest, invariant status, checksum, and
rehydration report with wake-eligibility checks.

So the repo does already have a concrete snapshot/rehydration contract, but not
the full older "sleep / seal / transfer / wake" migration world that the source
docs describe aspirationally.

### 6. Observatory And Operator Projection

`adl/src/runtime_v2/observatory.rs` connects run, boot/admission, and wake
continuity artifacts into a reviewer-visible observatory packet and operator
report. This is one of the strongest pieces of current runtime evidence because
it ties otherwise separate runtime contracts into a human-reviewable projection.

### 7. Runtime Governance And Control Surfaces

`adl/src/control_plane.rs` is workflow-oriented path and issue/worktree routing
infrastructure, not the semantic runtime control plane described in the older
Runtime v2 concept docs. We should therefore keep the terms separate:

- `control_plane.rs` today: ADL issue/worktree/lifecycle path resolution
- runtime control plane in `v0.91.1`: still mostly represented through
  runtime-v2 contracts such as kernel loop, transition authority, admission,
  operator projection, recovery, and security packets

The exported Runtime v2 governance/control cluster is therefore broader than
just kernel loop plus standing/access checks. It also includes:

- `adl/src/runtime_v2/operator.rs` for reviewer/operator-facing control reports
- `adl/src/runtime_v2/quarantine.rs` for quarantine state transitions, blocked
  action custody, and evidence-preservation artifacts
- `adl/src/runtime_v2/recovery.rs` for recovery eligibility and safe-resume
  boundaries
- `adl/src/runtime_v2/security.rs` for security-boundary proof and refusal
  behavior

## Current Runtime/Polis Inventory

| Surface | Current implementation home | Current truth |
| --- | --- | --- |
| Runtime v2 root | `adl/src/runtime_v2/mod.rs` | Public integration surface for bounded runtime artifacts and proof packets |
| Manifold root | `adl/src/runtime_v2/manifold.rs` | Structured artifact linking kernel, citizens, traces, snapshots, invariants, and review hooks |
| Kernel services | `adl/src/runtime_v2/kernel_loop.rs` | Deterministic registry/state/event artifacts, not yet a long-lived live scheduler |
| Citizen lifecycle records | `adl/src/runtime_v2/citizen.rs` | Provisional citizen records plus active/pending indices |
| Snapshot/rehydration | `adl/src/runtime_v2/snapshot.rs` | Concrete snapshot manifest and rehydration report contract |
| Boot/admission | `adl/src/runtime_v2/boot_admission.rs` | Citizen admission receipts, boot manifest, and admission trace |
| Standing | `adl/src/runtime_v2/standing.rs` | Standing classes, communication examples, and negative cases |
| Access-control boundary | `adl/src/runtime_v2/access_control.rs` | Authority matrix, auditable event packet, and denial fixtures |
| Transition authority | `adl/src/runtime_v2/transition_authority.rs` | Transition matrix, authority basis, and negative cases |
| Contract lifecycle | `adl/src/runtime_v2/contract_lifecycle_state.rs` | Runtime-facing lifecycle scenarios and invalid-transition negative cases |
| Observatory projection | `adl/src/runtime_v2/observatory.rs` | Reviewer-visible packet and operator report |
| Operator control | `adl/src/runtime_v2/operator.rs` | Reviewer/operator-facing control reports tied to runtime proof surfaces |
| Quarantine boundary | `adl/src/runtime_v2/quarantine.rs` | Quarantine artifacts, blocked-action custody, and evidence-preservation surfaces |
| Recovery boundary | `adl/src/runtime_v2/recovery.rs` | Recovery eligibility, safe-resume decisions, and quarantine-adjacent checks |
| Security boundary | `adl/src/runtime_v2/security.rs` | Security-boundary proof packets and refusal behavior for unsafe runtime actions |
| Run/integrated proof | `adl/src/runtime_v2/csm_run.rs`, `adl/src/runtime_v2/integrated_csm_run.rs` | End-to-end proof packet surfaces for bounded runtime execution claims |

## What "Polis" Means In Current Repo Truth

The source docs describe the polis as constitution, governance, economics, and
security for an inhabited manifold. That is still a useful planning direction,
but it is too broad to describe the current code literally.

For `v0.91.1`, the tracked polis boundary is distributed across these surfaces:

- standing classification and communication rights in
  `adl/src/runtime_v2/standing.rs`
- access-path authority and denial behavior in
  `adl/src/runtime_v2/access_control.rs`
- transition authority and basis requirements in
  `adl/src/runtime_v2/transition_authority.rs`
- boot/admission constraints in `adl/src/runtime_v2/boot_admission.rs`
- observatory-visible review projections in `adl/src/runtime_v2/observatory.rs`
- operator-facing control reporting in `adl/src/runtime_v2/operator.rs`
- quarantine custody and evidence-preservation boundaries in
  `adl/src/runtime_v2/quarantine.rs`
- recovery and safe-resume boundaries in `adl/src/runtime_v2/recovery.rs`
- security-boundary refusal/proof surfaces in `adl/src/runtime_v2/security.rs`

This means:

- the manifold is currently the runtime substrate artifact root
- the polis is currently the set of governed authority, standing, admission,
  and visibility contracts that civilize action inside that substrate

That framing is strong enough for downstream work on lifecycle, observatory,
standing, and state without overclaiming a full governance or economics system.

## Drift Report Against Older Runtime v2 Source Docs

### Drift 1: "Persistent world" vs "bounded proof-bearing substrate"

Older docs such as `.adl/docs/TBD/runtime_v2/ADL_RUNTIME_v2.md` describe Runtime
v2 as a persistent cognitive spacetime world with resident staff and long-lived
citizens. Current code truth is narrower: the repo has durable artifact
contracts and integrated proof packets, but not an always-on inhabited runtime.

Disposition:

- keep the older world-language as upstream design intent
- do not describe it as already implemented in `v0.91.1`

### Drift 2: "Kernel services and control plane" vs current implementation split

`.adl/docs/TBD/runtime_v2/KERNEL_SERVICES_AND_CONTROL_PLANE.md` describes a
semantic kernel/control-plane stack with scheduler, migration manager, and
invariant engine. Current code does have kernel-loop and related proof surfaces,
but the operational `adl/src/control_plane.rs` file is about issue/worktree
routing, not inhabited-runtime governance.

Disposition:

- use "kernel loop" and "runtime authority surfaces" for current repo truth
- avoid implying that `control_plane.rs` already implements the conceptual
  runtime control plane

### Drift 3: Polis as unified constitution/economy/governance layer

`.adl/docs/TBD/runtime_v2/POLIS_SPEC.md` presents polis as a full constitution,
governance system, economic substrate, and security boundary. The current code
has meaningful slices of that idea, but only as distributed contracts for
standing, access, transition authority, admission, and observability.

Disposition:

- treat polis as a distributed governed-runtime boundary in `v0.91.1`
- defer unified constitution/economy claims to later work

### Drift 4: Full migration/sleep/wake world vs current snapshot contract

`.adl/docs/TBD/runtime_v2/MANIFOLD_AND_SNAPSHOT_SPEC.md` and related docs
describe a fuller seal/transfer/rehydrate model than the current snapshot code
proves. `adl/src/runtime_v2/snapshot.rs` does provide real snapshot and
rehydration artifacts, but cross-environment migration remains out of scope.

Disposition:

- current claim: snapshot + rehydration contract exists
- non-claim: portable cross-polis migration is implemented

### Drift 5: Full citizen-state machine vs bounded provisional lifecycle

`.adl/docs/TBD/runtime_v2/CITIZEN_LIFECYCLE_AND_STATE_MACHINE.md` describes a
broader lifecycle model. Current repo truth is a provisional citizen record
surface plus separate standing, admission, transition, and contract-lifecycle
artifacts. The fuller state-model unification belongs to downstream `WP-03`
through `WP-06`, not to `WP-02`.

Disposition:

- current claim: bounded citizen/lifecycle substrate exists
- non-claim: the full unified lifecycle-state model is already landed

## Downstream Assumptions For Sprint 1

Later Sprint 1 work may safely assume:

- a tracked Runtime v2 module tree already exists
- manifold, kernel, citizen, and snapshot artifact contracts are real
- observatory/operator runtime projection already has concrete proof surfaces
- polis-facing governance is currently distributed across standing, access,
  transition, and admission artifacts

Later Sprint 1 work should not assume:

- a live always-on runtime scheduler
- a single canonical polis constitution file
- cross-polis transport or migration
- complete identity continuity or birthday semantics

## Recommended Terminology For v0.91.1 Docs

Use these terms consistently:

- `Runtime v2 substrate`: the current artifact-contract runtime module tree
- `manifold`: the structured root artifact that links runtime services
- `kernel loop`: the current service-loop registry/state/event surface
- `citizen lifecycle substrate`: provisional citizen records plus later
  lifecycle contracts
- `polis boundary`: standing, access, transition, admission, and visibility
  governance surfaces taken together

Avoid these overclaims in tracked `v0.91.1` docs:

- "always-on world kernel"
- "resident staff already implemented"
- "full polis constitution exists"
- "migration across environments is supported"
- "birthday/identity continuity is solved"

## Review Hooks

Reviewers of later `v0.91.1` WPs should block claims that:

- treat older `.adl/docs/TBD/runtime_v2/` design prose as already-landed code
- collapse workflow control-plane helpers into runtime semantic control-plane
  claims
- assume polis governance exists outside the current standing/access/authority
  surfaces
- imply cross-polis or birthday-level identity support

## Non-Claims

This package does not claim:

- the first true birthday
- complete identity continuity
- constitutional citizenship
- external federation or cross-polis transport
- a live resident-staff kernel
- a finished economics or governance layer

## Result

`WP-02` gives `v0.91.1` a truthful runtime/polis baseline: later work can build
on a real manifold/kernel/citizen/snapshot/observatory substrate without
pretending that the broader older Runtime v2 world-design is already fully
implemented.

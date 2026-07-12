# C-SDLC v2 Clean-Room Architecture

Status: Gate 1 design contract for issue #5228
Decision authority: operator-reviewed; later implementation remains gated
Product boundary: standalone Rust workspace, independent from ADL Runtime

## Decision

C-SDLC v2 will be authored from scratch as a small software-development
control plane. It preserves the core C-SDLC ideas—an explicit lifecycle state
machine, six structured prompts, Parallel Validation Fabric (PVF), deterministic
scheduling, shepherd-owned observation, claims, evidence, review, and truthful
closeout—without carrying forward the v1 implementation.

V1 is behavioral evidence, not source material. V2 must not import, copy, move,
extract, adapt, vendor, or begin from existing ADL repository implementation,
schema, test, fixture, template, or skill code. Black-box observations and
documented outcomes may inform independently authored contracts and fixtures.

Runtime v3 supplies the successful method and a qualified third-party
dependency reference. C-SDLC v2 may select the same COTS crates and compatible
versions, but it has no crate or runtime dependency on `adl`, `adl-runtime`, or
`adl-runtime-kernel`.

## Product Goals

- Implement the SDLC state machine directly with typed transitions, guards,
  invariants, and outcomes.
- Keep every owner binary small, independently installable, and independently
  validatable.
- Fully automate construction and editing of SIP, STP, SPP, VPP, SRP, and SOR.
- Use `markdown.rs` mdast for Markdown structure and `strum` for every closed
  vocabulary.
- Select the smallest proving validation DAG through PVF.
- Make scheduler and shepherd authority explicit and non-overlapping.
- Require an issue-specific design and diagram before implementation readiness.
- Target at most 8,000 implementation LoC and 60–100 tests, with a hard ceiling
  of 150 tests.
- Target 90% removal of the incumbent control-plane implementation; 80% is the
  minimum acceptable result and requires justification below 90%.
- Keep warm focused validation under two minutes and complete deterministic,
  non-live validation under ten minutes.

## System Boundary

The canonical block diagram is
[`csdlc-v2/csdlc_v2_block_diagram.mmd`](csdlc-v2/csdlc_v2_block_diagram.mmd).

C-SDLC v2 owns:

- lifecycle state and transition authority;
- automated structured-prompt values, rendering, AST validation, and commits;
- repository/worktree binding and claims;
- PVF planning and result convergence;
- deterministic readiness scheduling;
- shepherd observation and classification;
- GitHub issue/PR/check/review normalization;
- evidence, publication, and closeout records.

C-SDLC v2 does not own:

- ADL agent-runtime behavior;
- Runtime v3 supervision or components;
- product-specific test logic;
- merge decisions before review and required checks are green;
- arbitrary shell evaluation;
- manual Markdown editing.

ADL or Runtime validation may be invoked only through an explicitly selected
cross-product proof lane. That does not create a build-time or runtime
dependency.

## Lifecycle State Machine

The initial persisted phases are:

```text
initialized
  -> ready
  -> bound
  -> implemented
  -> reviewed
  -> published
  -> merge_ready
  -> merged
  -> closed_out
```

`blocked`, `failed`, `waiting`, and `deferred` are typed operation results, not
additional phases. Every mutation supplies the expected prior generation and
digest. Invalid or stale transitions fail closed.

The operational index stores issue/repository identity, phase, branch/worktree,
claim generation, protected paths, card/template/schema/digest projections,
validation plans/results, review dispositions, PR state, closeout state, and an
append-only transition log. It is not a seventh narrative card.

## Structured Prompts And Automated Editing

The canonical lifecycle remains:

```text
SIP -> STP -> SPP -> VPP -> SRP -> SOR
```

The prompts are human-readable durable truth, but their files are generated
artifacts. Operators, agents, and skills submit typed values or semantic
operations. Only the Rust card engine writes cards.

Each edit transaction:

1. resolves the active independently authored v2 template and schema;
2. loads or initializes typed values;
3. validates the actor, claim, phase, field owner, and expected digest;
4. applies one typed semantic operation;
5. renders deterministic Markdown;
6. parses Markdown through `markdown.rs` into mdast;
7. validates semantic anchors, AST shape, and cross-card invariants;
8. computes values/template/AST/rendered digests;
9. atomically commits values, Markdown, operational index, and audit event.

There is no regex parsing, heading scan, line-number edit, or raw string
replacement. Unknown imported AST nodes are retained or reported; they never
silently disappear. Direct Markdown edits cause a digest mismatch and fail
doctor until an explicit import-or-regenerate decision is recorded.

`csdlc-init` constructs all six cards. Later-phase cards begin in explicit
pre-phase states. `csdlc-edit` is the sole general card-mutation boundary.

## Enum Contract

Every closed vocabulary is a Rust enum. `strum` supplies `Display`,
`EnumString`, `AsRefStr`, and `EnumIter` as needed. Serde names, Clap values,
JSON Schema values, card values, and human output use one canonical spelling.
Legacy aliases are accepted only by the legacy importer. Unknown values fail
closed.

## Seven Owner Binaries

| Binary | Responsibility |
| --- | --- |
| `csdlc-init` | Resolve issue/repository identity; initialize index, design surfaces, and six cards. |
| `csdlc-doctor` | Read-only invariant, readiness, collision, design, diagram, card, and external-state diagnosis. |
| `csdlc-edit` | Apply typed semantic card operations and atomically commit cards/index/audit. |
| `csdlc-bind` | Bind or verify branch, worktree, claim, and protected paths. |
| `csdlc-validate` | Select and execute PVF lanes and retain typed proof. |
| `csdlc-publish` | Enforce current review truth; push and create/update draft PR; normalize checks/reviews. |
| `csdlc-closeout` | Verify terminal remote state; reconcile SOR/index; safely prune execution state. |

Every binary uses direct Clap parsing, stable JSON stdout, diagnostics on
stderr, typed exit codes, and feature-gated dependencies. No binary links the
main ADL or Runtime product graph. Initial size targets are 15 MiB stripped per
binary and 70 MiB for the installed set, subject to Gate 1 normalization.

## Nine Thin Skills

Skills interpret, review, and route; they never mutate lifecycle state.

1. `csdlc-conductor`
2. `csdlc-issue-designer`
3. `csdlc-card-editor`
4. `csdlc-run`
5. `csdlc-validator`
6. `csdlc-review`
7. `csdlc-publish`
8. `csdlc-shepherd`
9. `csdlc-closeout`

The generic card editor takes `CardKind`, expected digest, claim, actor, reason,
operation, and typed values, then calls `csdlc-edit`. Its initial operations are
`set_field`, `append_reference`, `record_validation`, `record_finding`,
`dispose_finding`, `advance_status`, `record_execution`, and `record_closeout`.

## PVF

Each validation lane declares:

- lane and proof role;
- determinism posture;
- resource profile and expected cost;
- credential/network posture;
- dependencies and parallel group;
- release-gate status;
- executable-plus-argument arrays;
- timeout and evidence policy.

PVF maps changed scope and outcome type to the smallest required DAG. Ordinary
tests contain no sharding, CI-mode, or release-routing policy. Independent
lanes may run concurrently within declared budgets. Results converge into one
typed disposition that separates local proof, deferred CI, waiting, failure,
and accepted non-goals.

## Scheduler

The scheduler is a deterministic eligibility function over lifecycle phase,
card readiness, design/diagram readiness, issue dependencies, active claims,
protected paths, validation budgets, and operator policy. It reports eligible
next operations but cannot claim, execute, publish, merge, or close.

## Shepherd

The shepherd observes bound and published work: claim heartbeat, proof-lane
state, PR checks, conflicts, reviews, and dependency gates. It emits `ready`,
`waiting`, `retryable`, `repair_required`, or `operator_required`. It cannot
edit implementation, widen scope, merge, or reinterpret waiting as failure.
Initially it is exposed through doctor and `publish status --watch`, not a
large resident binary.

## Claims And Concurrency

Mutations require an active issue/session claim. A claim includes owner,
generation, acquisition, expiry, heartbeat, branch/worktree, protected paths,
and purpose. Overlapping protected paths fail closed. Stale recovery is an
explicit compare-and-swap transition that retains old owner, observed expiry,
recovery actor, and reason. A missed heartbeat does not silently prove staleness.

## Required Issue Design And Diagram

Before `ready` or `bound`, every issue has:

- a design covering problem, invariants, boundary, state/data changes, COTS
  choices, failure behavior, concurrency/security, validation, rollback, and
  non-goals;
- at least one source-grounded Mermaid diagram;
- SPP/VPP references to both;
- a design-review disposition.

Small issues use a small diagram; they do not waive the boundary. Mermaid source
validation belongs in the fast lane. Asset rendering is publication proof, not
a requirement for every local command.

## COTS Selection

Gate 1 chooses third-party crates, never existing repository modules. The
starting set is Clap, Serde/Serde JSON, Schemars plus JSON Schema validation,
Thiserror, Strum, Markdown.rs, Octocrab, Tokio, Tokio-util, Tracing, `fs2`, and
`tempfile`. Cryptographic digest/signature crates are used only where the
integrity/authentication contract requires them.

The authoritative version requirements, feature flags, default-feature policy,
compatibility basis, unresolved state-machine/Markdown-serializer choices, and
Gate 2 resolver obligations are recorded in
[`csdlc_v2_contracts_and_cots.v1.json`](csdlc-v2/csdlc_v2_contracts_and_cots.v1.json).
The per-card typed fields, ownership, semantic anchors, pre-phase states,
cross-card invariants, compatibility rules, and worked construction example are
recorded in
[`csdlc_v2_card_contracts.v1.json`](csdlc-v2/csdlc_v2_card_contracts.v1.json).

Git remains a typed subprocess boundary unless a COTS library demonstrably
makes worktree behavior smaller and more reliable. Commands are argument arrays;
the control plane never invokes `bash -c`.

## Independent Validation

C-SDLC v2 has its own manifest, lockfile, target/cache strategy, and focused CI
job. Normal validation does not build or test ADL or Runtime crates. Initial
budgets:

- clean construction at most 50% of normalized v1;
- warm incremental construction at most 25% of normalized v1;
- focused validation at most 25% of normalized v1 and under two minutes;
- complete deterministic non-live validation under ten minutes;
- local doctor p95 under one second without network;
- local init/bind planning p95 under two seconds without network/build/fetch.

## Review, Publication, And Closeout

Bounded review occurs before PR creation. Review evidence records reviewer,
scope, exact revision, findings, and dispositions. A changed revision
invalidates review unless policy proves the change non-substantive. All
actionable in-scope findings are fixed before publication.

Publication and closeout are idempotent resumable operations. Merge remains
blocked until required review and checks are green. Git/GitHub mutations record
intent, idempotency, observed result, and reconciliation evidence.

## Migration And Deletion

V2 begins opt-in beside v1. Parity compares normalized outcomes, not internal
layouts or byte-identical Markdown. The executable rollback path expires 14
days after default cutover. The read-only legacy importer expires after 30
days. Extensions require explicit review before expiry.

The deletion target is 90%. A result of 80–89% requires an enumerated retained
surface, owner, justification, and cutover approval. Below 80% is not completion.

## Gate 1 Recommendation Boundary

This issue may recommend `proceed`, `incubate`, or `stop`. It does not implement
later gates, switch defaults, delete v1, or start the compatibility window.

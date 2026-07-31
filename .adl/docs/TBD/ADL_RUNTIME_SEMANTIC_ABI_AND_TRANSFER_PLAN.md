# ADL Runtime Semantic ABI And Transfer Plan

Status: Reviewed architecture plan
Tracking issue: `#5724`
Target: v0.92 planning, with production implementation scheduled explicitly
Review: Gemini 3.1 Pro Preview PASS on 2026-07-30

## Purpose

Define one portable contract that allows an ADL Runtime to:

1. upgrade in place from one immutable native release to another;
2. relocate to another machine, operating system, or Runtime release;
3. migrate back to the original platform after it has been upgraded; and
4. replace selected capability implementations without depending on Rust's
   unstable native ABI.

The central decision is:

> Canonical ADL, its deterministic `ExecutionPlan`, declared capabilities, and
> a signed continuity-transfer envelope form a semantic ABI between Runtime
> implementations.

This is not a native object ABI. Rust structs, stack frames, futures, dynamic
libraries, sockets, and process-local handles are not portable contracts.

## Source Baseline

The plan builds on existing repository contracts:

- `AdlDocument.version` and the canonical ADL provider, tool, agent, task,
  workflow, run, and placement model;
- `adl.execution-plan.v1`, deterministic source digests, stable node identity,
  ports, edges, and provenance;
- `adl.engine.v1` and `adl.engine-checkpoint.v1`;
- versioned canonical ACIP JSON, Protobuf, HTTP, and WSS carriers;
- signed Runtime v3 checkpoint manifests, lineage validation, topology and
  configuration identity, and rollback rejection;
- the redb durable-state boundary;
- Guardian process ownership and Runtime checkpointed shutdown;
- the proposed distributed Guardian mesh, membership, capability,
  snapshot-catalog, fencing, and migration architecture.

The current `LiveContinuity` path correctly uses `MigrationPolicy::Exact`.
That policy remains the normal local-restart rule. Cross-release and
cross-machine movement require a distinct compatibility decision and must not
silently relax exact restore.

## Goals

- Keep ADL behavior portable across Runtime releases and supported platforms.
- Preserve agent, workflow, governance, and continuity identity.
- Prevent two authoritative writers for one Runtime lineage.
- Permit fast in-place updates with automatic rollback.
- Permit blue-green migration and an optional migrate-back operation.
- Keep machine-local configuration and secret custody outside portable state.
- Make compatibility machine-checkable before admission or cutover.
- Reuse existing ADL, ACIP, OpenAPI, Protobuf, checkpoint, redb, Guardian,
  tracing, Vector, Rustls, and Wasmtime/WIT surfaces.
- Keep the native kernel small and avoid an OSGi-style native plugin framework.

## Non-Goals

- Stable Rust ABI or hot replacement of Rust dynamic libraries.
- Serialization of Tokio tasks, Rust futures, stacks, sockets, file
  descriptors, mutexes, or allocator state.
- Copying state and declaring the copy to be the same continuing agent.
- Moving private keys, provider credentials, or host secrets inside a transfer
  bundle.
- Simultaneous active ownership of one continuity lineage.
- A custom package manager, cryptographic primitive, transport protocol,
  database, or service manager.
- Production migration claims from a plan, fixture, schema, or shadow-only
  proof.

## Architectural Decision

Use two nested compatibility boundaries.

### 1. ADL Semantic ABI

The semantic ABI describes what the Runtime must preserve:

- canonical ADL definition version and digest;
- deterministic ExecutionPlan contract and digest;
- stable run, workflow, node, agent, provider, tool, and port identities;
- required capabilities and policy profiles;
- versioned external protocols;
- durable state domains and schema versions;
- continuity lineage and accepted event position;
- placement requirements and constraints.

Any Runtime implementation may host the definition if it proves that it can
interpret the required contracts and provide their declared behavior.

### 2. Component ABI

Replaceable capability implementations may use the WebAssembly Component Model
with versioned WIT interfaces. Wasmtime is the host boundary. Components never
receive ambient kernel authority; every filesystem, network, secret, clock,
state, model, provider, and tool operation is an explicit capability.

Native Guardian and kernel releases are replaced as whole immutable binaries.
They are not hot-linked or hot-patched.

## Runtime Transfer Envelope

Introduce a canonical, signed contract:

```text
adl.runtime-transfer.v1
```

Required fields:

```yaml
schema: adl.runtime-transfer.v1
transfer_id: content-addressed identifier
created_at: trusted-time observation
source:
  guardian_id: stable Guardian identity
  runtime_instance_id: source Runtime instance
  runtime_release: exact release digest
  platform: target triple and declared platform profile
target:
  guardian_id: expected destination Guardian
  placement_target: ADL placement identity
semantic_contract:
  adl_document_version: version
  adl_document_digest: digest
  execution_plan_contract: adl.execution-plan.v1
  execution_plan_digest: digest
  engine_contract: adl.engine.v1
  runtime_api_version: version
  acip_versions: supported required versions
capabilities:
  required: canonical capability requirements
  optional: canonical optional capability requirements
state:
  checkpoint_contract: adl.engine-checkpoint.v1
  domains: ordered schema and digest inventory
  continuity_generation: monotonic generation
  continuity_head: signed accepted-through position
  previous_integrity: prior continuity link
authority:
  source_epoch: current fencing epoch
  proposed_target_epoch: strictly greater epoch
  authority_id: trusted fencing authority
  lease_id: current time-bounded write lease
  lease_not_before: trusted-time lower bound
  lease_not_after: trusted-time upper bound
  maximum_clock_skew: admitted skew bound
  quorum_ref: optional distributed authorization reference
migration:
  policy: exact | compatible | transform
  migrators: ordered content-addressed migrator identities
  migrator_trust_policy: required governance trust policy
  rollback_release: source release digest
integrity:
  manifest_digest: canonical manifest digest
  signature_algorithm: Ed25519 or approved successor
  signing_key_id: non-secret key identity
  signature: detached canonical-manifest signature
```

The envelope references independently checksummed state blobs. It does not
embed secret material.

## Compatibility Model

Compatibility is a deterministic target-side decision.

### Exact

Use for restart under the same semantic and state contracts:

- identical ADL and ExecutionPlan digests;
- identical topology and canonical semantic configuration;
- identical service schemas;
- no migrators;
- existing signed lineage remains valid.

This is the current Runtime v3 restart model.

### Compatible

Use when the target Runtime differs but directly understands all contracts:

- ADL document version is accepted;
- ExecutionPlan and engine contracts are accepted;
- every required capability is implemented at the required contract version;
- every state domain is directly readable;
- policy and governance contracts are equal or explicitly stronger;
- host-local bindings are resolved independently;
- target re-computes the same semantic identity.

### Transform

Use only with explicit deterministic schema migration:

- each input and output schema is named;
- every migrator is content-addressed and signed;
- every signature chains to a governance or deployment key accepted by the
  destination's configured trust store;
- transient, unknown, expired, revoked, and wrong-purpose signing keys are
  rejected before any transformation;
- migrators have no network, clock, random, secret, or ambient filesystem
  authority;
- transformation output is canonical and independently hashed;
- original state remains immutable for rollback;
- the target performs restore and replay validation before authority transfer.

Unknown versions, missing capabilities, weaker policy, ambiguous identity, and
unavailable migrators refuse transfer before source quiescence.

## Portable State Boundary

Portable state includes only governed, versioned semantic information:

- stable agent and Runtime lineage identities;
- canonical ADL and ExecutionPlan references;
- durable agent state and checkpoints;
- event and lifelog positions;
- idempotency and replay records;
- governance decisions and active policy references;
- capability grants and revocations;
- scheduler-owned durable admissions;
- memory and identity references authorized for movement;
- continuity signatures and witness evidence.

Machine-local bindings must not affect semantic identity:

- absolute paths and worktree locations;
- ports, interface addresses, hostnames, and DNS results;
- PIDs, process handles, sockets, and file descriptors;
- Tokio task or channel handles;
- TLS private keys and private-key hashes;
- provider credentials and bearer tokens;
- OS keystore handles;
- transient queues reconstructable from durable admissions;
- cache contents and build artifacts;
- Vector process IDs and local log paths.

The target resolves machine bindings from its local init and secret providers
after semantic compatibility succeeds.

## In-Place Upgrade

In-place upgrades use immutable native releases:

1. Obtain or build a release for one exact source revision.
2. Verify signed release metadata and binary hashes.
3. Install into a new immutable release directory.
4. Validate init, platform support, schema compatibility, and available space.
5. Start the candidate on isolated state and alternate listeners.
6. Run HTTPS, mTLS, ACIP HTTP/WSS, Runtime API, component, state, logging, and
   observability qualification.
7. Ask Guardian to close admission and create a final signed checkpoint.
8. Atomically switch the executable release pointer.
9. Restart through Guardian and the host service manager.
10. Require bounded readiness and live WSS qualification.
11. If failure occurs before the candidate admits mutating work, restore the
    previous release pointer and checkpoint-compatible state, then restart.
12. If the candidate has admitted mutating work, preserve it as the authority
    and perform any return as a new governed state transfer. Never discard
    accepted writes by switching back to an older checkpoint.
13. Retain the failed candidate and causal logs for diagnosis.

Packaging and distribution may use cargo-dist or another signed artifact
publisher. Runtime correctness does not depend on the distribution tool.

## Blue-Green Machine Migration

The safe cross-machine flow is:

```text
prepare target
  -> negotiate semantic and capability compatibility
  -> copy immutable checkpoint baseline
  -> shadow restore and replay
  -> stream durable post-baseline events
  -> close source admission
  -> write final source checkpoint
  -> transfer higher fencing epoch
  -> activate destination
  -> switch ingress and discovery
  -> retain source as fenced rollback candidate
  -> commit after qualification window
```

### Prepare

- Destination Guardian proves node identity, platform profile, certificate
  health, capacity, storage, time, and required capabilities.
- Destination loads the same ADL definition and independently compiles or
  verifies the deterministic ExecutionPlan.
- Source and destination compare semantic ABI manifests.

### Shadow

- Destination imports an immutable baseline without serving authoritative
  traffic.
- Destination replays through the accepted continuity head.
- Destination exposes shadow health and parity evidence.
- No external success is emitted from shadow execution.

### Final Transfer

- Source stops new admission but finishes or durably defers owned work.
- Source writes and signs the final checkpoint.
- Destination imports the final delta and verifies complete lineage.
- Source relinquishes or allows expiry of its time-bounded write lease.
- Source storage refuses further mutations once that lease is absent or
  expired, even if network clients can still reach the source process.
- Fencing authority waits through the admitted maximum lease duration and
  clock-skew bound, then grants a strictly higher epoch and lease to the
  destination.
- Destination cannot open mutating admission until the higher lease is active.

### Activation

- Destination opens admission and publishes signed readiness.
- Stable ingress, discovery, or Guardian routing moves clients to destination.
- HTTP clients retry idempotently.
- WSS clients reconnect with continuity and subscription resume tokens.
- Open sockets themselves are not migrated.

### Commit And Rollback

Before destination activation, rollback may abandon the candidate and retain
or restore source authority without losing accepted work. Once the destination
opens mutating admission, rollback by reactivating an older source checkpoint
is forbidden. Returning authority to the source is a new governed transfer
from destination to source with a new transfer ID, final checkpoint, higher
epoch, and time-bounded lease.

## Migrate Away, Upgrade, And Migrate Back

The same protocol supports platform maintenance:

1. Migrate Runtime lineage from machine A to qualified machine B.
2. Keep A fenced while B is authoritative.
3. Upgrade or replace A's Guardian, Runtime release, OS, or hardware.
4. Qualify A as a new destination against B's semantic ABI.
5. Perform a second normal transfer from B to A.

There is no special "return" operation. Migration back is an ordinary transfer
with a new transfer ID, a higher fencing epoch, and a new signed continuity
head.

## Authority And Split-Brain Prevention

Migration requires an authority record independent of either Runtime process:

- stable lineage ID;
- current authoritative Guardian;
- monotonically increasing fencing epoch;
- time-bounded write lease with trusted-time bounds and admitted clock skew;
- quorum or operator authorization reference;
- activation and revocation timestamps;
- signed transition history.

Every durable write, admitted operation, and externally visible success carries
the current epoch and lease identity. The redb transaction boundary verifies
that the local lease is active and unexpired before committing a mutation.
Losing authority connectivity never extends a lease: after expiry the Runtime
self-fences all mutation while keeping health, diagnostics, and transfer
recovery surfaces available. Peers reject stale epochs and expired leases.
Merely losing a heartbeat never authorizes a second writer.

The first implementation uses one strongly consistent authority provider.
It cannot grant a destination lease until the source lease has been explicitly
revoked with proof or the maximum lease duration plus admitted clock skew has
elapsed. Distributed quorum can follow behind the same typed contract. The
protocol must not pretend that local redb state or two independent local files
form consensus.

## Protocol And Client Continuity

- OpenAPI remains the versioned HTTP client contract.
- ACIP remains the agent invocation and communication contract.
- Protobuf remains the compact internal envelope.
- WSS remains the full-duplex event, progress, cancellation, and invocation
  carrier.
- Capability negotiation occurs before transfer and before accepting resumed
  client work.
- Request IDs, idempotency keys, event sequence, subscription cursor, and
  continuity generation survive reconnect.
- DNS, a load balancer, or Guardian directory supplies a stable endpoint name;
  machine addresses are not part of Runtime identity.

## Secrets, TLS, And Trust

- Transfer uses mutually authenticated, fully verified TLS.
- State blobs are encrypted in transit and at rest by approved providers.
- Transfer authorization and state integrity signatures are distinct.
- Private keys do not move inside transfer bundles.
- Destination obtains or activates its own certificate and secret references.
- Certificate rotation and Runtime migration are independently auditable.
- Trust-domain, SAN, EKU, expiry, revocation, and purpose checks fail closed.

## Observability

Every phase emits tracing events through the existing Vector pipeline:

- compatibility negotiation and refusal;
- baseline and delta transfer progress;
- source quiesce;
- checkpoint generation and integrity;
- fencing epoch transition;
- destination activation;
- ingress switch;
- WSS resume;
- qualification success or causal failure;
- rollback or commit.

Required correlation fields:

```text
transfer_id
lineage_id
source_guardian_id
target_guardian_id
source_release
target_release
continuity_generation
fencing_epoch
adl_document_digest
execution_plan_digest
phase
result
reason_code
```

Logging failure must not crash the Runtime, but missing required audit
durability prevents migration commit. Audit durability means successful append
to the bounded local durable audit spool, not successful delivery to Vector or
an external sink. Vector drains that spool asynchronously. Sink unavailability
does not stall Runtime service or an otherwise safe transfer; spool exhaustion
does fail the transfer before authority changes.

## Failure Semantics

| Failure | Required result |
| --- | --- |
| Candidate binary fails isolated qualification | Source continues; candidate retained |
| Incompatible ADL or plan contract | Refuse before quiesce |
| Missing capability | Refuse or choose another destination |
| State schema requires absent migrator | Refuse before quiesce |
| Corrupt or unauthenticated blob | Quarantine transfer; source continues |
| Network loss before authority transfer | Source remains authoritative |
| Source loses authority connectivity | Source self-fences mutation when its lease expires |
| Network loss after source fencing but before destination activation | Reconcile the authority lease; do not guess |
| Destination failure before mutating admission | Abandon destination and retain or restore source authority |
| Destination failure after mutating admission | Recover destination or perform a new governed destination-to-source transfer |
| Source failure after destination commit | Destination remains authoritative |
| Duplicate or reordered transfer messages | Reject using transfer ID, sequence, and phase state |
| Stale source attempts a write | Reject stale epoch or absent/expired lease at the storage boundary |
| Observatory or telemetry consumer fails | Runtime continues; record consumer failure |
| Vector or remote audit sink is unavailable | Buffer durably and continue while local spool capacity remains |

## Implementation Plan

### Issue 1: Semantic ABI Manifest And Compatibility Evaluator

- canonical `adl.runtime-compatibility.v1`;
- ADL, ExecutionPlan, engine, API, ACIP, capability, policy, and state inventory;
- deterministic compatible/incompatible result with reason codes.

### Issue 2: Transfer Envelope And State-Domain Catalog

- canonical `adl.runtime-transfer.v1`;
- signed manifest and independently checksummed blobs;
- bounded encoding and schema validation;
- no secrets or machine-local bindings.

### Issue 3: State Schema Registry And Deterministic Migrators

- state-domain version registry;
- direct-read compatibility;
- constrained Wasmtime/WIT migration component contract;
- original-state retention and rollback.

### Issue 4: Immutable In-Place Updater

- immutable release staging;
- release signature and hash verification;
- isolated qualification;
- atomic current-release switch;
- Guardian restart and automatic rollback.

### Issue 5: Fencing Leases And Migration State Machine

- prepare, shadow, quiesce, transfer, activate, and commit transitions;
- time-bounded write leases and monotonic epoch enforcement at redb commit;
- durable idempotent transitions;
- crash, clock-skew, and network-partition recovery;
- no reverse activation after mutating admission.

### Issue 6: Transfer Transport And Shadow Restore

- mTLS Guardian transfer carrier using an existing bounded transport;
- baseline plus ordered delta transfer;
- shadow restore, replay, and parity status;
- cancellation and interrupted-transfer cleanup.

### Issue 7: Stable Ingress And Client Resume

- Guardian directory or deployment-provider integration;
- HTTP idempotent retry;
- WSS subscription and continuity resume;
- no open-socket migration claim.

### Issue 8: Cross-Platform Qualification

- macOS, Linux, and native Windows;
- in-place upgrade and rollback;
- A-to-B and B-to-A migration;
- partition, crash, corruption, stale writer, and failed-migrator tests;
- clean tracing/Vector audit and no dual-active lineage.

Issues 1-3 may proceed in parallel after this architecture is accepted.
Issue 4 may proceed alongside Issue 5 because it uses the same compatibility
contract without distributed transfer. Issue 5 is a strict prerequisite for
Issue 6 and gates authoritative cross-machine activation. Issue 7 follows the
activation contract. Issue 8 is final acceptance.

## Acceptance Criteria

The architecture is ready for implementation when:

1. ADL semantic identity and machine-local binding are explicitly separated.
2. Compatibility decisions are deterministic and versioned.
3. Exact local restart remains strict.
4. Compatible and transform migration cannot bypass policy or schema checks.
5. Source and destination cannot both hold valid write authority.
6. Failed transfer before commit leaves or restores one healthy authority.
7. In-place update automatically rolls back a failed candidate.
8. Migrate-away and migrate-back use the same protocol.
9. External clients have versioned retry and resume behavior.
10. Every transition is correlated in tracing and Vector audit.
11. Tests prove real processes on macOS, Linux, and native Windows.
12. No claim depends on fixtures, metadata, copied state, or simulated traffic.
13. A partitioned source loses mutation authority no later than its bounded
    lease expiry.
14. Once a destination accepts a mutation, every return to the old platform is
    a forward state transfer rather than checkpoint rollback.
15. Migrator signatures chain to an explicitly configured governance trust
    policy.
16. External telemetry availability is not part of the authority safety path.

## Gemini Architecture Review

Gemini 3.1 Pro Preview reviewed this plan on 2026-07-30 as a production
architecture proposal.

The initial verdict accepted the semantic-ABI direction, immutable native
release model, and separation of semantic state from machine-local bindings.
It found five corrections:

1. a monotonic epoch in local redb could not prevent a partitioned source from
   continuing to write;
2. reactivating a source after destination admission could discard accepted
   destination writes;
3. deterministic migrators lacked an explicit signing trust root;
4. transport work was ordered before storage-level fencing proof; and
5. authority commit was too tightly coupled to an external Vector sink.

This revision incorporates all five findings through storage-enforced
time-bounded leases, forward-only post-admission recovery, governed migrator
trust, reordered implementation issues, and a bounded local durable audit
spool.

A second bounded Gemini verification pass returned `PASS` with no remaining
P0 or P1 finding and found no contradiction in the revised lease, activation,
forward-transfer, audit-spool, or implementation-order rules.

Residual implementation risks remain:

- WSS resume must tolerate proxy timeout and reconnect behavior without
  claiming open-socket migration.
- Transform migrations need preflight capacity for immutable source, output,
  and rollback state.
- Wasmtime and WIT versions must be pinned and compatibility-tested because the
  component ecosystem continues to evolve.

## Simplification Test

The implementation should preserve this conceptual model:

```text
ADL definition
  -> deterministic ExecutionPlan
  -> compatibility decision
  -> signed state transfer
  -> fenced authority change
  -> health-qualified activation
```

If an implementation requires another package manager, scheduler, database,
service manager, native plugin ABI, telemetry pipeline, or custom transport to
explain the basic flow, it is too complicated.

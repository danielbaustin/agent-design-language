# Issue 5821 Design: Distributed Guardian Architecture And Child-Wave Gate

## Outcome And Boundary

Issue 5821 is the WP-04 architecture, security, and child-wave planning gate.
It freezes the distributed Guardian trust and failure contract, publishes an
exact 16-child implementation ledger, and schedules a separate `WP-04-IMP`
implementation umbrella. It does not implement, integrate, or close the
distributed runtime program.

Completion means the architecture and threat model are reviewed, every child
has a stable identity, owner, dependency set, protected paths, proof boundary,
and rollback responsibility, and the implementation umbrella is created with
those sixteen children as its immutable denominator. No child receives
implementation credit from this planning gate.

## Architecture And Security Freeze

Guardian remains process 0 on every node. Node and Guardian identity,
enrollment, separate certificate purposes, discovery, membership, authenticated
transport, failure detection, epochs, leases, fencing, signed capability and
resource-weather advertisements, placement, snapshot catalog, migration,
rollback, audit events, and operator projections must be frozen before the
implementation wave opens. A maintained QUIC/TLS stack is the first transport;
custom cryptography, custom framing, plaintext, and verification bypasses are
forbidden.

Migration is `prepare -> quiesce -> checkpoint -> transfer -> validate ->
fence -> activate -> commit`. Source authority remains valid until target
validation and fencing succeed. Any ambiguity aborts to one authoritative
owner. The threat model covers partition, replay, stale lease, cloned state,
wrong node or trust domain, certificate compromise/expiry, relocation failure,
rollback failure, and split-brain activation.

## Exact 16-Child Ledger

WP-04-IMP is live as issue #5862. Every row below names the actual child issue,
the issue owner, required outcome, dependencies, exclusive product paths,
proving boundary, and rollback responsibility. Additions or denominator changes
require a new architecture/security gate review before binding.

| Child | Issue | Owner and required outcome | Depends on | Exclusive protected paths | Proof boundary | Rollback responsibility |
| --- | --- | --- | --- | --- | --- | --- |
| WP-04.01 | #5863 | Issue #5863: node identity and authenticated enrollment | WP-03, #5821 | `adl-runtime/src/distributed/identity.rs`, `adl-runtime/tests/distributed_identity.rs` | Exact nonzero `distributed_identity` test proves signed enrollment, restart stability, wrong-domain and replay denial | Remove issue-created enrollment records and preserve WP-03 single-node identity |
| WP-04.02 | #5864 | Issue #5864: certificate purposes, rotation, revocation, expiry | WP-04.01 | `adl-runtime/src/distributed/certificates.rs`, `adl-runtime/tests/distributed_certificates.rs` | Exact nonzero `distributed_certificates` test proves purpose separation, rotation, revocation, expiry and compromised-key denial | Restore last valid certificate generation without disabling verification |
| WP-04.03 | #5865 | Issue #5865: maintained QUIC/TLS transport adapter | WP-04.02 | `adl-runtime/src/distributed/transport.rs`, `adl-runtime/tests/distributed_transport.rs`, `adl-runtime/Cargo.toml`, `adl-runtime/Cargo.lock` | Exact nonzero `distributed_transport` test proves mTLS, bounds, cancellation, malformed frames and peer mismatch | Remove feature and restore manifest/lock while retaining single-node API |
| WP-04.04 | #5866 | Issue #5866: seed discovery and authenticated join | WP-04.03 | `adl-runtime/src/distributed/discovery.rs`, `adl-runtime/tests/distributed_discovery.rs` | Exact nonzero `distributed_discovery` test proves configured seeds, authenticated join, timeout and stale/wrong-domain refusal | Disable discovery and discard partial uncommitted membership |
| WP-04.05 | #5867 | Issue #5867: membership state and topology convergence | WP-04.04 | `adl-runtime/src/distributed/membership.rs`, `adl-runtime/tests/distributed_membership.rs` | Exact nonzero `distributed_membership` test proves convergence, epochs, ordering and restart recovery | Restore last committed membership epoch |
| WP-04.06 | #5868 | Issue #5868: failure detection and partition classification | WP-04.05 | `adl-runtime/src/distributed/failure_detection.rs`, `adl-runtime/tests/distributed_failure_detection.rs` | Exact nonzero `distributed_failure_detection` test proves bounded suspicion, partition, recovery and flapping behavior | Disable distributed decisions and retain committed membership |
| WP-04.07 | #5869 | Issue #5869: epoch and lease authority | WP-04.05 | `adl-runtime/src/distributed/lease.rs`, `adl-runtime/tests/distributed_lease.rs` | Exact nonzero `distributed_lease` test proves monotonic epochs, renewal, expiry, stale-holder denial and restart recovery | Expire issue-created leases and restore last durable epoch |
| WP-04.08 | #5870 | Issue #5870: fencing and single-owner enforcement | WP-04.06, WP-04.07 | `adl-runtime/src/distributed/fencing.rs`, `adl-runtime/tests/distributed_fencing.rs` | Exact nonzero `distributed_fencing` test proves stale, cloned, split-brain and post-partition fencing | Fence uncertain owners and return to last durable single owner |
| WP-04.09 | #5871 | Issue #5871: signed capability advertisements | WP-04.03 | `adl-runtime/src/distributed/capability_advertisement.rs`, `adl-runtime/tests/distributed_capability_advertisement.rs` | Exact nonzero `distributed_capability_advertisement` test proves signatures, expiry, replay, bounds and redaction | Withdraw advertisements and treat capability as unavailable |
| WP-04.10 | #5872 | Issue #5872: signed resource-weather advertisements | WP-04.03 | `adl-runtime/src/distributed/resource_weather.rs`, `adl-runtime/tests/distributed_resource_weather.rs` | Exact nonzero `distributed_resource_weather` test proves freshness, signatures, bounds, replay denial and redaction | Withdraw observations and apply declared no-data policy |
| WP-04.11 | #5873 | Issue #5873: bounded deterministic placement | WP-04.05, WP-04.08, WP-04.09, WP-04.10 | `adl-runtime/src/distributed/placement.rs`, `adl-runtime/tests/distributed_placement.rs` | Exact nonzero `distributed_placement` test proves deterministic choice, limits, stale input and fenced-node exclusion | Disable remote placement and retain current owner |
| WP-04.12 | #5874 | Issue #5874: snapshot catalog and transfer manifest | WP-04.02, WP-04.08 | `adl-runtime/src/distributed/snapshot_catalog.rs`, `adl-runtime/tests/distributed_snapshot_catalog.rs` | Exact nonzero `distributed_snapshot_catalog` test proves digest binding, authorization, redaction and corruption denial | Delete incomplete transfers and retain last valid local catalog |
| WP-04.13 | #5875 | Issue #5875: migration state machine | WP-04.08, WP-04.11, WP-04.12 | `adl-runtime/src/distributed/migration.rs`, `adl-runtime/tests/distributed_migration.rs` | Exact nonzero `distributed_migration` test proves every transition, idempotence, source retention, validation and fencing | Abort before commit, fence target and resume validated source |
| WP-04.14 | #5876 | Issue #5876: rollback, recovery and relocation failure | WP-04.13 | `adl-runtime/src/distributed/recovery.rs`, `adl-runtime/tests/distributed_recovery.rs` | Exact nonzero `distributed_recovery` test proves each failure stage, restart, target/source loss and one-owner restoration | Fence both sides on ambiguity and recover from last validated owner |
| WP-04.15 | #5877 | Issue #5877: versioned distributed projection | WP-04.05, WP-04.08, WP-04.13, WP-04.14 | `adl-runtime/src/distributed/projection.rs`, `adl-runtime/tests/distributed_projection.rs`, `docs/api/runtime-v3/v1/distributed.openapi.json` | Exact nonzero `distributed_projection` test and OpenAPI proof validate parity, redaction, ordering and compatibility | Disable new projection version without weakening auth or exposing state |
| WP-04.16 | #5878 | Issue #5878: module registration, integration, adversarial and native proof | WP-04.01 through WP-04.15 | `adl-runtime/src/distributed/mod.rs`, `adl-runtime/src/lib.rs`, `adl-runtime/tests/distributed_guardian.rs`, `adl/tools/validate_v092_distributed_guardian.sh`, `adl/tools/validate_v092_distributed_native_receipts.rb` | Production Guardian/kernel multi-node API/WSS, partition, fencing, migration, recovery, shutdown and digest-bound macOS/Linux/Windows proof | Remove module registration, fence remote ownership and prove WP-03 single-node health |

## Owned Paths

- `.csdlc/issues/5821`
- `.csdlc/prepared/issues/5821`
- `.csdlc/evidence/5821`
- `docs/architecture/runtime-v3/DISTRIBUTED_GUARDIAN_ARCHITECTURE.md`
- `docs/security/runtime-v3/DISTRIBUTED_GUARDIAN_THREAT_MODEL.md`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Dependencies And Scheduling

WP-03 issue 5820 must be terminal before the architecture is approved.
WP-04-IMP issue #5862 is the separately scheduled implementation umbrella
depending on this gate. Its exactly sixteen live children are #5863 through
#5878. It preserves the graph above, schedules only dependency-ready claims,
and blocks WP-14 issue #5832 until terminal integrated output exists.

## Validation And Review

Gate validation parses the architecture, threat model, and seven-field ledger;
requires exactly sixteen live mapped identities; rejects missing owner, proof,
rollback, duplicate or overlapping paths, dependency cycles, card/readiness
drift, and denominator mismatch against #5862. A separate architecture/security
packet validator checks required sections, threat classes, COTS boundary,
reviewer-authored report identity and exact packet digests. Product tests and
multi-node proof remain child and #5862 obligations, not gate completion.

## Estimate

Budget this planning gate at 12 elapsed hours, 140,000 reasoning tokens, and 2
hours of document/schema/ledger validation and review. The later implementation
umbrella must estimate each child independently and may not inherit this gate
budget.

## Rollback

Withdraw only the unapproved distributed child wave and WP-04-IMP umbrella,
retain the architecture, threat-model, and review evidence for correction, and
leave the terminal WP-03 single-node Runtime unchanged. Do not modify product
paths, weaken authentication, or activate distributed ownership during rollback.

## Non-Goals

- Distributed Runtime implementation or integration in issue 5821.
- Creating fewer, more, or differently scoped children during execution.
- v0.93 constitutional governance or polis policy redesign.
- Custom cryptography, Runtime v2 changes, or cognition-owned placement.

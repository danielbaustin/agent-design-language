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

The implementation umbrella uses these stable identities. Each protected path
is exclusive to that child; additions require architecture-gate review before
binding.

| Child | Owner and required outcome | Depends on | Protected paths |
| --- | --- | --- | --- |
| WP-04.01 | Node identity and enrollment | WP-03 | `adl-runtime/src/distributed/identity.rs`, `adl-runtime/tests/distributed_identity.rs` |
| WP-04.02 | Certificate purposes, rotation, revocation, expiry | WP-04.01 | `adl-runtime/src/distributed/certificates.rs`, `adl-runtime/tests/distributed_certificates.rs` |
| WP-04.03 | Maintained QUIC/TLS transport adapter | WP-04.02 | `adl-runtime/src/distributed/transport.rs`, `adl-runtime/tests/distributed_transport.rs` |
| WP-04.04 | Seed discovery and authenticated join | WP-04.03 | `adl-runtime/src/distributed/discovery.rs`, `adl-runtime/tests/distributed_discovery.rs` |
| WP-04.05 | Membership state and topology convergence | WP-04.04 | `adl-runtime/src/distributed/membership.rs`, `adl-runtime/tests/distributed_membership.rs` |
| WP-04.06 | Failure detection and partition classification | WP-04.05 | `adl-runtime/src/distributed/failure_detection.rs`, `adl-runtime/tests/distributed_failure_detection.rs` |
| WP-04.07 | Epoch and lease authority | WP-04.05 | `adl-runtime/src/distributed/lease.rs`, `adl-runtime/tests/distributed_lease.rs` |
| WP-04.08 | Fencing and single-owner enforcement | WP-04.06, WP-04.07 | `adl-runtime/src/distributed/fencing.rs`, `adl-runtime/tests/distributed_fencing.rs` |
| WP-04.09 | Signed capability advertisements | WP-04.03 | `adl-runtime/src/distributed/capability_advertisement.rs`, `adl-runtime/tests/distributed_capability_advertisement.rs` |
| WP-04.10 | Signed resource-weather advertisements | WP-04.03 | `adl-runtime/src/distributed/resource_weather.rs`, `adl-runtime/tests/distributed_resource_weather.rs` |
| WP-04.11 | Bounded placement decisions | WP-04.05, WP-04.08, WP-04.09, WP-04.10 | `adl-runtime/src/distributed/placement.rs`, `adl-runtime/tests/distributed_placement.rs` |
| WP-04.12 | Snapshot catalog and transfer manifest | WP-04.02, WP-04.08 | `adl-runtime/src/distributed/snapshot_catalog.rs`, `adl-runtime/tests/distributed_snapshot_catalog.rs` |
| WP-04.13 | Migration state machine | WP-04.08, WP-04.11, WP-04.12 | `adl-runtime/src/distributed/migration.rs`, `adl-runtime/tests/distributed_migration.rs` |
| WP-04.14 | Rollback, recovery, and relocation failure | WP-04.13 | `adl-runtime/src/distributed/recovery.rs`, `adl-runtime/tests/distributed_recovery.rs` |
| WP-04.15 | Versioned topology, certificate, migration, and failure projection | WP-04.05, WP-04.08, WP-04.13, WP-04.14 | `adl-runtime/src/distributed/projection.rs`, `docs/api/runtime-v3/v1/distributed.openapi.json` |
| WP-04.16 | Real multi-node adversarial and platform proof | WP-04.01 through WP-04.15 | `adl-runtime/tests/distributed_guardian.rs`, `adl/tools/validate_v092_distributed_guardian.sh` |

The gate owns only `.csdlc/prepared/issues/5821/`, its issue cards/evidence,
`docs/architecture/runtime-v3/DISTRIBUTED_GUARDIAN_ARCHITECTURE.md`,
`docs/security/runtime-v3/DISTRIBUTED_GUARDIAN_THREAT_MODEL.md`, and the retained
16-row ledger. `WP-04-IMP` owns orchestration and reconciliation evidence only;
it does not preclaim child product paths.

## Dependencies And Scheduling

WP-03 issue 5820 must be terminal before the architecture is approved.
`WP-04-IMP` is a separately scheduled implementation umbrella depending on
this gate. It must create and prepare exactly WP-04.01 through WP-04.16 before
any child starts, preserve the dependency graph above, serialize path claims,
and block WP-14 issue 5832 until integrated distributed contracts are stable.

## Validation And Review

Gate validation parses the architecture, threat model, and ledger; requires
exactly sixteen unique identities; rejects duplicate or overlapping protected
paths; verifies every dependency resolves; and confirms the implementation
umbrella names the same denominator. Independent architecture and security
review must have no unresolved actionable findings. Product tests and
multi-node proof are future child and `WP-04-IMP` obligations, not commands or
completion evidence for issue 5821.

## Estimate And Rollback

Budget this planning gate at 12 elapsed hours, 140,000 reasoning tokens, and 2
hours of document/schema/ledger validation and review. The later implementation
umbrella must estimate each child independently and may not inherit this gate
budget. Rollback withdraws the unapproved child wave and umbrella, retains the
review packet, and leaves the WP-03 single-node Runtime unchanged.

## Non-Goals

- Distributed Runtime implementation or integration in issue 5821.
- Creating fewer, more, or differently scoped children during execution.
- v0.93 constitutional governance or polis policy redesign.
- Custom cryptography, Runtime v2 changes, or cognition-owned placement.

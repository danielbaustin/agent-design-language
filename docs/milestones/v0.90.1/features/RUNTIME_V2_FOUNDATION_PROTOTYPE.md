# Runtime v2 Foundation Prototype

## Purpose

Define the smallest implementation slice that makes Runtime v2 executable and
reviewable.

## Required Prototype

The prototype should include:

- one manifold
- two provisional citizens
- one bounded kernel loop
- one scheduler/resource signal
- one snapshot and rehydration path
- one invariant violation
- one operator-control report
- one security-boundary proof

## Proof Claim

ADL can host a bounded persistent runtime substrate with explicit continuity
handles and failure artifacts.

## Non-Claims

- no true Gödel-agent birthday
- no full moral/emotional civilization
- no complete migration system
- no full security ecology

## Candidate Artifacts

- `runtime_v2/proof_packet.json`
- `runtime_v2/manifold.json`
- `runtime_v2/kernel/service_loop.jsonl`
- `runtime_v2/citizens/*/record.json`
- `runtime_v2/snapshots/*/snapshot.json`
- `runtime_v2/rehydration_report.json`
- `runtime_v2/invariants/*.json`
- `runtime_v2/operator/control_report.json`
- `runtime_v2/security_boundary/proof_packet.json`

## WP-12 Integrated Demo

The integrated Runtime v2 foundation demo is now available as demo matrix row
D7:

```bash
cargo run --manifest-path adl/Cargo.toml -- demo demo-l-v0901-runtime-v2-foundation --run --trace --out artifacts/v0901 --no-open
```

The same contract can also be generated directly through the Runtime v2 CLI:

```bash
cargo run --manifest-path adl/Cargo.toml -- runtime-v2 foundation-demo --out artifacts/v0901/demo-l-v0901-runtime-v2-foundation
```

The demo writes one integrated artifact graph rooted at
`runtime_v2/proof_packet.json`. The proof packet indexes the WP-05 through
WP-11 artifacts, records reviewer-visible proof claims and non-claims, and
classifies the demo as `proving` for the bounded v0.90.1 foundation prototype.

The proof remains intentionally bounded: it proves inspectability and
cross-artifact linkage for the foundation substrate, not true Gödel-agent
birth, live scheduling, cross-machine migration, or the full security ecology.

## WP-11 Security Boundary Proof

The foundation prototype now exposes one bounded security-boundary proof through
`runtime_v2_security_boundary_proof_contract()` and:

```bash
adl runtime-v2 security-boundary --out .adl/state/runtime_v2_security_boundary_proof.v1.json
```

The proof consumes the invariant violation and operator-control report and
shows that an invalid resume attempt is refused before the paused Runtime v2
state changes. This is a first wall stone for Runtime v2 safety evidence, not a
complete defensive ecology.

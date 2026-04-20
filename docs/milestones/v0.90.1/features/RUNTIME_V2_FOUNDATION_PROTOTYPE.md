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

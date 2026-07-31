# ADR 0053: Portable Signed Records And External Trust

- Status: Accepted
- Date: 2026-07-30
- Accepted in: v0.91.8
- Related issues: #5342
- Related ADRs: ADR 0002, ADR 0017, ADR 0028
- Source evidence:
  - `adl-v2/crates/adl-records/README.md`
  - `.csdlc/issues/5342/cards/sor.md`
  - merge commit `34186ad77`

## Context

Errors, events, traces, execution results, and artifact descriptors cross
process and storage boundaries. Serde shape alone does not establish stable
identity, integrity, authorization, replay behavior, or bounded decoding.

## Decision

ADL uses versioned, bounded portable record contracts with deterministic
canonical bytes and Ed25519 signed envelopes.

Verification is fail closed and requires:

- an immutable trust policy supplied by the caller;
- key, record-kind, profile, logical-validity, and revocation checks;
- exact payload identity and signature verification;
- an external replay guard;
- strict decoding that rejects unknown or duplicate fields.

An envelope cannot authorize itself or modify the trust policy that verifies
it. The records crate performs no key discovery, trust-on-first-use, clock,
network, filesystem, or Runtime operations.

## Consequences

- Record identity and verification are portable across transports.
- Hosts retain custody of keys, trusted time, policy, replay state, and durable
  storage.
- Schema or canonicalization changes require an explicit version transition.
- Callers must handle verification failure without accepting partial state.

## Alternatives Considered

### Sign ordinary serialized JSON bytes

Rejected. Serializer and map-order variation would make identity unstable.

### Let records carry their own trust policy

Rejected. Self-issued authority is not a trust boundary.

## Validation Notes

Validate canonical-byte determinism, schema limits, fresh-process identity,
signature and payload tampering, unknown/revoked keys, logical expiry, replay,
duplicate fields, and bounded channel decoding.

## Non-Claims

- This ADR does not define key generation, storage, rotation, or discovery.
- This ADR does not make every signed record trusted by every polis.


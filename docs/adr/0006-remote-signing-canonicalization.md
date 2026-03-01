# ADR 0006: Remote Request Signing Canonicalization (v0.7)

Status: Accepted

## Context

v0.7 D-11 requires deterministic, verifiable signed remote execution requests.
Signature verification previously failed when client/server canonical byte views
drifted due to envelope field ordering/attachment timing.

## Decision

Remote request signature bytes are defined by one canonicalization contract:

1. Start from `ExecuteRequest`.
2. Exclude only `security.request_signature`.
3. Recursively sort JSON object keys (including `HashMap`-backed fields).
4. Serialize as compact JSON bytes.

Both client and server must use the same routine:
- signing: `sign_execute_request_v1` -> `canonical_request_bytes`
- verification: `verify_execute_request_signature_v1` -> `canonical_request_bytes`

## Guarantees

- Deterministic signatures for identical request semantics.
- Signature verification is independent of map insertion order.
- Post-sign payload mutation fails with `REMOTE_REQUEST_SIGNATURE_MISMATCH`.
- Unsupported signature schema/version fails with deterministic malformed code.

## Non-Goals

- Changing signing algorithm (stays Ed25519 in v0.7).
- Expanding trust policy grammar in this ADR.
- Defining long-term key management/KMS flows.

## Consequences

- Any change to canonicalization behavior is a wire-compatibility change and
  must be treated as a security contract update.
- Regression tests are required to protect ordering/schema/tamper behavior.

## References

- `swarm/src/remote_exec.rs`
- `swarm/tools/demo_d11_signed_remote.sh`
- `docs/milestones/v0.7/DEMOS_v0.7.md`

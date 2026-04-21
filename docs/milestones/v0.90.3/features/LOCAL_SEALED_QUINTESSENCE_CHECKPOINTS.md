# Local Sealed Quintessence Checkpoints

## Status

WP-05 landed the local key policy, deterministic sealed checkpoint fixture, and
backend seam for v0.90.3.

## Purpose

Define a local-first protected checkpoint strategy for citizen continuity.

The internal name "quintessence checkpoint" refers to the sealed,
continuity-bearing core of citizen state. Public-facing docs may use "citizen
continuity checkpoint."

## Core Shape

A checkpoint package includes the following WP-05 fields now, with later WPs
adding ledger, witness, and receipt authority:

- sealed private state blob
- redacted projection
- content hash
- prior checkpoint hash
- citizen id
- manifold id
- sequence number
- schema id
- key id
- writer signature
- continuity witness (later WP)
- citizen-facing receipt (later WP)

## Local-First Backend

v0.90.3 should not depend on cloud confidential computing.

Acceptable first backend directions:

- deterministic test sealing fixture
- age-style local envelope encryption
- OS keychain integration
- TPM or platform Secure Enclave adapter
- YubiHSM-style hardware key adapter

WP-05 defines the backend seam so OS keychain, TPM, Secure Enclave, HSM, Nitro
Enclaves, Google Confidential Space, confidential VMs, or other TEE backends can
be added later without changing the checkpoint semantics.

The landed runtime evidence is:

- `adl/src/runtime_v2/private_state_sealing.rs`
- `adl/src/runtime_v2/tests/private_state_sealing.rs`
- `adl/tests/fixtures/runtime_v2/private_state/key_policy.json`
- `adl/tests/fixtures/runtime_v2/private_state/sealing_backend_seam.json`
- `adl/tests/fixtures/runtime_v2/private_state/proto-citizen-alpha.sealed-checkpoint.json`
- `adl/tests/fixtures/runtime_v2/private_state/sealing_negative_cases.json`

The milestone evidence note is:

- `docs/milestones/v0.90.3/LOCAL_PRIVATE_STATE_SEALING_v0.90.3.md`

## Non-Claims

- local sealing does not prove hardware isolation
- cloud enclave support is not required in v0.90.3
- encryption alone does not solve rollback or equivocation
- enclave-ready does not mean enclave-dependent

## Validation Targets

- sealed checkpoint cannot be treated as raw JSON
- redacted projection can be generated without exposing private state
- unavailable key causes safe failure
- wrong key causes safe failure
- checkpoint lineage remains verifiable without decrypting unrelated state

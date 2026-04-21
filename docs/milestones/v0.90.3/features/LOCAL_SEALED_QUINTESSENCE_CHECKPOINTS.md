# Local Sealed Quintessence Checkpoints

## Status

Planning contract for v0.90.3.

## Purpose

Define a local-first protected checkpoint strategy for citizen continuity.

The internal name "quintessence checkpoint" refers to the sealed,
continuity-bearing core of citizen state. Public-facing docs may use "citizen
continuity checkpoint."

## Core Shape

A checkpoint package should include:

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
- continuity witness
- citizen-facing receipt

## Local-First Backend

v0.90.3 should not depend on cloud confidential computing.

Acceptable first backend directions:

- deterministic test sealing fixture
- age-style local envelope encryption
- OS keychain integration
- TPM or platform Secure Enclave adapter
- YubiHSM-style hardware key adapter

The milestone should define the backend seam so Nitro Enclaves, Google
Confidential Space, confidential VMs, or other TEE backends can be added later
without changing the checkpoint semantics.

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

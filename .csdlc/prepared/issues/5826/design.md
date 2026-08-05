# Issue 5826 Design: Stable Name And Identity Root

## Outcome And Sources

Define the WP-09 identity record from `docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md`, the candidate birthday record in `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`, and existing lineage/witness authority in `adl/src/runtime_v2/memory_identity_architecture.rs` and `adl/src/runtime_v2/private_state_witness.rs`.

## Owned Surface

Candidate protected paths are the identity feature contract, `adl/src/runtime_v2/` for a narrowly named v0.92 identity record, matching Runtime tests and fixtures, and `.csdlc/evidence/5826/`. The record carries schema version, stable name, identity root, aliases, origin evidence, continuity head, memory/capability/witness references, provenance, and redaction policy.

## Contract

Stable name is a label bound to an identity root, never the root itself. Aliases are ordered, provenance-bearing additions and cannot silently replace the root. Identity creation rejects empty or ambiguous roots, duplicate/conflicting aliases, missing origin evidence, path-unsafe references, and continuity heads that do not bind to prior evidence.

## Dependencies And Invariants

WP-08/#5825 must be terminal before implementation; prior citizen-state lineage remains authoritative substrate. Serialization and identity-root derivation are deterministic. Raw private state is never required for review, and a display name, boot admission, wake, snapshot, or copied state cannot establish identity alone.

## Validation And Rollback

Focused schema tests cover canonical valid records and deterministic ordering. Negative tests cover missing roots, alias collision, provenance mismatch, substituted continuity head, and private-path disclosure. Rollback removes the v0.92 record/fixtures while retaining prior lineage primitives and WP-08 outputs.

## Non-Goals

This issue does not prove multi-cycle continuity, migration, citizenship, reputation, legal personhood, or the birthday event.

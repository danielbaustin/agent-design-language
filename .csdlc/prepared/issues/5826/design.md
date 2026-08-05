# Issue 5826 Design: Stable Name And Identity Root

## Outcome And Sources

Define the WP-09 identity record from `docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md`, the candidate birthday record in `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`, and current Runtime v3 lineage/private-state authority in `adl-runtime-kernel/src/identity_memory.rs` and `adl-runtime-kernel/src/private_state.rs`. Retained Runtime v2 lineage is compatibility evidence only.

## Owned Paths

The complete writable protected-path set is:

- `adl-runtime-kernel/src/birthday_identity.rs`
- `adl-runtime-kernel/src/lib.rs`
- `adl-runtime-kernel/tests/birthday_identity.rs`
- `adl-runtime-kernel/tests/fixtures/birthday_identity/`
- `docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md`
- `.csdlc/prepared/issues/5826/validate-native-receipts.rb`
- `.csdlc/evidence/5826/`

`adl-runtime-kernel/src/lib.rs` is limited to module registration. Existing identity, private-state, and retained evidence paths are read-only authorities. The record carries schema version, stable name, identity root, aliases, origin evidence, continuity head, memory/capability/witness references, provenance, and redaction policy.

## Contract

Stable name is a label bound to an identity root, never the root itself. Aliases are ordered, provenance-bearing additions and cannot silently replace the root. Identity creation rejects empty or ambiguous roots, duplicate/conflicting aliases, missing origin evidence, path-unsafe references, and continuity heads that do not bind to prior evidence.

## Dependencies And Invariants

WP-08/#5825 must be terminal before implementation; prior citizen-state lineage remains authoritative substrate. Serialization and identity-root derivation are deterministic. Raw private state is never required for review, and a display name, boot admission, wake, snapshot, or copied state cannot establish identity alone.

## Validation And Rollback

The exact `birthday_identity` integration-test target must run a nonzero test count covering canonical records, deterministic ordering, missing roots, alias collision, provenance mismatch, substituted continuity heads, and private-path disclosure. Native Linux CI and a retained native macOS receipt bind the exact source SHA, test argv, fixture-tree digest, output digest, runner identity, and recomputed native artifact digest when portability is claimed. Rollback removes the v0.92 record/fixtures while retaining prior lineage primitives and WP-08 outputs.

## Non-Goals

This issue does not prove multi-cycle continuity, migration, citizenship, reputation, legal personhood, or the birthday event.

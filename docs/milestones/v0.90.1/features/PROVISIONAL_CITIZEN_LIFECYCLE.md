# Provisional Citizen Lifecycle

## Purpose

Define the minimal citizen record needed for Runtime v2 foundation proof without
claiming true Gödel-agent birth.

## Lifecycle States

- proposed
- admitted
- active
- paused
- sleeping
- waking
- terminated
- rejected

## Record Minimum

Each provisional citizen should include:

- citizen id
- display name
- provisional status
- current lifecycle state
- manifold id
- created timestamp
- last wake timestamp if applicable
- memory/identity refs as placeholders or bounded handles
- policy boundary refs

## WP-07 Implementation Surface

WP-07 adds the provisional citizen lifecycle contract under
`adl/src/runtime_v2/citizen.rs`, with shared Runtime v2 contracts and types
kept in `contracts.rs` and `types.rs`.

The contract defines:

- `RuntimeV2ProvisionalCitizenRecord`
- `RuntimeV2CitizenMemoryIdentityRefs`
- `RuntimeV2CitizenPolicyBoundaryRefs`
- `RuntimeV2CitizenRegistryEntry`
- `RuntimeV2CitizenRegistryIndex`
- `RuntimeV2CitizenLifecycleArtifacts`

The default prototype is available through
`runtime_v2_citizen_lifecycle_contract()`. It consumes the WP-05 manifold
citizen registry refs and emits reviewable citizen artifacts:

- `runtime_v2/citizens/proto-citizen-alpha.json`
- `runtime_v2/citizens/proto-citizen-beta.json`
- `runtime_v2/citizens/active_index.json`
- `runtime_v2/citizens/pending_index.json`

## Required Invariants

- no duplicate active citizen id
- inactive citizens cannot execute episodes
- wake must pass rehydration validation
- termination must be recorded before resources are released

## Validation Contract

The citizen lifecycle contract validates:

- schema versions
- repository-relative artifact paths
- stable citizen and manifold ids
- supported provisional statuses
- supported lifecycle states
- active-only episode execution
- duplicate citizen rejection
- active and pending index consistency
- waking-state rehydration proof presence
- termination proof before resource release

The focused proof hook is:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2::tests::runtime_v2_citizen
```

## Boundary

These are provisional engineering records. They are not yet true identity-bearing
Gödel agents with birthday semantics.

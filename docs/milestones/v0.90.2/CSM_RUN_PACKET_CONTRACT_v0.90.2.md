# CSM Run Packet Contract - v0.90.2

## Status

WP-03 / D2 contract artifact: LANDED.

This document defines the first bounded CSM run packet contract for
`proto-csm-01`. It is intentionally a contract and fixture gate, not a live run
claim. Later work packages must consume this shape instead of inventing their
own run packet surfaces.

## Source Evidence

| Evidence | Role |
| --- | --- |
| `adl/src/runtime_v2/csm_run.rs` | Code-backed contract type, prototype, validation, and serialization |
| `adl/tests/fixtures/runtime_v2/csm_run/run_packet_contract.json` | Golden contract fixture used by Runtime v2 tests |
| `demos/fixtures/csm_run/proto-csm-01-run-packet.json` | Reviewer-facing fixture definition for the first bounded run |
| `docs/milestones/v0.90.2/RUNTIME_V2_INHERITANCE_AND_COMPRESSION_AUDIT_v0.90.2.md` | WP-02 inheritance gate that this contract consumes |
| `docs/milestones/v0.90.2/DEMO_MATRIX_v0.90.2.md` | D2 proof target |

## Contract Scope

The CSM run packet contract fixes:

- the schema id: `runtime_v2.csm_run_packet_contract.v1`
- the target manifold: `proto-csm-01`
- the D2 demo/proof mapping
- the required pre-live-run artifact set
- the first-run stage sequence
- the reviewer entrypoint and validation command
- explicit non-claims for live execution, later invariant expansion, and later
  milestone scopes

## Required Artifacts

The first bounded run must use these artifact requirements:

| Artifact | Owner | Required By | Purpose |
| --- | --- | --- | --- |
| `runtime_v2/csm_run/run_packet_contract.json` | WP-03 | WP-04 | Stable first-run packet contract |
| `runtime_v2/csm_run/proto-csm-01-run-packet.json` | WP-03 | WP-05 | Fixture definition for the first live run |
| `runtime_v2/invariants/csm_run_invariant_map.json` | WP-04 | WP-05 | Expanded invariant map before live work widens |
| `runtime_v2/violations/violation_artifact_schema.json` | WP-04 | WP-08 | Stable invalid-action and violation artifact shape |
| `runtime_v2/csm_run/boot_manifest.json` | WP-05 | WP-14 | Live manifold boot evidence |
| `runtime_v2/csm_run/first_run_trace.jsonl` | WP-06 | WP-14 | Ordered trace spine for scheduling, mediation, and rejection |
| `runtime_v2/observatory/visibility_packet.json` | WP-10 | WP-14 | Operator-visible projection of the bounded run |

## Stage Contract

The first-run stage order is fixed and must remain contiguous:

| Sequence | Stage | Owner | Exit Artifact |
| --- | --- | --- | --- |
| 1 | `contract_and_fixture` | WP-03 | `runtime_v2/csm_run/run_packet_contract.json` |
| 2 | `invariant_and_violation_contract` | WP-04 | `runtime_v2/invariants/csm_run_invariant_map.json` |
| 3 | `boot_and_admission` | WP-05 | `runtime_v2/csm_run/boot_manifest.json` |
| 4 | `governed_episode_and_rejection` | WP-06-WP-08 | `runtime_v2/csm_run/first_run_trace.jsonl` |
| 5 | `snapshot_wake_and_observatory` | WP-09-WP-10 | `runtime_v2/observatory/visibility_packet.json` |

Later WPs may add evidence fields to their own artifacts, but they must not
silently reorder this spine or produce competing first-run packet contracts.

## D2 Classification

D2 is partially proving after WP-03.

Proved now:

- a code-backed CSM run packet contract exists
- the contract round-trips to a golden fixture
- the fixture has a bounded claim boundary and stable review target
- the run spine and pre-live-run gates are explicit

Not proved yet:

- WP-04 invariant map and violation schema have not landed
- `proto-csm-01` has not booted or executed a live run
- Observatory output has not been generated from live first-run artifacts

## Validation

Focused validation:

```sh
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_run_packet_contract -- --nocapture
```

This validates the contract prototype, golden fixture, path hygiene, and
negative cases for unsafe paths, non-contiguous stages, missing violation
schema, and live-run overclaiming.

## Non-Claims

This contract does not prove:

- a live CSM run
- complete WP-04 invariant expansion
- first true Gödel-agent birth
- full v0.91 moral or emotional civilization
- v0.92 identity, migration, capability rebinding, or birthday semantics

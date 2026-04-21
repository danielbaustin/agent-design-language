# CSM Run Packet Contract - v0.90.2

## Status

WP-03 / D2 packet contract artifact: LANDED.
WP-04 / D2 invariant and violation contract artifacts: LANDED.
WP-05 / D3 boot and admission artifacts: LANDED.
WP-06 / D4 resource-pressure scheduling artifacts: LANDED.
WP-07 / D4 Freedom Gate mediation artifacts: LANDED.

This document defines the first bounded CSM run packet contract for
`proto-csm-01`. It is intentionally a contract and fixture gate, not a live run
claim. Later work packages must consume this shape instead of inventing their
own run packet surfaces.

## Source Evidence

| Evidence | Role |
| --- | --- |
| `adl/src/runtime_v2/csm_run.rs` | Code-backed contract type, prototype, validation, and serialization |
| `adl/tests/fixtures/runtime_v2/csm_run/run_packet_contract.json` | Golden contract fixture used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/invariants/csm_run_invariant_map.json` | Golden invariant map fixture used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/violations/violation_artifact_schema.json` | Golden violation schema fixture used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/csm_run/boot_manifest.json` | Golden boot manifest fixture used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/csm_run/citizen_roster.json` | Golden citizen roster fixture used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/csm_run/boot_admission_trace.jsonl` | Golden boot/admission trace fixture used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/csm_run/resource_pressure_fixture.json` | Golden resource-pressure fixture used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/csm_run/scheduling_decision.json` | Golden scheduler decision fixture used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/csm_run/citizen_action_fixture.json` | Golden citizen action fixture used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/csm_run/freedom_gate_decision.json` | Golden Freedom Gate decision fixture used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/csm_run/first_run_trace.jsonl` | Golden first-run trace fixture used by Runtime v2 tests |
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
| `runtime_v2/csm_run/resource_pressure_fixture.json` | WP-06 | WP-14 | Bounded pressure input for the governed episode scheduler |
| `runtime_v2/csm_run/scheduling_decision.json` | WP-06 | WP-07 | Reviewable scheduler choice before Freedom Gate mediation |
| `runtime_v2/csm_run/citizen_action_fixture.json` | WP-07 | WP-08 | Scheduled non-trivial citizen action routed through the Freedom Gate |
| `runtime_v2/csm_run/freedom_gate_decision.json` | WP-07 | WP-08 | Bounded Freedom Gate mediation decision for the scheduled action |
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

D2 is contract-proving after WP-04.

D3 is proving after WP-05.

D4 has scheduling and Freedom Gate mediation evidence after WP-07. It is not
the WP-08 invalid-action proof.

Proved now:

- a code-backed CSM run packet contract exists
- the contract round-trips to a golden fixture
- the fixture has a bounded claim boundary and stable review target
- the run spine and pre-live-run gates are explicit
- the invariant map is code-backed and golden-fixture checked
- the violation artifact schema is code-backed and golden-fixture checked
- the positive packet fixture and negative violation fixture are paired
- `proto-csm-01` boot/admission evidence is code-backed and golden-fixture checked
- the boot manifest admits `proto-citizen-alpha` and `proto-citizen-beta` with traceable identity handles
- the citizen roster and boot/admission trace preserve the provisional boundary
- resource pressure exceeds the available bounded budget
- the scheduler chooses the only admitted executable worker under pressure
- the first-run trace records resource loading, candidate ranking, scheduling, and deferral in contiguous order
- the scheduled action fixture binds the non-trivial action to `proto-citizen-alpha`
- the Freedom Gate decision allows the action only after bounded mediation
- the first-run trace records the Freedom Gate mediation event after scheduling

Not proved yet:

- WP-08 invalid-action flow has not executed through the live runtime path
- Observatory output has not been generated from live first-run artifacts

## Validation

Focused validation:

```sh
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_run_packet_contract -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_invariant_and_violation_contract -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_boot_admission -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_governed_episode -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_freedom_gate_mediation -- --nocapture
```

This validates the contract prototypes, golden fixtures, path hygiene, positive
and negative fixture pairing, and negative cases for unsafe paths,
non-contiguous stages, missing invariant/violation coverage, boot/admission
trace ordering, resource-pressure scheduling ambiguity, first-run trace
ordering, Freedom Gate action/decision mismatches, and live-run overclaiming.

## Non-Claims

This contract does not prove:

- a live CSM run
- WP-08 invalid-action execution through the live runtime path
- first true Gödel-agent birth
- full v0.91 moral or emotional civilization
- v0.92 identity, migration, capability rebinding, or birthday semantics

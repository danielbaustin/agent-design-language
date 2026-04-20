# Invariant Expansion And Coverage

## Purpose

Map Runtime v2 surfaces to invariant classes and expand tests beyond happy-path
foundation behavior.

## Candidate Invariant Classes

- manifold integrity
- citizen continuity
- temporal ordering
- trace observability
- snapshot replay sufficiency
- recovery eligibility
- quarantine preservation
- operator authority
- security-boundary enforcement

## Required Output

- invariant coverage map
- negative-test inventory
- explicit gap list
- hardening priority order

## WP-04 Contract Artifact

WP-04 lands the D2 invariant coverage map as a code-backed contract artifact:

- schema: `runtime_v2.csm_run_invariant_map.v1`
- golden fixture: `adl/tests/fixtures/runtime_v2/invariants/csm_run_invariant_map.json`
- review artifact path: `runtime_v2/invariants/csm_run_invariant_map.json`
- positive fixture: `runtime_v2/csm_run/run_packet_contract.json`
- negative fixture: `runtime_v2/invariants/violation-0001.json`

The map covers the first CSM run spine before boot/admission work widens:

| Invariant | Class | Owner | Required Before | Fixture Status |
| --- | --- | --- | --- | --- |
| `single_active_manifold_instance` | manifold integrity | `kernel_runtime` | WP-05 | contracted |
| `no_duplicate_active_citizen_instance` | citizen continuity | `invariant_checker` | WP-05 | negative fixture backed |
| `trace_sequence_must_advance_monotonically` | temporal ordering | `trace_writer` | WP-06 | contracted for WP-06 |
| `invalid_action_must_be_refused_before_commit` | security boundary enforcement | `operator_control_interface` | WP-08 | negative fixture backed |
| `snapshot_restore_must_validate_before_active_state` | recovery eligibility | `snapshot_service` | WP-09 | contracted for WP-09 |

## Gap Policy

Missing or ambiguous coverage blocks WP-05 boot and any later live-run claim.
Later WPs may add concrete runtime evidence, but they must not replace this
map with a competing invariant set or silently weaken fail-closed behavior.

## Validation

Focused validation:

```sh
cargo test --manifest-path adl/Cargo.toml runtime_v2_invariant_and_violation_contract -- --nocapture
```

This validates schema identity, golden fixture stability, path hygiene,
positive/negative fixture pairing, required invariant coverage, and overclaim
rejection.

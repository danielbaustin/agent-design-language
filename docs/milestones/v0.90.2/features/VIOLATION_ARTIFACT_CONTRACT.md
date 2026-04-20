# Violation Artifact Contract

## Purpose

Make invariant failures stable, reviewable, and useful for demos and release
evidence.

## Minimum Fields

- artifact version
- invariant id
- attempted action
- actor or test harness
- kernel stage
- trace anchor
- decision
- resulting state
- recovery eligibility
- quarantine requirement

## Required Proof

At least one violation artifact should be generated from a real negative test
and checked for stable shape.

## WP-04 Schema Contract

WP-04 lands the violation artifact schema contract as a code-backed artifact:

- schema: `runtime_v2.violation_artifact_schema_contract.v1`
- described artifact schema: `runtime_v2.invariant_violation.v1`
- golden fixture: `adl/tests/fixtures/runtime_v2/violations/violation_artifact_schema.json`
- review artifact path: `runtime_v2/violations/violation_artifact_schema.json`
- positive fixture: `runtime_v2/csm_run/run_packet_contract.json`
- negative fixture: `runtime_v2/invariants/violation-0001.json`

The contract fixes the fields a reviewer can rely on when a runtime invariant
blocks a transition:

| Field | Purpose |
| --- | --- |
| `schema_version` | stable version for violation artifacts |
| `violation_id` | stable identifier for the rejected transition |
| `manifold_id` | manifold lineage affected by the attempted transition |
| `invariant_id` | invariant that caused refusal |
| `policy_enforcement_mode` | policy mode used for the decision |
| `attempted_transition` | actor, action, state, and source artifact for the attempt |
| `evaluated_refs` | artifacts checked before refusing the transition |
| `affected_citizens` | citizen ids affected by the attempted transition |
| `refusal_reason` | human-reviewable reason for the refusal |
| `source_error` | validator error or policy failure that caused refusal |
| `result` | resulting state, before-commit block proof, recovery action, and trace ref |

The required negative decision values are:

- `transition_refused_state_unchanged`
- `blocked_before_commit`
- `retain_existing_active_index_and_record_violation`

## Boundary

This contract fixes artifact shape for WP-08 and later invalid-action work. It
does not execute WP-08, does not prove a live CSM run, and does not claim first
true Godel-agent birth.

## Validation

Focused validation:

```sh
cargo test --manifest-path adl/Cargo.toml runtime_v2_invariant_and_violation_contract -- --nocapture
```

This validates the schema contract, required fields, decision values,
positive/negative fixture refs, write path hygiene, and overclaim rejection.

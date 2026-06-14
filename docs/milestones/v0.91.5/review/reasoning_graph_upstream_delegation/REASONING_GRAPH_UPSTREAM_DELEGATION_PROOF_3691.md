# Reasoning Graph Plus Upstream Delegation Proof Packet

Issue: #3691
Milestone: v0.91.5
Status: implementation proof packet

## Purpose

This packet proves the current ADL runtime can produce a reviewable reasoning
graph surface that carries a bounded upstream delegation trace record without
transferring hidden authority or exposing private reasoning.

## Proven Surface

- A reasoning graph artifact can carry a public contract reference.
- A reasoning graph artifact can carry one upstream delegation trace record.
- The delegation record keeps parent responsibility and parent review required.
- The delegation record does not claim delegated output or parent integration.
- The delegation record links back to the reasoning graph artifact by
  repo-relative artifact reference.
- The validator rejects hidden parent-authority inheritance.
- The validator rejects private-reasoning leakage in public delegation fields.

## Deterministic Proof

The focused proof is the Rust test:

```text
reasoning_graph_upstream_delegation_proof_flow_exercises_valid_and_negative_cases
```

The test constructs a deterministic run-summary-derived reasoning graph with
delegation counters, validates the generated graph, and then exercises two
negative cases:

- `parent_authority_inherited = true`
- `public_summary` containing private reasoning language

Both negative cases must fail closed through
`validate_upstream_delegation_trace_record`.

## Validation

Focused validation run before PR publication:

```text
cargo test --manifest-path adl/Cargo.toml reasoning_graph_upstream_delegation_proof_flow_exercises_valid_and_negative_cases -- --nocapture
```

Result: passed. The filtered run executed the focused proof test successfully in
the ADL CLI binary contexts and did not run live provider, browser, Unity,
Ollama, or remote-runtime work.

Additional local checks:

```text
cargo fmt --manifest-path adl/Cargo.toml --all -- --check
git diff --check
```

Result: both passed.

## Valid Delegation Semantics

The valid record is intentionally summary-only. It records delegation activity
derived from run-summary counters and uses:

- `target_class: local_agent`
- `policy_decision: needs_approval`
- `acc_decision: denied`
- `grant_status: denied`
- `failure_code: delegation_details_not_materialized`
- `parent_responsibility_retained: true`
- `parent_review_required: true`
- `parent_authority_inherited: false`

That shape is deliberately conservative: it records that delegation activity
exists, but it does not claim a delegated result, merged output, or completed
governed handoff.

## Non-Claims

- This does not prove live hosted-provider delegation.
- This does not prove browser, Unity, Ollama, or remote-runtime execution.
- This does not complete the v0.93 governance design.
- This does not claim delegated output is safe to integrate without parent
  review.
- This does not expose private reasoning or chain-of-thought.

## Review Notes

This packet is meant to be boring evidence. It proves the current deterministic
runtime contract boundary before any future live-provider, multi-agent, or
governed-execution expansion.

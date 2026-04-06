# Continuity Validation Schema

## Purpose

Define the canonical schema for continuity validation in ADL.

This document specifies the structured fields that a continuity validator must produce when evaluating whether an agent has preserved identity, temporal integrity, causal continuity, and subjective temporal coherence across execution, interruption, and resumption.

This schema is the executable companion to:
- `CONTINUITY_VALIDATION.md`
- `CHRONOSENSE_AND_IDENTITY.md`
- `TEMPORAL_SCHEMA_V01.md`
- `AGENT_LIFECYCLE.md`
- `SHEPHERD_RUNTIME_MODEL.md`

---

## Core Principle

> Continuity validation must produce a bounded, reviewable, machine-readable record of whether the same agent has continued through time without corruption, silent reset, or false continuity.

The schema must capture both:
- **objective temporal integrity**
- **subjective temporal coherence**

while preserving the distinction between them.

---

## Validation Subject

A continuity validation record evaluates one continuity subject.

Canonical subject types:
- run
- resumed run
- checkpoint restore
- agent session
- fork/join branch set
- memory-linked continuity chain

---

## Canonical Schema

```yaml
continuity_validation:
  validation_id: <uuid>
  subject_type: <run|resumed_run|checkpoint_restore|agent_session|fork_join_set|continuity_chain>
  subject_id: <string>
  agent_id: <string>
  run_id: <string|optional>
  validation_time_utc: <timestamp>
  validator_version: <string>
  result: <pass|fail|degraded|indeterminate>
  overall_assessment: <string>

  objective_temporal_integrity:
    status: <pass|fail|degraded|indeterminate>
    checks:
      agent_age_non_decreasing: <pass|fail|indeterminate>
      monotonic_order_preserved: <pass|fail|indeterminate>
      temporal_anchor_complete: <pass|fail|indeterminate>
      prior_event_delta_consistent: <pass|fail|indeterminate>
      reference_frame_consistent: <pass|fail|indeterminate>
      temporal_gap_explicit: <pass|fail|indeterminate>
    notes: <string|optional>

  subjective_temporal_coherence:
    status: <pass|fail|degraded|indeterminate>
    checks:
      narrative_position_coherent: <pass|fail|indeterminate>
      integration_window_continuous: <pass|fail|indeterminate>
      temporal_gap_representation_valid: <pass|fail|indeterminate>
      experienced_duration_plausible: <pass|fail|indeterminate>
      temporal_density_coherent: <pass|fail|indeterminate>
      subjective_objective_alignment: <pass|fail|indeterminate>
      simulated_vs_experienced_boundary_preserved: <pass|fail|indeterminate>
    notes: <string|optional>

  identity_continuity:
    status: <pass|fail|indeterminate>
    checks:
      stable_agent_id: <pass|fail|indeterminate>
      stable_agent_birth_mapping: <pass|fail|indeterminate>
      no_implicit_identity_substitution: <pass|fail|indeterminate>
      resumed_state_matches_prior_identity: <pass|fail|indeterminate>
    notes: <string|optional>

  causal_continuity:
    status: <pass|fail|degraded|indeterminate>
    checks:
      prior_trace_accessible: <pass|fail|indeterminate>
      reasoning_history_accessible: <pass|fail|indeterminate>
      no_causal_history_loss: <pass|fail|indeterminate>
      branch_lineage_preserved: <pass|fail|indeterminate>
    notes: <string|optional>

  state_coherence:
    status: <pass|fail|degraded|indeterminate>
    checks:
      checkpoint_consistent: <pass|fail|indeterminate>
      memory_trace_alignment: <pass|fail|indeterminate>
      artifact_trace_alignment: <pass|fail|indeterminate>
      resumed_state_valid: <pass|fail|indeterminate>
    notes: <string|optional>

  failure_modes:
    - type: <string>
      severity: <P1|P2|P3|P4>
      description: <string>
      evidence: <string|array>
      affected_layer: <objective|subjective|identity|causal|state>

  evidence:
    trace_refs: <array|optional>
    checkpoint_refs: <array|optional>
    memory_refs: <array|optional>
    artifact_refs: <array|optional>
    review_refs: <array|optional>

  recommended_action:
    action: <allow_resume|allow_with_warning|require_operator_review|forbid_resume|terminate>
    rationale: <string>

  continuity_boundary:
    boundary_type: <none|interruption|resume|fork|join|checkpoint_restore|termination>
    explicit_gap_recorded: <true|false|unknown>
    notes: <string|optional>
```

---

## Field Semantics

### Top-Level Result

- `pass`
  - continuity is preserved and resumption/continuation is valid

- `fail`
  - continuity is broken or corrupted

- `degraded`
  - continuity is mostly preserved but some coherence or recoverability loss exists

- `indeterminate`
  - insufficient evidence to validate continuity honestly

### Objective Temporal Integrity

This section evaluates ground-truth continuity constraints:
- lifetime clock continuity
- monotonic order continuity
- completeness of temporal anchors
- explicit representation of temporal gaps
- consistency of reference-frame mappings

This is the hard floor of continuity.

### Subjective Temporal Coherence

This section evaluates whether the agent’s subjective temporal state remains coherent relative to objective time.

The fields validated here MUST correspond directly to the subjective temporal primitives defined in:
- `TEMPORAL_SCHEMA_V01.md` → "Subjective Time: Minimum Contract (v0.1)"

Validators MUST NOT introduce alternative subjective temporal models or fields.

This section must never overwrite or supersede objective temporal integrity.

### Identity Continuity

This section determines whether the same agent has continued, rather than a new identity being silently substituted.

### Causal Continuity

This section evaluates whether prior trace, reasoning history, and branch lineage remain accessible and coherent.

### State Coherence

This section evaluates whether resumed state is internally valid and consistent with prior checkpoints, memory, artifacts, and trace.

---

## Required Invariants

A continuity validation result MUST be marked `fail` if any of the following are true:

- `agent_age_non_decreasing = fail`
- `monotonic_order_preserved = fail`
- `stable_agent_id = fail`
- `stable_agent_birth_mapping = fail`
- `no_implicit_identity_substitution = fail`
- `temporal_gap_explicit = fail` when interruption or resumption is present
- `temporal_gap_representation_valid = fail`
- `simulated_vs_experienced_boundary_preserved = fail`

These are non-negotiable continuity breaks.

---

## Simulation Boundary Rule

The continuity schema must preserve a strict distinction between:
- **experienced events** (trace-backed, temporally anchored)
- **simulated or reconstructed events** (derived, imagined, replayed, or projected)

Therefore:
- simulated events MAY contribute to subjective coherence checks
- simulated events MUST NOT modify objective continuity checks
- simulated events MUST NOT rewrite temporal anchors, monotonic order, or agent lifetime

Violating this rule constitutes continuity corruption.

---

## Fork/Join Considerations

When the subject involves fork/join reasoning, validators must additionally check:
- whether all branches preserve lineage from a shared prior identity
- whether branch-local subjective time remains marked as branch-local
- whether join synthesis preserves causal honesty
- whether no branch is silently promoted as if no divergence occurred

Fork/join must remain within a single continuity-bearing identity unless explicitly modeled otherwise.

---

## Recommended Action Semantics

- `allow_resume`
  - continuity preserved; safe to continue

- `allow_with_warning`
  - continuity preserved with bounded degradation

- `require_operator_review`
  - unclear or borderline continuity state; human judgment required

- `forbid_resume`
  - continuity not sufficiently preserved for honest continuation

- `terminate`
  - continuity irrecoverably broken; treat as ended

---

## Design Notes

- This schema is intended for machine validation and human review.
- It should remain deterministic in structure even if some evidence is uncertain.
- Subjective time is first-class, but objective temporal integrity remains authoritative.
- The schema is meant to support runtime enforcement, review surfaces, and future continuity tooling.
- Subjective temporal validation is strictly derived from the canonical temporal schema; this document does not define independent subjective-time structures

---

## Future Extensions

Later versions may add:
- quantitative scoring
- richer branch-local continuity structures
- longitudinal continuity across many runs
- distributed continuity validation across hosts
- policy-specific validation overlays

But the core distinction must remain:
- objective integrity
- subjective coherence
- identity continuity
- causal continuity
- state coherence

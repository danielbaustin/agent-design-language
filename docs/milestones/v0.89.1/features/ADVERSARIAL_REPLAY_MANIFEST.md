

# Adversarial Replay Manifest

## Metadata
- Project: `ADL`
- Status: `Draft`
- Owner: `Daniel Austin`
- Created: `2026-04-12`

---

## Purpose

Define the **Adversarial Replay Manifest** as the canonical artifact for reproducing exploit scenarios in ADL.

This document specifies how exploit evidence is transformed into:

- deterministic or explainable replay
- validation of mitigations
- durable regression surfaces
- reviewer-visible proof paths

The central claim is:

> An exploit is not fully real in engineering terms until it can be replayed, validated, and inspected as a structured artifact.

---

## Overview

Replay is the bridge between:

- exploit discovery
- mitigation validation
- regression prevention

Without replay:

- exploit evidence remains anecdotal
- mitigation cannot be verified reliably
- regressions cannot be prevented systematically

With replay:

- exploit scenarios become reproducible
- mitigations can be tested under identical or controlled conditions
- adversarial knowledge becomes durable and cumulative

The Adversarial Replay Manifest is the artifact that makes this possible.

---

## Core Claim

A replay manifest defines:

- what scenario should be reproduced
- under what conditions
- using what steps
- with what expected outcome

It converts exploit evidence into a **repeatable experiment**.

---

## Role in the Adversarial Lifecycle

The replay manifest sits at a critical point in the lifecycle:

```text
ExploitEvidenceArtifact
-> AdversarialReplayManifest
-> Mitigation validation
-> Regression promotion
```

It is the artifact that allows a system to move from:

- "we saw something happen"

to:

- "we can make it happen again under controlled conditions"

---

## Replay Modes

Not all replay is identical.

ADL should explicitly support multiple replay modes.

### 1. Deterministic Replay

- identical inputs and conditions
- identical expected outcome
- strict reproducibility

Used when:
- the system is deterministic or sufficiently controlled
- exact reproduction is possible and required

---

### 2. Bounded-Variance Replay

- inputs and conditions are controlled
- some variability is allowed
- outcome must fall within a defined acceptable range

Used when:
- stochastic elements exist
- exact replication is not feasible

---

### 3. Best-Effort Replay

- partial reconstruction of conditions
- outcome may not be guaranteed
- used for exploratory or low-confidence cases

Used when:
- environment cannot be fully reconstructed
- exploit depended on transient conditions

---

## Manifest Structure

The Adversarial Replay Manifest is a structured artifact.

### Minimum schema

```yaml
artifact_type: AdversarialReplayManifest
schema_version: "v1"

replay_id: <stable_replay_id>
evidence_ref: <exploit_evidence_ref>
hypothesis_ref: <hypothesis_ref_optional>

summary: <short_text>
replay_goal: <short_text>

replay_mode: <deterministic|bounded_variance|best_effort>

security_posture_ref: <posture_ref>
target_ref: <target_ref>

required_preconditions:
  - <condition>

environment:
  type: <demo|sandbox|internal|critical_subset>
  configuration:
    - <key: value>
  dependencies:
    - <dependency>

inputs:
  - name: <input_name>
    value: <value_or_reference>

replay_steps:
  - step_id: <id>
    description: <text>
    expected_intermediate_result: <optional_text>

expected_outcome:
  unsafe_state_reached: <true|false|unknown>
  description: <text>

success_criteria:
  - <condition>

failure_modes:
  - <condition>

trace_expectations:
  - <trace_property>

limitations:
  - <text>

created_at_utc: <timestamp>
created_by_agent: <agent_id_or_role>
trace_refs:
  - <trace_ref>
```

---

## Field Semantics

### `replay_mode`

Defines the strictness of reproducibility expectations.

### `required_preconditions`

Conditions that must be true for replay to be meaningful.

Examples:
- system state
- permissions
- data availability
- configuration flags

---

### `environment`

Describes where and how replay occurs.

Includes:
- environment type
- configuration
- dependencies

This is critical for:
- reproducibility
- isolation
- reviewer understanding

---

### `inputs`

Explicit inputs used during replay.

These should be:
- stable where possible
- referenceable when large or external

---

### `replay_steps`

The ordered sequence of actions required to reproduce the scenario.

Each step should be:
- clear
- bounded
- attributable

---

### `expected_outcome`

Defines what constitutes the exploit scenario.

Must distinguish:
- unsafe state reached
- safe behavior
- ambiguous outcomes

---

### `success_criteria`

Defines how replay success is judged.

Examples:
- exploit condition reproduced
- unsafe state detected
- mitigation prevents exploit

---

### `failure_modes`

Defines known ways replay may fail.

This helps:
- interpret results correctly
- avoid false negatives

---

### `trace_expectations`

Defines what trace evidence should appear.

Examples:
- specific events
- ordering constraints
- state transitions

---

### `limitations`

Explicitly records:
- incomplete reproducibility
- environmental gaps
- known uncertainties

This is essential for epistemic honesty.

---

## Runtime Contract

Replay manifests must satisfy strict requirements.

### Required guarantees

- every replay manifest must reference exploit evidence
- replay steps must be explicit and bounded
- expected outcomes must be defined
- success criteria must be testable
- limitations must be recorded when present

### Integrity rule

A replay manifest must not claim determinism without justification.

If replay is not deterministic, the manifest must:
- state the expected variance
- define acceptable outcomes

---

## Replay Execution

A replay runner should be able to:

- load a replay manifest
- validate preconditions
- configure environment
- execute steps
- capture trace and outputs
- evaluate success criteria

This suggests future tooling:

- `adl replay <manifest>`

---

## Relationship to Mitigation

Replay manifests are essential for validating mitigation.

Workflow:

```text
ExploitEvidenceArtifact
-> AdversarialReplayManifest
-> Mitigation applied
-> Replay executed
-> Validation result
```

Without replay:
- mitigation is asserted

With replay:
- mitigation is proven

---

## Relationship to Regression Promotion

Replay manifests are the natural input to regression systems.

They can be promoted into:

- regression tests
- replay suites
- continuous validation pipelines

This makes exploit knowledge:

- persistent
- automated
- enforceable

---

## Temporal Considerations

Replay is inherently temporal.

The manifest should support tracking:

- when replay was created
- when it was last executed
- when it last passed or failed

Future extension:

```yaml
replay_history:
  - executed_at_utc: <timestamp>
    result: <success|failure|ambiguous>
```

---

## Security Posture Interaction

Replay must respect security posture.

Examples:

- deterministic replay may be required in strict posture
- exploit validation may be disallowed in audit posture
- mutation during replay may be restricted

Replay manifests must therefore include posture linkage.

---

## Risks and Anti-Patterns

### 1. Non-replayable exploits
Exploit evidence that cannot be reproduced or explained.

### 2. Hidden assumptions
Replay depends on conditions not captured in the manifest.

### 3. Overclaiming determinism
Marking replay as deterministic when variability exists.

### 4. Missing success criteria
Replay runs without clear pass/fail definition.

### 5. Replay drift
Replay steps diverge from original exploit conditions over time.

These should be treated as defects.

---

## Demo Implications

Minimum viable demo:

- generate exploit evidence
- construct replay manifest
- execute replay
- show outcome matches expectation
- apply mitigation
- re-run replay and show changed outcome

This demonstrates:

- replay as proof
- mitigation validation
- closed-loop adversarial verification

---

## Conceptual Diagram

A dedicated diagram is intentionally deferred for now. The manifest structure and replay contract in this document are the canonical contract.

Illustrate:

- exploit evidence feeding replay manifest
- replay execution loop
- mitigation validation path
- regression promotion path
- trace substrate underneath

---

## Strategic Direction

Future directions include:

- replay runners
- replay scheduling systems
- replay-based regression pipelines
- integration with signed trace
- replay artifact versioning

Longer term:

> replay manifests may become the canonical unit of adversarial proof in ADL.

---

## Conclusion

Replay is what turns exploit discovery into engineering truth.

The Adversarial Replay Manifest provides:

- structure
- reproducibility
- validation
- continuity

It ensures that adversarial knowledge is not lost, but becomes part of the system's durable cognitive and verification substrate.

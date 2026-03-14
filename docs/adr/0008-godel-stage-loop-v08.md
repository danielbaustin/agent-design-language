# ADR 0008: Bounded Gödel Stage Loop for v0.8

## Status

Accepted (v0.8).

## Context

v0.8 introduces a real bounded Gödel runtime surface under `swarm/src/godel/`
with deterministic tests, canonical artifacts, and runnable demos. Review
feedback highlighted that the implementation is now substantial enough to need
an explicit architecture decision record, not just milestone prose.

The shipped runtime surfaces are centered on:
- `swarm/src/godel/stage_loop.rs`
- `swarm/src/godel/hypothesis.rs`
- `swarm/src/godel/mutation.rs`
- `swarm/src/godel/evaluation.rs`
- `swarm/src/godel/experiment_record.rs`
- `swarm/src/godel/obsmem_index.rs`
- `swarm/src/godel/workflow_template.rs`
- `swarm/src/demo.rs` (`demo-c-godel-runtime`, `demo-d-godel-obsmem-loop`)

The main architectural question for v0.8 is not whether ADL has a full
autonomous self-improvement agent. It does not. The question is how to expose a
credible, reviewable first Gödel loop without weakening determinism, replay, or
runtime comprehensibility.

## Decision

For v0.8, ADL ships a bounded Gödel stage loop with a fixed seven-stage
runtime sequence:

1. `failure`
2. `hypothesis`
3. `mutation`
4. `experiment`
5. `evaluation`
6. `record`
7. `indexing`

The first six stages are contractual and come from the embedded canonical
workflow template. `indexing` is appended by the runtime as a deterministic
post-record persistence step so the loop remains queryable through ObsMem-style
surfaces without changing the scientific-loop contract itself.

The v0.8 loop is bounded-only:
- no open-ended planning
- no autonomous recursive self-modification
- no unconstrained mutation execution
- no hidden state outside explicit artifacts

The runtime integrates the loop by producing deterministic artifacts and stable
IDs at each stage boundary rather than by running an unbounded agent policy.

## Why bounded-only in v0.8

Bounded-only execution was chosen because it preserves the core ADL invariants
already established in earlier milestones:
- deterministic execution for identical inputs
- replay-compatible artifacts
- explicit failure surfaces
- reviewable, schema-backed state transitions

An unconstrained Gödel or AEE implementation at this milestone would have
blurred the boundary between:
- experimental design surfaces,
- runtime semantics,
- policy-learning ambitions,
- and reviewer-facing architecture truth.

The bounded loop gives v0.8 a real scientific-loop substrate while keeping the
execution model small enough to test and explain.

## Hypothesis, mutation, and evaluation structure

### Hypothesis

Hypothesis generation is deterministic and failure-driven.

Inputs:
- `run_id`
- `failure_code`
- `failure_summary`
- normalized `evidence_refs`

Outputs:
- stable `HypothesisCandidate` values
- deterministic candidate ordering by `hypothesis_id`

This means v0.8 ships a bounded hypothesis pipeline, not a general hypothesis
engine.

### Mutation

Mutation remains bounded and policy-shaped.

The runtime currently derives deterministic `MutationProposal` records from the
selected hypothesis and maps failures into a small set of target surfaces such
as:
- `tool-invocation-config`
- `delegation-policy-input`
- `verification-gate-input`
- `workflow-step-config`

The goal is to preserve an explicit candidate-change surface without introducing
free-form patch semantics or unconstrained runtime self-editing.

### Evaluation

Evaluation in v0.8 is intentionally narrow and deterministic.

The runtime uses explicit decision outcomes such as:
- `Adopt`
- `Reject`
- `Review`

These decisions are derived from bounded inputs such as failure code,
experiment result, and score delta. They are recorded into canonical
experiment-record artifacts instead of driving a broader autonomous promotion
system.

## Template and runtime relationship

The canonical Gödel workflow template is authoritative for the contractual loop
order (`failure` through `record`). The runtime derives the executable sequence
from that template and appends `indexing` as a runtime-managed suffix.

This preserves two useful properties:
- milestone docs and runtime code use the same shared stage vocabulary
- the runtime can add deterministic persistence/indexing work without mutating
  the scientific-loop contract

## Consequences

Positive:
- provides a real Gödel runtime surface for v0.8 instead of design-only prose
- keeps the loop deterministic, bounded, and testable
- aligns demos, artifacts, and milestone docs around one stage vocabulary
- makes experiment artifacts reviewable without claiming a full autonomous
  improvement agent

Trade-offs:
- the v0.8 loop is intentionally smaller than the broader Gödel/AEE vision
- hypothesis generation is bounded and hand-shaped rather than model-rich
- mutation remains descriptive/policy-shaped rather than a full mutation
  executor
- evaluation remains a bounded decision surface rather than a general
  experiment-admission framework

## Explicit deferrals

Deferred beyond v0.8:
- autonomous Gödel agent behavior
- open-ended recursive self-improvement
- generalized hypothesis-engine learning
- unconstrained mutation execution or patch application
- richer experiment portfolio management and promotion policies
- broader AEE strategy feedback loops

These may be built later on top of the bounded artifact and stage substrate, but
they are not part of the accepted v0.8 design.

## Related references

- `docs/milestones/v0.8/GODEL_LOOP_INTEGRATION_V0.8.md`
- `docs/milestones/v0.8/GODEL_EXPERIMENT_WORKFLOW_TEMPLATE_V1.md`
- `docs/milestones/v0.8/GODEL_SCIENTIFIC_METHOD.md`
- `docs/milestones/v0.8/CANONICAL_EVIDENCE_VIEW_V1.md`
- `docs/milestones/v0.8/EXPERIMENT_RECORD_V1.md`
- `docs/milestones/v0.8/DEMOS_V0.8.md`

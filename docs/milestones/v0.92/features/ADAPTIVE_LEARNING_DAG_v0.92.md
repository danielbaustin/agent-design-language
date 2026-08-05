# Adaptive Learning DAG

## Metadata

- Feature Name: Adaptive Learning DAG
- Milestone Target: `v0.92`
- Status: issue opened; Runtime v3 loop prerequisite requalified by WP-01
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: architecture, runtime, validation
- Proof Modes: review, schema, replay, negative-test

## Purpose

Deliver the full adaptive-learning work that follows the historical `v0.91.7`
reasoning-graph and loop-runtime tranche through v0.92 WP-13A.

This feature turns the post-`#5104` loop-runtime boundary into the next
planned runtime sequence:

```text
Prompt
  -> Loop
  -> Adaptive Loop
  -> Reasoning Graph
  -> Adaptive Learning DAG
```

The v0.92 implementation question is no longer whether loops are only prompt
conventions. WP-13A must prove how a validated, replayable loop accepts
evaluation feedback, update runtime state, and eventually mutate a reasoning
graph under policy.

## Context

Source comments on PR `#5104` indicate that the loop-runtime work establishes
bounded recurrent execution over reasoning graphs, including graph/state
binding, structural validation, termination constraints, deterministic replay,
resume-prefix continuity, replay-forgery rejection, and canonical ordering.

WP-01 verified the merged `#5104` evidence and requalified its reusable
semantics against current Runtime v3 source and focused tests. The retained
decision is recorded in
`.csdlc/evidence/5817/prerequisite-and-loop-runtime-requalification.md`.

## Requalified Runtime Status

Current Runtime v3 source and focused tests satisfy these reusable loop
contracts:

- Current Runtime v3 authority accepts bounded loop execution over reasoning
  graphs.
- Loop definitions bind to reasoning graphs and runtime state.
- The runtime validates referenced graph nodes and edges.
- Loop bounds are structurally validated before execution.
- Termination limits are enforced.
- Loop execution emits deterministic replay events.
- Resumed execution checks prior-state prefix continuity.
- Forged, substituted, or discontinuous replay histories are rejected.
- The supervised Runtime v3 reasoning component executes the loop through the
  kernel component context and cancellation hierarchy.

This requalification establishes the loop prerequisite only. It does not
upgrade adaptive learning or graph mutation to implementation truth.

## Scope

In scope for v0.92 WP-13A:

- evaluation and feedback bindings for loop iterations;
- stateful adaptation records that explain what changed and why;
- policy gates for graph modification;
- deterministic graph-mutation proposals and accepted/rejected dispositions;
- replay-safe adaptive-loop histories;
- negative tests for state substitution, replay forgery, invalid graph binding,
  discontinuous execution, and unbounded recurrence;
- an Adaptive Learning DAG proof packet that connects loop events, feedback,
  state deltas, policy decisions, graph deltas, and replay evidence.

Out of scope for v0.92 unless WP-01 explicitly promotes it with evidence:

- unconstrained self-modification;
- production autonomous learning;
- graph mutation without policy and review evidence;
- hidden model-memory mutation;
- claims that adaptive learning proves consciousness, personhood, or
  production citizenship;
- v0.94 signed/queryable trace completion.

## Required Work Sequence

| Step | Work | Required output |
| --- | --- | --- |
| 1 | Loop-runtime consumption | Completed by WP-01: verified `#5104` merge evidence, current Runtime v3 qualification, and explicit non-claims. |
| 2 | Evaluation bindings | Schema and fixtures connecting loop iterations to evaluation signals, feedback source, confidence, and proof refs. |
| 3 | Stateful adaptation | Runtime records for bounded state deltas with before/after state hashes, rationale, and rollback notes. |
| 4 | Policy-governed graph modification | Mutation proposal schema, policy decision, reviewer visibility, and accepted/rejected graph-delta fixtures. |
| 5 | Adaptive Learning DAG | Integrated DAG packet linking prompt, loop, adaptive loop, reasoning graph, feedback, state delta, graph delta, and replay events. |
| 6 | Negative and replay proof | Rejection fixtures for forged history, discontinuous prefix, invalid binding, unbounded recurrence, unauthorized mutation, and unsupported feedback. |

## Determinism and Constraints

- Adaptive learning must be replayable from durable inputs.
- Runtime state changes must have explicit deltas, references, and policy
  decisions.
- Graph modifications must be proposed before they are accepted.
- Canonical ordering must be deterministic across feedback, state-delta, and
  graph-delta records.
- Resume must fail closed when prior-state continuity cannot be proven.
- Missing evaluation evidence must not silently become learning evidence.

## Integration Points

- `docs/milestones/v0.91.7/features/REASONING_GRAPH_LOOP_SKILL_STANDARD_BRIDGE_v0.91.7.md`
- `docs/milestones/v0.92/WBS_v0.92.md`
- `docs/milestones/v0.92/SPRINT_v0.92.md`
- `docs/milestones/v0.92/DEMO_MATRIX_v0.92.md`
- `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`
- historical reasoning-graph and loop-runtime input consumed from `#5104` only
  after current Runtime v3 requalification
- Future v0.94 reasoning/provenance graph and signed/queryable trace work

## Validation

WP-01 validated the prerequisite by checking:

- the merged loop-runtime command and tests exist;
- current Runtime v3 loop proofs include negative cases for forged or
  discontinuous replay histories;
- WP-13A owns the opened adaptive-learning implementation issue;
- no v0.92 birthday claim depends on unproved graph mutation.

Later implementation WPs should add focused tests for evaluation bindings,
state deltas, graph mutation policy, replay determinism, and fail-closed
negative cases.

## Acceptance Criteria

- v0.92 planning names adaptive learning as WP-13A and preserves its explicit
  implementation boundary.
- The implementation separates bounded loop runtime from learning-driven graph mutation.
- Evaluation, adaptation, graph modification, and Adaptive Learning DAG proof
  are distinct deliverables.
- Negative tests cover the failure modes that would make adaptive execution
  untrustworthy.
- Birthday evidence can consume loop/runtime proof without claiming full
  adaptive learning before it is implemented.

## Risks

- Current Runtime v3 loop proof may be overstated as adaptive learning unless
  WP-13A preserves the graph-mutation boundary.
- Adaptive-learning language may sound like autonomous self-improvement unless
  policy gates and replay proof remain explicit.
- Graph mutation may widen v0.92 beyond first-birthday scope if WP-01 does not
  keep it governed and evidence-bound.

## Future Work

Future milestones may connect the v0.92 Adaptive Learning DAG implementation
to v0.94 signed/queryable trace, reasoning/provenance graph completion, and
longer-lived governance review.

## Notes

The architectural boundary to preserve is simple:

```text
Loops are control flow.
Adaptive loops are evaluated control flow.
Adaptive learning is policy-governed graph change with replay evidence.
```

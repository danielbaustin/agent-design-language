# Adaptive Learning DAG

## Metadata

- Feature Name: Adaptive Learning DAG
- Milestone Target: `v0.92`
- Status: forward-planning queue
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: architecture, runtime, validation
- Proof Modes: review, schema, replay, negative-test

## Purpose

Queue the full adaptive-learning work that follows the historical `v0.91.7`
reasoning-graph and loop-runtime tranche without creating GitHub issues yet.

This feature turns the post-`#5104` loop-runtime boundary into the next
planned runtime sequence:

```text
Prompt
  -> Loop
  -> Adaptive Loop
  -> Reasoning Graph
  -> Adaptive Learning DAG
```

The v0.92 planning question is no longer whether loops are only prompt
conventions. The queued question is how a validated, replayable loop can accept
evaluation feedback, update runtime state, and eventually mutate a reasoning
graph under policy.

## Context

Source comments on PR `#5104` indicate that the loop-runtime work establishes
bounded recurrent execution over reasoning graphs, including graph/state
binding, structural validation, termination constraints, deterministic replay,
resume-prefix continuity, replay-forgery rejection, and canonical ordering.

WP-01 must verify the merged `#5104` evidence before treating those claims as
repo truth. Until then, this document records a v0.92 queue, not completed
runtime evidence.

## Runtime Status To Verify

After `#5104` is merged and consumed by v0.92 WP-01, the loop-status section
for the upstream loop/runtime document should be updated only if the merged
code and tests still satisfy current Runtime v3 contracts for these claims:

- Current Runtime v3 authority accepts bounded loop execution over reasoning
  graphs.
- Loop definitions bind to reasoning graphs and runtime state.
- The runtime validates referenced graph nodes and edges.
- Continuous loops are structurally valid before execution.
- Termination limits are enforced.
- Loop execution emits deterministic replay events.
- Resumed execution checks prior-state prefix continuity.
- Forged, substituted, or discontinuous replay histories are rejected.
- The operator surface exposes the current loop-runtime proof through the
  Runtime v3 authority path.

If any of those claims are missing or partial, WP-01 must record the gap rather
than upgrading this feature to implementation truth.

## Scope

In scope for the v0.92 queue:

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
| 1 | Loop-runtime consumption | Verified `#5104` merge evidence, runtime-status update, and explicit non-claims. |
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

WP-01 should validate this queue by checking:

- the merged loop-runtime command and tests exist;
- current Runtime v3 loop proofs include negative cases for forged or
  discontinuous replay histories;
- the adaptive-learning work remains a v0.92 queue until implementation WPs
  exist;
- no v0.92 birthday claim depends on unproved graph mutation.

Later implementation WPs should add focused tests for evaluation bindings,
state deltas, graph mutation policy, replay determinism, and fail-closed
negative cases.

## Acceptance Criteria

- v0.92 planning names adaptive learning as a queued work package without
  creating issues early.
- The queue separates bounded loop runtime from learning-driven graph mutation.
- Evaluation, adaptation, graph modification, and Adaptive Learning DAG proof
  are distinct deliverables.
- Negative tests cover the failure modes that would make adaptive execution
  untrustworthy.
- Birthday evidence can consume loop/runtime proof without claiming full
  adaptive learning before it is implemented.

## Risks

- Loop-runtime evidence may be overstated before `#5104` is fully merged and
  reviewed.
- Adaptive-learning language may sound like autonomous self-improvement unless
  policy gates and replay proof remain explicit.
- Graph mutation may widen v0.92 beyond first-birthday scope if WP-01 does not
  keep it queued and evidence-bound.

## Future Work

Future milestones may promote the Adaptive Learning DAG from planning queue to
runtime implementation, then connect it to v0.94 signed/queryable trace,
reasoning/provenance graph completion, and longer-lived governance review.

## Notes

The architectural boundary to preserve is simple:

```text
Loops are control flow.
Adaptive loops are evaluated control flow.
Adaptive learning is policy-governed graph change with replay evidence.
```

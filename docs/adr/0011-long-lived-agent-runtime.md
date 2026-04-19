# ADR 0011: Long-Lived Agent Runtime Architecture

- Status: Accepted
- Date: 2026-04-18
- Related issue: #2137
- Related milestone: v0.90
- Review source: v0.90 third-party review P2 remediation

## Context

v0.90 introduces the first bounded long-lived-agent runtime surface in ADL.
Before this milestone, ADL could execute bounded workflows, emit artifacts, and
support reviewable runs, but it did not have a durable runtime architecture for
agents that persist across repeated cycles.

The third-party v0.90 review found no P0 or P1 issues and marked the milestone
ready for release after normal release-tail work. Its only substantive P2
finding was that the long-lived-agent architecture is important enough to need
an accepted ADR.

This ADR is grounded in:

- `docs/milestones/v0.90/features/FEATURE_LONG_LIVED_SUPERVISOR_HEARTBEAT.md`
- `docs/milestones/v0.90/features/FEATURE_LONG_LIVED_AGENT_CYCLE_CONTRACT.md`
- `docs/milestones/v0.90/features/FEATURE_LONG_LIVED_STATE_AND_CONTINUITY.md`
- `docs/milestones/v0.90/features/FEATURE_LONG_LIVED_OPERATOR_CONTROL_AND_SAFETY.md`
- `docs/milestones/v0.90/repo_visibility/CODE_DOC_DEMO_LINKAGE_REPORT_v0.90.md`
- `docs/milestones/v0.90/V090_PRE_THIRD_PARTY_READINESS_REPORT.md`
- `adl/src/long_lived_agent.rs`
- `adl/src/long_lived_agent/schema.rs`
- `adl/src/long_lived_agent/types.rs`
- `adl/src/long_lived_agent/tests.rs`

This ADR does not introduce new runtime behavior. It records the architecture
that v0.90 has already implemented and validated.

## Decision

ADL adopts a bounded, cycle-scoped architecture for long-lived agents.

Long-lived agents are not modeled as unbounded autonomous processes. They are
modeled as supervised runtime subjects that advance through explicit cycles.
Each cycle has operator-visible state, bounded execution, reviewable artifacts,
and clear safety boundaries.

At the v0.90 boundary, this decision requires:

1. Supervisor and heartbeat are explicit runtime surfaces

   The runtime owns an agent supervisor layer with tick, run, status, and stop
   behavior. The heartbeat defines when the supervisor may attempt another
   bounded cycle, but the heartbeat does not grant unbounded autonomy.

2. Leases serialize active cycles

   Each active cycle must acquire a lease before execution. An active lease
   blocks overlapping ticks. A stale lease must be visible to status and must
   require explicit recovery before another cycle proceeds.

3. Cycles emit reviewable artifact packets

   A cycle must leave enough durable evidence for review. The v0.90 contract
   includes cycle-scoped heartbeat, observation, decision, run reference,
   guardrail, memory, ledger, status, and inspection artifacts.

4. Continuity is explicit but pre-identity

   The runtime records a continuity handle so repeated cycles are not merely
   unrelated executions. That handle is explicitly a pre-v0.92 continuity
   surface. It must not be described as full identity, personhood, citizenship,
   or a true Gödel-agent birthday.

5. Operator authority is part of the architecture

   Operators can inspect, stop, and constrain long-lived execution. Stop files,
   operator events, guardrail summaries, failure reports, and stale-lease
   recovery are architectural requirements, not optional UI polish.

6. Long-lived demos remain bounded proof surfaces

   The stock-league demo demonstrates recurring supervised behavior with
   fixture-backed, paper-only outputs. It must not claim live trading,
   financial advice, broker execution, or production autonomy.

## Rationale

ADL needs long-lived behavior before it can support later identity, memory,
capability, moral, emotional, or polis-level work. But long-lived behavior is
dangerous if it is represented as a hidden daemon or a vague agent loop.

The v0.90 architecture keeps long-lived agency reviewable by making every
important boundary explicit:

- the supervisor decides when a bounded cycle may start
- the lease prevents overlapping cycle execution
- the heartbeat is schedule evidence, not permission for unbounded action
- the cycle artifact packet records what happened
- the continuity file explains what persists
- the operator stop and inspection surfaces preserve human authority
- the guardrail and safety reports keep demo claims bounded

This preserves core ADL invariants: determinism where claimed, bounded
execution, traceability, reviewability, and explicit trust boundaries.

## Consequences

### Positive

- Gives the v0.90 long-lived runtime one durable architecture decision record.
- Makes reviewer questions about heartbeat, lease, cycle artifacts, continuity,
  and operator authority easier to answer.
- Creates a stable bridge from ADR 0009 and ADR 0010 into later Runtime v2,
  identity, memory, and citizen-birth milestones.
- Keeps long-lived execution grounded in artifacts rather than prompt theater.
- Prevents future docs from overstating v0.90 as full identity or autonomous
  release authority.

### Negative

- Commits ADL to maintaining explicit cycle artifacts and status surfaces as
  the long-lived runtime evolves.
- Adds architectural weight to future changes in leases, heartbeat semantics,
  continuity records, and operator-control behavior.
- Requires later milestones to preserve the distinction between continuity
  handles and full identity unless an explicit future ADR changes that boundary.

## Alternatives Considered

### 1. Treat long-lived agents as ordinary repeated runs

Pros:

- Smaller implementation surface.
- Less architectural documentation.

Cons:

- Repeated runs would not have clear continuity semantics.
- Reviewers could not easily distinguish separate executions from a supervised
  long-lived subject.
- Operator stop, stale-lease recovery, and cycle inspection would be weaker or
  scattered across ad hoc behavior.

### 2. Implement an unbounded daemon-style agent loop

Pros:

- More familiar shape for some agent frameworks.
- Could appear simpler to demonstrate continuous behavior.

Cons:

- Conflicts with ADL boundedness and reviewability.
- Makes operator authority and failure recovery harder to prove.
- Risks hidden state, overlapping execution, and vague autonomy claims.

### 3. Wait until v0.92 identity work before recording the decision

Pros:

- Later identity work may refine continuity and memory semantics.

Cons:

- v0.90 has already shipped a substantive long-lived runtime boundary.
- Review found the decision important enough to document now.
- Deferring the ADR would leave the release story less legible.

## Validation Evidence

The decision is supported by:

- long-lived runtime implementation in `adl/src/long_lived_agent.rs`
- schema and type boundaries in `adl/src/long_lived_agent/schema.rs` and
  `adl/src/long_lived_agent/types.rs`
- focused long-lived-agent tests in `adl/src/long_lived_agent/tests.rs`
- v0.90 demo matrix rows D1 through D5
- repo-visibility linkage report for the v0.90 long-lived runtime slice
- v0.90 pre-third-party readiness report
- updated third-party review finding that classified ADR 0011 as the only P2
  remediation item and non-blocking for release

## Non-Claims

This ADR does not claim:

- full persistent identity
- the first true Gödel-agent birthday
- personhood or citizenship semantics
- live trading
- financial advice
- broker or exchange execution
- unbounded autonomy
- autonomous release approval
- full Runtime v2 completion
- v0.91 moral, emotional, or polis governance completion
- v0.92 birthday, identity, capability, or migration semantics

## Notes

This ADR promotes the long-lived-agent candidate decision recorded in
`docs/architecture/adr/CANDIDATE_ADRS.md`.

Future ADRs may refine identity, memory, capability rebinding, citizen-birth,
or polis-governance consequences. Those future decisions should cite this ADR
when they build on the v0.90 supervised-cycle substrate.

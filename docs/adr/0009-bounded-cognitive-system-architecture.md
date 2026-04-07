# ADR 0009: Bounded Cognitive System Architecture

- Status: Accepted
- Date: 2026-04-06
- Related issue: #1288
- Related milestone: v0.87

## Context

External v0.86 review requested that ADL make explicit whether the
"bounded cognitive system" is a first-class architectural decision.

Across the repository, ADL already operates under a consistent model:
- execution occurs in a deterministic runtime
- cognition is grounded in traceable, replayable workflows
- memory is derived from trace (ObsMem), not free-form accumulation
- identity, causality, and review surfaces are explicit and inspectable
- adaptation (AEE / GHB) is bounded, not unconstrained

However, this model has been distributed across design docs, feature docs,
and milestone narratives rather than captured as a single architectural
commitment.


This ADR is grounded in and consistent with the canonical decisions and design
artifacts already established in:
- docs/milestones/v0.86/DECISIONS_v0.86.md
- docs/milestones/v0.86/DESIGN_v0.86.md

It does not expand scope beyond what those documents and the v0.87 milestone
README already define; it consolidates them into a single architectural statement.

## Decision

ADL adopts the **Bounded Cognitive System Architecture** as a canonical
architectural principle.

A bounded cognitive system is defined as a system in which:

1. **Execution is deterministic and replayable**
   - Workflows form a causal DAG
   - Runs can be reproduced or divergence explained

2. **Trace is the authoritative history**
   - All cognition is grounded in recorded execution events
   - No hidden or implicit reasoning surfaces are allowed

3. **Memory is derived, not invented**
   - ObsMem records are deterministically derived from trace
   - All memory carries provenance back to execution

4. **Causality is explicit and inspectable**
   - Outputs are attributable to prior inputs, events, and decisions

5. **Identity is execution-grounded (bounded in v0.87)**
   - Agents are associated with runs and trace-linked execution history
   - Full persistent identity and chronosense are explicitly out of scope for v0.87
   - This ADR does not assert continuity guarantees beyond what trace and run structure provide

6. **Adaptation is bounded and governed**
   - Learning/refinement occurs via replayable, inspectable processes
   - No unconstrained self-modification is permitted

7. **All surfaces are reviewable**
   - A reviewer can reconstruct what happened, why, and based on what inputs

This architecture is the foundation for:
- trace v1
- shared ObsMem v1
- provider substrate
- review and audit surfaces
- future identity and governance layers (e.g., Freedom Gate v2 evolution beyond the v0.86 baseline)

## Rationale

Without a bounded architecture, agent systems exhibit:
- drift across runs
- inconsistent memory
- irreproducible behavior
- weak auditability
- loss of trust in enterprise settings

The bounded cognitive system ensures:
- reproducibility
- inspectability
- causal accountability
- safe, governed evolution

This aligns with ADL’s core goal: not just more capable agents, but
**reliable, governable cognitive systems**.

The ADR consolidates existing implicit design into an explicit contract,
improving clarity for contributors, reviewers, and external stakeholders.

## Consequences

### Positive

- Establishes a clear architectural invariant across the codebase
- Aligns trace, memory, identity, and adaptation under one model
- Strengthens external credibility (reviewers can point to a formal decision)
- Reduces ambiguity in future feature design

### Negative

- Constrains future designs that might prefer more flexible or probabilistic
  memory/adaptation models
- Requires discipline to maintain determinism and provenance guarantees
- May increase implementation overhead for new features

## Alternatives Considered

### 1. Author the ADR now (chosen)

Pros:
- Captures the architecture already in practice
- Resolves external review feedback
- Provides a stable reference point for v0.87 and beyond

Cons:
- Commits to terminology and framing that may evolve

### 2. Defer the ADR explicitly

Pros:
- Allows further iteration before formalization

Cons:
- Leaves a core architectural principle implicit
- Weakens alignment across docs and features
- Fails to respond cleanly to review feedback

## Notes

- This ADR does not introduce new behavior; it formalizes existing design.
- Future ADRs may refine specific components (trace, ObsMem, identity,
  governance) without invalidating the bounded system principle.
- Identity persistence and chronosense are deferred to later milestones and are not asserted as delivered capabilities in v0.87.
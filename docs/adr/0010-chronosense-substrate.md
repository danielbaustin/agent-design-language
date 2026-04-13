# ADR 0010: Chronosense as a First-Class Substrate

- Status: Accepted
- Date: 2026-04-13
- Related issue: #1771
- Related milestone: v0.88

## Context

`v0.88` introduces the temporal / chronosense package as a real bounded runtime
surface rather than a scattered set of planning notes.

Before `v0.88`, ADL already depended on time in practice:
- execution had ordering and duration
- replay and trace required temporal structure
- pause/resume and continuity reasoning implicitly depended on temporal state
- later identity and agency ideas were already pointing toward a temporal
  substrate

But that temporal story was not yet captured as a single architecture decision.
It remained split across milestone design, feature docs, review notes, and
runtime implementation details.

This ADR is grounded in and consistent with:
- `docs/adr/0009-bounded-cognitive-system-architecture.md`
- `docs/milestones/v0.88/DESIGN_v0.88.md`
- `docs/milestones/v0.88/features/SUBSTANCE_OF_TIME.md`
- `docs/milestones/v0.88/features/TEMPORAL_SCHEMA_V01.md`
- `docs/milestones/v0.88/features/CHRONOSENSE_AND_IDENTITY.md`

It does not claim later-band temporal governance, social coordination,
counterfactual timelines, or full persistent identity guarantees.

## Decision

ADL adopts **chronosense as a first-class substrate**.

In ADL, chronosense means that temporal structure is an explicit architectural
surface rather than incidental metadata.

At the `v0.88` boundary, this decision requires:

1. **Temporal self-location is explicit**
   - the system can represent a bounded identity profile and present-tense
     temporal context
   - the repo exposes reviewable proof hooks for those surfaces

2. **Temporal schema is canonical**
   - anchors, execution posture, realization, and cost are defined through one
     explicit contract
   - later temporal features cite that contract instead of inventing parallel
     field sets

3. **Continuity is inspectable**
   - interruption, refusal, recovery, and continuous completion are visible as
     runtime semantics rather than hidden assumptions

4. **Temporal reasoning surfaces are bounded and reviewable**
   - retrieval, commitments/deadlines, causality/explanation, PHI-style
     integration comparison, and instinct-sensitive runtime behavior each expose
     bounded proof surfaces

5. **Chronosense remains subordinate to broader ADL invariants**
   - determinism, traceability, bounded execution, and reviewability still
     govern the design
   - chronosense does not introduce unconstrained autonomy or hidden temporal
     reasoning layers

## Rationale

ADL cannot make strong claims about continuity, commitments, cost reviewability,
or bounded agency unless temporal structure is first-class.

Without chronosense as an architectural substrate:
- identity remains under-specified
- continuity cannot be reviewed cleanly
- commitments and deadlines become shallow metadata
- cost and execution posture are harder to interpret together
- instinct and bounded agency lose an important part of their runtime context

Making chronosense explicit improves:
- coherence across the `v0.88` package
- reviewer legibility
- future alignment with identity, memory, and agency work
- architectural honesty about what the runtime now depends on

## Consequences

### Positive

- Gives the temporal package one durable architectural home
- Clarifies that time is part of the system substrate, not only an annotation
- Makes reviewer questions about continuity and temporal semantics easier to
  answer
- Creates a stable bridge between `0009` and later identity / agency ADRs

### Negative

- Commits the project to maintaining explicit temporal contracts rather than
  leaving time as ad hoc implementation detail
- Raises the quality bar for test depth and review surfaces in temporal code
- Requires care not to overclaim later temporal/governance capabilities too
  early

## Alternatives Considered

### 1. Leave chronosense only in milestone and feature docs

Pros:
- less architecture overhead

Cons:
- leaves a major substrate decision implicit
- weakens reviewer clarity
- makes later temporal/identity work look more accidental than deliberate

### 2. Fold chronosense into ADR 0009 only

Pros:
- fewer ADR files

Cons:
- `0009` is broader and milestone-earlier
- chronosense becomes a subsection inside a more general bounded-cognition ADR
- the `v0.88` temporal package loses a focused durable decision record

### 3. Defer the ADR until a later milestone

Pros:
- allows more implementation to accumulate first

Cons:
- review feedback has already shown the decision is large enough to deserve its
  own durable record
- deferral would keep the architecture less legible during the current release
  tail

## Notes

- This ADR does not claim full persistent identity guarantees.
- This ADR does not claim cross-agent temporal alignment, temporal
  accountability, or broader social/governance time semantics.
- This ADR formalizes the bounded chronosense substrate introduced in `v0.88`;
  later ADRs may refine identity, memory, agency, or governance consequences
  without invalidating this decision.

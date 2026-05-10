# ADR 0019: Theory Of Mind Foundation

- Status: Accepted
- Date: 2026-05-09
- Related milestone: v0.91.1
- Builds on: ADR 0012, ADR 0013, ADR 0016

## Context

`v0.91.1` is the inhabited-runtime milestone. It adds runtime-visible citizen
standing and citizen state, then layers memory/identity and bounded Theory of
Mind (ToM) over those runtime contracts before the observatory-visible
inhabitant proof.

At this stage, ADL needs a first-class way to represent evidence-bound
hypotheses about other agents without implying hidden introspection authority,
mind-reading, private-state disclosure, or governance override. The repo
already contains a landed Runtime v2 ToM foundation packet, focused tests, and
fixture evidence, but the architecture intent was not yet captured in the
accepted ADR chain.

This ADR is grounded in:

- `docs/milestones/v0.91.1/features/THEORY_OF_MIND_FOUNDATION.md`
- `docs/milestones/v0.91.1/DECISIONS_v0.91.1.md`
- `docs/milestones/v0.91.1/SPRINT_v0.91.1.md`
- `docs/milestones/v0.91.1/WBS_v0.91.1.md`
- `docs/milestones/v0.91.1/FEATURE_PROOF_COVERAGE_v0.91.1.md`
- `adl/src/runtime_v2/theory_of_mind_foundation.rs`
- `adl/src/runtime_v2/tests/theory_of_mind_foundation.rs`
- `adl/tests/fixtures/runtime_v2/theory_of_mind/theory_of_mind_foundation.json`

## Decision

ADL adopts a bounded Theory of Mind foundation as a first-class Runtime v2
subsystem in `v0.91.1`.

This decision requires:

1. Theory of Mind is evidence-bound model state.

   A ToM model is a bounded hypothesis about another agent's state, intent,
   behavior, or uncertainty. It must be grounded in observable evidence or
   policy-authorized state rather than hidden introspection.

2. ToM updates are first-class runtime events.

   Model changes must be represented as explicit update events with evidence
   references, authority basis, uncertainty changes, and visibility scope.

3. Uncertainty, correction, and privacy restrictions are mandatory.

   Unknown states, corrections, and privacy-restricted observations must remain
   explicit in the ToM surface. A plausible narrative is not enough.

4. ToM may inform reasoning, review, and coordination, but it is not authority.

   ToM can contribute context for reviewable runtime decisions. It must not
   override standing, authority, Freedom Gate policy, access control, or other
   governed boundaries.

5. `v0.91.1` only adopts the foundation slice.

   The accepted scope here is the bounded schema, packet, fixtures, update
   contract, and proof posture. Broader social cognition, reputation,
   governance-facing projection, and stronger identity/birthday claims remain
   later work.

## Rationale

ADL is trying to build long-lived, reviewable agent systems rather than chat
transcripts with hidden intuition. If Theory of Mind matters, it should exist
as an explicit architectural surface with evidence, uncertainty, and policy
boundaries.

This decision gives the runtime a durable place for bounded social cognition
without overstating what has been built. It also keeps ToM connected to
citizen-state and memory/identity evidence rather than letting it collapse into
free-form prompt inference.

## Consequences

### Positive

- Makes ToM an explicit runtime surface rather than an implicit prompt habit.
- Keeps multi-agent reasoning tied to evidence, uncertainty, and reviewable
  update events.
- Preserves the boundary between cognitive context and execution authority.
- Gives later milestones a stable foundation for deeper social-cognition and
  governance work.

### Negative

- Future ToM work must preserve compatibility with the bounded `v0.91.1`
  packet, fixtures, and non-claims.
- Public descriptions must stay disciplined: `v0.91.1` proves a bounded ToM
  foundation, not complete social cognition or consciousness.

## Alternatives Considered

### 1. Defer all ToM work to a later milestone

This would avoid defining a new runtime surface now, but it would leave the
inhabited-runtime milestone without an explicit substrate for bounded
agent-model reasoning.

### 2. Treat ToM as prompt-only intuition

This is cheaper in the short term, but it is not reviewable enough for ADL's
architecture. It hides evidence, uncertainty, and correction paths inside model
text.

### 3. Fold ToM entirely into memory or citizen state

This would reduce the number of named subsystems, but it would blur the
difference between observed evidence and hypotheses inferred from that evidence.

## Validation Evidence

The decision is supported by:

- the `v0.91.1` feature doc and proof-coverage row for `WP-08`
- the Runtime v2 ToM foundation packet and focused tests
- the golden fixture at
  `adl/tests/fixtures/runtime_v2/theory_of_mind/theory_of_mind_foundation.json`
- the explicit milestone dependency chain from citizen state and
  memory/identity into ToM and then into runtime inhabitant integration

## Non-Claims

This ADR does not claim:

- mind-reading or hidden introspection authority
- unrestricted access to raw private state
- reputation scoring as part of the `v0.91.1` ToM foundation
- policy override based on inferred mental state
- final identity continuity, first birthday, or consciousness claims

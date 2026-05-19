# ADR 0022 Candidate: Speculative Decoding Deterministic Commit Boundary

- Status: Candidate
- Target milestone: v0.91.2
- Related issue: #3124
- Related ADRs: ADR 0012, ADR 0013, ADR 0015, ADR 0018, ADR 0021 after acceptance

## Context

v0.91.2 includes a speculative decoding evaluation lane. Speculative decoding
can be valuable runtime acceleration, but it touches ADL's most important
runtime distinction: a candidate proposal is not an authoritative commit.

ADL already depends on deterministic commit semantics, replayable evidence,
bounded execution, and governed side-effect authority. Speculative generation
must fit inside those boundaries rather than becoming a hidden execution path.

## Decision

ADL may evaluate speculative decoding only as a bounded proposal-acceleration
surface.

Speculative tokens, candidate steps, draft tool calls, or draft reasoning
fragments may improve latency or exploration. They may not silently commit
state, execute tools, bypass replay, bypass ACC/Freedom Gate, or hide rejected
paths from audit.

Authoritative commit remains deterministic, reviewable, and governed.

## Requirements

- Speculative output must be classified as proposal material until accepted by
  the normal commit path.
- Accepted, rejected, and fallback speculative paths must be representable in
  review evidence.
- No speculative branch may perform hidden external side effects.
- ACC and Freedom Gate remain authoritative for actions.
- Performance claims must remain bounded to measured evaluation surfaces.

## Consequences

### Positive

- Lets ADL explore acceleration without weakening its governance model.
- Gives implementers a clear boundary between draft generation and committed
  runtime state.
- Preserves replay and audit semantics even if speculative paths become useful.

### Negative

- Speculative decoding integration is more constrained than a raw performance
  optimization.
- Benchmarks must measure both speed and governance-preservation behavior.
- Runtime traces or evidence packets need enough detail to explain rejected and
  accepted paths.

## Alternatives Considered

### Treat speculative output as ordinary committed output

This would be faster to integrate, but it would blur proposal and commit.

### Defer all speculative decoding until later milestones

This avoids risk, but loses a useful v0.91.2 evaluation opportunity.

## Validation Notes

This candidate should be reviewed against the speculative decoding feature doc,
benchmark/evaluation packet, and any prototype results produced in v0.91.2.

## Non-Claims

- This ADR does not claim production-grade acceleration.
- This ADR does not require speculative decoding in all runtimes.
- This ADR does not permit hidden side effects or bypassed authority checks.

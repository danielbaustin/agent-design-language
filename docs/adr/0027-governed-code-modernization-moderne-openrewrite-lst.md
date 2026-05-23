# ADR 0027: Governed Code Modernization With Moderne/OpenRewrite LST

- Status: Accepted
- Date: 2026-05-23
- Accepted in: v0.91.3
- Candidate source: docs/architecture/adr/0027-governed-code-modernization-moderne-openrewrite-lst.md
- Target milestone: v0.91.2
- Related issue: #3124
- Related ADRs: ADR 0020, ADR 0021, ADR 0025

## Context

v0.91.2 includes a bounded modernization demo around Moderne, OpenRewrite,
Lossless Semantic Trees (`LSTs`), and deterministic recipes.

Modernization can be high-value, but it is also write-capable and potentially
large-scale. ADL needs a decision boundary that distinguishes governed
deterministic transformation from vague "AI rewrites code automatically"
language.

## Decision

ADL may integrate Moderne/OpenRewrite-style modernization as a governed
deterministic transformation workflow only when discovery, dry-run, review,
patch generation, validation, and acceptance remain explicit and review-bound.

Modernization recipes are proposed transformations. They do not become accepted
source changes without issue/PR authority and review.

## Requirements

- Use accurate terminology: Moderne as platform, OpenRewrite as framework, LST
  as lossless semantic code model, recipes as deterministic transformations.
- Start with discovery and dry-run before patch authority.
- Record affected files, recipe intent, validation expectations, and rollback
  posture.
- Keep generated patches reviewable and reversible.
- Route write-capable modernization through ACC/Freedom Gate policy where live
  tool execution is involved.
- Connect modernization outputs to CodeFriend review packets and normal issue/PR
  promotion only through explicit review boundaries.

## Consequences

### Positive

- Lets ADL use powerful modernization tooling without losing review discipline.
- Creates a clean story for deterministic refactoring and productized review.
- Keeps large-scale code change under issue/PR governance.

### Negative

- Mass modernization remains intentionally slower than unreviewed automation.
- Recipe outputs need validation and human review.
- Tool integration must preserve authority and rollback semantics.

## Alternatives Considered

### Treat modernization as ordinary generated patching

This loses the value of LST-aware deterministic recipes and weakens proof.

### Allow automatic bulk rewrite

This is outside ADL's governance posture and too risky for source authority.

## Validation Notes

This ADR was reviewed against the modernization feature doc,
Moderne/OpenRewrite/LST planning notes, dry-run evidence, and CodeFriend review
boundaries.

## Non-Claims

- This ADR does not authorize unreviewed bulk rewrite.
- This ADR does not claim production modernization service readiness.
- This ADR does not require Moderne for all future code modernization.

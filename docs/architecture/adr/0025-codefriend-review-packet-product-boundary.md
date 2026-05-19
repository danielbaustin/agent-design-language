# ADR 0025 Candidate: CodeFriend Review Packet Product Boundary

- Status: Candidate
- Target milestone: v0.91.2
- Related issue: #3124
- Related ADRs: ADR 0018, ADR 0024 after acceptance

## Context

v0.91.2 moves CodeFriend from an internal review-skill cluster toward a
productizable review-packet workflow. The feature package includes evidence
requirements, review packet workflow, product report template, skill/demo
alignment, and the `CodeFriend.ai` naming boundary.

This work needs an ADR because productization can easily overclaim what review
automation proves.

## Decision

ADL treats CodeFriend as an evidence-bound review-packet and product-report
workflow.

CodeFriend may produce source-grounded findings, diagrams, remediation plans,
test recommendations, synthesis packets, redaction checks, and customer-grade
reports. It does not replace human judgment, customer approval, or
publication-time responsibility.

## Requirements

- Review packets must identify source evidence, scope, skipped surfaces, and
  residual risk.
- Findings must be ordered by severity and tied to concrete evidence.
- Diagrams and reports must distinguish source-backed claims from inference.
- Redaction and publication boundaries must be explicit before external use.
- Product language must preserve uncertainty and avoid unsupported claims.
- Human review and operator approval remain part of the delivery boundary.

## Consequences

### Positive

- Gives CodeFriend a credible product boundary without hype.
- Connects review skills, diagrams, test planning, synthesis, and reporting
  into one repeatable workflow.
- Helps future customer-facing work stay evidence-bound.

### Negative

- Product reports must carry caveats and residual risk instead of sounding
  magically certain.
- Review-quality failures become product defects, not mere internal nits.
- Publication and redaction review remain required work.

## Alternatives Considered

### Treat CodeFriend as just a bundle of skills

This is accurate historically but too weak for productization.

### Market CodeFriend as autonomous code-review authority

This would be false and unsafe. CodeFriend produces governed evidence and
recommendations, not unchallengeable authority.

## Validation Notes

This candidate should be reviewed against the v0.91.2 CodeFriend feature doc,
review packet workflow package, evidence requirements, product report template,
and review heuristics demo results.

## Non-Claims

- This ADR does not authorize external publication.
- This ADR does not claim CodeFriend replaces human review.
- This ADR does not claim every CodeFriend report is automatically correct.

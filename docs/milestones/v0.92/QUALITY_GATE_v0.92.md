# v0.92 Quality Gate

This file defines the evidence required for v0.92 release credit. It is a gate
plan, not evidence that a gate has passed.

| Gate | Owning work | Required evidence |
| --- | --- | --- |
| Milestone truth and issue graph | WP-01, WP-01B | Live issue-number map, dependency validation, six valid typed cards per issue, current canonical docs and version declarations |
| Repository ownership | WP-02 | Reviewed migration plan, five verified transfers, preserved GitHub surfaces including issue/PR assignee retention or explicit reassignment, negative `asksifu` control, Horust exclusion |
| CI and coverage | WP-02A | Deterministic lane selection, separated fast/slow work, nonduplicated coverage, platform parity, exact-head green checks |
| Runtime resilience | WP-03, WP-04 | Guardian-owned launch, recovery and relocation proof, clean logs, distributed security review, cross-platform validation |
| Workflow efficiency | WP-05 through WP-07 | Measured cycle-time improvement, portable validation, prompt-card contract parity, regression proof |
| Birthday contract | WP-08 through WP-17 | Identity, continuity, memory, capability, profile, protocol, witness, receipt, review packet, and cross-polis semantics with negative cases |
| Integrated demonstrations | WP-18, WP-18A, WP-18B | Real first-birthday proof, working Observatory/Unity consumers, provider-neutral multi-agent evidence |
| Governance handoff | WP-19 | Evidence map for v0.93 without claiming v0.93 governance is implemented |
| Cleanup and maintainability | WP-20, WP-21, WP-21A | Proven deletion eligibility, behavior-preserving reduction, focused Rust refactoring, no parity regression |
| Review and release | WP-22, WP-23, WP-25 through WP-30 | Quality review, release evidence, claim-bounded publication, external review, remediation, ceremony, handoff |

## Global Rules

- Every issue must complete its declared outcome at the exact reviewed
  revision. Scaffolding, placeholders, and partial work are not completion
  unless the issue explicitly defines that bounded slice as its full outcome.
- Planning text, fixtures, receipts, and simulated success do not replace real
  behavior where a work package requires execution.
- Runtime, protocol, provider, consumer, migration, and integration work must
  prove real positive and negative production-path behavior. Demo mode,
  synthetic success, and substituted providers receive no release credit.
- Documentation and planning work must be source-grounded and decision-ready,
  with owners, boundaries, dependencies, acceptance criteria, and executable
  next steps. Restating intent is not a useful deliverable.
- Tooling and cleanup work must demonstrate measured operator or
  maintainability value and focused regression safety.
- Focused validation is preferred, but every claimed platform or integration
  must have evidence at the reviewed revision.
- Product changes require exact-head review and green required checks.
- Release notes and public materials may describe only landed, reviewed work.
- Legal personhood, production citizenship, consciousness, and completed v0.93
  constitutional governance remain non-claims.
- Any issue that fails these rules blocks WP-22 and cannot enter internal
  review as completed work.

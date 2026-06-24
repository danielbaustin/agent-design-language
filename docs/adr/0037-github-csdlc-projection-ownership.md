# ADR 0037: GitHub/C-SDLC Projection Ownership

- Status: Accepted
- Date: 2026-06-23
- Accepted in: v0.91.6
- Candidate source: docs/architecture/adr/0037-github-csdlc-projection-ownership.md
- Target milestone: v0.91.6
- Related issues: #3935, #4047, #4286
- Related ADRs: ADR 0024, ADR 0028, ADR 0033
- Source evidence:
  - `docs/milestones/v0.91.6/review/github_projection/GITHUB_CSDLC_PROJECTION_MAP_4047.md`
  - `docs/milestones/v0.91.6/review/CSDLC_GITHUB_PROJECTION_CONVERGENCE_REVIEW_3935.md`

## Context

C-SDLC workflow truth spans local cards, tracked artifacts, branches, PRs,
issues, reviews, labels, comments, and closeout records. GitHub is essential to
the workflow, but it is not uniformly authoritative for every C-SDLC fact.

v0.91.6 identified repeated drift risk when GitHub state, local cards, issue
records, and release evidence disagree. The fix is not to ignore GitHub or to
make GitHub the only truth source. The architecture needs explicit projection
ownership.

## Decision

ADL should classify GitHub-facing C-SDLC surfaces by ownership type.

The primary classes are:

- managed projection: generated or repaired from tracked C-SDLC truth
- drift-checked projection: checked against C-SDLC truth and reported when it
  diverges
- linked external state: real GitHub state that C-SDLC records must consume
  without pretending to own
- card-local truth: lifecycle facts that remain authoritative in SIP/STP/SPP/
  SRP/SOR and tracked records
- deferred surface: known surface that is not yet safely automated

C-SDLC cards and tracked records remain authoritative for lifecycle intent,
planning, review disposition, validation claims, and closeout truth. GitHub
issues and PRs remain authoritative for their external state: open/closed PRs,
checks, reviews, mergeability, labels, comments, and issue closure.

## Consequences

### Positive

- Reduces silent drift between GitHub and C-SDLC records.
- Lets tooling repair owned projections without overreaching into external
  state.
- Gives workflow-conductor, pr-janitor, and closeout tools a shared language
  for GitHub/C-SDLC convergence.

### Negative

- Every new GitHub integration must declare its ownership class.
- Some surfaces remain manual or drift-checked until automation is trustworthy.
- Legacy linkage bugs can block promotion unless explicitly routed.

## Alternatives Considered

### Treat GitHub as the only workflow truth

This loses C-SDLC card semantics, review evidence, proof claims, and local
execution context.

### Treat tracked cards as the only workflow truth

This ignores real PR checks, review comments, merge state, labels, and closure
state that only GitHub can provide.

## Validation Notes

Promotion should review the projection map and convergence review. If the
legacy closing-linkage guard tracked by #4286 is still unresolved, the accepted
ADR must either wait or carry that residue as an explicit deferred
implementation note.

## Non-Claims

- This ADR does not implement all projection repairs.
- This ADR does not remove GitHub from the workflow.
- This ADR does not allow local card truth to overrule real PR check failures.
- This ADR does not claim every legacy issue/PR record is already converged.

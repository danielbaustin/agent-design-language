# ADR 0034 Candidate: C-SDLC Evidence Convergence, Signed Trace, And ObsMem Handoff

- Status: Candidate
- Target milestone: v0.91.4
- Related issues: #3444
- Related ADRs: ADR 0002, ADR 0006, ADR 0007, ADR 0028, ADR 0029 after acceptance
- Source evidence:
  - `docs/milestones/v0.91.4/features/EVIDENCE_CONVERGENCE_REVIEW_SYNTHESIS_AND_SIGNED_TRACE.md`
  - `docs/milestones/v0.91.4/features/OBSMEM_TRANSITION_MEMORY_INTEGRATION.md`
  - `docs/milestones/v0.91.4/DESIGN_v0.91.4.md`
  - `docs/milestones/v0.91.4/DECISIONS_v0.91.4.md`

## Context

ADR 0028 established that durable C-SDLC truth should become tracked in Git and
signed-trace-backed. v0.91.4 makes that direction operational enough for
default-operation claims: evidence bundles, review synthesis, signed trace
proof, and ObsMem handoff need to converge.

Without convergence, evidence remains scattered. A transition can have cards,
review notes, logs, PR checks, and memory notes, but no single inspectable path
from what was planned to what was reviewed, what was verified, and what future
agents should remember.

## Decision

ADL should treat C-SDLC evidence convergence, signed trace proof, and ObsMem
handoff as one architecture boundary for durable transition memory.

Durable transition proof should include:

- tracked evidence bundle
- review synthesis output
- `SRP` linkage to findings, dispositions, and residual risk
- `SOR` linkage to execution, validation, integration, and closeout truth
- minimal signed trace bundle or explicit blocker
- verification result in repo-relative paths
- ObsMem ingestion surface derived from tracked evidence

Missing or unverifiable signed trace proof blocks default-operation claims that
depend on durable C-SDLC proof.

## Consequences

### Positive

- Gives future agents and humans one inspectable proof path.
- Reduces drift between review findings, SOR outcome truth, release evidence,
  and memory.
- Makes ObsMem ingestion safer by sourcing it from tracked evidence rather
  than local-only session state.
- Brings signed trace into C-SDLC early enough to matter.

### Negative

- Evidence bundles and signed trace fixtures add maintenance overhead.
- Some transitions may need explicit blockers rather than incomplete proof.
- Memory handoff quality now depends on evidence quality.

## Alternatives Considered

### Keep evidence, trace, and memory as separate surfaces

This preserves modularity, but it leaves reviewers and future agents to
reconstruct transition truth manually.

### Defer signed trace until full trace-query work

Full trace query/TQL can wait. Minimal signed trace proof is needed before
C-SDLC default-operation claims become credible.

## Validation Notes

This candidate should be reviewed against v0.91.4 evidence bundle packet,
review synthesis output, signed trace fixtures, validation scripts, ObsMem
transition-memory docs, and release evidence.

## Non-Claims

- This ADR does not require full trace query/TQL completion.
- This ADR does not replace normal CI or PR checks.
- This ADR does not make unsigned local logs acceptable durable proof.
- This ADR does not claim every historical transition has already been
  backfilled.

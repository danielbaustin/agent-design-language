# ObsMem Transition Memory Integration

## Status

Planned `v0.91.4` feature.

## Purpose

Make C-SDLC transition memory real enough that future agents can learn from
reviewed software-development history without relying on oral reconstruction or
untracked local notes.

C-SDLC should feed ObsMem from tracked evidence: cards, review results, outcome
truth, trace proof, residual risks, and follow-on routing.

## Scope

`v0.91.4` should define and prove:

- memory handoff records derived from final `SRP` and `SOR` truth
- links to issue, PR, branch, evidence bundle, and signed trace proof
- fact/judgment/risk/follow-on separation
- write/read fixture or dry-run output for transition memory
- blocked-state handling when evidence is stale, untracked, or unverifiable

## Acceptance Criteria

- ObsMem ingestion consumes tracked transition evidence, not local-only files.
- Review findings and dispositions are represented distinctly from final
  outcome facts.
- Residual risks and follow-on issues remain visible to later agents.
- The memory handoff can be replayed or inspected from repo state.
- The release packet includes memory-integration evidence or an explicit
  blocker.

## Non-Goals

- This feature does not ingest every historical issue retroactively.
- This feature does not treat draft cards as canonical memory.
- This feature does not depend on external workspace infrastructure.

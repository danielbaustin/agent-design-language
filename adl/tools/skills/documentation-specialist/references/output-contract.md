# Documentation Specialist Output Contract

Use this contract for documentation handoff packets, documentation audits, and
edit summaries.

## Required Sections

### Documentation Scope

- Target path or artifact:
- Documentation type:
- Audience:
- Mode:
- Edit scope:
- Publication attempted: true | false

### Source Evidence

- Evidence paths:
- Commands or demos referenced:
- Issues, PRs, or review artifacts referenced:
- Evidence not available:

### Claim Ledger

Classify important claims as:

- Source-backed fact:
- Assumption:
- Recommendation:
- Gap or missing proof:
- Planned but not implemented:

Do not collapse assumptions or recommendations into facts.

### Edits Or Recommendations

For edits:

- Paths changed:
- Claims clarified:
- Stale commands repaired:
- Overclaims removed or qualified:
- Missing validation evidence surfaced:

For audit or planning:

- Recommended changes:
- Priority:
- Rationale:
- Blocking gaps:

### Validation

- Commands run:
- Commands not run and why:
- Paths checked:
- Link or renderer checks:
- Secret or host-path scan:
- Result: pass | partial | fail | not_run

### Residual Risk

- Remaining uncertainty:
- Human decisions required:
- Follow-up skills recommended:
- Publication blockers:

## Guardrails

The output must state:

- Publication attempted: false, unless the user explicitly asked for an external
  publication action and another authorized workflow performed it.
- Release approval claimed: false.
- Review approval claimed: false.
- ADR accepted: false, unless a human decision explicitly accepted it.
- Broad rewrite performed: false, unless the target scope explicitly allowed it.

## Unsafe Output Patterns

Do not write:

- "This is release-ready" without release proof and human approval.
- "The system does X" when evidence only shows planned behavior.
- "All docs are updated" after touching only a bounded slice.
- Absolute host paths in public docs.
- Secret markers, prompt fragments, or private tool arguments.

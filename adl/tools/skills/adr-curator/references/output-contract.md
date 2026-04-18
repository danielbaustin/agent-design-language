# Output Contract

The ADR curator skill produces proposed Architecture Decision Record candidates
from CodeBuddy review evidence without accepting decisions or mutating
repositories.

Default artifact root:

```text
.adl/reviews/codebuddy/<run_id>/adr-curation/
```

## Required Artifacts

### adr_candidates.md

Required sections:

- Metadata
- Scope
- ADR Candidate Catalog
- Proposed ADR Drafts
- Accepted Or Superseded Existing Decisions
- Deferred Decision Candidates
- Supersession Map
- Approval Boundary
- Validation Notes
- Residual Risk

Each ADR candidate must include:

- ADR id
- title
- status
- source evidence
- context
- decision
- consequences
- alternatives considered
- supersession links
- validation notes
- approval boundary

### adr_candidates.json

Required top-level fields:

- `schema`
- `source`
- `repo_name`
- `status`
- `candidate_count`
- `deferred_count`
- `adr_candidates`
- `deferred_candidates`
- `supersession_map`
- `approval_boundary`

Each candidate object must include:

- `adr_id`
- `title`
- `status`
- `source_artifact`
- `source_evidence`
- `context`
- `decision`
- `consequences`
- `alternatives_considered`
- `supersession_links`
- `validation_notes`
- `approval_boundary`

## Status Values

- `pass`: candidates were produced and no decision candidates were deferred.
- `partial`: candidates were produced and at least one decision candidate was
  deferred.
- `not_run`: no readable decision evidence was available.
- `blocked`: explicit requested behavior would cross the decision acceptance or
  repository mutation boundary.

## Rules

- Use repo-relative or packet-relative paths.
- Do not write absolute host paths into artifacts.
- Do not accept, reject, supersede, publish, or commit ADRs.
- Do not create issues, PRs, remediation branches, tests, or tracker items.
- Mark proposed ADRs as proposed unless accepted status is explicit in source
  evidence.
- Preserve supersession links when source evidence names them.
- Mark weak or unsupported decisions as deferred rather than ready.


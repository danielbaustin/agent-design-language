# ADR 0042 Candidate: Public Prompt Records Publication Boundary

- Status: Candidate
- Target milestone: v0.91.6
- Related issues: #3969, #4002, #4003, #4004, #4005, #4006
- Related ADRs: ADR 0024, ADR 0028
- Source evidence:
  - `docs/milestones/v0.91.6/features/PUBLIC_PROMPT_RECORDS_EXPORT_v0.91.6.md`
  - `docs/milestones/v0.91.6/review/V0916_WP04_PUBLIC_PROMPT_RECORDS_SPRINT_REVIEW_3969.md`
  - `docs/milestones/v0.91.6/review/public_prompt_records/PUBLIC_PROMPT_RECORDS_EXPORT_CONTRACT_4002.md`
  - `docs/milestones/v0.91.6/review/public_prompt_records/PUBLIC_PROMPT_RECORDS_REDACTION_PUBLICATION_SAFETY_4003.md`
  - `docs/milestones/v0.91.6/review/public_prompt_records/PUBLIC_PROMPT_RECORDS_VALIDATION_INDEXING_4004.md`
  - `docs/milestones/v0.91.6/review/public_prompt_records/PUBLIC_PROMPT_RECORDS_SECURITY_CAV_HANDOFF_4005.md`
  - `docs/milestones/v0.91.6/review/public_prompt_records/PUBLIC_PROMPT_RECORDS_DISTRIBUTION_CLOSEOUT_4006.md`

## Context

ADL authoring and C-SDLC prompt records are created in local and repository
workflow contexts. They are valuable public evidence only after they have been
exported, redacted, validated, indexed, and reviewed for publication safety.

v0.91.6 established that `.adl` remains the editable authoring surface, while
public prompt records are projections intended for distribution. This boundary
must remain explicit before public prompt records become a v0.92 activation
surface.

## Decision

ADL should treat public prompt records as reviewed projections, not raw
publication of private/local authoring state.

Public prompt records may be published only through a governed path that covers:

- export contract
- redaction and publication safety
- validation and indexing
- security/CAV handoff
- distribution closeout
- clear source/projection linkage

The `.adl` authoring surface remains editable and may contain local execution
context, private workflow state, or non-public material. It is not
automatically public-safe.

## Consequences

### Positive

- Preserves public transparency without leaking unsafe local authoring context.
- Gives reviewers clear gates for export, redaction, validation, indexing, and
  distribution.
- Keeps public records traceable to governed source records.

### Negative

- Public distribution requires extra review and validation work.
- Historical prompt records may need selective routing instead of bulk export.
- The public repository may lag behind local authoring until projection checks
  complete.

## Alternatives Considered

### Publish raw `.adl` records directly

This is rejected because local authoring records are not automatically
redacted, validated, indexed, or safe for public consumption.

### Keep all prompt records private

This avoids publication risk but weakens C-SDLC transparency, external review,
and public evidence value.

## Validation Notes

Promotion should review the WP-04 sprint review and the export, redaction,
validation, security/CAV, and distribution packets. It should verify that the
ADR does not claim unrestricted historical publication or completed public
activation beyond the reviewed projection path.

## Non-Claims

- This ADR does not publish any prompt records.
- This ADR does not claim every `.adl` bundle is public-safe.
- This ADR does not bypass security/CAV review.
- This ADR does not make the public repository the authoring source of truth.

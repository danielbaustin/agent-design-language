# Issue 5848 Design: Review Findings Remediation

Status: design-time ready; remediation waits for the received WP-26 review.

## Authority And Sources

Issue #5848 and WP-27 own disposition of the complete WP-25/WP-26 finding
universe. The internal register, external report, external findings index,
exact reviewed revision, and any later current recheck are inputs. Historical
v0.91.8 remediation registers provide a format precedent only; v0.92 findings
must be resolved from current source and exact evidence.

## Outcome Contract

Create one canonical disposition row per unique finding with original IDs,
source reviewer, severity, evidence, affected owner, in-scope decision,
disposition, remediation issue/PR, fix head, proving validation, review head,
merge state, residual risk, and downstream release-doc impact. Group findings
into the smallest coherent owner-aligned remediation slices. Fix all actionable
in-scope findings; route true out-of-scope items to explicit follow-ons; accept
risk only with operator authority and evidence. Never erase, renumber, or mark a
finding fixed from intent alone.

## Execution Sequence

1. Verify WP-26 terminal/ancestral truth and freeze the complete internal plus
   external finding universe.
2. Deduplicate only when evidence and failure mode are genuinely identical;
   preserve provenance and reviewer disagreement.
3. Assign each finding to an owner-aligned remediation slice with exact paths,
   acceptance criteria, negative cases, and rollback.
4. Implement and validate slices through their issue-bound lifecycles; record
   exact fix/review/merge identity in the disposition register.
5. Re-run affected review and quality-gate checks, including release-facing
   claim corrections where behavior changed.
6. Obtain exact-head review of the complete disposition register and block
   WP-28 while any actionable finding remains open or unproven.

## Owned Paths

- `docs/reviews/v0.92/remediation-5848`
- `.csdlc/evidence/5848`
- `.csdlc/prepared/issues/5848/validate-remediation-regressions.rb`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Validation And Failure Policy

Required lanes are finding-universe completeness, dedup/provenance checks,
disposition-schema validation, focused positive and negative tests per fix,
platform/security/privacy lanes required by the finding, exact fix/review/merge
readback, execution of every affected WP-22 row validator and every impacted
release-claim validator at the target SHA, and open-finding rejection. Any
unresolved actionable finding, missing proof, stale fix SHA, or unauthorized
risk acceptance blocks completion.

## Rollback

Revert only remediation commits whose live PR, review, merge, ancestry, terminal, or regression proof fails, leaving accepted-risk records and unrelated remediations intact. Reopen the affected disposition, rerun its allowlisted regression validator, and rebuild the canonical disposition register from internal and external findings.

## Non-Goals
- No suppression of reviewer findings or blanket risk acceptance.
- No unrelated cleanup, milestone replanning, or release ceremony.
- No claim that opening a remediation PR equals a fixed or merged disposition.

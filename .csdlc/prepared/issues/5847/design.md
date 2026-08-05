# Issue 5847 Design: External Or Third-Party Review

Status: design-time ready; dispatch waits for truthful WP-25 completion.

## Authority And Sources

Issue #5847 and WP-26 own the formal external-review handoff and received
review. The WP-25 report/register are mandatory inputs. The retained v0.91.8
third-party handoff demonstrates the required exact target revision, source
manifest, digest freshness, reviewer authority, redaction, and non-approval
boundaries; no v0.91.8 result is current v0.92 evidence.

## Outcome Contract

Produce a self-contained, publication-safe handoff naming repository, base,
head, exact commit, packet manifest/digest, predecessor states, included and
excluded surfaces, review questions, acceptance vocabulary, and return format.
Freeze the packet before dispatch. Receive and retain the reviewer-authored
report without rewriting its findings, then normalize only a separate findings
index that maps each returned item to evidence and a WP-27 route.

The external reviewer is read-only and cannot edit code, mutate lifecycle or
GitHub state, merge, publish, approve release, access secrets/private state, or
infer completion from unavailable evidence. A blocked or findings-returned
review is a valid outcome; only an actual received report proves review occurred.

## Execution Sequence

1. Verify WP-25 terminal/ancestral truth and resolve every internal-review
   blocker that prevents a coherent handoff.
2. Build and redact the exact source/evidence manifest; compute a stable digest
   over tracked packet objects and normalized handoff metadata.
3. Validate all links, commands, claims, issue/PR identities, and reviewer
   authority; freeze the packet at the named SHA.
4. Dispatch through the operator-approved reviewer channel and record only
   truthful send/receive state, cost, and limits.
5. Fail closed if source or digest changes; refresh and re-authorize instead of
   presenting a stale review as current.
6. Retain the received report verbatim where permitted, create the findings
   index, and hand every actionable item to WP-27.

## Protected-Path Candidates

- `REVIEW.md` only if it remains the active external-review entrypoint
- `docs/milestones/v0.92/review/THIRD_PARTY_REVIEW_HANDOFF_v0.92.md`
- `docs/reviews/v0.92/external-review-5847`
- `.csdlc/evidence/5847`

Reviewed implementation/docs paths remain read-only. Reviewer output is never
silently edited to make the result more favorable.

## Validation And Failure Policy

Required lanes are predecessor/readiness checks, packet object inventory and
digest reproducibility, stale-digest rejection, link/command validation,
secret/private-path/redaction scanning, reviewer-authority checks, sent/received
state validation, report integrity, and findings-index completeness. Missing
reviewer output, changed packet identity, inaccessible evidence, or unsafe
content yields blocked/deferred truth, not a synthetic review pass.

## Non-Goals

- No product remediation, issue mutation by the reviewer, or release approval.
- No paid/provider dispatch without explicit operator-approved credentials and
  channel at execution time.
- No conversion of per-issue shadow reviews into formal milestone review.

# v0.91.6 ADR Docs Review For #4383

Date: 2026-06-23
Issue: #4383 `[v0.91.6][adr] Create and route v0.91.6 ADR candidates`
Review status: `reviewed_findings_resolved`
Reviewer mode: ADR/documentation review, issue-graph truth review

## Scope

Reviewed the v0.91.6 ADR candidate surfaces associated with #4383:

- `docs/milestones/v0.91.6/ADR_MINI_SPRINT_PACKET_v0.91.6.md`
- `docs/milestones/v0.91.6/DECISIONS_v0.91.6.md`
- `docs/milestones/v0.91.6/review/V0916_ADR_CANDIDATE_CONTENT_REVIEW_4416.md`
- `docs/milestones/v0.91.6/review/V0916_ADR_RELEASE_TAIL_MINI_SPRINT_REVIEW_4324.md`
- `docs/architecture/adr/README.md`
- `docs/architecture/adr/CANDIDATE_ADRS.md`
- `docs/architecture/adr/0035-local-polis-ssm-operations-boundary.md`
- `docs/architecture/adr/0036-validation-lane-selector-pvf-test-cost-policy.md`
- `docs/architecture/adr/0037-github-csdlc-projection-ownership.md`
- `docs/architecture/adr/0038-runtime-integration-soak-boundary.md`
- `docs/architecture/adr/0039-cognitive-scheduler-v1-authority-boundary.md`
- `docs/architecture/adr/0040-workflow-lockfile-discipline.md`
- `docs/architecture/adr/0041-provider-model-suitability-boundary-v2.md`
- `docs/architecture/adr/0042-public-prompt-records-publication-boundary.md`

This review does not accept or promote any candidate ADR. It reviews candidate quality, routing truth, and readiness for later explicit ADR acceptance work.

## Findings

### P1: #4383 did not have complete issue machinery when review started

The GitHub issue existed and was open, but the local issue machinery was missing before this review pass: no `#4383` branch, no `.worktrees/adl-wp-4383`, and no local `.adl/v0.91.6` task/card bundle. That made the ADR review hard to locate and left the issue graph misleading even though related ADR work had already landed through #4324 and #4416.

Repair performed during this review setup:

- Restored #4383 to `version:v0.91.6` after a bad `pr init` inference temporarily rewrote it as `v0.91.7`.
- Created the canonical local `.adl/v0.91.6` task bundle with `pr init 4383 --version v0.91.6`.
- Created a narrow issue worktree manually after `pr run` stalled in repeated `pr.list.wave` calls.

Follow-up tooling issues created:

- #4474: fix `pr init` version inference before remote mutation.
- #4475: make `pr init` duplicate local identity checks fail before GitHub edits.

Disposition: fixed for #4383 closeout by recording the missing-machinery
incident, preserving the review packet as tracked evidence, and routing the
tooling defects to `#4474` and `#4475`. This review does not hand-edit or
invent local lifecycle cards.

### P2: Candidate ADR content is good, but promotion routing is still not concrete enough

The candidate ADR files exist and are mostly high quality. The review docs correctly say the candidates remain proposed and require a later explicit acceptance issue before promotion. However, the docs do not yet provide a concrete disposition table that names the follow-on issue, target milestone, and evidence gate for each candidate.

This matters because #4383's required outcome includes “ADR candidate files or explicit ADR follow-on routes.” The candidate files exist, but the follow-on routes remain policy prose rather than an executable issue-graph plan.

Recommended fix: add a routing table to the ADR packet or candidate index.

Disposition: fixed. `docs/milestones/v0.91.6/ADR_MINI_SPRINT_PACKET_v0.91.6.md`
and `docs/architecture/adr/CANDIDATE_ADRS.md` now include a post-review
candidate disposition route. `#4476` owns the acceptance/defer pass for ADR
0029-0042, ADR 0037 is marked `refresh_then_acceptance_review`, and ADR 0040
is explicitly marked `defer_until_evidence`.

### P2: DECISIONS still leaves ADR promotion as an open question after the ADR review pass

`DECISIONS_v0.91.6.md` still asks which v0.91.6 ADR candidates should be promoted or deferred. That was appropriate before #4324/#4416 completed, but after the candidate review pass it should point to the concrete post-review disposition path instead of staying as an unresolved generic question.

Recommended fix:

Update the decision log to say the release-tail ADR candidate packet and candidate content review are complete, candidates remain proposed, and promotion/defer decisions are routed to explicit follow-on acceptance issues.

Disposition: fixed. `docs/milestones/v0.91.6/DECISIONS_v0.91.6.md` now marks
D-08 as routed and points the remaining promotion/defer question to `#4476`.

### P3: ADR 0040 is intentionally not acceptance-ready, but should be tracked as an evidence-gated candidate

ADR 0040 correctly says it is candidate-only and requires evidence capture before acceptance. That honesty is good. The remaining gap is tracking: if ADR 0040 remains evidence-gated, the candidate index should name the issue or milestone that owns the evidence-capture proof.

Recommended fix:

Mark ADR 0040 as `defer_until_evidence` in the routing table and link its evidence owner.

Disposition: fixed. ADR 0040 is routed as `defer_until_evidence` under `#4476`
with the lockfile-discipline source packet or exact tracked files and validation
proof as the required evidence gate.

### P3: #4383 should be closed or narrowed after routing is made explicit

Once the concrete ADR disposition table exists, #4383 should not remain open as a vague “create and route ADR candidates” issue. It should either close as satisfied by the candidate packet/review artifacts or be narrowed to the explicit routing-table repair.

Disposition: fixed. With the routing table added, #4383 may close as satisfied
by the candidate packet, retained review artifacts, and follow-on disposition
route.

## What Looks Good

- Candidate ADRs 0035-0042 are source-backed and disciplined.
- The docs consistently avoid silently promoting candidates to accepted ADRs.
- The ADR packet makes the proposed/accepted distinction clear.
- The candidate review for #4416 is appropriately cautious and does not overclaim v0.92 readiness.
- ADR 0040 does not hide its evidence gap.
- The candidate ADRs have the expected ADR shape: status, context, decision, consequences, alternatives, validation notes, and non-claims.

## Candidate-by-Candidate Notes

| ADR | Review note |
| --- | --- |
| 0035 | Good boundary around Local Polis SSM operations. Candidate shape is adequate for later acceptance review. |
| 0036 | Strong PVF/test-cost policy candidate. Should be routed with VPP/PVF lane-registry work before acceptance. |
| 0037 | Good GitHub/C-SDLC projection ownership candidate. Refresh against current typed closing-linkage/tooling truth before promotion. |
| 0038 | Good runtime integration soak boundary candidate. Keep tied to runtime fire-up and soak evidence. |
| 0039 | Good cognitive scheduler authority-boundary candidate. Acceptance should wait for scheduler v1 implementation/proof truth. |
| 0040 | Correctly evidence-gated. Do not promote until workflow lockfile evidence capture is proven and linked. |
| 0041 | Good provider/model suitability boundary candidate. Acceptance should align with provider suitability proof outputs. |
| 0042 | Good public prompt-record boundary candidate. Acceptance should align with prompt-template/public-record publication policy. |

## Non-claims

This review does not claim:

- Any candidate ADR is accepted.
- v0.92 activation is approved.
- Runtime, provider, PVF, scheduler, or workflow-lockfile implementation is complete.
- #4383 accepts any ADR candidate.

## Recommended Closeout Path For #4383

1. Concrete ADR candidate disposition/routing table: complete.
2. `DECISIONS_v0.91.6.md` post-review routing truth: complete.
3. #4474 and #4475 tracked as tooling defects discovered during #4383 setup:
   complete.
4. Close #4383 after docs-only validation and publication/merge closeout.

## Validation Performed

- Reviewed the ADR packet, decision log, candidate index, candidate ADR files, and existing #4324/#4416 review artifacts.
- Checked GitHub/local workflow state for #4383.
- Created missing issue machinery for #4383 after confirming it was absent.
- Did not run code tests; this is a docs/ADR review issue.

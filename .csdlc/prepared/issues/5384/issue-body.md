## Summary

Own the v0.91.8 WP-14A integrated platform acceptance and deployment gate.

## Required Outcome

Accept the deployed ADL v2, Runtime v3, and C-SDLC v2 revisions as one coherent
platform baseline for the release tail.

## Direct Inputs

- C-SDLC v2 acceptance: #5358
- Runtime v3 acceptance: #5361
- ADL v2 soak and rollback proof: #5344
- ADL v2 reversible default switch: #5343

These inputs are closed and must be checked against current repository and
GitHub truth during execution.

## Deliverables

- Exact-revision three-product acceptance ledger.
- Stable-install and fresh-consumer proof.
- Operations and rollback/recovery evidence index.
- Explicit accepted-feature and non-claim boundaries for downstream work.

## Acceptance Criteria

- The four direct inputs are closed and their merged revisions are contained in
  the accepted repository baseline.
- ADL v2, Runtime v3, and C-SDLC v2 operational entrypoints pass focused
  fresh-consumer checks.
- Existing rollback and recovery proofs are indexed rather than rerun without
  need.
- The acceptance ledger names exact revisions and unresolved residual risks.
- One bounded Gemini review passes before publication.
- SRP/SOR and GitHub issue truth agree at closeout.

## Scheduling Decision

WP-13 deletion issues #5346 and #5347 are deliberately deferred until
immediately before internal review #5356. They do not block WP-14A preparation
or execution. #5356 must not start until the deferred deletion wave has merged
and its focused post-deletion validation passes.

## Routing Boundary

- Unity Observatory work belongs to WP-15 #5354.
- C-SDLC tooling defects belong to WP-20 #5363.
- v0.92 handoff, Memory Palace, identity/birthday, capability-envelope, and
  Adaptive Learning planning belong to WP-21 #5362.
- Downstream tracks consume WP-14A acceptance; they do not block it.

## Non-goals

- Do not execute WP-13 deletion in this issue.
- Do not absorb Unity, tooling-remediation, Memory Palace, or v0.92 planning.
- Do not infer identity, consciousness, birthday, or unsupported-provider
  claims.
- Do not rerun expensive proofs when exact retained evidence is sufficient.

## Tooling Notes

Use typed C-SDLC v2 lifecycle routing and the existing issue-bound worktree.
Never write on `main`. Use one bounded Gemini review before the PR.

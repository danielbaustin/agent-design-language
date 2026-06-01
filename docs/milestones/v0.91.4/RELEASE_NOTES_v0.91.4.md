# v0.91.4 Release Notes

## Metadata

- Product: `Agent Design Language`
- Version: `v0.91.4`
- Release date: `2026-06-01`
- Tag: `v0.91.4`
- Release status: ceremony complete after WP-21 merge and tag publication

## Summary

`v0.91.4` closes the Cognitive SDLC rollout-closeout milestone. It turns the
v0.91.3 first C-SDLC slice into a repeatable, reviewable default operating lane
for ADL software-development work: issue cards, conductor/editor discipline,
bounded worktrees, review truth, closeout truth, durable evidence, and
next-milestone handoff now converge through tracked repository surfaces.

This release does not claim that all future C-SDLC acceleration work is done.
Multi-agent stabilization, provider/model breadth, public prompt records,
first-birthday preflight, Unity Observatory/demo work, and remaining activation
readiness are routed to the `v0.91.5` bridge milestone.

## How To Use

Use these notes as the source for the GitHub Release body after the WP-21 PR
merges and tag `v0.91.4` is created from clean `main`. Do not treat this file
as proof that the tag or GitHub Release already exists before the post-merge
publication step runs.

## Highlights

- Makes `SIP -> STP -> SPP -> SRP -> SOR` the durable C-SDLC issue lifecycle
  expected for ADL software-development work.
- Hardens workflow validators, doctor checks, conductor/editor routing, PR
  publication, and closeout truth around that lifecycle.
- Adds Software Development Polis actor-standing and shard-ownership proof
  surfaces for bounded coordination.
- Records merge-readiness, GitHub truth, review truth, and evidence convergence
  as governed proof surfaces rather than informal operator memory.
- Lands repeatability and validation-tail/PVF evidence so fast sprint work can
  separate fast default checks from slower proof lanes without hiding pending or
  deferred validation.
- Records internal review, external review, accepted-finding remediation,
  next-milestone planning, next-milestone review, and release ceremony evidence
  in the milestone package.
- Keeps CodeFriend and WildClawBench as bounded sidecar evidence, not C-SDLC
  default-operation blockers.

## What's New In Detail

### C-SDLC Lifecycle And Routing

- The card lifecycle is now the default issue shape for ADL work.
- Doctor and lifecycle validation identify stale card state instead of letting
  old issue truth silently pass.
- Editor skills and conductor routing are documented as required operational
  practice for card and planning-doc changes.

### Evidence, Review, And Closeout

- Sprint 4 produced tracked demo/proof coverage, quality gate, docs/adoption
  review, internal review, external review, remediation, next-milestone
  planning, next-milestone review, and release ceremony surfaces.
- Closed-issue SOR truth was normalized through the post-review-tail closeout
  sweep before ceremony.
- The release evidence packet identifies exactly which claims are proven,
  partial, routed, or not claimed.

### Validation Tail And PVF

- Parallel Validation Fabric policy work is available as release input.
- Slow proof lanes are treated as explicit release-gate evidence rather than
  hidden blockers inside ordinary fast PR feedback.
- The release does not claim that every future test is fully sharded; it claims
  the policy boundary and release-tail evidence are now explicit.

### Next Milestone

- `v0.91.5` is the selected bridge milestone.
- First-birthday readiness issue `#3377` and bridge work from `#3415+` are
  routed to `v0.91.5`.
- AEE completion routing, multi-agent usefulness testing, provider/model
  matrix, OpenRouter/DeepSeek provider work, public prompt records, demo
  readiness, and v0.92 activation testing are not hidden inside this release.

## Known Limitations And Non-Claims

- `v0.91.4` does not claim v0.92 first-birthday readiness.
- `v0.91.4` does not claim live multi-agent C-SDLC execution is production
  useful yet; that is `v0.91.5` work.
- `v0.91.4` does not claim Unity Observatory/demo completion.
- `v0.91.4` does not claim CodeFriend or WildClawBench product success as
  default-operation proof.
- `v0.91.4` does not claim enterprise-security feature separation is complete.
- Signed trace evidence is the milestone's minimal proof surface, not a claim
  that all future runtime/polis observability work is complete.

## Known Limitations

- The v0.91.5 bridge milestone is required before v0.92 begins.
- Some public prompt-record and `.adl` cleanup/archive work remains staged for
  v0.91.5.
- Provider/model breadth remains intentionally incomplete until v0.91.5
  exercises hosted, local, remote Ollama, OpenRouter, and DeepSeek surfaces.
- Multi-agent workcell evidence exists, but useful default multi-agent sprint
  operation is not claimed by this release.

## Reviewer Entry Points

- [Release evidence](RELEASE_EVIDENCE_v0.91.4.md)
- [Release readiness](RELEASE_READINESS_v0.91.4.md)
- [End-of-milestone report](END_OF_MILESTONE_REPORT_v0.91.4.md)
- [Quality gate](QUALITY_GATE_v0.91.4.md)
- [Demo matrix](DEMO_MATRIX_v0.91.4.md)
- [Feature proof coverage](FEATURE_PROOF_COVERAGE_v0.91.4.md)
- [Internal review packet](review/internal_review/README.md)
- [External review findings](review/third_party_review/V0914_EXTERNAL_REVIEW_FINDINGS_2026-06-01.md)
- [External review remediation](review/third_party_review/V0914_EXTERNAL_REVIEW_REMEDIATION_2026-06-01.md)
- [Next-milestone handoff](NEXT_MILESTONE_HANDOFF_v0.91.4.md)
- [Next-milestone review](review/next_milestone/V0914_NEXT_MILESTONE_REVIEW_2026-06-01.md)

## Upgrade Notes

- Future ADL software-development issues should use the C-SDLC lane by default
  unless explicitly scoped out.
- Issue cards are durable workflow state, not disposable chat artifacts.
- Local `.adl` execution state may support work, but release-facing proof must
  be tracked, sanitized, and reviewable from repository paths.
- New tests should carry PVF lane/proof metadata at authoring time.

## Validation Notes

- Closed issue/card truth was validated with
  `bash adl/tools/check_milestone_closed_issue_sor_truth.sh --version v0.91.4`,
  which passed with `checked=95`.
- Release-facing docs were validated with focused planning-template, Markdown
  link, YAML, diff hygiene, and redaction/path scans in WP-21.
- This release-notes refresh does not claim broad Rust/runtime tests were rerun
  in the docs-only ceremony PR.

## What's Next

`v0.91.5` stabilizes the machinery this release makes operational: multi-agent
C-SDLC execution, provider/model breadth, OpenRouter and DeepSeek provider
planning/implementation, public prompt records, activation testing for v0.92,
and the final v0.92 first-birthday readiness pass.

## Exit Criteria

- Release notes describe only tracked v0.91.4 evidence and routed follow-on
  work.
- Known limitations and non-claims are explicit.
- The text is suitable for the GitHub Release body after WP-21 merges and the
  tag is created.

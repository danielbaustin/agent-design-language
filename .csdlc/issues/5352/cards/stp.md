# Structured Task Prompt

Template: 1.0.0

Issue: 5352

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Finalize only the WP-21 exact-revision consumption handoff and integrate #5352; do not implement v0.92 features or absorb deferred #5713/#5733 work.

## Deliverables

- truthful six-card C-SDLC v2 packet
- exact-revision handoff ledger
- strict row-binding and baseline/ancestry validators
- focused retained validation evidence
- one exact pre-PR review and full WP-21 sprint review
- ready PR with Closes #5352 and green merge

## Acceptance

1. AC-1: origin/main exactly resolves to c34f0c9412495039a6374f7ce88fa39e34bb5042 for the recorded handoff baseline
2. AC-2: every accepted platform and WP-21 table row binds the exact issue, PR, reviewed head, and merge revision
3. AC-3: all recorded merge revisions are ancestors of the recorded baseline and all eight child issues are closed
4. AC-4: focused handoff, ancestry, implemented-packet, typed-doctor, and diff checks pass with current evidence
5. AC-5: exactly one GPT-5.5 pre-PR review covers the exact publication revision and all actionable findings are fixed
6. AC-6: a full findings-first WP-21 sprint review passes before #5352 merge
7. AC-7: the PR targets main, contains Closes #5352, passes required CI, merges, and closes #5352 without blocking on typed closeout

## Dependencies

- #5384 / PR #5726 merge 72fbf30c74a5193ea41f042c76c5986a48e59d6c
- #5358 / PR #5606 merge fc75f4fc697262f89f99461679a406be0b4b3775
- #5361 / PR #5650 merge f7258b07e9da414bfee518f0c89a76071bc03ee8
- #4758/#5739, #4759/#5738, #4760/#5740, #4761/#5741, #4762/#5744, #4763/#5734, #5007/#5743, and #5107/#5742 are merged and closed
- #5558 / PR #5749 is merged and closed at c34f0c9412495039a6374f7ce88fa39e34bb5042

## Inputs

- GitHub issue #5352
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/review/V0918_WP14A_PLATFORM_ACCEPTANCE_5384.md
- .csdlc/evidence/5384/platform-acceptance-ledger.v1.json
- .csdlc/evidence/5361/acceptance-proof-summary.json

## Non Goals

- v0.92 birthday execution or production-readiness declaration
- Adaptive Learning implementation
- AWS, provider, Unity, or broad runtime execution
- typed closeout
- changes to #5713 or #5733

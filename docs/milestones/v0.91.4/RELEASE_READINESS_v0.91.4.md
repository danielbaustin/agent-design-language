# Release Readiness - v0.91.4

## Status

`ceremony_ready`

`v0.91.4` is ready for release ceremony after WP-21 merge. The release manager
may tag and publish the GitHub Release only from clean `main` after the ceremony
PR lands.

## Review Entry Points

- [README.md](README.md)
- [WBS_v0.91.4.md](WBS_v0.91.4.md)
- [SPRINT_v0.91.4.md](SPRINT_v0.91.4.md)
- [WP_ISSUE_WAVE_v0.91.4.yaml](WP_ISSUE_WAVE_v0.91.4.yaml)
- [WP_EXECUTION_READINESS_v0.91.4.md](WP_EXECUTION_READINESS_v0.91.4.md)
- [DEMO_MATRIX_v0.91.4.md](DEMO_MATRIX_v0.91.4.md)
- [FEATURE_PROOF_COVERAGE_v0.91.4.md](FEATURE_PROOF_COVERAGE_v0.91.4.md)
- [QUALITY_GATE_v0.91.4.md](QUALITY_GATE_v0.91.4.md)
- [RELEASE_PLAN_v0.91.4.md](RELEASE_PLAN_v0.91.4.md)
- [RELEASE_EVIDENCE_v0.91.4.md](RELEASE_EVIDENCE_v0.91.4.md)
- [RELEASE_NOTES_v0.91.4.md](RELEASE_NOTES_v0.91.4.md)
- [NEXT_MILESTONE_HANDOFF_v0.91.4.md](NEXT_MILESTONE_HANDOFF_v0.91.4.md)
- [V0914_NEXT_MILESTONE_REVIEW_2026-06-01.md](review/next_milestone/V0914_NEXT_MILESTONE_REVIEW_2026-06-01.md)

## Current State

- WP-01 through WP-20 are closed.
- External review `#3367` is closed.
- Review remediation `#3368` is closed.
- Next-milestone planning `#3369` is closed.
- Next-milestone review `#3370` is closed.
- Closeout-normalization sweep `#3564` is closed.
- WP-21 `#3371` is the active release ceremony issue.
- Sprint 4 umbrella `#3362` remains open until WP-21 closes.
- `v0.91.5` is selected as the bridge milestone for pre-v0.92 stabilization.

## Gate Results

| Gate | Status | Evidence |
| --- | --- | --- |
| Demo/proof coverage | `pass` | [DEMO_MATRIX_v0.91.4.md](DEMO_MATRIX_v0.91.4.md), [FEATURE_PROOF_COVERAGE_v0.91.4.md](FEATURE_PROOF_COVERAGE_v0.91.4.md) |
| Quality gate | `pass_after_wp21_updates` | [QUALITY_GATE_v0.91.4.md](QUALITY_GATE_v0.91.4.md) |
| Internal review | `pass` | [internal review synthesis](review/internal_review/V0914_INTERNAL_REVIEW_SYNTHESIS_2026-05-31.md) |
| External review | `pass_with_remediation` | [external findings](review/third_party_review/V0914_EXTERNAL_REVIEW_FINDINGS_2026-06-01.md), [remediation](review/third_party_review/V0914_EXTERNAL_REVIEW_REMEDIATION_2026-06-01.md) |
| Closed issue/card truth | `pass` | `bash adl/tools/check_milestone_closed_issue_sor_truth.sh --version v0.91.4` passed with `checked=95` |
| Next-milestone handoff | `pass` | [NEXT_MILESTONE_HANDOFF_v0.91.4.md](NEXT_MILESTONE_HANDOFF_v0.91.4.md), [next-milestone review](review/next_milestone/V0914_NEXT_MILESTONE_REVIEW_2026-06-01.md) |
| Release notes | `ready` | [RELEASE_NOTES_v0.91.4.md](RELEASE_NOTES_v0.91.4.md) |

## Remaining Release Actions

These actions happen after the WP-21 PR merges:

1. Confirm `main` is clean and at the WP-21 merge commit.
2. Create and push tag `v0.91.4`.
3. Publish the GitHub Release using [RELEASE_NOTES_v0.91.4.md](RELEASE_NOTES_v0.91.4.md).
4. Close Sprint 4 umbrella `#3362` with the release link.
5. Begin `v0.91.5` bridge execution.

## Non-Claims

- This readiness packet does not claim that the tag already exists before
  WP-21 merge.
- This readiness packet does not claim v0.92 first-birthday readiness.
- This readiness packet does not claim multi-agent stabilization, provider
  breadth, public prompt records, or Unity Observatory work is complete.
- This readiness packet does not claim broad Rust tests were rerun during the
  docs-only ceremony PR.

## Release Decision

`go_for_wp21_merge_then_tag`

No unresolved v0.91.4 P0/P1 release blockers are known from the release-tail
evidence. Remaining work is explicitly post-merge publication or `v0.91.5`
bridge scope.

# Release Evidence - v0.91.4

## Metadata

- Milestone: `v0.91.4`
- Version: `v0.91.4`
- Ceremony issue: `#3371`
- Date: `2026-06-01`
- Status: `ceremony_ready`

## Purpose

This packet is the reviewer-facing evidence index for the `v0.91.4` release
ceremony. It gathers the tracked proof surfaces that support the C-SDLC
rollout-closeout claim and records the surfaces that are routed to `v0.91.5`
instead of being hidden as release-complete work.

This packet is evidence. It is not a substitute for GitHub release publication,
tag creation, or post-merge closeout.

## Core Release Claim

`v0.91.4` proves that ADL can use the Cognitive SDLC as its default
software-development operating lane with:

- issue-local card lifecycle: `SIP -> STP -> SPP -> SRP -> SOR`
- conductor/editor/worktree/PR/closeout discipline
- bounded review and remediation truth
- durable tracked release evidence
- next-milestone handoff before ceremony

The release does not claim that all acceleration and activation work is done.
That bridge work is selected and routed to `v0.91.5`.

## Required Evidence Families

| Evidence family | Status | Primary surfaces |
| --- | --- | --- |
| Demo/proof coverage | `present` | [DEMO_MATRIX_v0.91.4.md](DEMO_MATRIX_v0.91.4.md), [FEATURE_PROOF_COVERAGE_v0.91.4.md](FEATURE_PROOF_COVERAGE_v0.91.4.md), [BEST_AVAILABLE_CSDLC_DEMO_SHOWCASE_v0.91.4.md](review/demo_showcase/BEST_AVAILABLE_CSDLC_DEMO_SHOWCASE_v0.91.4.md) |
| Quality gate | `present` | [QUALITY_GATE_v0.91.4.md](QUALITY_GATE_v0.91.4.md), [V0914_TEST_COVERAGE_GAP_ANALYSIS_2026-05-30.md](review/quality_gate/V0914_TEST_COVERAGE_GAP_ANALYSIS_2026-05-30.md), [V0914_REDACTION_AUDIT_2026-05-30.md](review/quality_gate/V0914_REDACTION_AUDIT_2026-05-30.md) |
| C-SDLC lifecycle/adoption | `present` | [WP15_DOCS_ADOPTION_REVIEW_2026-05-31.md](review/docs_adoption/WP15_DOCS_ADOPTION_REVIEW_2026-05-31.md), [C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md](C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md) |
| Internal review | `present` | [internal review README](review/internal_review/README.md), [findings register](review/internal_review/V0914_INTERNAL_REVIEW_FINDINGS_REGISTER_2026-05-31.md), [synthesis](review/internal_review/V0914_INTERNAL_REVIEW_SYNTHESIS_2026-05-31.md) |
| External review | `present` | [third-party review handoff](review/third_party_review/ADL_v0.91.4_THIRD_PARTY_REVIEW_HANDOFF.md), [external review findings](review/third_party_review/V0914_EXTERNAL_REVIEW_FINDINGS_2026-06-01.md) |
| Review remediation | `present` | [external review remediation](review/third_party_review/V0914_EXTERNAL_REVIEW_REMEDIATION_2026-06-01.md), closed issue `#3368`, closed remediation issue `#3560` |
| Next-milestone planning | `present` | [NEXT_MILESTONE_HANDOFF_v0.91.4.md](NEXT_MILESTONE_HANDOFF_v0.91.4.md), closed issue `#3369`, merged PR `#3563` |
| Next-milestone review | `present` | [V0914_NEXT_MILESTONE_REVIEW_2026-06-01.md](review/next_milestone/V0914_NEXT_MILESTONE_REVIEW_2026-06-01.md), closed issue `#3370`, merged PR `#3565` |
| Closed issue/card truth | `present` | closed issue `#3564`; `bash adl/tools/check_milestone_closed_issue_sor_truth.sh --version v0.91.4` passed with `checked=95` |
| Release ceremony | `in_progress` | this packet, [RELEASE_READINESS_v0.91.4.md](RELEASE_READINESS_v0.91.4.md), [END_OF_MILESTONE_REPORT_v0.91.4.md](END_OF_MILESTONE_REPORT_v0.91.4.md), issue `#3371` |

## Core Proof Surfaces

| Surface | Status | Evidence |
| --- | --- | --- |
| Software Development Polis actor standing and shard ownership | `present` | [SOFTWARE_DEVELOPMENT_POLIS_PROOF_PACKET_v0.91.4.md](review/software_development_polis/SOFTWARE_DEVELOPMENT_POLIS_PROOF_PACKET_v0.91.4.md) |
| Merge readiness and GitHub truth | `present` | [MERGE_READINESS_GATE_PACKET_v0.91.4.md](review/merge_readiness/MERGE_READINESS_GATE_PACKET_v0.91.4.md) |
| ObsMem transition memory handoff | `present` | [OBSMEM_TRANSITION_MEMORY_PACKET_v0.91.4.md](review/obsmem_transition_memory/OBSMEM_TRANSITION_MEMORY_PACKET_v0.91.4.md) |
| Five-minute sprint repeatability | `present` | [FIVE_MINUTE_SPRINT_REPEATABILITY_REPORT_2026-05-27.md](FIVE_MINUTE_SPRINT_REPEATABILITY_REPORT_2026-05-27.md) |
| Process drift regression | `present` | [PROCESS_DRIFT_REGRESSION_REPORT_2026-05-28.md](PROCESS_DRIFT_REGRESSION_REPORT_2026-05-28.md) |
| PVF release policy | `present` | [PVF_CI_RELEASE_POLICY_v0.91.4.md](features/PVF_CI_RELEASE_POLICY_v0.91.4.md) |
| Minimal signed trace posture | `present_as_minimal_release_input` | [C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md](C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md), [EVIDENCE_CONVERGENCE_REVIEW_SYNTHESIS_AND_SIGNED_TRACE.md](features/EVIDENCE_CONVERGENCE_REVIEW_SYNTHESIS_AND_SIGNED_TRACE.md), remote request signing support in `adl/src/remote_exec/signing_support.rs` |

## Review And Remediation Chain

| Stage | Issue | Status | Evidence |
| --- | --- | --- | --- |
| Docs/adoption review | `#3365` | `closed` | [WP15 docs/adoption review](review/docs_adoption/WP15_DOCS_ADOPTION_REVIEW_2026-05-31.md) |
| Internal review | `#3366` | `closed` | [internal review synthesis](review/internal_review/V0914_INTERNAL_REVIEW_SYNTHESIS_2026-05-31.md) |
| External review | `#3367` | `closed` | [external review findings](review/third_party_review/V0914_EXTERNAL_REVIEW_FINDINGS_2026-06-01.md) |
| Remediation | `#3368` | `closed` | [external review remediation](review/third_party_review/V0914_EXTERNAL_REVIEW_REMEDIATION_2026-06-01.md) |
| Next-milestone planning | `#3369` | `closed` | [next-milestone handoff](NEXT_MILESTONE_HANDOFF_v0.91.4.md) |
| Next-milestone review | `#3370` | `closed` | [next-milestone review packet](review/next_milestone/V0914_NEXT_MILESTONE_REVIEW_2026-06-01.md) |
| Closeout truth sweep | `#3564` | `closed` | closed-issue SOR truth checker passed |
| Release ceremony | `#3371` | `in_progress` | this packet and companion readiness/report docs |

## v0.91.5 Routing

The following work is intentionally not claimed as complete in `v0.91.4`:

- first-birthday readiness and final preflight: `#3377`
- multi-agent stabilization and five-minute sprint proof: `#3415`, `#3501`,
  `#3503`, `#3504`
- provider/model matrix and provider breadth, including OpenRouter/DeepSeek
  planning: `#3502`, `#3505`, follow-on v0.91.5 provider issues
- public C-SDLC prompt records and `.adl` cleanup/archive: `#3472`-`#3476`
- demo readiness, including Unity/Observatory preparation: `#3455`, `#3460`,
  `#3461`
- AEE completion tranche and activation testing for v0.92: `#3526`, `#3534`,
  `#3377`, and the v0.91.5 activation map
- enterprise-security feature separation planning

## Non-Claims

This evidence packet does not claim:

- v0.92 first-birthday readiness
- production-grade multi-agent sprint execution
- Unity Observatory completion
- CodeFriend or WildClawBench product success as C-SDLC proof
- full runtime/polis observability
- enterprise-security codebase separation completion

## Validation Evidence

- `bash adl/tools/check_milestone_closed_issue_sor_truth.sh --version v0.91.4`
  passed with `checked=95`.
- Focused WP-21 docs validation is recorded in the issue `SOR`.
- Broad Rust validation is not rerun for this docs/release ceremony PR because
  the release-tail changes are documentation/evidence convergence only.

## Ceremony Decision

`go_for_release_after_wp21_merge`

The release evidence is sufficient for WP-21 to publish the release ceremony PR.
After that PR merges, the release manager may tag `v0.91.4` from `main` and
publish the GitHub Release from [RELEASE_NOTES_v0.91.4.md](RELEASE_NOTES_v0.91.4.md).

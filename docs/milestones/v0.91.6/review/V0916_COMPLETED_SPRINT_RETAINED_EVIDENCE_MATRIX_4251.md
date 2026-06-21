# v0.91.6 Completed Sprint Retained Evidence Matrix

Date: 2026-06-19
Updated: 2026-06-20 by `#4292` for `#4212` retained review coverage; by
`#4303` for `#4160`, `#4237`, and `#4250` retained review coverage.
Updated: 2026-06-20 by `#4357` to mark sprint-review truth defects that must
not be treated as clean completion.
Owner issue: `#4251`
Follow-up owners: `#4292`, `#4303`
Purpose: normalize reviewer-facing retained evidence for the completed
`v0.91.6` sprint umbrellas without rewriting historical packets or inventing
review artifacts that were never retained.

This matrix does not replace issue-local `.adl` lifecycle truth. It gives one
tracked review surface that answers three questions for each completed umbrella:

1. Is there a local task bundle now?
2. What tracked retained evidence should reviewers read first?
3. Is the retained packet already final, or is it a historical pre-closeout
   snapshot that now needs to be consumed with current-state caveats?

## Normalization Rules

- Treat live GitHub closure state and retained tracked evidence together.
- Do not invent missing pre-PR review packets after the fact.
- When the retained packet was authored before final merge/closure, mark it as
  a historical closeout-stage packet rather than pretending it was final.
- When no dedicated retained sprint-review packet was recovered, name the best
  available retained proof surface and keep the gap explicit.
- Local `.adl` bundle recovery performed during `#4251` is recorded here for
  reviewer awareness even though `.adl` remains ignored.

## Completed Umbrella Matrix

| Umbrella | Local bundle status at `#4251` audit | Primary retained evidence | Evidence posture after normalization | Notes |
| --- | --- | --- | --- | --- |
| `#3967` WP-02 resilience | present | `docs/milestones/v0.91.6/features/RESILIENCE_PERSISTENCE_SLEEP_WAKE_v0.91.6.md`; `docs/milestones/v0.91.6/review/provider/WP02_RESILIENCE_LAYER_INTEGRATION_PROOF_3993.md` | retained closeout evidence present | Child-wave and integration proof are retained; no separate umbrella review packet was recovered. |
| `#3968` WP-03 logging/tooling | present | `docs/milestones/v0.91.6/LOGGING_COMPLETION_LEDGER_v0.91.6.md`; `docs/milestones/v0.91.6/review/logging_observability/WP03_TOOLING_PROOF_LOOP_CLOSEOUT_4048.md` | retained closeout evidence present | Logging proof is retained through the closeout packet family rather than one umbrella review packet. |
| `#3969` WP-04 public prompt records | present | `docs/milestones/v0.91.6/features/PUBLIC_PROMPT_RECORDS_EXPORT_v0.91.6.md`; WP-04 closeout packet set under `docs/milestones/v0.91.6/review/public_prompt_records/` | retained closeout evidence present | Existing WP-04 closeout packet family is already reviewer-consumable. |
| `#3970` WP-05 provider/model reliability | present | `docs/milestones/v0.91.6/review/provider/WP05_PROVIDER_MINI_SPRINT_CLOSEOUT_3970.md` | retained closeout packet present | This is the strongest existing umbrella closeout packet in the completed set. |
| `#3971` WP-06 ACIP/A2A/provider communications | present | `docs/milestones/v0.91.6/features/ACIP_A2A_PROVIDER_COMMUNICATIONS_v0.91.6.md` under "WP-06 Protocol Decision Closeout Package" | retained closeout evidence present | Final umbrella proof is embedded in the feature doc rather than a standalone sprint review packet. |
| `#3972` WP-07 security bridge / CAV | present | `docs/milestones/v0.91.6/review/security/WP07_SECURITY_BRIDGE_CLOSEOUT_4024.md` | retained closeout packet normalized in `#4251` | The retained WP-07 packet now reads as post-closeout evidence instead of a still-open branch-time closeout step. |
| `#3973` WP-08 identity continuity / capability selector | present | `docs/milestones/v0.91.6/review/sprint_execution_packets/V0916_ACTIVE_SPRINT_EXECUTION_PACKETS_2026-06-18.md`; `docs/milestones/v0.91.6/review/security/UNITY_OBSERVATORY_INHABITANT_READINESS_SECURITY_REVIEW_4023.md`; `docs/milestones/v0.91.6/review/security/PUBLIC_RECORD_MEMORY_PROFILE_SECURITY_REVIEW_4022.md` | no standalone retained umbrella review packet recovered | Retained evidence exists through active-sprint and security-consuming packets, but no dedicated final umbrella review packet was recovered in this pass. The cited security packets include historical open-state wording and must be read as pre-closeout evidence, not current closure truth. |
| `#3975` WP-10 AEE / Memory / ObsMem / ACP | present | `docs/milestones/v0.91.6/features/AEE_MEMORY_ACP_BRIDGE_ACCOUNTING_v0.91.6.md`; `docs/milestones/v0.91.6/review/sprint_execution_packets/V0916_ACTIVE_SPRINT_EXECUTION_PACKETS_2026-06-18.md`; `docs/milestones/v0.91.6/review/security/PUBLIC_RECORD_MEMORY_PROFILE_SECURITY_REVIEW_4022.md` | no standalone retained umbrella review packet recovered | Retained proof is spread across the feature closeout matrix and consuming review packets; the absence of one final umbrella review packet remains explicit. The cited security packet includes historical open-state wording and must be read as pre-closeout evidence, not current closure truth. |
| `#4069` local-agent acceleration | present | `docs/milestones/v0.91.6/review/sprint_execution_packets/V0916_SEP_LOCAL_AGENT_ACCELERATION_MINI_SPRINT_4069.md` | retained umbrella packet normalized in `#4251` | The retained SEP now explicitly reads as a closed-sprint packet instead of a pending closeout step. |
| `#4141` flagship demo mini-sprint | recovered during `#4251` | none recovered | no standalone retained umbrella review packet recovered | The missing local bundle was reconstructed during `#4251`; reviewers must rely on closed GitHub issue state plus the recovered local bundle because no tracked retained review packet was recovered in this pass. |
| `#4149` workflow-control tools | present | `docs/milestones/v0.91.6/review/V0916_WORKFLOW_CONTROL_TOOLS_MINI_SPRINT_REVIEW_4149.md`; `docs/milestones/v0.91.6/review/sprint_execution_packets/V0916_SEP_WORKFLOW_CONTROL_TOOLS_MINI_SPRINT_4149.md`; `docs/milestones/v0.91.6/review/sprint_execution_packets/V0916_WORKFLOW_CONTROL_TOOLS_MINI_SPRINT_ACTIVITY_LOG_4149.md` | retained review packet normalized in `#4251` | The main review packet now explicitly reads as retained post-closeout evidence rather than a still-pending umbrella closeout step. |
| `#4158` current-model suitability mini-sprint | present after PR `#4289` | `docs/milestones/v0.91.6/review/provider/CURRENT_MODEL_SUITABILITY_MINI_SPRINT_REVIEW_4158.md`; `docs/milestones/v0.91.6/review/provider/CURRENT_MODEL_SUITABILITY_MINI_SPRINT_CLOSEOUT_4158.md` | retained review packet present | Nested umbrellas `#4095` and `#4154` are closed; `#4034` remains open and explicitly out of scope. |
| `#4160` ACIP runtime mini-sprint | present through `#4303` review cleanup | `docs/milestones/v0.91.6/review/V0916_ACIP_RUNTIME_MINI_SPRINT_REVIEW_4160.md`; `docs/milestones/v0.91.6/features/ACIP_A2A_PROVIDER_COMMUNICATIONS_v0.91.6.md`; `docs/milestones/v0.91.6/features/TOKIO_RUNTIME_SUBSTRATE_v0.91.6.md` | retained review packet present | `#4163` closure evidence is issue-comment-only rather than PR-merged; this remains an explicit caveat. |
| `#4212` validation manager / test-tax recovery mini-sprint | present through issue `#4292` review cleanup | `docs/milestones/v0.91.6/review/V0916_VALIDATION_MANAGER_TEST_TAX_MINI_SPRINT_REVIEW_4212.md`; `docs/milestones/v0.91.6/review/PVF_LONG_VALIDATION_LANE_INDEX_4223.md`; `docs/architecture/VALIDATION_LANE_SELECTOR.md` | retained review packet present | `#4212` is labeled `type:task` but functioned as a mini-sprint umbrella; `#4213` closure evidence is indirect because PR `#4227` closed unmerged while the inventory surface landed through later merged work. |
| `#4237` session-goal workflow hardening mini-sprint | present through `#4303` review cleanup | `docs/milestones/v0.91.6/review/V0916_SESSION_GOAL_WORKFLOW_HARDENING_MINI_SPRINT_REVIEW_4237.md`; `AGENTS.md`; `adl/tools/skills/pr-run/SKILL.md`; `adl/tools/skills/sprint-conductor/SKILL.md`; `adl/tools/skills/workflow-conductor/SKILL.md` | retained review packet present | Umbrella issue body still contains bootstrap `status: "draft"` metadata; child PR closure evidence is retained and current. |
| `#4250` completed-sprints review remediation mini-sprint | present through `#4303` review cleanup | `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINTS_REVIEW_REMEDIATION_MINI_SPRINT_REVIEW_4250.md`; `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_RETAINED_EVIDENCE_MATRIX_4251.md`; `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_REVIEW_FINDINGS_RESOLUTION_PLAN_4303.md` | retained review packet present | Folded issues `#4252`, `#4254`, and `#4256` are closed but need issue-comment hygiene for issue-local replacement visibility. |
| `#4177` Tokio runtime substrate | present | `docs/milestones/v0.91.6/features/TOKIO_RUNTIME_SUBSTRATE_v0.91.6.md`; `docs/milestones/v0.91.6/RUNTIME_FIRE_UP_PLAN_v0.91.6.md` | no standalone retained umbrella review packet recovered | Retained proof exists, but any remaining stale live-state wording in the feature/doc layer is residual records hygiene now that the milestone-doc truth lane `#4253` is closed. |
| `#4241` runtime resilience follow-on sprint | present | `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_REVIEW_TRUTH_REPAIR_4357.md` | issue-local review present; retained packet missing | `#4357` records that issue-local review truth was found in the ignored local task bundle, but no tracked retained review packet was found. Add a retained packet or keep this caveat visible before external review consumes the sprint. |
| `#4276` predictable execution fabric sprint | present | `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_REVIEW_TRUTH_REPAIR_4357.md` | closed; retained review record incomplete | `#4357` records that ignored local sprint/card surfaces still say review was not started, closeout readiness needed remediation, and SOR was stale. Do not consume this sprint as reviewed until a real retained sprint-review repair runs. This is a review-record defect, not an assessment of the sprint work itself. |
| `#4324` ADR mini-sprint | present | `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_REVIEW_TRUTH_REPAIR_4357.md` | closed but not consumable as completed sprint | The issue is closed, but current retained evidence is insufficient to treat it as completed ADR sprint work. Re-open/recreate execution before release-tail ADR claims depend on it. |
| `#4325` runtime AWS signal bridge mini-sprint | present | `docs/milestones/v0.91.6/review/runtime_aws_signal_bridge/RUNTIME_AWS_SIGNAL_BRIDGE_MINI_SPRINT_CLOSEOUT_4325.md`; `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_REVIEW_TRUTH_REPAIR_4357.md` | closeout present; review incomplete | The closeout packet is substantive, but no retained sprint-review packet was found and the issue-local SRP still says `not_run`. Either add a retained review packet or explicitly accept the closeout packet as the review surface. |

## Resolved In `#4251`

- Recovered the missing local task bundle for `#4141` with canonical source
  prompt plus `SIP/STP/SPP/SRP/SOR` bundle scaffolds.
- Added this retained-evidence matrix so completed-sprint reviewers no longer
  need to infer which packet family is authoritative for each umbrella.
- Normalized the retained packet posture for `#4069` and `#4149` so those
  tracked review surfaces no longer describe closeout as still pending after
  umbrella closure.

## Added In `#4292`

- Added a retained review packet for `#4212`, the validation-manager/test-tax
  recovery mini-sprint.
- Recorded the `#4212` label mismatch, the indirect `#4213` closure evidence,
  and the planning-rationale source-doc accounting as explicit review findings.
- Added caveats for `#3973` and `#3975` retained security packets whose content
  still contains historical open-state wording.

## Added In `#4303`

- Added retained review packets for `#4160`, `#4237`, and `#4250`.
- Added a findings-resolution plan for remaining review findings.
- Updated stale follow-up routing for `#4253` and `#4255` now that both
  remediation lanes are closed.

## Added In `#4357`

- Added `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_REVIEW_TRUTH_REPAIR_4357.md`.
- Marked `#4276` as closed with incomplete retained review truth, not reviewed.
- Marked `#4324` as closed but not consumable as completed ADR sprint work.
- Marked `#4325` as closeout-present but review-incomplete.
- Marked `#4241` as issue-local-review-present but missing a retained review
  packet.
- Reclassified open sprint umbrellas as ineligible for completed-sprint review:
  `#3974`, `#3976` through `#3984`, `#4310`, `#4332`, and `#4343`.

## Still Routed Elsewhere

- Local `.adl` lifecycle-card normalization for closed umbrellas remains local
  records hygiene, not tracked repo content by itself.
- `#4276` must receive a real retained sprint-review repair before it is
  consumed as reviewed.
- `#4324` must not be consumed as completed ADR sprint work until it is
  actually executed or recreated.
- `#4325` needs retained review normalization unless the operator explicitly
  accepts its closeout packet as the review surface.
- `#4241` needs retained review packet backfill or an explicit retained matrix
  caveat before external review consumes it.
- `#4253` closed the milestone-doc truth lane; any remaining stale issue-body
  metadata is now treated as residual records hygiene, not an open sprint
  blocker.
- `#4255` closed the typed GitHub issue-close transport wording lane; any
  future GitHub convergence work should route through the later projection
  convergence plan rather than this completed-sprints review matrix.
- Issue-comment hygiene for folded issues `#4252`, `#4254`, and `#4256`, plus
  the Cargo.lock lifecycle-tooling side effect, are tracked in
  `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_REVIEW_FINDINGS_RESOLUTION_PLAN_4303.md`.

## Non-Claims

- This matrix does not claim every completed sprint has a perfect standalone
  umbrella review packet.
- This matrix does not backfill fictional pre-PR reviews for umbrellas that
  closed through child proof plus closeout truth.
- This matrix does not make milestone-level docs current; it only normalizes
  the retained sprint-evidence view for reviewers.

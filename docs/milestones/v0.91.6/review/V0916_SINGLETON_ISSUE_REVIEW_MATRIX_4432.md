# v0.91.6 Singleton Issue Review Matrix

Date: 2026-06-23
Issue: `#4432`
Scope: closed `version:v0.91.6` issues only; excludes closed sprint umbrellas and mini-sprint umbrellas already handled by the retained sprint-evidence matrix.

## Findings

### No new open evidence-topology findings remain after retained review normalization
The initial draft identified two topology problems:

- `#4388` lacked a retained sprint-review packet even though its closed child
  lanes were already reviewable; and
- `#3966`, `#4095`, `#4105`, and `#4154` still needed explicit consumption as
  `mis_labeled_sprint_work` instead of being read as true singleton
  deliveries.

Both problems are now normalized in tracked evidence. `#4388` has a retained
sprint-review packet plus an explicit `#4396` routed reliability packet, and
the task-shaped sprint-management lanes remain classified as
`mis_labeled_sprint_work` rather than open singleton-review defects.

Evidence:
- `docs/milestones/v0.91.6/review/V0916_CSDLC_INTEGRATION_CONTROL_PLANE_SPRINT_REVIEW_4388.md`
- `docs/milestones/v0.91.6/review/V0916_CSDLC_CONTROL_PLANE_RELIABILITY_ROUTE_4396.md`
- `docs/milestones/v0.91.6/review/provider/CURRENT_MODEL_SUITABILITY_MINI_SPRINT_REVIEW_4158.md`
- `docs/milestones/v0.91.6/features/COGNITIVE_SCHEDULER_v0.91.6.md`

### No new code-level correctness findings were identified in sampled high-risk singleton work products
Focused code/docs/tests review over the high-risk singleton set did not uncover new correctness regressions. The main residual risk is evidence topology, not a newly discovered runtime or tooling defect in the sampled surfaces.

Sampled high-risk singleton work products: `#4047`, `#4049`, `#4106`, `#4107`, `#4111`, `#4190`, `#4262`, `#4286`, `#4306`, `#4322`, `#4356`, `#4378`, `#4405`, and `#4431`, plus the closed `#4388` child `#4398`.

## Method

- Started from the authoritative closed issue list returned by `adl/tools/pr.sh issue list --state closed --limit 500 --json` and filtered to issues carrying `version:v0.91.6`.
- Excluded 26 closed sprint umbrellas / mini-sprint umbrellas or umbrella-equivalent management issues already covered by `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_RETAINED_EVIDENCE_MATRIX_4251.md`.
- Used retained review packets first for routing evidence. For the `#4388`
  child set, this matrix now consumes the retained sprint-review packet and the
  retained `#4396` route packet instead of treating the singleton-review matrix
  as the only reviewer-facing evidence surface.
- Classified the remaining 216 closed non-umbrella issues into `true_singleton`, `sprint_child`, `mis_labeled_sprint_work`, `folded_duplicate`, or `already_covered`.
- Read retained review packets, feature docs, and code/test surfaces for the sampled high-risk singleton group rather than re-executing broad milestone validation.

## Population Summary

| Population | Count |
| --- | ---: |
| Closed `version:v0.91.6` issues returned by the repo-native issue list | 242 |
| Excluded closed umbrellas / mini-sprints / umbrella-equivalent management issues | 26 |
| Reviewed closed non-umbrella singleton or singleton-like issues | 216 |
| `true_singleton` | 37 |
| `sprint_child` | 166 |
| `mis_labeled_sprint_work` | 4 |
| `already_covered` | 6 |
| `folded_duplicate` | 3 |

## High-Risk Detailed Review Notes

| Issue | Review notes | Main evidence |
| --- | --- | --- |
| `#4047` | Projection-map ownership remains explicit and bounded. Closing-linkage helpers and live-body/live-closing-refs fallback tests are present; no new drift bug surfaced in the sampled code review. | `docs/milestones/v0.91.6/review/github_projection/GITHUB_CSDLC_PROJECTION_MAP_4047.md`; `adl/src/cli/pr_cmd/github/tests/closing_linkage.rs` |
| `#4049` | Owner-binary focused finish-validation support is issue-local tooling, not sprint-owned. No new regression surfaced from the sampled finish-support path. | `adl/src/cli/pr_cmd/finish_support.rs` |
| `#4106` | Scheduler economics inputs are implemented as a bounded deterministic contract and remain explicitly non-oracular. No new schema or determinism defect was found in the sampled review. | `adl/src/scheduler.rs`; `docs/milestones/v0.91.6/review/scheduler/SCHEDULER_ECONOMICS_INPUTS_4106.md` |
| `#4107` | Scheduler v1 remains bounded to deterministic planning output, not execution authority. The current retained packet and code still match that boundary. | `adl/src/scheduler.rs`; `docs/milestones/v0.91.6/review/scheduler/COGNITIVE_SCHEDULER_V1_4107.md` |
| `#4111` | Provider-profile V2 reconciliation remains a tracked reconciliation/documentation lane over the provider substrate rather than hidden TBD-only state. No new provider-profile correctness drift surfaced in the sampled code review. | `adl/src/provider/profiles.rs`; `docs/milestones/v0.91.6/review/provider/PROVIDER_PROFILES_V2_RECONCILIATION_4111.md` |
| `#4190` | Native merge-path repair remains a true singleton tooling fix. No retained review packet was found, but the current issue body and surrounding tooling evidence are consistent with a bounded one-off finish-path repair. | issue `#4190` body; `adl/src/cli/pr_cmd/finish_support.rs` context |
| `#4262` | The restaging guard behaves correctly for ignored local cards roots in sampled focused tests; no new regression surfaced. | `adl/src/cli/pr_cmd/finish_support.rs`; focused test `restage_finish_output_truth_paths*` |
| `#4286` | Rust/PVF closing-linkage ownership is now represented by typed helper tests and the projection packet lineage. No new fail-open path surfaced in sampled review. | `adl/src/cli/pr_cmd/github/tests/closing_linkage.rs`; `adl/src/bin/adl_pr_closing_linkage.rs` |
| `#4306` | Lockfile-discipline routing is still bounded and documented. The sampled surfaces did not reveal a new mutation-before-refusal regression. | `adl/tools/test_pr_run_locked_cargo_fallback_refuses_cleanly.sh`; `docs/milestones/v0.91.6/ADR_MINI_SPRINT_PACKET_v0.91.6.md` |
| `#4322` | The CI-cost review remains a singleton review/evidence lane rather than an umbrella. The retained packet is present and remains the main consumer surface. | `docs/milestones/v0.91.6/review/ADL_CI_VALIDATION_COST_REVIEW_4322.md` |
| `#4356` | Slug-drift binding remains a bounded bind-path tooling fix. No new contradictory evidence surfaced during classification review. | issue `#4356` body; `adl/src/cli/pr_cmd.rs` / `adl/src/control_plane.rs` routing context |
| `#4378` | Disposable-worktree child-bundle durability is backed by explicit regression tests that keep canonical child bundles in the primary lifecycle home. No new regression surfaced. | `adl/src/cli/tests/pr_cmd_inline/basics.rs` |
| `#4398` | FastContext evaluation is clearly a child of `#4388` and remains a defer-direct-adoption packet, not an implementation lane. The evaluation packet is detailed and bounded. | `docs/milestones/v0.91.6/review/context/FASTCONTEXT_EVALUATION_4398.md` |
| `#4405` | Session-coordination/root-checkout policy is present and clear. In the retained repo evidence sampled here it behaves as a bounded singleton documentation/policy lane rather than a consumed sprint child. | `docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md`; `AGENTS.md` |
| `#4431` | Goal-metrics restoration now has concrete repo-native recording scripts plus focused helper coverage. In the retained repo evidence sampled here it behaves as a bounded singleton tooling/metrics lane rather than a consumed sprint child. | `adl/tools/skills/sprint-conductor/scripts/issue_goal_metrics.py`; `adl/tools/skills/sprint-conductor/scripts/record_issue_goal_metrics.py`; `adl/tools/test_sprint_conductor_helpers.sh` |

## Focused Validation

- `cargo test --manifest-path adl/Cargo.toml closing_linkage_guard -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml restage_finish_output_truth_paths -- --nocapture`

## Appendix A: True Singleton Issues

| Issue | Title | Owner / route | Review posture | Primary retained evidence |
| --- | --- | --- | --- | --- |
| `#3902` | [v0.91.6] Create agent-logic.ai AWS account | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `docs/milestones/v0.91.6/review/AGENT_LOGIC_AWS_ACCOUNT_DECISION_RECORD_3902.md` |
| `#3922` | [v0.91.6][runtime-observability] Schedule runtime logging and observability completion | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `docs/milestones/v0.91.6/RUNTIME_OBSERVABILITY_COMPLETION_SCHEDULE_v0.91.6.md` |
| `#3925` | [v0.91.6][quality] Add repo-quality and documentation-staleness checks | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `docs/milestones/v0.91.6/review/REPO_QUALITY_STALENESS_REMEDIATION_3925.md` |
| `#3927` | [v0.91.6][tools] Add sprint-review skill for sprint and mini-sprint reviews | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `docs/milestones/v0.91.6/review/sprint_execution_packets/V0916_AGENT_PER_TASK_SPRINT_CONDUCTOR_SIMULATION_4074.md` |
| `#3934` | [v0.91.6][docs][templates] Create reusable repo README planning template | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `` |
| `#3935` | [v0.91.6][tools][csdlc] Converge GitHub PR truth on SOR and define card projection policy | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `docs/milestones/v0.91.6/review/CSDLC_GITHUB_PROJECTION_CONVERGENCE_REVIEW_3935.md` |
| `#3945` | [v0.91.6][tools] Replace historical manual GitHub provenance with ADL-native proof where useful | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `` |
| `#3946` | [v0.91.6][provider] Sanitize private LAN endpoint fixtures before durable proof packets | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `docs/milestones/v0.91.6/review/provider/PRIVATE_ENDPOINT_FIXTURE_SANITATION_4011.md` |
| `#3963` | [v0.91.6][tools] Make GitHub token loading deterministic and reusable | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `docs/milestones/v0.91.6/RUNTIME_OBSERVABILITY_COMPLETION_SCHEDULE_v0.91.6.md` |
| `#3965` | [v0.91.6][tools] Fix GitHub release publish for draft releases | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `docs/milestones/v0.91.6/RUNTIME_OBSERVABILITY_COMPLETION_SCHEDULE_v0.91.6.md` |
| `#3985` | [v0.91.6][tools] Fix ADL issue metadata repair for existing issues | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `docs/milestones/v0.91.6/RUNTIME_OBSERVABILITY_COMPLETION_SCHEDULE_v0.91.6.md` |
| `#3994` | [v0.91.6][tools] Add sprint and mini-sprint issue label taxonomy | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `` |
| `#4044` | [v0.91.6][tools][pr-finish] Classify resilience and provider communication paths into finish validation lanes | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `` |
| `#4047` | [v0.91.6][tools][projection] Complete GitHub/C-SDLC projection convergence tranche | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `detailed_review` | `docs/milestones/v0.91.6/review/github_projection/GITHUB_CSDLC_PROJECTION_MAP_4047.md` |
| `#4049` | [v0.91.6][tools][pr-finish] Execute owner-binary focused adl test selector chosen by finish validation plan | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `detailed_review` | `` |
| `#4051` | [v0.91.6][tools][observability] Share CLI env lock in observability tests to prevent full-suite heartbeat flake | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `` |
| `#4066` | [v0.91.6][templates][sprints] Add Sprint Execution Packet template | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `docs/milestones/v0.91.6/review/sprint_execution_packets/V0916_ACTIVE_SPRINT_EXECUTION_PACKETS_2026-06-18.md` |
| `#4074` | [v0.91.6][SEP][local-agents] Test agent-per-task sprint conductor simulation | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `docs/milestones/v0.91.6/review/sprint_execution_packets/V0916_AGENT_PER_TASK_SPRINT_CONDUCTOR_SIMULATION_4074.md` |
| `#4106` | [v0.91.6][scheduler][economics] Implement scheduler economics inputs | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `detailed_review` | `docs/milestones/v0.91.6/review/scheduler/SCHEDULER_ECONOMICS_INPUTS_4106.md` |
| `#4107` | [v0.91.6][scheduler] Implement Cognitive Scheduler v1 | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `detailed_review` | `docs/milestones/v0.91.6/review/scheduler/COGNITIVE_SCHEDULER_V1_4107.md` |
| `#4111` | [v0.91.6][provider][profiles] Implement provider profiles V2 reconciliation | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `detailed_review` | `docs/milestones/v0.91.6/review/provider/PROVIDER_PROFILES_V2_RECONCILIATION_4111.md` |
| `#4116` | [v0.91.6][review][resilience] Remediate remaining WP-02 closeout truth and provider-adapter defects | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `` |
| `#4121` | [v0.91.6][review][public-records] Remediate remaining WP-04 closeout truth drift | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `` |
| `#4126` | [v0.91.6][SEP][sprints] Backfill remaining sprint execution packets and durable notes | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `docs/milestones/v0.91.6/review/sprint_execution_packets/V0916_ACTIVE_SPRINT_EXECUTION_PACKETS_2026-06-18.md` |
| `#4129` | [v0.91.6][SEP][sprints] Normalize active sprint packet truth and mini-sprint metadata | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `docs/milestones/v0.91.6/review/sprint_execution_packets/V0916_ACTIVE_SPRINT_EXECUTION_PACKETS_2026-06-18.md` |
| `#4190` | [v0.91.6][tools][finish] Fix native merge path stalling on green draft PR validation | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `detailed_review` | `` |
| `#4196` | [v0.91.6][ci][runtime] Route manifest-only Rust PRs to bounded PR-fast validation | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `docs/milestones/v0.91.6/review/V0916_VPP_PVF_LANE_TEMPLATE_MINI_SPRINT_REVIEW_4332.md` |
| `#4229` | [v0.91.6][docs][agents] Document local GitHub token-file source | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `` |
| `#4262` | [v0.91.6][tools][pr-finish] Stop re-staging ignored local output cards during finish merge | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `detailed_review` | `` |
| `#4286` | [v0.91.6][tools][projection] Move PR closing-linkage guard into Rust/PVF | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `detailed_review` | `docs/milestones/v0.91.6/review/V0916_VPP_PVF_LANE_TEMPLATE_MINI_SPRINT_REVIEW_4332.md` |
| `#4306` | [v0.91.6][tools] Prevent lifecycle delegation from dirtying Cargo.lock before clean-root refusal | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `detailed_review` | `docs/milestones/v0.91.6/ADR_MINI_SPRINT_PACKET_v0.91.6.md` |
| `#4322` | [v0.91.6][ci][validation] Review adl-ci checks and reduce unnecessary validation cost | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `detailed_review` | `docs/milestones/v0.91.6/review/ADL_CI_VALIDATION_COST_REVIEW_4322.md` |
| `#4356` | [v0.91.6][tools] Fix pr run slug-drift identity binding | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `detailed_review` | `` |
| `#4368` | [v0.91.6][planning] Prepare v0.91.7 planning documents | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `classification_only` | `` |
| `#4378` | [v0.91.6][tooling] Preserve child issue bundles created from disposable worktrees | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `detailed_review` | `` |
| `#4405` | [v0.91.6][tools][coordination] Add session coordination and root checkout policy | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `detailed_review` | `docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md` |
| `#4431` | [v0.91.6][tools][metrics] Restore authoritative workflow time and token accounting | closed as a standalone implementation, closeout, or planning lane without retained sprint ownership evidence | `detailed_review` | `adl/tools/test_sprint_conductor_helpers.sh` |

## Appendix B: Closed Sprint Children

| Issue | Title | Owner / route | Review posture | Primary retained evidence |
| --- | --- | --- | --- | --- |
| `#3986` | [v0.91.6][WP-02][resilience][R-00] Architecture, schemas, and substrate foundation | #3967 | `classification_only` | `docs/milestones/v0.91.6/SPRINT_PLAN_v0.91.6.md` |
| `#3987` | [v0.91.6][WP-02][resilience][R-01] Implement retry policy with backoff, jitter, and budgets | #3967 | `classification_only` | `` |
| `#3988` | [v0.91.6][WP-02][resilience][R-02] Implement timeout, deadline, and cancellation policy | #3967 | `classification_only` | `` |
| `#3989` | [v0.91.6][WP-02][resilience][R-03] Implement circuit breaker policy | #3967 | `classification_only` | `` |
| `#3990` | [v0.91.6][WP-02][resilience][R-04] Implement rate limiting policy | #3967 | `classification_only` | `` |
| `#3991` | [v0.91.6][WP-02][resilience][R-05] Implement bulkhead and fault-domain isolation | #3967 | `classification_only` | `` |
| `#3992` | [v0.91.6][WP-02][resilience][R-06] Implement fallback and degraded execution policy | #3967 | `classification_only` | `docs/milestones/v0.91.6/review/provider/WP02_RESILIENCE_LAYER_INTEGRATION_PROOF_3993.md` |
| `#3993` | [v0.91.6][WP-02][resilience][R-07] Integrate and prove resilience layer coverage | #3967 | `classification_only` | `docs/milestones/v0.91.6/review/provider/WP02_RESILIENCE_LAYER_INTEGRATION_PROOF_3993.md` |
| `#3995` | [v0.91.6][WP-03][logging][L-00] Create logging completion ledger | #3968 | `classification_only` | `docs/milestones/v0.91.6/SPRINT_PLAN_v0.91.6.md` |
| `#3996` | [v0.91.6][WP-03][logging][L-01] Complete control-plane logging coverage | #3968 | `classification_only` | `docs/milestones/v0.91.6/review/logging_observability/CONTROL_PLANE_LOGGING_PROOF_3996.md` |
| `#3997` | [v0.91.6][WP-03][logging][L-02] Complete runtime and provider action logging | #3968 | `classification_only` | `docs/milestones/v0.91.6/review/logging_observability/RUNTIME_PROVIDER_LOGGING_PROOF_3997.md` |
| `#3998` | [v0.91.6][WP-03][logging][L-03] Complete heartbeat, timeout, and progress diagnostics | #3968 | `classification_only` | `docs/milestones/v0.91.6/review/logging_observability/HEARTBEAT_TIMEOUT_PROGRESS_PROOF_3998.md` |
| `#3999` | [v0.91.6][WP-03][logging][L-04] Complete OTel boundary and Observatory consumption proof | #3968 | `classification_only` | `docs/milestones/v0.91.6/review/logging_observability/OTEL_OBSERVATORY_CONSUMPTION_PROOF_3999.md` |
| `#4000` | [v0.91.6][WP-03][logging][L-05] Complete logging validation and redaction proof | #3968 | `classification_only` | `docs/milestones/v0.91.6/review/logging_observability/LOGGING_VALIDATION_REDACTION_PROOF_4000.md` |
| `#4001` | [v0.91.6][WP-03][tools][L-06] Complete GitHub, token, release, and projection observability | #3968 | `classification_only` | `docs/milestones/v0.91.6/review/logging_observability/GITHUB_TOKEN_RELEASE_PROJECTION_PROOF_4001.md` |
| `#4002` | [v0.91.6][WP-04][public-records][P-00] Define export shape and source-selection contract | #3969 | `classification_only` | `docs/milestones/v0.91.6/review/public_prompt_records/PUBLIC_PROMPT_RECORDS_EXPORT_CONTRACT_4002.md` |
| `#4003` | [v0.91.6][WP-04][public-records][P-01] Complete redaction and publication safety | #3969 | `classification_only` | `docs/milestones/v0.91.6/review/public_prompt_records/PUBLIC_PROMPT_RECORDS_REDACTION_PUBLICATION_SAFETY_4003.md` |
| `#4004` | [v0.91.6][WP-04][public-records][P-02] Complete validation and public indexing | #3969 | `classification_only` | `docs/milestones/v0.91.6/review/public_prompt_records/PUBLIC_PROMPT_RECORDS_VALIDATION_INDEXING_4004.md` |
| `#4005` | [v0.91.6][WP-04][public-records][P-03] Complete security review and CAV handoff | #3969 | `classification_only` | `docs/milestones/v0.91.6/review/public_prompt_records/PUBLIC_PROMPT_RECORDS_SECURITY_CAV_HANDOFF_4005.md` |
| `#4006` | [v0.91.6][WP-04][public-records][P-04] Complete distribution proof and closeout package | #3969 | `classification_only` | `docs/milestones/v0.91.6/review/public_prompt_records/PUBLIC_PROMPT_RECORDS_DISTRIBUTION_CLOSEOUT_4006.md` |
| `#4007` | [v0.91.6][WP-05][provider][M-00] Define provider and capability profile catalog | #3970 | `classification_only` | `docs/milestones/v0.91.6/review/provider/PROVIDER_CAPABILITY_PROFILE_CATALOG_4007.md` |
| `#4008` | [v0.91.6][WP-05][provider][M-01] Complete provider/model role suitability matrix | #3970 | `classification_only` | `docs/milestones/v0.91.6/review/provider/PROVIDER_ROLE_SUITABILITY_MATRIX_4008.md` |
| `#4009` | [v0.91.6][WP-05][provider][M-02] Complete Gemma and OpenRouter reliability proof | #3970 | `classification_only` | `docs/milestones/v0.91.6/review/provider/GEMMA_OPENROUTER_RELIABILITY_4009.md` |
| `#4010` | [v0.91.6][WP-05][provider][M-03] Complete provider failure-mode and resilience integration | #3970 | `classification_only` | `docs/milestones/v0.91.6/review/provider/PROVIDER_FAILURE_MODE_INTEGRATION_4010.md` |
| `#4011` | [v0.91.6][WP-05][provider][M-04] Complete private endpoint fixture sanitation | #3970 | `classification_only` | `docs/milestones/v0.91.6/review/provider/PRIVATE_ENDPOINT_FIXTURE_SANITATION_4011.md` |
| `#4012` | [v0.91.6][WP-05][provider][M-05] Complete provider/model reliability closeout matrix | #3970 | `classification_only` | `docs/milestones/v0.91.6/review/provider/PROVIDER_RELIABILITY_CLOSEOUT_MATRIX_4012.md` |
| `#4013` | [v0.91.6][WP-06][acip][C-00] Define communication schema catalog and profile boundaries | #3971 | `classification_only` | `docs/milestones/v0.91.6/SPRINT_PLAN_v0.91.6.md` |
| `#4014` | [v0.91.6][WP-06][acip][C-01] Define capability-based delegation and provider-message boundary | #3971 | `classification_only` | `docs/milestones/v0.91.6/review/security/ACIP_A2A_ACCESS_RULE_SECURITY_REVIEW_4021.md` |
| `#4015` | [v0.91.6][WP-06][acip][C-02] Define ACIP/A2A access rules and authority boundaries | #3971 | `classification_only` | `docs/milestones/v0.91.6/review/security/ACIP_A2A_ACCESS_RULE_SECURITY_REVIEW_4021.md` |
| `#4016` | [v0.91.6][WP-06][acip][C-03] Decide JSON protobuf and WebSocket projection boundaries | #3971 | `classification_only` | `docs/milestones/v0.91.6/review/security/ACIP_A2A_ACCESS_RULE_SECURITY_REVIEW_4021.md` |
| `#4017` | [v0.91.6][WP-06][acip][C-04] Define external-agent citizen guild and capability-market routing | #3971 | `classification_only` | `docs/milestones/v0.91.6/review/security/ACIP_A2A_ACCESS_RULE_SECURITY_REVIEW_4021.md` |
| `#4018` | [v0.91.6][WP-06][acip][C-05] Complete protocol decision closeout proof | #3971 | `classification_only` | `docs/milestones/v0.91.6/SPRINT_PLAN_v0.91.6.md` |
| `#4019` | [v0.91.6][WP-07][security][S-00] Create security bridge completion ledger | #3972 | `classification_only` | `docs/milestones/v0.91.6/SPRINT_PLAN_v0.91.6.md` |
| `#4020` | [v0.91.6][WP-07][security][S-01] Complete provider model and CAV trust-boundary review | #3972 | `classification_only` | `docs/milestones/v0.91.6/review/security/PROVIDER_MODEL_CAV_TRUST_BOUNDARY_REVIEW_4020.md` |
| `#4021` | [v0.91.6][WP-07][security][S-02] Complete ACIP A2A access-rule security review | #3972 | `classification_only` | `docs/milestones/v0.91.6/review/security/ACIP_A2A_ACCESS_RULE_SECURITY_REVIEW_4021.md` |
| `#4022` | [v0.91.6][WP-07][security][S-03] Complete public-record and memory-profile security review | #3972 | `classification_only` | `docs/milestones/v0.91.6/review/security/PUBLIC_RECORD_MEMORY_PROFILE_SECURITY_REVIEW_4022.md` |
| `#4023` | [v0.91.6][WP-07][security][S-04] Complete Unity Observatory inhabitant-readiness security review | #3972 | `classification_only` | `docs/milestones/v0.91.6/review/security/UNITY_OBSERVATORY_INHABITANT_READINESS_SECURITY_REVIEW_4023.md` |
| `#4024` | [v0.91.6][WP-07][security][S-05] Complete security bridge closeout proof | #3972 | `classification_only` | `docs/milestones/v0.91.6/review/security/WP07_SECURITY_BRIDGE_CLOSEOUT_4024.md` |
| `#4025` | [v0.91.6][WP-08][identity][I-00] Define identity capability citizen and profile boundary | #3973 | `classification_only` | `docs/milestones/v0.91.6/features/IDENTITY_CONTINUITY_CAPABILITY_SELECTOR_BRIDGE_v0.91.6.md` |
| `#4026` | [v0.91.6][WP-08][identity][I-01] Complete capability evidence ingestion boundary | #3973 | `classification_only` | `` |
| `#4027` | [v0.91.6][WP-08][identity][I-02] Complete identity continuity positive and negative cases | #3973 | `classification_only` | `` |
| `#4028` | [v0.91.6][WP-08][identity][I-03] Define capability selector MVP bridge | #3973 | `classification_only` | `` |
| `#4029` | [v0.91.6][WP-08][identity][I-04] Complete identity capability selector closeout proof | #3973 | `classification_only` | `` |
| `#4030` | [v0.91.6][WP-09][observatory][O-00] Define Unity Observatory implementation baseline | #3974 | `classification_only` | `docs/milestones/v0.91.6/review/observatory/UNITY_OBSERVATORY_IMPLEMENTATION_BASELINE_4030.md` |
| `#4031` | [v0.91.6][WP-09][observatory][O-01] Implement launchable Unity Observatory baseline | #3974 | `classification_only` | `docs/milestones/v0.91.6/DEMO_MATRIX_v0.91.6.md` |
| `#4032` | [v0.91.6][WP-09][observatory][O-02] Implement ADL evidence data contract for Observatory | #3974 | `classification_only` | `docs/milestones/v0.91.6/DEMO_MATRIX_v0.91.6.md` |
| `#4033` | [v0.91.6][WP-09][observatory][O-03] Implement inhabitant-readiness surfaces | #3974 | `classification_only` | `docs/milestones/v0.91.6/DEMO_MATRIX_v0.91.6.md` |
| `#4034` | [v0.91.6][WP-09][observatory][O-04] Complete logging OTel and security consumption proof | #3974 | `classification_only` | `docs/milestones/v0.91.6/review/observatory/UNITY_OBSERVATORY_LOGGING_OTEL_SECURITY_CONSUMPTION_4034.md` |
| `#4035` | [v0.91.6][WP-09][observatory][O-05] Complete working Unity Observatory closeout proof | #3974 | `classification_only` | `docs/milestones/v0.91.6/review/observatory/WP09_WORKING_UNITY_OBSERVATORY_CLOSEOUT_4035.md` |
| `#4036` | [v0.91.6][WP-10][memory][A-00] Create AEE ObsMem Memory Palace ACP completion ledger | #3975 | `classification_only` | `docs/milestones/v0.91.6/review/security/PUBLIC_RECORD_MEMORY_PROFILE_SECURITY_REVIEW_4022.md` |
| `#4037` | [v0.91.6][WP-10][memory][A-01] Complete AEE readiness and completion proof | #3975 | `classification_only` | `docs/milestones/v0.91.6/features/AEE_MEMORY_ACP_BRIDGE_ACCOUNTING_v0.91.6.md` |
| `#4038` | [v0.91.6][WP-10][memory][A-02] Complete Memory ObsMem handoff proof | #3975 | `classification_only` | `docs/milestones/v0.91.6/features/AEE_MEMORY_ACP_BRIDGE_ACCOUNTING_v0.91.6.md` |
| `#4039` | [v0.91.6][WP-10][memory][A-03] Plan and prove Memory Palace long-context bridge | #3975 | `classification_only` | `docs/milestones/v0.91.6/features/AEE_MEMORY_ACP_BRIDGE_ACCOUNTING_v0.91.6.md` |
| `#4040` | [v0.91.6][WP-10][memory][A-04] Complete ACP cognitive profile scope and privacy boundary | #3975 | `classification_only` | `docs/milestones/v0.91.6/features/AEE_MEMORY_ACP_BRIDGE_ACCOUNTING_v0.91.6.md` |
| `#4041` | [v0.91.6][WP-10][memory][A-05] Complete WP-10 feature closeout matrix | #3975 | `classification_only` | `docs/milestones/v0.91.6/features/AEE_MEMORY_ACP_BRIDGE_ACCOUNTING_v0.91.6.md` |
| `#4053` | [v0.91.6][WP-05][provider][M-06] Define C-SDLC role-provider profiles | #3970 | `classification_only` | `docs/milestones/v0.91.6/review/provider/ROLE_PROVIDER_PROFILES_4053.md` |
| `#4055` | [v0.91.6][WP-06][acip][C-06] Define Agent Comms 1.0 message substrate | #3971 | `classification_only` | `docs/milestones/v0.91.6/SPRINT_PLAN_v0.91.6.md` |
| `#4064` | [v0.91.6][WP-07][security][S-06] Define CAV threat taxonomy and corpus route | #3972 | `classification_only` | `docs/milestones/v0.91.6/review/security/CAV_THREAT_TAXONOMY_AND_CORPUS_ROUTE_4064.md` |
| `#4076` | [v0.91.6][SEP][sprints] Automate sprint readiness sweep | #4069 | `classification_only` | `docs/milestones/v0.91.6/review/sprint_execution_packets/V0916_AGENT_PER_TASK_SPRINT_CONDUCTOR_SIMULATION_4074.md` |
| `#4077` | [v0.91.6][SEP][sprints] Add deterministic sprint closeout mode | #4069 | `classification_only` | `docs/milestones/v0.91.6/review/sprint_execution_packets/V0916_AGENT_PER_TASK_SPRINT_CONDUCTOR_SIMULATION_4074.md` |
| `#4078` | [v0.91.6][github][octocrab] Add typed issue mutation command surface | #4069 | `classification_only` | `docs/milestones/v0.91.6/review/sprint_execution_packets/V0916_SEP_LOCAL_AGENT_ACCELERATION_MINI_SPRINT_4069.md` |
| `#4083` | [v0.91.6][tools][workflow] Doctor blocks new WP execution on unrelated open-wave PR pressure | #4149 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_WORKFLOW_CONTROL_TOOLS_MINI_SPRINT_REVIEW_4149.md` |
| `#4084` | [v0.91.6][SEP][closeout] Automate standard milestone closeout-tail sprint | #4149 | `classification_only` | `` |
| `#4085` | [v0.91.6][tools][pr-finish] Auto-declare non-closing lifecycle PRs when finish runs with --no-close | #4149 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_WORKFLOW_CONTROL_TOOLS_MINI_SPRINT_REVIEW_4149.md` |
| `#4086` | [v0.91.6][tools][pr-closeout] Ignore or quarantine stale broken worktrees when a clean issue closeout surface exists | #4149 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_WORKFLOW_CONTROL_TOOLS_MINI_SPRINT_REVIEW_4149.md` |
| `#4087` | [v0.91.6][tools][github-client] Inherit configured token context for repo-native closeout and other live GitHub operations | #4149 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_WORKFLOW_CONTROL_TOOLS_MINI_SPRINT_REVIEW_4149.md` |
| `#4088` | [v0.91.6][tools][workflow] Materialize the expected issue-local task bundle in fresh issue worktrees | #4149 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_WORKFLOW_CONTROL_TOOLS_MINI_SPRINT_REVIEW_4149.md` |
| `#4089` | [v0.91.6][provider][docs] Normalize provider sprint closeout truth after merge and closure | #3970 | `classification_only` | `` |
| `#4094` | [v0.91.6][tools] Harden pr finish publication truth for output-card and card-path routing | #4149 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_WORKFLOW_CONTROL_TOOLS_MINI_SPRINT_REVIEW_4149.md` |
| `#4096` | [v0.91.6][provider][deepseek] Run DeepSeek C-SDLC role suitability proof | #4095 | `classification_only` | `docs/milestones/v0.91.6/review/provider/CURRENT_MODEL_SUITABILITY_MINI_SPRINT_REVIEW_4158.md` |
| `#4097` | [v0.91.6][provider][agents] Define reusable C-SDLC agent suitability panel | #4095 | `classification_only` | `docs/milestones/v0.91.6/review/provider/C_SDLC_AGENT_SUITABILITY_PANEL_4097.md` |
| `#4109` | [v0.91.6][tools][aws-ssm] Review local polis SSM operations bridge | #4343 | `classification_only` | `docs/milestones/v0.91.6/review/security/LOCAL_POLIS_SSM_OPERATIONS_BRIDGE_4109.md` |
| `#4113` | [v0.91.6][tools][aws-ssm] Implement local polis SSM proof | #4343 | `classification_only` | `docs/milestones/v0.91.6/review/security/LOCAL_POLIS_SSM_PROOF_4113.md` |
| `#4136` | [v0.91.6][tools] Add permission-safe process status helper | #4149 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_WORKFLOW_CONTROL_TOOLS_MINI_SPRINT_REVIEW_4149.md` |
| `#4142` | [v0.91.6][demo][D00] Create demo mini-sprint inventory and SEP | #4141 | `classification_only` | `` |
| `#4143` | [v0.91.6][demo][D01] Refresh Starharvest browser playability proof | #4141 | `classification_only` | `` |
| `#4144` | [v0.91.6][demo][D02] Preserve D17 multi-agent workcell proof lane | #4141 | `classification_only` | `` |
| `#4145` | [v0.91.6][demo][D03] Prove Celestial Rescue Unity demo readiness | #4141 | `classification_only` | `` |
| `#4146` | [v0.91.6][demo][D04] Publish demo proof index and sprint closeout | #4141 | `classification_only` | `` |
| `#4155` | [v0.91.6][provider][openai] Run current direct-hosted OpenAI/Codex C-SDLC suitability proof | #4158 | `classification_only` | `docs/milestones/v0.91.6/review/provider/PROVIDER_ROLE_SUITABILITY_MATRIX_4008.md` |
| `#4156` | [v0.91.6][provider][anthropic] Run current direct-hosted Anthropic C-SDLC suitability proof | #4158 | `classification_only` | `docs/milestones/v0.91.6/review/provider/PROVIDER_ROLE_SUITABILITY_MATRIX_4008.md` |
| `#4157` | [v0.91.6][provider][gemini] Run current direct-hosted Gemini C-SDLC suitability proof | #4158 | `classification_only` | `docs/milestones/v0.91.6/review/provider/PROVIDER_ROLE_SUITABILITY_MATRIX_4008.md` |
| `#4162` | [v0.91.6][WP-06][acip][docs] Normalize merged WP-06 closeout truth in ACIP feature doc | #3971 | `classification_only` | `` |
| `#4163` | [v0.91.6][acip][runtime][R-00] Implement executable message schema and deterministic JSON substrate | #4160 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_REVIEW_FINDINGS_RESOLUTION_PLAN_4303.md` |
| `#4164` | [v0.91.6][acip][runtime][R-01] Implement local carrier and invocation execution path | #4160 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_ACIP_RUNTIME_MINI_SPRINT_REVIEW_4160.md` |
| `#4165` | [v0.91.6][acip][runtime][R-02] Implement authority and fail-closed access enforcement | #4160 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_ACIP_RUNTIME_MINI_SPRINT_REVIEW_4160.md` |
| `#4166` | [v0.91.6][acip][runtime][R-03] Add artifact refs and provider-boundary adapter | #4160 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_ACIP_RUNTIME_MINI_SPRINT_REVIEW_4160.md` |
| `#4167` | [v0.91.6][acip][runtime][R-04] Prove the first local multi-agent ACIP runtime slice | #4160 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_ACIP_RUNTIME_MINI_SPRINT_REVIEW_4160.md` |
| `#4178` | [v0.91.6][runtime][tokio][T-00] Expand Tokio runtime feature baseline | #4177 | `classification_only` | `docs/milestones/v0.91.6/features/TOKIO_RUNTIME_SUBSTRATE_v0.91.6.md` |
| `#4179` | [v0.91.6][runtime][tokio][T-01] Migrate long-lived agent cadence to Tokio timers and tasks | #4177 | `classification_only` | `docs/milestones/v0.91.6/features/TOKIO_RUNTIME_SUBSTRATE_v0.91.6.md` |
| `#4180` | [v0.91.6][runtime][tokio][T-02] Consolidate runtime bootstrap and supervision boundaries | #4177 | `classification_only` | `docs/milestones/v0.91.6/features/TOKIO_RUNTIME_SUBSTRATE_v0.91.6.md` |
| `#4181` | [v0.91.6][runtime][tokio][T-03] Prepare the ACIP-facing Tokio runtime substrate | #4177 | `classification_only` | `docs/milestones/v0.91.6/features/TOKIO_RUNTIME_SUBSTRATE_v0.91.6.md` |
| `#4182` | [v0.91.6][runtime][tokio][T-04] Define bounded CAV cadence integration on the shared runtime | #4177 | `classification_only` | `docs/milestones/v0.91.6/features/TOKIO_RUNTIME_SUBSTRATE_v0.91.6.md` |
| `#4183` | [v0.91.6][runtime][tokio][T-05] Complete Tokio runtime integration closeout proof | #4177 | `classification_only` | `docs/milestones/v0.91.6/RUNTIME_FIRE_UP_PLAN_v0.91.6.md` |
| `#4185` | [v0.91.6][runtime] Plan integrated runtime soak sprint | #4241 | `classification_only` | `docs/milestones/v0.91.6/RUNTIME_INTEGRATION_SOAK_SPRINT_v0.91.6.md` |
| `#4199` | [v0.91.6][tools][quality] Add manifest-driven validation lane selector | #4212 | `classification_only` | `` |
| `#4213` | [v0.91.6][tools][validation] Build test inventory and attribution report | #4212 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_VALIDATION_MANAGER_TEST_TAX_MINI_SPRINT_REVIEW_4212.md` |
| `#4214` | [v0.91.6][tools][validation] Create validation surface manifest | #4212 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_VALIDATION_MANAGER_TEST_TAX_MINI_SPRINT_REVIEW_4212.md` |
| `#4215` | [v0.91.6][tools][validation] Implement validation manager profiles | #4212 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_VALIDATION_MANAGER_TEST_TAX_MINI_SPRINT_REVIEW_4212.md` |
| `#4216` | [v0.91.6][tools][rust] Split issue and validation hot paths into small binaries | #4212 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_VALIDATION_MANAGER_TEST_TAX_MINI_SPRINT_REVIEW_4212.md` |
| `#4217` | [v0.91.6][tools][validation] Integrate validation manager with CI and pr finish | #4212 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_VALIDATION_MANAGER_TEST_TAX_MINI_SPRINT_REVIEW_4212.md` |
| `#4218` | [v0.91.6][tools][validation] Reduce ordinary Rust target multiplication | #4212 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_VALIDATION_MANAGER_TEST_TAX_MINI_SPRINT_REVIEW_4212.md` |
| `#4219` | [v0.91.6][tools][validation] Split slow-proof validation families | #4212 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_VALIDATION_MANAGER_TEST_TAX_MINI_SPRINT_REVIEW_4212.md` |
| `#4220` | [v0.91.6][tools][validation] Add validation growth guardrails | #4212 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_VALIDATION_MANAGER_TEST_TAX_MINI_SPRINT_REVIEW_4212.md` |
| `#4223` | [v0.91.6][tests][pvf] Define A/B validation lanes for long-test fan-out | #4212 | `classification_only` | `docs/milestones/v0.91.6/review/PVF_LONG_VALIDATION_LANE_INDEX_4223.md` |
| `#4225` | [v0.91.6][tools][pvf] Archive CI build logs to S3-backed evidence store | #4212 | `classification_only` | `docs/milestones/v0.91.6/review/ci_log_archive/CI_LOG_ARCHIVE_S3_CONTRACT_4225.md` |
| `#4231` | [v0.91.6][tools] Require goal creation for tracked issue sessions | #4237 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_SESSION_GOAL_WORKFLOW_HARDENING_MINI_SPRINT_REVIEW_4237.md` |
| `#4234` | [v0.91.6][planning] Route unaccounted active TBD documents | #4237 | `classification_only` | `docs/milestones/v0.91.6/review/planning/TBD_ACTIVE_DOC_ROUTING_4234.md` |
| `#4235` | [v0.91.6][tools] Shepherd tracked issue goals through green PR state and immediate closeout | #4237 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_SESSION_GOAL_WORKFLOW_HARDENING_MINI_SPRINT_REVIEW_4237.md` |
| `#4236` | [v0.91.6][tools] Auto-create session goals for Sprint Execution Packet work | #4237 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_SESSION_GOAL_WORKFLOW_HARDENING_MINI_SPRINT_REVIEW_4237.md` |
| `#4242` | [v0.91.6][runtime][tokio] Add async tracing and runtime health correlation | #4241 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_RUNTIME_RESILIENCE_FOLLOW_ON_SPRINT_REVIEW_4241.md` |
| `#4243` | [v0.91.6][runtime][resilience] Add shared timeout retry backpressure middleware | #4241 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_RUNTIME_RESILIENCE_FOLLOW_ON_SPRINT_REVIEW_4241.md` |
| `#4244` | [v0.91.6][runtime][concurrency] Add loom proof for runtime coordination races | #4241 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_RUNTIME_RESILIENCE_FOLLOW_ON_SPRINT_REVIEW_4241.md` |
| `#4245` | [v0.91.6][runtime][soak] Execute integrated runtime soak proof | #4241 | `classification_only` | `docs/milestones/v0.91.6/review/runtime/V0916_INTEGRATED_RUNTIME_SOAK_PROOF_4245.md` |
| `#4246` | [v0.91.6][runtime][continuity] Add checkpoint restore replay continuity slice | #4241 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_RUNTIME_RESILIENCE_FOLLOW_ON_SPRINT_REVIEW_4241.md` |
| `#4247` | [v0.91.6][runtime][autonomy] Add governed autonomous verification controls | #4241 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_RUNTIME_RESILIENCE_FOLLOW_ON_SPRINT_REVIEW_4241.md` |
| `#4248` | [v0.91.6][runtime][autonomy] Execute first bounded autonomous red blue proof | #4241 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_RUNTIME_RESILIENCE_FOLLOW_ON_SPRINT_REVIEW_4241.md` |
| `#4251` | [v0.91.6][review-fix][evidence] Repair sprint lifecycle and review evidence truth | #4250 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_RETAINED_EVIDENCE_MATRIX_4251.md` |
| `#4253` | [v0.91.6][review-fix][docs] Align milestone docs with completed sprint truth | #4250 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_REVIEW_FINDINGS_RESOLUTION_PLAN_4303.md` |
| `#4255` | [v0.91.6][tools][github] Fix typed issue close and transport wording | #4250 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_REVIEW_FINDINGS_RESOLUTION_PLAN_4303.md` |
| `#4257` | [v0.91.6][tools][usage] Add Codex usage watcher and reset warnings | #4276 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_PREDICTABLE_EXECUTION_FABRIC_SPRINT_REVIEW_4276.md` |
| `#4264` | [v0.91.6][tools][metrics] Capture issue goal token and time statistics | #4276 | `classification_only` | `docs/milestones/v0.91.6/features/PER_ISSUE_EXECUTION_METRICS_FOUNDATION_v0.91.6.md` |
| `#4277` | [v0.91.6][process][pvf] Assign PVF lane during issue creation and planning | #4276 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_VPP_PVF_LANE_TEMPLATE_MINI_SPRINT_REVIEW_4332.md` |
| `#4278` | [v0.91.6][process][metrics] Add SPP estimates and SOR actuals | #4276 | `classification_only` | `docs/milestones/v0.91.6/features/PER_ISSUE_EXECUTION_METRICS_FOUNDATION_v0.91.6.md` |
| `#4279` | [v0.91.6][process][metrics] Add variance analysis for estimate misses | #4276 | `classification_only` | `docs/milestones/v0.91.6/features/PER_ISSUE_EXECUTION_METRICS_FOUNDATION_v0.91.6.md` |
| `#4280` | [v0.91.6][observability][telemetry] Plan issue resource telemetry and S3 archive | #4276 | `classification_only` | `docs/milestones/v0.91.6/review/issue_resource_telemetry/ISSUE_RESOURCE_TELEMETRY_V1_AND_S3_ARCHIVE_PLAN_4280.md` |
| `#4281` | [v0.91.6][process][pvf] Add opportunistic lane-parallelization planning | #4276 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_VPP_PVF_LANE_TEMPLATE_MINI_SPRINT_REVIEW_4332.md` |
| `#4284` | [v0.91.6][aws][ddns] Implement Wuji dynamic IP Route 53 updater | #4343 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_RUNTIME_AWS_LOCAL_OPERATIONS_MINI_SPRINT_REVIEW_4343.md` |
| `#4294` | [v0.91.6][runtime][aws] Design runtime AWS signal bridge | #4325 | `classification_only` | `docs/milestones/v0.91.6/review/runtime_aws_signal_bridge/RUNTIME_AWS_SIGNAL_BRIDGE_DESIGN_4294.md` |
| `#4295` | [v0.91.6][runtime][aws] Implement AWS runtime heartbeat publisher | #4325 | `classification_only` | `docs/milestones/v0.91.6/review/runtime_aws_signal_bridge/RUNTIME_AWS_HEARTBEAT_PUBLISHER_PROOF_4295.md` |
| `#4296` | [v0.91.6][runtime][aws] Implement ACIP-to-SNS bridge | #4325 | `classification_only` | `docs/milestones/v0.91.6/review/runtime_aws_signal_bridge/RUNTIME_ACIP_SNS_BRIDGE_PROOF_4296.md` |
| `#4298` | [v0.91.6][observability][telemetry] Implement wuji issue resource telemetry collector | #4276 | `classification_only` | `docs/milestones/v0.91.6/review/build_throughput/SAFE_BUILD_ARTIFACT_CLEANUP_4314.md` |
| `#4299` | [v0.91.6][observability][telemetry] Add private archive and multi-host rollout for issue resource telemetry | #4276 | `classification_only` | `docs/milestones/v0.91.6/review/build_throughput/SAFE_BUILD_ARTIFACT_CLEANUP_4314.md` |
| `#4308` | [v0.91.6][pvf][templates] Add VPP validation planning prompt and externalized lane registry | #4332 | `classification_only` | `docs/milestones/v0.91.6/features/VPP_VALIDATION_PLANNING_AND_PVF_LANE_REGISTRY_v0.91.6.md` |
| `#4309` | [v0.91.6][templates] Plan next prompt-template version with VPP time token and goal fields | #4332 | `classification_only` | `docs/milestones/v0.91.6/features/VPP_VALIDATION_PLANNING_AND_PVF_LANE_REGISTRY_v0.91.6.md` |
| `#4329` | [v0.91.6][process][metrics] Define per-issue execution metrics foundation | #4332 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_VPP_PVF_LANE_TEMPLATE_MINI_SPRINT_REVIEW_4332.md` |
| `#4331` | [v0.91.6][csdlc][goals] Define first-class nested goal accounting | #4332 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_VPP_PVF_LANE_TEMPLATE_MINI_SPRINT_REVIEW_4332.md` |
| `#4311` | [v0.91.6][build] Evaluate and enable sccache for local Rust validation | #4310 | `classification_only` | `docs/milestones/v0.91.6/review/build_throughput/SCCACHE_LOCAL_VALIDATION_4311.md` |
| `#4312` | [v0.91.6][build] Evaluate faster Rust linker options | #4310 | `classification_only` | `docs/milestones/v0.91.6/review/build_throughput/RUST_LINKER_LOCAL_VALIDATION_4312.md` |
| `#4313` | [v0.91.6][build] Move Cargo target artifacts off the system volume | #4310 | `classification_only` | `docs/milestones/v0.91.6/review/build_throughput/CARGO_TARGET_DIR_RELOCATION_4313.md` |
| `#4314` | [v0.91.6][build] Add safe cleanup policy for stale build artifacts | #4310 | `classification_only` | `docs/milestones/v0.91.6/review/build_throughput/SAFE_BUILD_ARTIFACT_CLEANUP_4314.md` |
| `#4315` | [v0.91.6][build] Measure rebuild amplification and workspace hotspots | #4310 | `classification_only` | `docs/milestones/v0.91.6/review/build_throughput/BUILD_THROUGHPUT_MEASUREMENT_4315.md` |
| `#4316` | [v0.91.6][build][aws] Evaluate CodeBuild-hosted GitHub Actions runners for remote validation | #4310 | `classification_only` | `docs/milestones/v0.91.6/review/build_throughput/CODEBUILD_REMOTE_VALIDATION_EVALUATION_4316.md` |
| `#4317` | [v0.91.6][build][nessus] Set up Nessus as local remote Rust validation runner | #4310 | `classification_only` | `docs/milestones/v0.91.6/review/build_throughput/NESSUS_LOCAL_REMOTE_RUST_VALIDATION_RUNNER_4317.md` |
| `#4318` | [v0.91.6][runtime-aws][aws-ssm] Enroll nessus.local as local polis managed node | #4343 | `classification_only` | `docs/milestones/v0.91.6/review/security/LOCAL_POLIS_SSM_PROOF_4318.md` |
| `#4319` | [v0.91.6][runtime-aws][aws-ssm] Enroll opticon.local as local polis managed node | #4343 | `classification_only` | `docs/milestones/v0.91.6/review/security/LOCAL_POLIS_SSM_PROOF_4319.md` |
| `#4320` | [v0.91.6][runtime-aws][access] Prepare Codex login for nessus.local | #4343 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_RUNTIME_AWS_LOCAL_OPERATIONS_MINI_SPRINT_REVIEW_4343.md` |
| `#4321` | [v0.91.6][runtime-aws][access] Prepare Codex login for opticon.local | #4343 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_RUNTIME_AWS_LOCAL_OPERATIONS_MINI_SPRINT_REVIEW_4343.md` |
| `#4330` | [v0.91.6][aws][ddns] Install Wuji DDNS client and launchd automation | #4343 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_RUNTIME_AWS_LOCAL_OPERATIONS_MINI_SPRINT_REVIEW_4343.md` |
| `#4341` | [v0.91.6][WP-09][observatory][O-06] Rebuild the HTML Observatory as a mobile-capable governed surface | #3974 | `classification_only` | `docs/milestones/v0.91.6/review/observatory/HTML_MOBILE_GOVERNED_OBSERVATORY_PROOF_4341.md` |
| `#4369` | [v0.91.6][ADR][A-01] Review Local Polis SSM Operations Boundary candidate ADR | #4324 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_ADR_RELEASE_TAIL_MINI_SPRINT_REVIEW_4324.md` |
| `#4370` | [v0.91.6][ADR][A-02] Review Validation Lane Selector and PVF Test-Cost Policy candidate ADR | #4324 | `classification_only` | `` |
| `#4371` | [v0.91.6][ADR][A-03] Review GitHub/C-SDLC Projection Ownership candidate ADR | #4324 | `classification_only` | `` |
| `#4372` | [v0.91.6][ADR][A-04] Review Runtime Integration Soak Boundary candidate ADR | #4324 | `classification_only` | `` |
| `#4373` | [v0.91.6][ADR][A-05] Review Cognitive Scheduler v1 Authority Boundary candidate ADR | #4324 | `classification_only` | `` |
| `#4374` | [v0.91.6][ADR][A-06] Review Workflow Lockfile Discipline candidate ADR | #4324 | `classification_only` | `` |
| `#4375` | [v0.91.6][ADR][A-07] Review Provider/Model Suitability Boundary v2 candidate ADR | #4324 | `classification_only` | `` |
| `#4376` | [v0.91.6][ADR][A-08] Review Public Prompt Records Publication Boundary candidate ADR | #4324 | `classification_only` | `docs/milestones/v0.91.6/review/V0916_ADR_RELEASE_TAIL_MINI_SPRINT_REVIEW_4324.md` |
| `#4389` | [v0.91.6][tools][vpp] Make VPP the default validation planning surface | #4388 | `classification_only` | repo-native closed-issue list snapshot captured for `#4432`; issue body explicitly marks this issue as child work for `#4388` |
| `#4390` | [v0.91.6][tools][pvf] Externalize PVF lane selection and configuration | #4388 | `classification_only` | repo-native closed-issue list snapshot captured for `#4432`; issue body explicitly marks this issue as child work for `#4388` |
| `#4391` | [v0.91.6][tools][sep] Automate sprint execution packets end to end | #4388 | `classification_only` | repo-native closed-issue list snapshot captured for `#4432`; issue body explicitly marks this issue as child work for `#4388` |
| `#4392` | [v0.91.6][tools][metrics] Complete issue and sprint goal accounting | #4388 | `classification_only` | repo-native closed-issue list snapshot captured for `#4432`; issue body explicitly marks this issue as child work for `#4388` |
| `#4393` | [v0.91.6][tools][github] Finish Octocrab GitHub convergence | #4388 | `classification_only` | repo-native closed-issue list snapshot captured for `#4432`; issue body explicitly marks this issue as child work for `#4388` |
| `#4394` | [v0.91.6][tools][templates] Repair prompt-card template edge cases | #4388 | `classification_only` | repo-native closed-issue list snapshot captured for `#4432`; issue body explicitly marks this issue as child work for `#4388` |
| `#4395` | [v0.91.6][tools][runtime] Route runtime integration dependencies | #4388 | `classification_only` | repo-native closed-issue list snapshot captured for `#4432`; issue body explicitly marks this issue as child work for `#4388` |
| `#4398` | [v0.91.6][tools][context] Evaluate FastContext for C-SDLC workflows | #4388 | `detailed_review` | `docs/milestones/v0.91.6/review/context/FASTCONTEXT_EVALUATION_4398.md` |

## Appendix C: Mis-Labeled Sprint Work

| Issue | Title | Owner / route | Review posture | Primary retained evidence |
| --- | --- | --- | --- | --- |
| `#3966` | [v0.91.6][WP-01][planning] Promote v0.91.6 issue wave and schedule existing work | functioned as a sprint or nested-umbrella management lane despite singleton/task labeling | `classification_only` | `` |
| `#4095` | [v0.91.6][provider][agents] Set up reusable agent suitability testing | #4158 | `classification_only` | `docs/milestones/v0.91.6/review/provider/CURRENT_MODEL_SUITABILITY_MINI_SPRINT_REVIEW_4158.md` |
| `#4105` | [v0.91.6][scheduler] Build Cognitive Scheduler v1 | functioned as a sprint or nested-umbrella management lane despite singleton/task labeling | `classification_only` | `docs/milestones/v0.91.6/features/COGNITIVE_SCHEDULER_v0.91.6.md` |
| `#4154` | [v0.91.6][provider][agents] Run current direct-hosted frontier-model suitability proof wave | #4158 | `classification_only` | `docs/milestones/v0.91.6/review/provider/CURRENT_MODEL_SUITABILITY_MINI_SPRINT_REVIEW_4158.md` |

## Appendix D: Already Covered By Retained Review

| Issue | Title | Owner / route | Review posture | Primary retained evidence |
| --- | --- | --- | --- | --- |
| `#4048` | [v0.91.6][tools][WP-03] Close GitHub tooling proof-loop issue graph | closed review-retention lane already represented by tracked retained review packet | `covered_by_retained_review` | `docs/milestones/v0.91.6/review/logging_observability/WP03_TOOLING_PROOF_LOOP_CLOSEOUT_4048.md` |
| `#4292` | [v0.91.6][review][validation] Retain #4212 validation-manager sprint review evidence | closed review-retention lane already represented by tracked retained review packet | `covered_by_retained_review` | `docs/milestones/v0.91.6/review/V0916_VALIDATION_MANAGER_TEST_TAX_MINI_SPRINT_REVIEW_4212.md` |
| `#4303` | [v0.91.6][review] Complete retained reviews for remaining closed sprint umbrellas | closed review-retention lane already represented by tracked retained review packet | `covered_by_retained_review` | `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_REVIEW_FINDINGS_RESOLUTION_PLAN_4303.md` |
| `#4305` | [v0.91.6][review] Resolve retained review hygiene findings from #4303 | closed review-retention lane already represented by tracked retained review packet | `covered_by_retained_review` | `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_REVIEW_FINDINGS_RESOLUTION_PLAN_4303.md` |
| `#4357` | [v0.91.6][review] Repair completed-sprint review truth drift | closed review-retention lane already represented by tracked retained review packet | `covered_by_retained_review` | `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_REVIEW_TRUTH_REPAIR_4357.md` |
| `#4416` | [v0.91.6][review][observatory-adr] Repair WP-09 and ADR sprint review findings | closed review-retention lane already represented by tracked retained review packet | `covered_by_retained_review` | `docs/milestones/v0.91.6/review/V0916_ADR_CANDIDATE_CONTENT_REVIEW_4416.md` |

## Appendix E: Folded Or Duplicate Issues

| Issue | Title | Owner / route | Review posture | Primary retained evidence |
| --- | --- | --- | --- | --- |
| `#4252` | [v0.91.6][review-fix][folded] Recover missing 4141 task bundle | explicit fold/duplicate routing in issue body or title | `classification_only` | `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_REVIEW_FINDINGS_RESOLUTION_PLAN_4303.md` |
| `#4254` | [v0.91.6][review-fix][folded] Standardize sprint review closeout packets | explicit fold/duplicate routing in issue body or title | `classification_only` | `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_REVIEW_FINDINGS_RESOLUTION_PLAN_4303.md` |
| `#4256` | [v0.91.6][tools][folded] Correct ADL GitHub transport wording | explicit fold/duplicate routing in issue body or title | `classification_only` | `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_REVIEW_FINDINGS_RESOLUTION_PLAN_4303.md` |

# v0.91.7 Sprint Review Register

Status: active_review_register

Last updated: 2026-07-16

Issue: #5134

Current update: #5423

## Purpose

This register is the canonical v0.91.7 sprint-review status list. It records
what has been reviewed, what findings remain open, and what must happen before
the milestone can claim release readiness.

It does not close any WP by itself. A WP counts as release-ready only when its
implementation/proof work is complete, reviewed, remediated, and truthfully
closed out in the issue/card/PR surfaces.

## Current Summary

- WP-01, WP-02, and WP-04 have tracked review packets and are closed.
- WP-05 is closed and review-remediated by `#4932`; the local-agent proof
  artifact now agrees with the fail-closed provider/model identity guard.
- WP-06 is closed and review-remediated by `#4936`; selected build-throughput
  work and remote-builder follow-ons have landed and are recorded in the WP-06
  packet.
- WP-07 has substantial runtime review artifacts and its former CSM
  survival/post-blocker follow-ons are closed through the final #4906 and
  #5121 reconciliation wave, but the retained #4906 coherence gate remains
  `blocked_with_evidence`; final release readiness still requires explicit
  operator disposition or follow-on review of the blocking rows.
- WP-08 is closed with retained runtime AWS/signal proof and closeout truth;
  the later WP-07/WP-08 CSM governed-notice bridge tail `#4998` also merged
  and closed after adding retained control-plane notice proof.
  WP-09 through WP-20 are not yet release-review-clean.
- WP-21 is closed; WP-22 and WP-23 remain open for next-milestone review and
  release ceremony.
- Tools sprint #4806 is closed and review-remediated by #4961 for stale child
  card truth, tracked review evidence, and remaining sprint-conductor raw-`gh`
  helper paths. The former #4950 watcher closeout-state residual is now closed
  with settled-state proof retained in
  `docs/milestones/v0.91.7/review/V0917_WP03_CLOSEOUT_SETTLED_STATE_PROOF_4950.md`.
- Late sprint review reconciliation in #5134 records #5027, #5035, and #5036
  as closed on 2026-07-10 with retained sprint review or closeout evidence.
  These rows do not claim full milestone release readiness.
- Closed-sprint coverage review in #4649 adds the previously table-missing
  closed sprint umbrellas #4699, #4765, #4927, #5045, and #5121, and updates
  stale follow-up status for #4950, #4960, and #4907.
- Follow-up #5143 adds separate WP-07 and WP-08 findings review documents so
  fixed findings, remaining blockers, and non-claims are reviewable without
  mining sprint-chat context.
- WP-11 review in #4638 records the canonical reasoning graph, loop, skill,
  AEE/ObsMem, Godel, and GHB follow-on PRs as merged/closed while keeping the
  umbrella closeout blockers, stale child-card truth, retained-proof gap, and
  validation caveats visible.
- Remaining closed-sprint review #5403 adds ten retained findings-first packets,
  reconciles every declared child with its closing PR and merged revision, and
  routes 42 findings to #5404-#5413. Current disposition is 39 open, 2 partly
  fixed, and 1 fixed/superseded; these reviews do not claim remediation.
- Register reconciliation #5423 records the tools reliability tail #5036 as
  review-remediated after terminal #5406 records authority and terminal #5407
  issue-wave remediation. The other #5403 remediation rows remain unchanged
  until their own terminal retained evidence is available.

## Review Status Table

| WP | Umbrella | Status | Review Packet | Findings / Remediation | Next Action |
| --- | ---: | --- | --- | --- | --- |
| WP-01 | #4628 | closed | `docs/milestones/v0.91.7/review/V0917_WP01_PLANNING_PROMOTION_4628.md` | No active finding recorded in this register. | Keep as source truth for release-tail review. |
| WP-02 | #4629 | closed | `docs/milestones/v0.91.7/review/V0917_WP02_V0916_CLOSEOUT_TRUTH_CONSUMPTION_4661.md` | Child cleanup `#4661`-`#4665` and `#4699` are closed. | Keep as source truth for release-tail review. |
| WP-03 | #4630 | closed; review finding remains | `docs/milestones/v0.91.7/review/V0917_WP03_REVIEW_4972.md`; remediation packet `docs/milestones/v0.91.7/review/V0917_WP03_REVIEW_REMEDIATION_4953.md` | #4953 is closed and repaired the merged-PR/no-PR shepherd classifier confusion, stale #4713 residue, and WP-03 label discoverability. Current review still finds #4630 shepherd output reports `merged_needs_closeout`; its local readiness also fails in this review worktree because that worktree lacks the #4630 source prompt even though the root checkout retains it. Cross-cutting resilience/workflow issue #4780 is now closed via PR #5008. | Resolve the #4630 closeout-state and worktree-local readiness drift, or record it as an approved release-tail blocker before WP-03 is consumed as clean. |
| WP-04 | #4631 | closed | `docs/milestones/v0.91.7/review/V0917_WP04_CLOSEOUT_4631.md`; `docs/milestones/v0.91.7/review/V0917_WP04_CLOSEOUT_REMEDIATION_4747.md` | Remediation issue `#4747` is closed. | Keep metrics limitations visible; do not treat unknown metrics as zero. |
| WP-05 | #4632 | closed; review-remediated | `docs/milestones/v0.91.7/review/V0917_WP05_SCHEDULER_PROVIDER_LOCAL_AGENT_CLOSEOUT_4632.md` | #4932 is closed and repaired the stale `#4675` local-agent artifact after `#4849`; provider route and model suitability now both select Gemini while local Gemma remains shadow-only. | Keep as source truth unless new findings appear. |
| WP-06 | #4633 | closed; review-remediated | `docs/milestones/v0.91.7/review/V0917_WP06_BUILD_THROUGHPUT_VALIDATION_COST_REDUCTION_4633.md` | #4936 is closed and repaired review-truth records. The WP-06 packet now records the selected sprint lane plus reconciled remote-builder follow-ons: `#4837`, `#4838`, `#4879`, `#4680`, and `#4679`. | Keep build-throughput residual non-claims visible; paid AWS lanes remain explicit operator-triggered paths. |
| WP-07 | #4634 | closed umbrella; post-blocker coherence blocked with evidence | `docs/milestones/v0.91.7/review/V0917_WP07_FINDINGS_REVIEW_5143.md`; runtime review packets under `docs/milestones/v0.91.7/review/runtime/` plus `docs/milestones/v0.91.7/review/observability_4718/` | Runtime/OTel/Soak artifacts exist. The separate #5143 findings record captures the fixed quiet-mode OTel logging bug, fixed #4718 observability proof defects, and the remaining `#4906` `blocked_with_evidence` release-readiness rows. Former CSM survival/post-blocker follow-ons `#4906`, `#4910`, `#4911`, `#4918`, `#4919`, `#4921`, `#4922`, `#4929`, `#4933`, Chronosense follow-up `#5098`, and WP-07A topology sprint `#5121` are closed. | Keep #4906 blocking rows visible and run or record the final WP-07 release-readiness disposition before consuming WP-07 as clean. |
| WP-08 | #4635 | closed | `docs/milestones/v0.91.7/review/V0917_WP08_FINDINGS_REVIEW_5143.md`; `docs/milestones/v0.91.7/review/V0917_WP08_RUNTIME_AWS_SIGNAL_OPERATIONS_4635.md` | Runtime AWS/signal child issues `#4684`-`#4688`, `#4913`, `#4915`, and proof-hygiene follow-up `#5006` are closed with retained proof. The separate #5143 findings record captures fixed heartbeat, ACIP/SNS, local polis SSM, S3 archive classification, and #4998 AWS profile-binding findings. Adjacent WP-07/WP-08 CSM governed-notice bridge tail `#4998` / PR `#5016` is also closed/merged with `adl-ci` and `adl-coverage` green and repo-native closeout validation; its retained proof lives under `docs/milestones/v0.91.7/review/runtime/csm_governed_notice_4998/`. | Keep live AWS proof boundaries visible; do not claim broader WP-07 or release readiness from WP-08 alone. |
| WP-09 | #4636 | open | Unity review artifacts under `docs/milestones/v0.91.7/review/unity_observatory_*` | Umbrella and child issues `#4689`-`#4691` remain open. | Finish Observatory/demo proof and review as WP-09. |
| WP-10 | #4637 | closeout active; children merged and closed | `docs/milestones/v0.91.7/review/V0917_WP10_CURIOSITY_CONSTRUCTABILITY_REVIEW_4637.md` | Curiosity `#4692` / PR `#5161` and constructability `#4693` / PR `#5163` are merged and closed with Runtime v2 core, CLI, negative-case, and validation proof. CSM supervisor hosting remains correctly owned by WP-07A. | Finish #4637 through the repo-native PR and closeout lifecycle; preserve the CSM-hosting non-claim. |
| WP-11 | #4638 | open; child PRs merged; umbrella closeout review active | `docs/milestones/v0.91.7/review/V0917_WP11_REASONING_LOOPS_SKILL_REVIEW_4638.md` | Canonical children `#4694`-`#4697` are closed with merged PRs, and follow-ons `#4912`, `#5096`, and `#5136` are also closed. Review found the umbrella was not closeout-ready; the #4638 SRP/SOR have since been moved out of bootstrap/pre-review state and validated, stale #4638 review claims were released, and a retained Runtime v2 cognitive-control evidence packet was added at `docs/milestones/v0.91.7/review/V0917_WP11_RUNTIME_V2_COGNITIVE_CONTROL_EVIDENCE_4638.md`. Stale local child-card truth is explicitly superseded by that retained packet for WP-11 release consumption. | Finish #4638 through the normal lifecycle and keep the retained packet's non-claims visible; do not use stale local child cards as current release proof. |
| WP-12 | #4639 | closed; changes required | `docs/reviews/v0.91.7/remaining-sprints-5403/WP12_REVIEW_4639.md` | Five findings: 3 P1 and 2 P2. Security/protocol proof, activation-gate, validation-lane, and lifecycle findings are routed to #5404 and #5406. | Resolve #5404/#5406 before consuming WP-12 as review-clean. |
| WP-13 | #4640 | closed; changes required | `docs/reviews/v0.91.7/remaining-sprints-5403/WP13_REVIEW_4640.md` | Four findings: 1 P1 and 3 P2. Guild, Godel, economics, and lifecycle findings are routed to #5405 and #5406. | Resolve #5405/#5406 before consuming WP-13 as review-clean. |
| WP-14 | #4641 | open | none yet | Launch/birthday handoff children `#4758`-`#4763` remain open. | Finish launch and v0.92 birthday handoff proof. |
| WP-15 | #4642 | open | none yet | Demo matrix / proof coverage not yet review-clean. | Execute after implementation WPs are sufficiently proven. |
| WP-16 | #4643 | open | none yet | Quality gate not yet review-clean. | Execute after implementation/demo evidence is current. |
| WP-17 | #4644 | open | none yet | Docs/adoption review pass not yet complete. | Run docs alignment after WPs stabilize. |
| WP-18 | #4645 | open | none yet | Internal review not yet run for v0.91.7. | Run after WP-17. |
| WP-19 | #4646 | open | none yet | External review not yet run for v0.91.7. | Run after WP-18 remediation is ready. |
| WP-20 | #4647 | open | none yet | Review remediation not yet started. | Fix findings from WP-18/WP-19. |
| WP-21 | #4648 | closed; superseded planning with records finding | `docs/reviews/v0.91.7/remaining-sprints-5403/WP21_REVIEW_4648.md` | Two findings: the missed historical review gate is fixed/superseded for current planning consumption; durable lifecycle review evidence remains routed to #5406. Current v0.91.8 planning supersedes direct consumption of the old v0.92 candidate package. | Keep the historical boundary and resolve #5406; use current v0.91.8 planning authority. |
| WP-22 | #4649 | open | none yet | Next milestone review pass not yet complete. | Review v0.92 planning after WP-21/WP-20 truth is stable. |
| WP-23 | #4650 | open | none yet | Release ceremony not yet complete. | Run only after all required review/remediation gates are clean or explicitly blocked with operator approval. |

## Sprint Review Records

| Sprint | Status | Review / Remediation Packet | Findings / Residuals | Next Action |
| --- | --- | --- | --- | --- |
| Repo-native workflow stabilization | #4806 closed; review-remediated | `docs/milestones/v0.91.7/review/V0917_TOOLS_SPRINT_4806_REVIEW_REMEDIATION_4961.md`; `docs/milestones/v0.91.7/review/V0917_WP03_CLOSEOUT_SETTLED_STATE_PROOF_4950.md` | #4961 is closed and repaired stale child card truth, tracked release-visible review evidence, remaining sprint-conductor raw-`gh` helper paths, and owner-binary fallback wording. #4950 is now closed and retained as the settled-state proof for the former watcher `closeout_needed` ambiguity. | Keep #4950 proof visible as historical remediation evidence; no longer treat it as an open sprint residual. |
| WP-02 carryover cleanup mini-sprint | #4699 closed | `docs/milestones/v0.91.7/review/V0917_WP02_V0916_CLOSEOUT_TRUTH_CONSUMPTION_4661.md`; GitHub closeout comment on #4699 | #4699 closed on 2026-07-01 after #4661-#4665 and #4622 were dispositioned. The WP-02 packet is retained as the durable packet, but the sprint wrapper closeout comment is the current mini-sprint completion evidence. Non-claims remain: v0.92 readiness, production Observatory/Unity readiness, and ADR promotion are not claimed. | Keep the WP-02 child-disposition truth visible; do not use #4699 as a v0.92 activation proof. |
| Chronosense implementation sprint | #4765 closed; post-closeout-remediated | GitHub closeout comments on #4765; remediation issue #4807 | #4765 closed on 2026-07-03 after children #4766-#4771 closed with merged PRs and green required checks. Review found stale lifecycle truth and a continuity proof-artifact defect; #4807 closed the remediation. The original closeout also records a local VPP/closeout-prune tooling caveat that must not be mistaken for product/runtime incompletion. | Keep #4807 as the retained remediation route; do not reopen #4765 unless new Chronosense evidence defects appear. |
| Resilience integration mini-sprint | #4778 closed | `docs/milestones/v0.91.7/review/V0917_RESILIENCE_INTEGRATION_MINI_SPRINT_4778.md` | #4780, #4781, #4782, #4783, and #4784 are closed with retained workflow, provider/model, AWS/remote, runtime middleware, and failure-injection proof. PR #5014 and umbrella PR #5018 merged with `adl-ci` and `adl-coverage` green; explicit `pr.sh closeout 4778` passed after SRP/SOR truth repair. | Keep resilience non-claims visible; do not claim full product resilience beyond the retained proof surfaces. |
| Workflow tooling stabilization follow-up wave | #4927 closed; residual routed and closed | GitHub closeout comment on #4927; #5034 | #4927 closed after #4907, #4882, #4908, #4916, #4905, #4924, and #4848 were all closed. Its closeout routed legacy merged-PR/no-watcher-attachment closeout hygiene to #5034; #5034 is now closed and was later consumed by the #5036 tools reliability tail. #4825 remains intentionally excluded as Unity-specific work. | Keep #5034 as closed residual evidence; do not treat #4825 as part of this sprint wave. |
| Provider native adapters mini-sprint | #5027 closed; review-record finding fixed | `docs/milestones/v0.91.7/review/provider/PROVIDER_MINI_SPRINT_REVIEW_5027.md`; `docs/milestones/v0.91.7/review/provider/PROVIDER_MINI_SPRINT_CLOSEOUT_5027.md` | Children #5024, #5025, #5026, #5044, #4653, #4654, and #5075 are closed. The closeout packet accepts Z.ai GLM-5, AWS Bedrock Nova Pro, and Fable 5 rows while keeping DSpark Qwen/Gemma candidate-only and DeepSeek V4 Flash GPU smoke blocked by AWS quota/shape constraints. The stale ignored `.adl` placeholder review is superseded and no longer consumed as release-review evidence. | Keep provider acceptance non-claims visible; consume the tracked review and closeout packets instead of ignored local placeholders. |
| Rust tooling simplification wave | #5035 closed | `.adl/v0.91.7/sprints/issue-5035__v0-91-7-rust-sprint-execute-rust-tooling-simplification-wave/review/SPRINT_REVIEW_PACKET.md`; `.adl/v0.91.7/sprints/issue-5035__v0-91-7-rust-sprint-execute-rust-tooling-simplification-wave/review/SPRINT_REVIEW_SYNTHESIS.md`; `.adl/v0.91.7/sprints/issue-5035__v0-91-7-rust-sprint-execute-rust-tooling-simplification-wave/SPRINT_CLOSEOUT.md` | Review records no blocking sprint finding. All ordered children #4892-#4899 are closed out with merged PRs. Residual risk: sprint rollup artifacts are local `.adl` operational evidence and should not be mistaken for product-code deltas merged by the child PRs. | Keep the local-operational-evidence boundary visible; do not claim broad Rust tooling regression absence beyond child VPP/SOR proof. |
| Tools workflow reliability tail | #5036 closed; review-remediated | `docs/reviews/v0.91.7/remaining-sprints-5403/TOOLS_RELIABILITY_REVIEW_5036.md`; `docs/reviews/v0.91.7/tools-5407/TOOLS_RELIABILITY_CLOSEOUT_5036.md`; `docs/reviews/v0.91.7/csdlc-v2-5406/TERMINAL_AUTHORITY.md` | All four findings are fixed through terminal #5406/#5407 evidence: build-action logging claims are limited to the implemented validation-manager producer, current CLI taxonomy names typed C-SDLC v2 authority, the complete eleven-child closeout matrix is retained, and unsupported hosted-speedup claims are withdrawn. | Keep the narrowed logging and performance non-claims visible; require a new reviewed issue for broader producers, consumers, or material speedup claims. |
| WP-07 remaining CSM/runtime hardening follow-on sprint | #5045 closed; blocked with findings | `docs/reviews/v0.91.7/remaining-sprints-5403/WP07_HARDENING_REVIEW_5045.md` | Three P1 findings cover unauthenticated emergency stop, incomplete API Gateway proof, and the unresolved #4906 serial release gate. The later #5068 wave remains correctly owned by WP-07A. | Resolve #5408 and retain #4906 as blocking until an explicit release disposition exists. |
| WP-07A CSM runtime rearchitecture and topology sprint | #5121, #5409, and #5494 closed; corrective PR #5504 merged and typed closeout complete | `docs/reviews/v0.91.7/remaining-sprints-5403/WP07A_REARCHITECTURE_REVIEW_5121.md`; `docs/review-fixes/runtime/WP07A_REARCHITECTURE_REPAIR_5409.md`; `.csdlc/issues/5494/` | The corrective implementation reports the actual daemon-supervised-cycle model, derives readiness from all required CSM component and policy-required typed-channel observations, runs 100 supervised Runtime v3 task cycles through all real typed channels with injected failure/restart/recovery and retained lifecycle-journal sequence/readiness replay, and separately proves the unmodified production daemon entrypoint across real ticks plus failure/recovery. It also provides serialized bounded credential overlap for bearer and matching gateway signatures, expired-generation recovery, monotonic terminal revocation, authorization/revocation serialization, and listener-before-readiness startup ordering. Runtime v3 retains separate weather ownership. All twelve findings are fixed, exact-head review is clean, and required CI including hosted coverage passed in run `29647927552`. The umbrella remains setup/topology evidence only. | Consume #5504 and the #5494 terminal record as the completion evidence; preserve the Runtime v3 weather ownership and external-cloud non-claims. |
| Runtime v3 parity mini-sprint | #5174 closed; changes required | `docs/reviews/v0.91.7/remaining-sprints-5403/RUNTIME_V3_PARITY_REVIEW_5174.md` | Five findings: 2 P1, 2 P2, and 1 P3 covering executable topology, continuity authenticity, time authority, lifecycle retention, and stale footprint claims. | Resolve #5410/#5406; Runtime v3 remains opt-in. |
| Runtime v3 cutover sprint | #5227 closed; no default cutover | `docs/reviews/v0.91.7/remaining-sprints-5403/RUNTIME_V3_CUTOVER_REVIEW_5227.md` | Five findings: 4 P1 and 1 P2 covering the non-operational selector, unsupported parity, direct-PID guardian behavior, missing pressure-stop serialization, and evidence overstatement. | Resolve #5411/#5413/#5406; retain Runtime v2 as default and rollback. |
| Runtime v3 cutover readiness sprint | #5247 closed; changes required | `docs/reviews/v0.91.7/remaining-sprints-5403/RUNTIME_V3_READINESS_REVIEW_5247.md` | Five findings: 2 P1 and 3 P2 covering unauthenticated memory/private-state restoration, ignored production-like soak, LoC target exception, and lifecycle closeout. | Resolve #5412/#5406 before cutover acceptance. |
| Runtime v3 live parity and Observatory sprint | #5276 closed; changes required | `docs/reviews/v0.91.7/remaining-sprints-5403/RUNTIME_V3_LIVE_PARITY_REVIEW_5276.md` | Five findings: 3 P1 and 2 P2. Nine fixtures are not live equivalence, Observatory proof is mocked, remote feed reads lack authentication, weather is stale, and the release packet omits the child wave. | Resolve #5413. Observatory access is loopback/trusted-local only; remote exposure is prohibited until reviewed read authentication passes. |

## WP-05 Repair Record

Review found that the retained local-agent delegation artifact still combined a
ChatGPT provider route with Gemini model-suitability selection. That no longer
regenerated under the current scheduler because `#4849` correctly added
fail-closed provider/model identity validation.

This issue repairs the WP-05 proof surface by:

- updating `adl/tests/fixtures/scheduler/local_agent_delegation_readiness_inputs_v1.json`
  so the eligible provider route is `google/gemini-2.5-flash`;
- marking the previous ChatGPT route ineligible for this proof because it is
  not the cheapest validated outcome for the task;
- regenerating
  `docs/milestones/v0.91.7/review/provider/artifacts/local_agent_delegation_readiness_plan_4675.json`;
- preserving local Gemma as `shadow_only` advisory delegation, with no
  autonomous execution, repo mutation, closeout, or merge authority.

## Non-Claims

- This register does not claim v0.91.7 is release-ready.
- This register does not close any WP or child issue.
- This register does not claim WP-07 release readiness or later open WP
  findings are fixed.
- This register does not claim live provider invocation, live local model
  quality, or autonomous multi-agent authority from WP-05 scheduler artifacts.

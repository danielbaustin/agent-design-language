# v0.92 Planning Review And Source Dispositions

## Authority

- Review issue: `#5359` (`v0.91.8` WP-22)
- Package: `docs/milestones/v0.92/`
- Status: planning only; no v0.92 implementation issue is opened by this review
- Source inventory: operator-local `.adl/docs/TBD/` documents inspected on
  2026-08-04

## Review Result

The existing first-birthday sequence remains the product spine. The revised
candidate plan adds early reliability and workflow-tooling tracks, folds
contract work into its natural owner WP, separates consumer and demo work from
the Runtime API, retains a bounded cleanup tranche, and restores the complete
review and closeout tail.

## Blockers

- No blocker prevents publication of this planning review after its bounded
  pre-PR review passes.
- Opening the v0.92 issue wave remains blocked until v0.92 WP-01 rechecks live
  prerequisites, updates canonical version/docs surfaces, and creates the six
  lifecycle cards for each concrete issue.
- Runtime, distributed Guardian/polis, protocol, consumer, demo, cleanup, and
  article work remain unimplemented until their candidate WPs are opened and
  executed.

## Stale Assumptions Corrected

- The old package named five cards in some places; current authority requires
  `SIP`, `STP`, `SPP`, `VPP`, `SRP`, and `SOR`.
- The old package treated earlier milestone tranches as the immediate opening
  gate; WP-01 now consumes the final v0.91.8 release package and WP-22
  dispositions.
- The old ACIP row described a mock WebSocket proof; the revised WP requires
  reconciled ACIP/A2A semantics and a real authenticated full-duplex WSS
  exchange.
- The old sequence omitted the early canonical-docs, CI/coverage, Runtime
  reliability, distributed-runtime, workflow-efficiency, remote validation,
  and prompt-typing tracks.
- The Agent Logic repository migration was listed only as a merged planning
  input rather than an executable v0.92 WP. It is now WP-01A and must complete
  before substantive milestone work.
- The Medium article was incorrectly placed at the end of implementation. It
  now starts as an early living draft after WP-01A and is finalized by WP-23.

## Overclaims Removed

- Scheduling a WP is not implementation or acceptance proof.
- The merged `#5765` planning change does not perform the Agent Logic account
  migration.
- The Distributed Guardian/polis WP does not collapse its documented 16 child
  issues into one unreviewable change.
- The repository reduction WP cannot claim success from line count alone.
- The removed zero-byte Medium placeholder is not an authored article.

## WP-23 Disposition

v0.91.8 WP-23 (`#5348`) is `eligible_after_5359_merge`. WP-21 and WP-21A are
closed by merged PRs and ancestral to this review branch. The ceremony must
still wait for this exact reviewed planning package to merge; it may not absorb
planning repairs, implementation, or hidden remediation.

## Scheduled For v0.92

| Track | v0.92 placement | Source inputs | Disposition |
| --- | --- | --- | --- |
| Canonical docs and issue-wave readiness | WP-01 | current milestone package and v0.91.8 release truth | First issue; updates canonical docs and opens the reviewed wave. |
| Agent Logic GitHub organization repository migration | WP-01A | `AGENT_LOGIC_ACCOUNT_REPO_MIGRATION_PLAN.md` | Execute the six-repository serial transfer before substantive milestone work; retain `danielbaustin/asksifu` as personal. |
| CI and coverage reliability | WP-02 | current CI/coverage findings | Second issue; repairs the proving substrate before broad execution. |
| Runtime launch and resilience consolidation | WP-03 | `RUNTIME_V3_LAUNCH_AND_OBSERVATORY_RECOVERY_PLAN.md`; `resilience/RUNTIME_V3_LONG_LIVED_AGENT_OS_PLAN.md` | One bounded Runtime reliability WP. |
| Distributed Guardian/polis runtime | WP-04 | `CSM_RUNTIME_DISTRIBUTED_DESIGN.md`; `CSM_RUNTIME_DISTRIBUTED_EXECUTION_PLAN.md` | Execute the documented 16-issue program under one architecture/security-gated WP. |
| C-SDLC estimation and cycle-time reduction | WP-05 | `CSDLC_V2_SESSION_ESTIMATION_RECONNECTION_PLAN.md`; `workflow_tooling/planning/SPRINT_CYCLE_TIME_REDUCTION_PLAN.md` | One workflow-efficiency WP. |
| Remote validation/build runner | WP-06 | `workflow_tooling/planning/REMOTE_BUILD_RUNNER_PILOT_PLAN.md` | Schedule the explicitly unscheduled pilot. |
| Prompt-card enum typing | WP-07 | `workflow_tooling/planning/V0917_PROMPT_CARD_ENUM_TYPING_PLAN.md` | Verify prior delivery first; implement only the proven remainder. |
| ACIP and A2A reconciliation | WP-14 | `acip/AGENT_COMMUNICATION_AND_INVOCATION_PROTOCOL.md`; `a2a/ADL_A2A_ADAPTER.md` | Fold into the ACIP schema/carrier WP rather than creating a competing protocol lane. |
| Observatory/Unity consumer integration | WP-18A | `OBSERVATORY_UNITY_DESIGN.md` | Separate consumer WP; the Observatory remains outside the Runtime API implementation. |
| Provider-neutral multi-agent proof | WP-18B | `multiagent_demos/` | Provider-neutral proof tranche after the birthday review packet is available. |
| Repository-wide code reduction remainder | WP-21 | `ADL_REPOSITORY_CODE_REDUCTION_PLAN_v0.91.8.md` | Bounded cleanup only; preserve behavior and prove the deletion denominator. |
| Medium launch article | WP-24 | former empty `publication/medium_launch_articles/1-WHY-ADL.md` placeholder | Start a real living draft immediately after WP-01A, maintain it as evidence lands, and finalize claims in WP-23. |

## Explicitly Deferred

| Track | Source inputs | Disposition | Retention rule |
| --- | --- | --- | --- |
| Capability taxonomy integration | `capability_testing/ADL_CAPABILITY_TAXONOMY.md` | Later capability milestone; not required for v0.92 birthday execution. | Keep the source indexed and require the later milestone to name it explicitly. |
| MLX local provider and OCI model packaging | `MLX_APPLE_METAL_PROVIDER_PLAN.md`; `OCI_MODEL_PACKAGING_METHOD_PLAN.md` | Later provider/model-distribution track. | Keep both sources paired as one future track. |
| Agent Logic website and investor publication | `AGENT_LOGIC_WEBSITE_DESIGN_v2.1.md`; `AGENT_LOGIC_INVESTOR_MATERIAL_TRACKING_AND_PUBLICATION_PLAN.md` | Separate Agent Logic launch backlog, outside the ADL v0.92 WBS. | Preserve both source pointers in the v0.92 issue-wave backlog section. |
| General-intelligence paper program | `general-intelligence-paper/` | Later research/publication milestone. | Preserve the source cluster as one program rather than fragmenting it into ADL product WPs. |

## Retired Or Provenance-Only Inputs

- Superseded designs, completed review artifacts, historical gap reviews, and
  one-off command captures were moved under `.adl/docs/TBD/retired/`.
- `OPUS_REVIEW_RUNBOOK.md` remains active and was not retired.
- The zero-byte Medium article placeholder was removed; its v0.92 WP owns a
  substantive article rather than file preservation.

## Sequencing Rules

1. WP-01 updates canonical docs and opens only the reviewed issue wave.
2. WP-01A executes and verifies the Agent Logic repository migration before substantive milestone work.
3. WP-24 begins its living draft after WP-01A; WP-23 owns final release-truth alignment.
4. WP-02 repairs CI/coverage before it is credited as milestone proof.
5. WP-03 through WP-07 may proceed in parallel where their dependencies allow.
6. The birthday product spine starts at WP-08 and retains identity,
   continuity, memory, capability, profile, protocol, witness, review, and
   negative-proof boundaries.
7. Distributed Runtime work has an architecture/security gate and explicit
   child issues; it is not treated as one oversized implementation commit.
8. Observatory/Unity remains a consumer of the versioned Runtime API.
9. WP-21 cleanup is behavior-preserving and cannot substitute deletion counts
   for product proof.
10. The release tail is quality, docs and article finalization, internal review, external
   review, remediation, next-milestone planning, closeout planning,
   next-milestone review, and ceremony.

## Non-Claims

- This review does not open or execute v0.92 issues.
- Deferred sources are not silently accepted as delivered.
- The Distributed Guardian/polis program is scheduled, not completed.
- The Medium article is scheduled, not authored.
- No legal-personhood, production-citizenship, consciousness, or v0.93
  constitutional-governance claim is introduced.

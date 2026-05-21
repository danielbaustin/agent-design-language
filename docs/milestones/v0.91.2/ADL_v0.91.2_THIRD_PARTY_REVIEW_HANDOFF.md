# Third-Party Review Handoff - v0.91.2

## Metadata

- Milestone: `v0.91.2`
- Version: `v0.91.2`
- Active crate version: `0.91.2`
- Review lane: `WP-21` external / 3rd-party review, following `WP-20`
  internal review and `WP-20B` corrective full internal review
- Prepared during: `v0.91.2` review tail
- Prepared for issue: `#3020`
- Controlling internal review source: `#3173`
- Current packet status: ready to begin `WP-21` external / 3rd-party review;
  accepted `WP-20B` remediation issues `#3175` through `#3179` are closed
- Date: 2026-05-21
- Publication attempted: false
- Release approval claimed: false
- Review approval claimed: false

## Review Entry Check

This handoff is the canonical tracked third-party review surface for `v0.91.2`,
and it has been refreshed from clean `main` after accepted `WP-20B` remediation
issues closed.

Before review, confirm:

- `README.md`, `CHANGELOG.md`, `adl/README.md`, `adl/Cargo.toml`,
  `adl/Cargo.lock`, and `docs/milestones/v0.91.2/` agree on `v0.91.2`.
- The `WP-20B` full internal review packet is the controlling internal review
  surface, not the earlier thin `WP-20` packet.
- accepted `WP-20B` remediation issues `#3175`, `#3176`, `#3177`, `#3178`,
  and `#3179` are closed
- `WP-22` remediation routing remains visible for all accepted `WP-20B`
  findings
- any future findings from `WP-21` external review are routed through `WP-22`
  remediation before release closeout
- No host-local paths, temporary paths, worktree-only artifacts, raw credential
  paths, or branch-only claims appear in the packet.
- UTS benchmark claims are still bounded by the current benchmark-methodology
  findings unless those findings have been fixed and reviewed.

This tracked handoff should stand on its own. Operator-local archives may exist,
but the external reviewer should not need hidden `.adl` notes, scratch files, or
private workstation paths to understand the milestone truth.

## Purpose

This handoff gives a third-party reviewer a bounded `v0.91.2` review packet.
Review `v0.91.2` as the pressure-release and productization milestone after
`v0.91.1`.

The review should cover:

- UTS + ACC multi-model benchmark work and its evidence boundaries
- runtime test-cycle recovery and CI/runtime-budget cleanup
- review heuristics and demo cleanup
- Google Workspace bridge planning and optionality boundaries
- CodeFriend productization docs and naming cleanup
- speculative decoding prototype planning
- repo-visibility follow-on work
- rustdoc and documentation cleanup
- general-intelligence paper packet handling
- publication program planning
- workflow guardrails, card-lifecycle corrections, and sprint-conductor lessons
- release-tail readiness, evidence, and non-claims

The reviewer should produce evidence-backed findings with severity, location,
impact, and recommended remediation. The reviewer should not rewrite docs,
perform remediation, create release tags, merge PRs, close issues, or run the
release ceremony.

## Current Milestone Truth

`v0.91.2` is active and the crate version is `0.91.2`.

The milestone has landed substantial implementation, documentation, planning,
and review work, but it is not release-ready. The release-tail quality gate is
still `NOT_READY` until external review, remediation, next-milestone planning,
and release ceremony work finish.

Current review-tail truth:

- `WP-01` through `WP-19` are recorded as closed in the milestone readiness
  docs.
- `WP-17A` is also recorded as closed as a bounded follow-on demo pass.
- The first `WP-20` internal review packet was too thin and must not be used as
  the sole review handoff.
- `WP-20B` / `#3173` produced the controlling full internal review packet.
- Accepted `WP-20B` remediation issues `#3175` through `#3179` have closed.
- `WP-21` external review can start from this refreshed handoff packet.
- `WP-22` remains the route for any findings accepted from the external review.
- `WP-23` remains next-milestone planning.
- `WP-24` remains release ceremony.

This handoff does not claim that the milestone is ready to close.

## Controlling Internal Review Packet

The controlling internal review packet is:

- `docs/milestones/v0.91.2/review/internal_review_full/README.md`
- `docs/milestones/v0.91.2/review/internal_review_full/SYNTHESIS_REPORT.md`
- `docs/milestones/v0.91.2/review/internal_review_full/FINDINGS_REGISTER.md`
- `docs/milestones/v0.91.2/review/internal_review_full/SPECIALIST_COVERAGE.md`
- `docs/milestones/v0.91.2/review/internal_review_full/REVIEW_TO_TEST_PLAN.md`
- `docs/milestones/v0.91.2/review/internal_review_full/FINDING_TO_ISSUE_PLAN.md`
- `docs/milestones/v0.91.2/review/internal_review_full/REDACTION_AND_EVIDENCE_AUDIT.md`
- `docs/milestones/v0.91.2/review/internal_review_full/REVIEW_QUALITY_EVALUATION.md`
- `docs/milestones/v0.91.2/review/internal_review_full/DIAGRAM_PLAN.md`

The older `docs/milestones/v0.91.2/review/internal_review/` packet is historical
context only. It should be treated as superseded for readiness and handoff
truth unless a later remediation document explicitly says otherwise.

## Active Findings And Remediation Routing

The `WP-20B` synthesis found no `P0` findings, but it did find `P1` and `P2`
risks that affect benchmark credibility, hosted-provider portability, security
hygiene, and review handoff truth.

Most important accepted findings:

- the governed Rust UTS + ACC benchmark can pass wrong task arguments
- hosted benchmark defaults include operator-local key-file paths
- canonical benchmark profiles can reference models absent from the canonical
  panel
- the older `WP-20` packet can mislead reviewers unless `WP-20B` is made
  controlling

Recommended remediation routing from the full review packet:

- Benchmark validity blocker: `P1-1`, `P1-3`, `P2-2`, `P2-3`, `P2-4`
- Hosted provider security and portability: `P1-2`, `P2-5`, `P2-6`, `P2-7`,
  `P3-1`
- Evidence and handoff truth: `P1-4`, `P2-8`, `P2-11`, `P3-3`, `P3-4`
- Tooling and supply-chain hygiene: `P2-9`, `P2-10`, `P3-2`
- Provider native-tool capability reporting: `P2-1`

Closed child remediation issues:

- `#3175`: benchmark scoring and failure gates: closed
- `#3176`: hosted benchmark adapter and artifact hardening: closed
- `#3177`: make `WP-20B` the controlling review packet: closed
- `#3178`: CI pinning and validation reproducibility: closed
- `#3179`: provider native-tool capability reporting: closed

The external reviewer should treat those accepted `WP-20B` findings as
remediated for review-entry purposes, while still reviewing the resulting
implementation, evidence, and non-claims independently.

## Previous Review Mistakes To Avoid

- Do not send stale version truth. Root docs, crate metadata, lockfile truth,
  and milestone docs must agree on `v0.91.2`.
- Do not use the thin `WP-20` packet as the controlling handoff after `WP-20B`.
- Do not claim branch-only artifacts, local `.adl` notes, temporary files, or
  workstation paths as release truth.
- Do not treat internal-review completion as release readiness.
- Do not treat UTS benchmark results as public superiority claims; review the
  remediated benchmark methodology and evidence boundaries directly.
- Do not hide docs-only, planning-only, fixture-only, or demo-only boundaries.
- Do not mix ADL-governed UTS + ACC evidence with standalone UTS claims without
  labeling the boundary.
- Do not imply Google Workspace bridge work is required for C-SDLC or the core
  ADL release line.
- Do not claim release ceremony, release approval, or v0.91.3/v0.91.4
  completion from this review lane.

## Required Review Scope

Review these top-level repository surfaces first:

- `README.md`
- `CHANGELOG.md`
- `adl/README.md`
- `adl/Cargo.toml`
- `adl/Cargo.lock`
- `docs/README.md`
- `docs/planning/ADL_FEATURE_LIST.md`

Review these `v0.91.2` milestone surfaces next:

- `docs/milestones/v0.91.2/README.md`
- `docs/milestones/v0.91.2/VISION_v0.91.2.md`
- `docs/milestones/v0.91.2/DESIGN_v0.91.2.md`
- `docs/milestones/v0.91.2/DECISIONS_v0.91.2.md`
- `docs/milestones/v0.91.2/ADR_PLAN_v0.91.2.md`
- `docs/milestones/v0.91.2/WBS_v0.91.2.md`
- `docs/milestones/v0.91.2/SPRINT_v0.91.2.md`
- `docs/milestones/v0.91.2/SPRINT_CONDUCTOR_EXECUTION_PLAN_v0.91.2.md`
- `docs/milestones/v0.91.2/WP_ISSUE_WAVE_v0.91.2.yaml`
- `docs/milestones/v0.91.2/WP_EXECUTION_READINESS_v0.91.2.md`
- `docs/milestones/v0.91.2/CARD_BUNDLE_READINESS_v0.91.2.md`
- `docs/milestones/v0.91.2/SPP_READINESS_v0.91.2.md`
- `docs/milestones/v0.91.2/CI_RUNTIME_BUDGETS_v0.91.2.md`
- `docs/milestones/v0.91.2/DEMO_MATRIX_v0.91.2.md`
- `docs/milestones/v0.91.2/FEATURE_PROOF_COVERAGE_v0.91.2.md`
- `docs/milestones/v0.91.2/QUALITY_GATE_v0.91.2.md`
- `docs/milestones/v0.91.2/MILESTONE_CHECKLIST_v0.91.2.md`
- `docs/milestones/v0.91.2/RELEASE_PLAN_v0.91.2.md`
- `docs/milestones/v0.91.2/RELEASE_READINESS_v0.91.2.md`
- `docs/milestones/v0.91.2/RELEASE_EVIDENCE_v0.91.2.md`
- `docs/milestones/v0.91.2/RELEASE_NOTES_v0.91.2.md`
- `docs/milestones/v0.91.2/NEXT_MILESTONE_HANDOFF_v0.91.2.md`
- `docs/milestones/v0.91.2/END_OF_MILESTONE_REPORT_v0.91.2.md`
- `docs/milestones/v0.91.2/features/README.md`
- `docs/milestones/v0.91.2/ADL_v0.91.2_THIRD_PARTY_REVIEW_HANDOFF.md`

Review these feature docs where they intersect milestone claims:

- `docs/milestones/v0.91.2/features/UTS_ACC_MULTI_MODEL_BENCHMARK.md`
- `docs/milestones/v0.91.2/features/RUNTIME_TEST_CYCLE_RECOVERY.md`
- `docs/milestones/v0.91.2/features/REVIEW_HEURISTICS_AND_DEMOS.md`
- `docs/milestones/v0.91.2/features/GOOGLE_WORKSPACE_CMS_BRIDGE.md`
- `docs/milestones/v0.91.2/features/CODEFRIEND_PRODUCTIZATION.md`
- `docs/milestones/v0.91.2/features/SPECULATIVE_DECODING_PROTOTYPE.md`
- `docs/milestones/v0.91.2/features/REPO_VISIBILITY_FOLLOW_ON.md`
- `docs/milestones/v0.91.2/features/RUSTDOC_DOC_CLEANUP.md`
- `docs/milestones/v0.91.2/features/GENERAL_INTELLIGENCE_PAPER_PACKET.md`
- `docs/milestones/v0.91.2/features/PUBLICATION_PROGRAM.md`
- `docs/milestones/v0.91.2/features/WORKFLOW_GUARDRAILS.md`

Review these evidence and review surfaces:

- `docs/milestones/v0.91.2/review/internal_review_full/`
- `docs/milestones/v0.91.2/review/uts_acc_multi_model_benchmark_report.json`
- `docs/milestones/v0.91.2/review/provider_native_tool_call_comparison_report.json`
- `docs/milestones/v0.91.2/review/uts_remote_open_model_evidence_memo_2026-05-20.md`
- `docs/milestones/v0.91.2/review/runtime_test_cycle_recovery_report.md`
- `docs/milestones/v0.91.2/review/coverage_gate_ergonomics_report.md`
- `docs/milestones/v0.91.2/review/sprint_1_closeout_truth_cleanup_3105.md`

Review these implementation and tooling surfaces if a claim depends on
executable behavior:

- `adl/src/uts_acc_multi_model_benchmark.rs`
- `adl/src/provider_substrate.rs`
- `adl/src/provider_native_tool_call_comparison.rs`
- `adl/tools/uts_benchmark_runner.py`
- `adl/tools/benchmark/`
- `adl/tools/run_uts_benchmark.sh`
- `adl/tools/test_uts_benchmark_runner_contracts.sh`
- `adl/tools/demo_v0912_quality_gate.sh`
- `adl/tools/real_chatgpt_gemini_claude_provider_adapter.py`
- `.github/workflows/`

If a listed path does not exist in the branch under review, report that as a
scope or handoff drift finding rather than assuming it exists elsewhere.

## Proof And Quality Evidence To Check

The external review should verify:

- version truth is consistent across root, crate, lockfile, and milestone docs
- `WP-20B` is the controlling internal review packet
- `WP-22` remediation issues cover the accepted findings without hiding scope
- benchmark claims are supported by scoring logic, task-argument checks,
  profile/model consistency, and non-zero failure gates
- hosted-provider docs and configs do not expose operator-local credential
  paths or assume a private machine layout
- benchmark reports and evidence do not persist unsafe raw excerpts or absolute
  outside paths
- release readiness remains honest while `WP-21` through `WP-24` are unfinished
- GWS, C-SDLC, UTS, ACC, CodeFriend, and publication-program claims stay in
  their correct lanes
- docs distinguish completed work, planned work, evidence, caveats, and
  non-claims

## Suggested Validation Commands

Run only the validation needed for the review question. Suggested focused
commands:

```sh
cargo test --manifest-path adl/Cargo.toml uts_acc -- --nocapture
python3 adl/tools/benchmark/deterministic_self_check.py
bash adl/tools/test_uts_benchmark_runner_contracts.sh
bash adl/tools/demo_v0912_quality_gate.sh
```

If a command is skipped, fails because of environment assumptions, or produces
provider-dependent output, record that as review evidence rather than smoothing
it over.

## Expected Reviewer Output

Expected reviewer output:

- severity-ranked findings with file and line references
- explicit non-findings for surfaces that are clean
- a recommendation on whether `WP-22` remediation is sufficient to proceed
- a list of findings that must be fixed before `WP-24` release ceremony
- a list of findings that may be explicitly deferred without misleading the
  release
- clear notes on any claims that depend on local-only evidence, branch-only
  artifacts, or unresolved provider behavior

## Non-Claims

This handoff does not claim:

- release readiness
- release ceremony completion
- external review completion
- accepted-finding remediation completion
- public UTS benchmark superiority
- provider/model conformance beyond the evidence reviewed
- production-grade hosted-provider adapter security
- Google Workspace bridge adoption
- C-SDLC completion
- v0.91.3 or v0.91.4 readiness
- publication approval for papers, articles, or investor-facing materials

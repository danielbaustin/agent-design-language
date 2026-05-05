# Changelog

All notable project-level changes are summarized here by milestone/release.

## v0.91 (Active development)

Status: Active. The v0.91 issue wave is open as `#2735-#2759`, and the crate
version has advanced to `0.91.0` for the moral governance, cognitive-being,
structured planning / SRP, and secure intra-polis Agent Comms development line.

Planning notes:
- The tracked v0.91 planning package lives under `docs/milestones/v0.91/`.
- The active issue wave is recorded in
  `docs/milestones/v0.91/WP_ISSUE_WAVE_v0.91.yaml`.
- The first SPP-readiness slice is recorded in
  `docs/milestones/v0.91/SPP_READINESS_v0.91.md`.
- This is not a release entry; v0.90.5 remains the most recently completed
  release line.

## v0.90.5 (Released 2026-05-05)

Status: Released. The Governed Tools v1.0 issue wave opened as `#2566-#2591`
and is now fully closed. Internal review, third-party review, explicit
no-remediation disposition, next-milestone handoff, and release ceremony are
complete.

Summary:
- `v0.90.5` is the Governed Tools v1.0 milestone.
- The milestone landed the tool-call threat model, `UTS` public-compatible
  schema and conformance, `ACC` authority/visibility/privacy/delegation
  surfaces, deterministic registry/compiler/normalization/policy/executor
  behavior, trace/replay/redaction evidence, dangerous negative proofs, bounded
  model-proposal benchmark and local/Gemma evaluation, and the flagship
  governed-tools demo.
- The first landed ACIP / Comms tranche also shipped in this milestone:
  protocol architecture, canonical envelope and identity shape,
  invocation/Freedom Gate linkage, conformance fixtures, review/coding
  specialization, and proof coverage.
- WP-20 records the canonical quality and coverage posture; WP-23 external
  review returned zero findings; WP-25 leaves the v0.91 package prepared with a
  reviewed candidate issue-wave YAML.
- The crate version for the completed release line remains `0.90.5`.

References:
- `docs/milestones/v0.90.5/README.md`
- `docs/milestones/v0.90.5/WBS_v0.90.5.md`
- `docs/milestones/v0.90.5/SPRINT_v0.90.5.md`
- `docs/milestones/v0.90.5/DEMO_MATRIX_v0.90.5.md`
- `docs/milestones/v0.90.5/FEATURE_PROOF_COVERAGE_v0.90.5.md`
- `docs/milestones/v0.90.5/QUALITY_GATE_v0.90.5.md`
- `docs/milestones/v0.90.5/RELEASE_READINESS_v0.90.5.md`
- `docs/milestones/v0.90.5/RELEASE_EVIDENCE_v0.90.5.md`
- `docs/milestones/v0.90.5/RELEASE_NOTES_v0.90.5.md`
- `docs/milestones/v0.90.5/END_OF_MILESTONE_REPORT_v0.90.5.md`

Not claimed in v0.90.5:
- `UTS` alone as execution authority
- arbitrary shell, network, or secret-bearing execution from model output
- payment rails, billing, legal contracting, or inter-polis economics
- full `v0.91` moral/cognitive-being substrate
- full `v0.91.1` adjacent-systems package

## v0.90.4 (Completed milestone)

Status: The bounded citizen economics and contract-market milestone is
complete. Its detailed package, external review packet, and ADR boundary remain
recorded under `docs/milestones/v0.90.4/`.

## v0.90.3 (Released 2026-04-23)

Status: Released. The issue wave opened on 2026-04-21 as #2327-#2347 and is now
fully closed. Internal review, external review, accepted-finding disposition,
next-milestone handoff, and release ceremony are complete.

Scope:
- v0.90.3 is the citizen-state substrate milestone.
- The milestone turned v0.90.2 bounded CSM run evidence into protected
  continuity substrate work: canonical private state, signed envelopes,
  local-first sealing, append-only lineage, continuity witnesses and receipts,
  anti-equivocation, sanctuary/quarantine behavior, redacted Observatory
  projections, standing and access-control semantics, challenge/appeal flow,
  threat modeling, and one integrated citizen-state proof demo.
- WP-14A landed the explicit demo matrix and feature-proof coverage lane before
  WP-15 quality/docs/review convergence.
- WP-15 adds `docs/milestones/v0.90.3/RELEASE_READINESS_v0.90.3.md` as the
  reviewer entry surface, records coverage/tracker truth, and keeps docs-only CI
  path-policy skips separate from full release coverage evidence.
- WP-19 leaves the v0.90.4 economics package, the v0.90.5 governed-tools
  package, and the downstream v0.91/v0.92 boundaries ready to start cleanly
  after v0.90.3 ceremony.
- The crate version is `0.90.3` for the released v0.90.3 line.

Not claimed in v0.90.3:
- first true Gödel-agent birthday
- full v0.91 moral, emotional, kindness, humor, or wellbeing substrate
- full v0.92 identity/capability rebinding, migration, or birthday scope
- full citizen economics, contract markets, payment rails, or inter-polis trade
- mandatory cloud enclave deployment

## v0.90.2 (Release closeout)

Status: Implementation, demo/proof coverage, docs convergence, internal
review, external review, accepted-finding remediation, next-milestone handoff,
and WP-20 release ceremony preflight are complete. Final tag/release
publication runs from clean main after the closeout PR merges.

Summary:
- v0.90.2 is the first bounded CSM run and Runtime v2 hardening milestone.
- The issue wave opened as #2245-#2264, with WP-14A added as #2301 to restore
  the explicit demo matrix and feature-proof coverage lane before WP-15.
- The milestone now has a code-backed CSM run packet contract, invariant and
  violation artifacts, boot/admission evidence, governed resource scheduling,
  Freedom Gate mediation, invalid-action rejection, snapshot/rehydrate/wake
  continuity, Observatory packet/report integration, recovery eligibility,
  quarantine evidence, governed adversarial hardening probes, and an integrated
  bounded first-run demo packet.
- WP-14A landed the D11 feature-proof coverage record, so every v0.90.2 feature
  claim has a runnable demo command, test-backed proof packet, fixture-backed
  artifact, documented non-proving status, or explicit deferral before review
  convergence.
- WP-15 reviewed the active coverage, gap-status, and Rust module watch
  trackers: coverage remains above the active gates at `92.40%`, the remaining
  gap risk is release-tail and lifecycle-record verification, and the Rust
  tracker was refreshed after the v0.90.2 refactoring pass.
- WP-16 internal review findings were fixed by #2317, #2318, #2319, and #2320.
  WP-17 external review found zero P0/P1/P2/P3 findings, and WP-18 closed with
  only optional backlog candidates for milestone-compression documentation and
  a Runtime v2 overview diagram.
- WP-19 polished the v0.90.3 planning package, restored the v0.87.1-style
  quality/review/release tail, and left v0.90.3 ready for fast issue-wave
  execution.
- WP-20 release ceremony preflight passed, including the closed-issue SOR truth
  gate across 40 closed v0.90.2 issues.
- The crate version is `0.90.2` for the v0.90.2 release line.

References:
- `docs/milestones/v0.90.2/README.md`
- `docs/milestones/v0.90.2/WBS_v0.90.2.md`
- `docs/milestones/v0.90.2/SPRINT_v0.90.2.md`
- `docs/milestones/v0.90.2/DEMO_MATRIX_v0.90.2.md`
- `docs/milestones/v0.90.2/FEATURE_DOCS_v0.90.2.md`
- `docs/milestones/v0.90.2/FEATURE_PROOF_COVERAGE_v0.90.2.md`
- `docs/milestones/v0.90.2/RELEASE_READINESS_v0.90.2.md`
- `docs/milestones/v0.90.2/RELEASE_EVIDENCE_v0.90.2.md`
- `docs/milestones/v0.90.2/RELEASE_PLAN_v0.90.2.md`
- `docs/milestones/v0.90.2/RELEASE_NOTES_v0.90.2.md`

Not claimed in v0.90.2:
- first true Gödel-agent birthday
- full v0.91 moral, emotional, kindness, humor, or wellbeing substrate
- full v0.92 identity/capability rebinding or cross-polis migration
- complete red/blue/purple security ecology

## v0.90.1 (Released 2026-04-20)

Status: Completed. Implementation, docs, quality, review, remediation,
readiness, handoff, release evidence, and WP-20 release ceremony preflight are
complete. Final tag/release publication runs from clean main after this closeout
PR merges.

Summary:
- The v0.90.1 issue wave is complete: WP-01 is #2141, WP-02 through WP-20 are
  #2142 through #2160, and WP-15A third-party review is #2215.
- The compression-enablement sprint landed first: issue-wave generation,
  worktree-first workflow hardening, and the compression-era execution policy.
- The Runtime v2 foundation slice has landed through the manifold contract,
  bounded kernel service loop, provisional citizen lifecycle, snapshot and
  rehydration, invariant violation artifacts, operator controls, one
  security-boundary proof, and the integrated Runtime v2 prototype demo.
- The CSM Observatory visibility lane has landed read-only reviewer/operator
  surfaces: visibility packet, operator report, CLI bundle, static console
  reference, and safe future command-packet design.
- WP-13 aligned the review-facing docs, demo matrix, feature list, changelog,
  README, and review guide; WP-14 established the quality gate; WP-15 and
  WP-15A completed internal and third-party review; the accepted WP-16
  remediation bundles are closed; WP-17 release readiness, WP-18 v0.91/v0.92
  handoff, WP-19 release evidence, and WP-20 ceremony preflight completed the
  release tail.
- The crate version is `0.90.1` for the v0.90.1 release line.

References:
- `docs/milestones/v0.90.1/README.md`
- `docs/milestones/v0.90.1/WBS_v0.90.1.md`
- `docs/milestones/v0.90.1/SPRINT_v0.90.1.md`
- `docs/milestones/v0.90.1/DEMO_MATRIX_v0.90.1.md`
- `docs/milestones/v0.90.1/FEATURE_DOCS_v0.90.1.md`
- `docs/milestones/v0.90.1/MILESTONE_CHECKLIST_v0.90.1.md`
- `docs/milestones/v0.90.1/THIRD_PARTY_REVIEW_v0.90.1.md`
- `docs/milestones/v0.90.1/RELEASE_READINESS_v0.90.1.md`
- `docs/milestones/v0.90.1/V091_V092_HANDOFF_v0.90.1.md`
- `docs/milestones/v0.90.1/RELEASE_EVIDENCE_v0.90.1.md`
- `docs/milestones/v0.90.1/RELEASE_PLAN_v0.90.1.md`
- `docs/milestones/v0.90.1/RELEASE_NOTES_v0.90.1.md`
- `docs/planning/ADL_FEATURE_LIST.md`

Not claimed in v0.90.1:
- first true Gödel-agent birthday
- full v0.92 identity/capability substrate
- full moral/emotional civilization
- complete cross-polis migration
- full red/blue/purple security ecology

## v0.90 (Released 2026-04-18)

Status: Completed and released.

Summary:
- ADL now has a v0.90 long-lived-agent runtime package:
  `supervisor -> heartbeat -> bounded cycle -> artifact root -> continuity handle -> operator control -> inspection/status -> stock-league proof`
- The runtime and demo implementation WPs landed through the long-lived supervisor, cycle contract, state/continuity handles, operator safety, inspection boundary, stock-league scaffold, integrated long-lived demo, and demo-extension proof lane
- The sidecar proof work landed milestone compression and repo visibility packets so reviewers can inspect milestone state, drift, and code-doc-demo linkage without treating those pilots as autonomous release tooling
- The coverage tracker reports workspace line coverage at `92.40%`, which rounds to the intended `93%` tranche, with the workspace gate passing and the per-file gate passing without active file-floor exclusions; the WP-10 validation pass also recorded `92.46%`
- The Rust tracker reports one `RATIONALE` item, nineteen `REVIEW` items, and fourteen `WATCH` items after the v0.90 WP-14 child refactoring wave, down from four `RATIONALE` items before the latest split pass
- Internal review, third-party review, accepted findings remediation, next-milestone planning, and release ceremony preparation are complete
- The crate version for the v0.90 release is `0.90.0`

References:
- `docs/milestones/v0.90/README.md`
- `docs/milestones/v0.90/WBS_v0.90.md`
- `docs/milestones/v0.90/SPRINT_v0.90.md`
- `docs/milestones/v0.90/DEMO_MATRIX_v0.90.md`
- `docs/milestones/v0.90/MILESTONE_CHECKLIST_v0.90.md`
- `docs/milestones/v0.90/RELEASE_PLAN_v0.90.md`
- `docs/milestones/v0.90/RELEASE_NOTES_v0.90.md`
- `docs/milestones/v0.90/V090_PRE_THIRD_PARTY_READINESS_REPORT.md`

Not claimed in v0.90:
- full v0.92 identity/capability substrate
- live trading, financial advice, or unbounded autonomy
- autonomous release approval or silent closeout automation

## v0.89.1 (Released 2026-04-17)

Status: Completed and released.

Summary:
- ADL now has a real `v0.89.1` milestone package on `main`, centered on one adversarial-runtime band:
  `adversarial posture -> red/blue/purple roles -> execution runner -> exploit artifact -> replay manifest -> continuous verification -> self-attack -> review evidence`
- The promoted `v0.89.1` feature-doc set covers the adversarial runtime model, red/blue agent architecture, adversarial execution runner, exploit artifact schema, replay manifest, continuous verification, self-attacking systems, and supporting operational-skill substrate
- The bounded `v0.89.1` proof package includes the adversarial/security demo rows, provider-proof packaging, quality-gate evidence, docs-review convergence, internal review, accepted internal-review remediation, third-party review with no additional P0/P1/P2 findings, and the final WP-20 release ceremony
- The milestone also introduces the bounded `arxiv-paper-writer` workflow and the initial three-paper manuscript program for `What Is ADL?`, `Gödel Agents and ADL`, and `Cognitive Spacetime Manifold`
- The crate version is `0.89.1`; the v0.90 planning package is tracked and ready for the next issue wave

References:
- `docs/milestones/v0.89.1/README.md`
- `docs/milestones/v0.89.1/WBS_v0.89.1.md`
- `docs/milestones/v0.89.1/SPRINT_v0.89.1.md`
- `docs/milestones/v0.89.1/DEMO_MATRIX_v0.89.1.md`
- `docs/milestones/v0.89.1/FEATURE_DOCS_v0.89.1.md`
- `docs/milestones/v0.89.1/QUALITY_GATE_v0.89.1.md`
- `docs/milestones/v0.89.1/DOCS_REVIEW_v0.89.1.md`
- `docs/milestones/v0.89.1/INTERNAL_REVIEW_v0.89.1.md`
- `docs/milestones/v0.89.1/RELEASE_PLAN_v0.89.1.md`
- `docs/milestones/v0.89.1/RELEASE_NOTES_v0.89.1.md`

Not claimed in v0.89.1:
- the later Runtime v2, moral-governance, birthday, and polis-defense bands planned for later milestones

## v0.89 (Completed Governed Adaptation Milestone)

Status: Completed.

Summary:
- ADL now has a real `v0.89` milestone on `main`, centered on one governed-adaptation package:
  `convergence -> judgment -> decision/action mediation -> skill execution -> experiment evidence -> ObsMem explanation -> security posture/trust`
- The promoted `v0.89` feature-doc set now covers AEE convergence, Freedom Gate v2, decision surfaces, action mediation, the skill model/protocol, the Godel experiment system, ObsMem evidence/ranking, and the main-band security/trust/posture contract
- The bounded `v0.89` proof package now exists through the canonical demo matrix and the landed D1-D7 walkthrough/proof rows
- Demo/proof convergence work landed through `WP-13`, and the tracked `v0.89.1` package became the bounded adversarial-runtime follow-on

References:
- `docs/milestones/v0.89/README.md`
- `docs/milestones/v0.89/WBS_v0.89.md`
- `docs/milestones/v0.89/SPRINT_v0.89.md`
- `docs/milestones/v0.89/DEMO_MATRIX_v0.89.md`
- `docs/milestones/v0.89/FEATURE_DOCS_v0.89.md`
- `docs/milestones/v0.89/MILESTONE_CHECKLIST_v0.89.md`
- `docs/milestones/v0.89/RELEASE_PLAN_v0.89.md`
- `docs/milestones/v0.89/RELEASE_NOTES_v0.89.md`

Not yet claimed in v0.89:
- the adversarial runtime/exploit-replay package, which belongs to `v0.89.1`
- later Runtime v2, moral-governance, birthday, and polis-defense work that belongs to later milestones

## v0.88 (Completed Temporal / Chronosense + Instinct Milestone)

Status: Completed.

Summary:
- ADL now has a real `v0.88` milestone on `main`, centered on two bounded substrate bands:
  `temporal / chronosense -> instinct / bounded agency`
- The promoted `v0.88` feature-doc package now covers temporal schema, continuity/identity semantics, temporal retrieval, commitments/deadlines, bounded temporal causality, PHI-style integration metrics, instinct, and instinct runtime influence
- The bounded `v0.88` proof package now exists through `demo_v088_temporal_review_surface.sh`, `demo_v088_phi_review_surface.sh`, `demo_v088_instinct_review_surface.sh`, `demo_v088_paper_sonata.sh`, `demo_v088_deep_agents_comparative_proof.sh`, and `demo_v088_review_surface.sh`
- Paper Sonata now serves as the flagship bounded public-facing `v0.88` demo, with the deep-agents comparative proof as a supporting reviewer-facing row
- Internal review has completed a full repo code-review pass, and the one concrete implementation finding from that pass was remediated before 3rd-party review

Version note:
- `v0.88` is a completed historical milestone; later adversarial/runtime work moved into `v0.89` and `v0.89.1`.

References:
- `docs/milestones/v0.88/README.md`
- `docs/milestones/v0.88/WBS_v0.88.md`
- `docs/milestones/v0.88/SPRINT_v0.88.md`
- `docs/milestones/v0.88/DEMO_MATRIX_v0.88.md`
- `docs/milestones/v0.88/FEATURE_DOCS_v0.88.md`
- `docs/milestones/v0.88/MILESTONE_CHECKLIST_v0.88.md`
- `docs/milestones/v0.88/RELEASE_PLAN_v0.88.md`
- `docs/milestones/v0.88/RELEASE_NOTES_v0.88.md`

Not claimed in v0.88:
- later-band governance, economics, aptitude, or broader social-agency systems beyond the bounded `v0.88` slice

## v0.87.1 (Completed Runtime Completion Milestone)

Status: Completed.

Summary:
- ADL now has a real `v0.87.1` runtime-completion milestone on `main`, centered on one coherent runtime package:
  `runtime environment -> lifecycle -> execution boundaries -> trace alignment -> resilience -> operator/review surfaces`
- The promoted `v0.87.1` feature-doc set now covers the runtime environment, lifecycle, execution boundaries, resilience, Shepherd preservation, and bounded capability-aware local-model execution
- The bounded `v0.87.1` demo suite and reviewer package now exist through `demo_v0871_suite.sh`, the D8 review walkthrough, the D10 docs-to-runtime check, the D11 quality-gate walkthrough, and the D12 release-review package
- Provider-family and multi-agent proof surfaces now distinguish CI-safe bounded proof from the credential-gated D13L live-provider companion path
- Trace/archive provenance surfaces now include run manifests and milestone-organized durable archive roots for later review/export

References:
- `docs/milestones/v0.87.1/README.md`
- `docs/milestones/v0.87.1/WBS_v0.87.1.md`
- `docs/milestones/v0.87.1/SPRINT_v0.87.1.md`
- `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`
- `docs/milestones/v0.87.1/FEATURE_DOCS_v0.87.1.md`
- `docs/milestones/v0.87.1/MILESTONE_CHECKLIST_v0.87.1.md`
- `docs/milestones/v0.87.1/RELEASE_PLAN_v0.87.1.md`
- `docs/milestones/v0.87.1/RELEASE_NOTES_v0.87.1.md`

Not claimed in v0.87.1:
- later-milestone chronosense, identity, governance, instinct, or broader bounded-agency systems beyond the runtime primitives landed here

## v0.87 (Completed Substrate Convergence Milestone)

Status: Completed.

Summary:
- ADL now has a real `v0.87` substrate milestone on `main`, centered on the canonical milestone spine:
  `contracts -> execution -> trace -> review -> documentation`
- The milestone’s promoted feature-doc set now covers trace, provider portability, shared ObsMem, operational skills, control-plane behavior, and reviewer-facing proof surfaces as canonical `v0.87` docs
- Canonical `v0.87` milestone docs now reflect the real implementation sequence and release-tail closeout that completed the milestone
- The bounded `v0.87` demo and reviewer package exists through the demo matrix, runbook, and `demo_v087_suite.sh` entry surfaces
- The next-milestone handoff into `v0.87.1` was established as the runtime-completion follow-on

References:
- `docs/milestones/v0.87/README.md`
- `docs/milestones/v0.87/WBS_v0.87.md`
- `docs/milestones/v0.87/SPRINT_v0.87.md`
- `docs/milestones/v0.87/DEMO_MATRIX_v0.87.md`
- `docs/milestones/v0.87/FEATURE_DOCS_v0.87.md`
- `docs/milestones/v0.87/MILESTONE_CHECKLIST_v0.87.md`
- `docs/milestones/v0.87/RELEASE_PLAN_v0.87.md`
- `docs/milestones/v0.87/RELEASE_NOTES_v0.87.md`

Not yet claimed in v0.87:
- later-milestone continuity, chronosense, governance, signed-trace, or broader runtime-completion work that belongs to `v0.87.1+`

## v0.86 (Completed Bounded Cognitive System Milestone)

Status: Completed.

Summary:
- ADL now has its first working bounded cognitive system on `main`, centered on one canonical bounded cognitive path:
  `signals -> candidate selection -> arbitration -> reasoning -> bounded execution -> evaluation -> reframing -> memory participation -> Freedom Gate`
- Canonical runtime artifacts now cover the bounded cognitive path and related proof surfaces, including:
  `signals.json`, `candidate_selection.json`, `arbitration.json`, `execution_iterations.json`, `evaluation.json`, `reframing.json`, `memory.json`, `freedom_gate.json`, and `final_result.json`
- Local demo and review surfaces exist for the integrated milestone proof set:
  D1 canonical bounded cognitive path, D2 fast/slow routing, D3 candidate selection, D4 Freedom Gate enforcement, and D5 review-surface walkthrough
- Sprint 7 quality-gate work landed with passing local `fmt`, `clippy`, `test`, coverage, and demo-validation proof
- Docs, release-tail surfaces, and reviewer entry points are being aligned so milestone truth matches implementation and proof artifacts

References:
- `docs/milestones/v0.86/README.md`
- `docs/milestones/v0.86/WBS_v0.86.md`
- `docs/milestones/v0.86/SPRINT_v0.86.md`
- `docs/milestones/v0.86/DEMO_MATRIX_v0.86.md`
- `docs/milestones/v0.86/MILESTONE_CHECKLIST_v0.86.md`
- `docs/milestones/v0.86/RELEASE_PLAN_v0.86.md`
- `docs/milestones/v0.86/RELEASE_NOTES_v0.86.md`

Not claimed in v0.86:
- later-milestone persistence, identity, governance, signed-trace, or broader AEE convergence work
- anything beyond the bounded `v0.86` cognitive-system scope

## v0.85 (Planning And Tooling Foundation Milestone)

Status: historical bridge milestone.

Summary:
- Established the tracked milestone-planning and execution architecture that later `v0.86` work now relies on
- Landed the core milestone surfaces for `v0.85`, including design, WBS, sprint, checklist, release, and roadmap-tracking docs
- Defined the editing/control-plane model around structured prompts, issue/task bundles, and the `init/create/start/run/finish` lifecycle
- Strengthened quality/release discipline and issue reconciliation so later milestone work could be executed in smaller reviewable units
- Preserved and promoted major planning surfaces for cognition, affect, reasoning, Layer 8/provider work, and future convergence bands

References:
- `docs/milestones/v0.85/README.md`
- `docs/milestones/v0.85/DESIGN_v0.85.md`
- `docs/milestones/v0.85/WBS_v0.85.md`
- `docs/milestones/v0.85/SPRINT_v0.85.md`
- `docs/milestones/v0.85/MILESTONE_CHECKLIST_v0.85.md`
- `docs/milestones/v0.85/RELEASE_PLAN_v0.85.md`
- `docs/milestones/v0.85/RELEASE_NOTES_v0.85.md`
- `docs/milestones/v0.85/EDITING_ARCHITECTURE.md`

Not yet claimed in v0.85:
- the full bounded cognitive system that later lands in `v0.86`
- later milestone runtime identity, governance, or signed-trace behavior
- final productionization of the longer-horizon planning concepts documented under the `v0.85` milestone corpus

## v0.8 (Active Development Milestone)

Status: In progress.

Summary:
- Bounded Godel runtime and demo surfaces now exist on `main`, including the explicit seven-stage loop:
  `failure -> hypothesis -> mutation -> experiment -> evaluation -> record -> indexing`
- Canonical runtime artifacts for the Godel review loop are now emitted and validated, including:
  `mutation.v1`, `canonical_evidence_view.v1`, `evaluation_plan.v1`, and `experiment_record.v1`
- New user-facing CLI and demo surfaces were added for bounded Godel execution and inspection, alongside the v0.8 demo matrix
- New reviewer-facing demo runbooks under `demos/` cover the bounded Gödel CLI flow and bounded AEE recovery flow
- The Rust transpiler remains a bounded demo scaffold for deterministic fixture-to-runtime verification, not a production transpiler
- Major review-tail work landed to align milestone docs, schemas, and release-facing repository truth with current implementation

References:
- `docs/milestones/v0.8/RELEASE_PLAN_V0.8.md`
- `docs/milestones/v0.8/RELEASE_NOTES_V0.8.md`
- `docs/milestones/v0.8/MILESTONE_CHECKLIST_V0.8.md`
- `docs/milestones/v0.8/RECOVERY_AUDIT_V0.8.md`
- `docs/milestones/v0.8/DEMOS_V0.8.md`
- `docs/milestones/v0.8/GODEL_LOOP_INTEGRATION_V0.8.md`

Not yet claimed in v0.8:
- fully finished Adaptive Execution Engine behavior
- unconstrained self-modification or autonomous policy learning
- production graduation of the Rust transpiler demo

## v0.75 (Previous Milestone)

Status: prior milestone reference.

References:
- `docs/milestones/v0.75/RELEASE_PLAN_0.75.md`
- `docs/milestones/v0.75/RELEASE_NOTES_0.75.md`
- `docs/milestones/v0.75/MILESTONE_CHECKLIST_0.75.md`

## v0.7.0 (Released)

Status: Released (`v0.7.0`).

Summary:
- Foundation runtime hardening for deterministic, replayable execution.
- Security envelope and trust/signing surfaces integrated into core execution flows.
- Runtime identity migration to canonical `adl` naming with compatibility-window shims.

References:
- `docs/milestones/v0.7/RELEASE_NOTES_v0.7.md`
- `docs/milestones/v0.7/RELEASE_PLAN_v0.7.md`

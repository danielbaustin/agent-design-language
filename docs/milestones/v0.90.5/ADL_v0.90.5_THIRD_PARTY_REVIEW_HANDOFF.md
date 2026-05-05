# Third-Party Review Handoff - v0.90.5

## Metadata

- Milestone: v0.90.5
- Version: v0.90.5
- Active crate version: 0.90.5
- Review lane: WP-23 external / 3rd-party review, following WP-22 internal review
- Prepared during: v0.90.5 review tail
- Prepared from issue: #2588
- Prepared from branch: codex/2588-v0-90-5-wp-23-external-3rd-party-review
- Current packet status: external review complete; zero-finding result recorded
- Date: 2026-05-04
- Publication attempted: false
- Release approval claimed: false
- Review approval claimed: false

## Update Before Review

This handoff intentionally exists before WP-22 fully closes so the external
review can be staged without losing time. Before sending it to the reviewer,
refresh this file from clean root `main` after WP-22 merges.

Minimum final refresh checks:

- Confirm WP-22 is closed and the final internal review is visible in the
  milestone docs or review directory.
- Confirm root `main` has been fast-forwarded after the final WP-22 merge.
- Confirm `README.md`, `CHANGELOG.md`, `REVIEW.md`, `adl/Cargo.toml`,
  `adl/Cargo.lock`, and `docs/milestones/v0.90.5` still agree on v0.90.5.
- Confirm this handoff does not point at branch-only artifacts unless it
  explicitly says they are pending.
- Confirm no host-local paths, ignored control-plane paths, temporary worktree
  paths, or copied terminal-output roots appear in the final packet.

Those refresh conditions were satisfied before the external packet was sent.

## Purpose

This handoff gives a third-party reviewer a bounded v0.90.5 review packet.
Review v0.90.5 as the Governed Tools v1.0 milestone plus the first landed
Comms / ACIP tranche:

- governed tool threat model and explicit non-goals
- Universal Tool Schema v1.0 portable description and conformance
- ADL Capability Contract v1.0 authority, visibility, privacy, delegation,
  trace, and replay boundaries
- deterministic registry binding, compiler normalization, policy mediation, and
  governed execution
- trace, replay, redaction, and evidence surfaces
- dangerous negative safety suite
- bounded model-proposal benchmark and local / Gemma evaluation
- flagship governed-tools demo
- bounded ACIP message, identity, invocation, and review/coding specialization
  surfaces landed in this milestone

The reviewer should produce evidence-backed findings with severity, location,
impact, and recommended remediation. The reviewer should not rewrite docs,
perform remediation, create release tags, merge PRs, close issues, or run the
release ceremony.

## Imported Review Artifacts

External review artifacts:

- `.adl/reviews/v0.90.5/ADL_v0.90.5_Comprehensive_Review_1.pdf`
- `.adl/reviews/v0.90.5/ADL_v0.90.5_REVIEW_SUMMARY.md`

## Current Milestone Truth

v0.90.5 is the Governed Tools v1.0 release line. It has landed through the
implementation and proof package:

- tool-call threat model and governed-capability non-goals
- Universal Tool Schema v1.0 public-compatible schema and conformance fixtures
- ADL Capability Contract v1.0 authority, privacy, visibility, delegation,
  trace, and replay semantics
- deterministic tool-registry binding and UTS-to-ACC compiler behavior
- policy authority and Freedom Gate mediation before execution
- governed executor behavior with trace, replay, and redaction constraints
- dangerous negative tests that fail closed
- bounded model-proposal benchmarking and local / Gemma evaluation
- Governed Tools v1.0 flagship demo
- explicit feature-proof coverage and demo-matrix classification
- reviewer-entry and release-truth convergence through WP-21
- ADR 0015 for governed tools execution authority architecture
- first landed ACIP / Comms tranche: protocol architecture, canonical message
  envelope, identity shape, invocation/Freedom Gate linkage, conformance
  fixtures, review-agent specialization, coding-agent specialization, and ACIP
  proof coverage

The milestone does not claim:

- that UTS alone grants execution authority
- that valid JSON or schema compatibility is sufficient to execute tools
- arbitrary shell, network, or secret-bearing execution by model output
- payment rails, billing, legal contracting, or inter-polis economics
- full v0.91 moral/cognitive-being substrate
- full v0.91.1 identity/capability, memory, ToM, ANRM/Gemma, or wider learning
  follow-on work
- release completion before WP-24 through WP-26 finish

## Review-Tail State To Consider

The reviewer should treat the release tail as active until Daniel confirms the
final branch under review is clean root `main` after all intended review-tail
PRs merge.

Known review-tail gates:

- WP-22 internal review is the local findings-first review pass.
- WP-23 external / 3rd-party review is the lane this handoff supports.
- WP-24 owns accepted finding remediation or explicit deferral.
- WP-25 owns next-milestone planning and handoff.
- WP-26 owns final release ceremony.

Current internal-review posture from the active WP-22 draft:

- WP-22 / #2587 is still open.
- Draft PR #2713 is the current internal-review branch.
- Before final external review, verify that WP-22 has merged and that any
  accepted findings are either routed to WP-24, explicitly deferred, or
  resolved in the merged milestone docs.

## Previous Review Mistakes To Avoid

The v0.90.5 reviewer packet should explicitly avoid the recurring issues that
have shown up in earlier review cycles:

- Do not send stale version truth. `Cargo.toml`, `Cargo.lock`, `README.md`,
  `CHANGELOG.md`, `REVIEW.md`, and milestone docs must all say v0.90.5 where
  they describe the active release line.
- Do not claim the issue wave, review tail, or release ceremony is complete
  before the relevant WP has actually closed.
- Do not claim branch-only review artifacts as merged release truth.
- Do not hide docs-only, fixture-only, benchmark-only, or design-only
  boundaries. If something is a design artifact, non-runtime proof, or
  documented deferral, say so.
- Do not expose ignored local control-plane paths, absolute host paths,
  temporary worktree paths, raw tool traces, or copied terminal output roots in
  the final packet.
- Do not mix internal and external review artifacts without labeling them.
- Do not let ACIP wording imply the milestone ships open-network agent
  federation, TLS-grade transport, or bypass of governed execution.
- Do not let future-roadmap language make v0.90.5 look like v0.91 moral
  governance or v0.91.1 identity/capability follow-on work.

## Required Review Scope

Review these top-level repository surfaces first:

- `README.md`
- `REVIEW.md`
- `CHANGELOG.md`
- `adl/Cargo.toml`
- `adl/Cargo.lock`
- `adl/README.md`
- `docs/README.md`
- `docs/planning/ADL_FEATURE_LIST.md`

Review these architecture-decision surfaces:

- `docs/adr/README.md`
- `docs/adr/0014-contract-market-architecture.md`
- `docs/adr/0015-governed-tools-execution-authority-architecture.md`

Review these v0.90.5 milestone surfaces next:

- `docs/milestones/v0.90.5/README.md`
- `docs/milestones/v0.90.5/VISION_v0.90.5.md`
- `docs/milestones/v0.90.5/DESIGN_v0.90.5.md`
- `docs/milestones/v0.90.5/WBS_v0.90.5.md`
- `docs/milestones/v0.90.5/SPRINT_v0.90.5.md`
- `docs/milestones/v0.90.5/DECISIONS_v0.90.5.md`
- `docs/milestones/v0.90.5/WP_ISSUE_WAVE_v0.90.5.yaml`
- `docs/milestones/v0.90.5/WP_EXECUTION_READINESS_v0.90.5.md`
- `docs/milestones/v0.90.5/MILESTONE_CHECKLIST_v0.90.5.md`
- `docs/milestones/v0.90.5/RELEASE_PLAN_v0.90.5.md`
- `docs/milestones/v0.90.5/RELEASE_NOTES_v0.90.5.md`
- `docs/milestones/v0.90.5/RELEASE_READINESS_v0.90.5.md`
- `docs/milestones/v0.90.5/QUALITY_GATE_v0.90.5.md`
- `docs/milestones/v0.90.5/DEMO_MATRIX_v0.90.5.md`
- `docs/milestones/v0.90.5/FEATURE_DOCS_v0.90.5.md`
- `docs/milestones/v0.90.5/FEATURE_PROOF_COVERAGE_v0.90.5.md`
- `docs/milestones/v0.90.5/ADL_v0.90.5_THIRD_PARTY_REVIEW_HANDOFF.md`

Review these feature and proof docs where they intersect release claims:

- `docs/milestones/v0.90.5/features/TOOL_CALL_THREAT_MODEL_AND_SEMANTICS.md`
- `docs/milestones/v0.90.5/features/UTS_PUBLIC_SPEC_AND_CONFORMANCE.md`
- `docs/milestones/v0.90.5/features/ACC_AUTHORITY_AND_VISIBILITY.md`
- `docs/milestones/v0.90.5/features/TOOL_REGISTRY_AND_COMPILER.md`
- `docs/milestones/v0.90.5/features/GOVERNED_EXECUTION_AND_TRACE.md`
- `docs/milestones/v0.90.5/features/MODEL_TESTING_AND_FLAGSHIP_DEMO.md`
- `docs/milestones/v0.90.5/features/AGENT_COMMS_v1.md`
- `docs/milestones/v0.90.5/review/CLAIM_BOUNDARY_REVIEW.md`
- `docs/milestones/v0.90.5/review/uts-conformance-report.json`
- `docs/milestones/v0.90.5/review/model-proposal-benchmark-report.json`
- `docs/milestones/v0.90.5/review/local-gemma-model-evaluation-report.json`
- `docs/milestones/v0.90.5/review/dangerous-negative-suite-report.json`

Review these implementation and test surfaces if a claim depends on executable
behavior:

- `adl/src/uts.rs`
- `adl/src/uts_conformance.rs`
- `adl/src/acc.rs`
- `adl/src/tool_registry.rs`
- `adl/src/uts_acc_compiler.rs`
- `adl/src/policy_authority.rs`
- `adl/src/freedom_gate.rs`
- `adl/src/agent_comms.rs`
- `adl/src/runtime_v2/`
- `adl/src/runtime_v2/tests/`

If a listed path does not exist in the exact branch under review, report that
as a scope or handoff drift finding rather than assuming the implementation
exists elsewhere.

## Proof And Quality Evidence To Check

At minimum, ask the reviewer to verify:

- `adl/Cargo.toml` reports version `0.90.5`.
- `adl/Cargo.lock` reports package `adl` version `0.90.5`.
- `README.md`, `CHANGELOG.md`, and `REVIEW.md` agree with the active crate
  version.
- `REVIEW.md` points reviewers at the active v0.90.5 review package.
- `DEMO_MATRIX_v0.90.5.md` and `FEATURE_PROOF_COVERAGE_v0.90.5.md` agree on
  demo and proof status.
- `QUALITY_GATE_v0.90.5.md` truthfully distinguishes green branch-proof
  evidence from any remaining main-branch coverage/runtime concerns.
- `RELEASE_READINESS_v0.90.5.md` does not claim WP-22 through WP-26 are
  complete before they are.
- UTS docs never imply that schema validity alone grants execution authority.
- ACC, registry/compiler, policy, Freedom Gate, executor, and trace surfaces
  remain clearly separated.
- dangerous negative and local-model evidence are reviewable and bounded rather
  than marketing-only claims.
- ACIP docs remain bounded to the landed tranche and do not imply full network
  federation, TLS transport, or open external-agent runtime authority.
- ADR 0015 matches the shipped governed-tools authority boundary rather than a
  speculative future architecture.

## Suggested Validation Commands

Run focused commands first:

```sh
cargo test --manifest-path adl/Cargo.toml uts_conformance -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2 -- --nocapture
cargo run --manifest-path adl/Cargo.toml -- runtime-v2 feature-proof-coverage --out artifacts/v0905/feature-proof-coverage.json
cargo run --manifest-path adl/Cargo.toml -- demo demo-v0905-governed-tools-flagship --run --trace --out artifacts/v0905/flagship-demo --no-open
```

Then use milestone docs and tracked review JSON artifacts to confirm that the
review packet and the shipped proof surfaces agree.

If the reviewer chooses not to run commands, findings should explicitly say the
review was docs-and-source-only.

## Finding Routing Rule

- Accepted findings route to WP-24 / #2589.
- Zero-finding outcome should be recorded explicitly rather than implied by
  silence.
- Findings that would widen v0.90.5 beyond Governed Tools v1.0 plus the landed
  first Comms tranche should be marked as non-blocking follow-on
  considerations rather than hidden release blockers.

## Review Outcome

- Overall result: zero findings
- Grade summary from the reviewer packet: perfect / release-ready package
- Accepted findings: none
- Required remediation: none

## Current Disposition

Current disposition: completed third-party review with zero findings. WP-24 may
close as a no-remediation disposition record rather than a code or docs repair
wave.

## Reviewer Output Requested

Ask the reviewer to return:

- overall judgment
- findings ordered by severity
- explicit non-findings where a commonly misunderstood claim was checked and
  rejected as a problem
- any release blockers versus follow-on recommendations
- a short residual-risk section

Preferred finding structure:

- severity
- file or surface
- concern
- why it matters
- recommended remediation

## Non-Claims

This handoff does not claim that v0.90.5 ships:

- arbitrary unrestricted tool execution
- production secret-management integration
- payment settlement, billing, or inter-polis economics
- full ACIP 1.0 or open-network federation
- production A2A adapter support
- full v0.91 moral/cognitive-being substrate
- full v0.91.1 identity/capability, memory, ToM, ANRM/Gemma, or wider learning
  work
- release completion before the remaining closeout steps finish

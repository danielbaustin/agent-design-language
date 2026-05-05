# v0.90.5 Coverage and Quality Gate

## Metadata

- Milestone: `v0.90.5`
- Version: `v0.90.5`
- Owner: `Daniel Austin / Codex`
- Canonical issue / WP: `#2585` / `WP-20`
- Scope: milestone quality, coverage, proof-package, and get-well posture

## Purpose

This document defines the canonical `v0.90.5` quality gate.

It is the release-truth surface for:

- the required repository merge-gate posture
- the live coverage policy and evidence authority
- the first-level milestone proof package for Governed Tools v1.0 and the
  landed Comms / ACIP tranche
- the first-level gap-review and Rust maintainability / Rustdoc watch posture
- the explicit exception register for still-pending or non-green validation
  posture
- the get-well wave disposition that must be recorded before release closeout

This document records the gate. It does not replace CI implementation, but the
gate, the milestone docs, and the live GitHub enforcement surfaces must agree.

## Why This Exists

`v0.90.5` now has two first-level milestone proof bands that must remain
reviewable together:

- Governed Tools v1.0, with core proof rows `D1` through `D12`
- the landed Comms / ACIP tranche, including `D13` and the coding/reviewer
  companion feature surfaces

That means "CI is green" is necessary, but not sufficient by itself.

The milestone also needs one canonical surface that states:

- which checks are required
- which checks are enforced in CI
- how coverage is judged
- which evidence is authoritative for coverage posture versus runtime-cost
  remeasurement
- which milestone proof surfaces count as first-level evidence
- which open release-tail, closeout-drift, and Rust maintainability gaps remain
  first-level release risks
- which exceptions remain explicit rather than implied

## Gate Structure

The `v0.90.5` gate has four layers:

1. baseline repository merge gate
2. coverage posture gate
3. milestone proof-package gate
4. exception and get-well disposition gate

The first layer proves the ordinary repository merge gate is green. The second
proves coverage is governed by explicit thresholds and visible authority rules.
The third proves the milestone is backed by reviewer-facing proof surfaces
rather than scattered feature claims. The fourth keeps unresolved runtime-cost
and watchdog posture explicit instead of pretending the remaining gaps are
already solved.

## First-Level Gap and Maintainability Inputs

WP-20 also treats the existing gap-review and Rustdoc tracking surfaces as
first-level evidence inputs rather than background notes.

Primary inputs:

- `.adl/docs/TBD/retired/gap_reviews/v0.90.5_gap_review.md`
- `.adl/docs/TBD/RUSTDOC_GAP_ANALYSIS.md`
- live GitHub issue list for `version:v0.90.5`
- `docs/milestones/v0.90.5/README.md`
- `docs/milestones/v0.90.5/RELEASE_NOTES_v0.90.5.md`

These inputs matter because they describe release-tail incompleteness and Rust
maintainability/documentation debt that CI-green alone does not capture.

## Required vs Documented Exceptions

- **Required** means the item must pass for the relevant phase.
- **Exception documented** means the owner, rationale, and disposition are
  explicit.
- Exceptions do not convert a blocker into a pass.

## 1) Baseline Repository Merge Gate

The canonical merge-gate workflow is `.github/workflows/ci.yaml`.

The required jobs are:

- `adl-ci`
- `adl-coverage`

These jobs are the minimum repo gate, but they are not the whole milestone
quality story. A green CI state is necessary, not sufficient, unless the
milestone proof package named below also exists and remains truthful.

### Current CI command and policy surfaces

The merge-gate posture currently depends on:

- `.github/workflows/ci.yaml`
- `adl/tools/ci_path_policy.sh`
- `adl/tools/run_pr_fast_test_lane.sh`
- `adl/tools/run_authoritative_coverage_lane.sh`
- `adl/tools/check_coverage_impact.sh`
- `adl/tools/enforce_coverage_gates.sh`

At a high level:

- `adl-ci` enforces shell/tooling sanity, contract checks, guardrails, docs
  command checks, CI runtime/cache contract checks, Rust validation when the
  path policy requires it, and demo smoke when the path policy requires it.
- `adl-coverage` enforces either a full authoritative all-features
  `cargo llvm-cov nextest` lane or a bounded PR-fast coverage summary lane,
  depending on path policy and changed-surface risk.

### Truth rule

WP-20 treats CI-green as required repository evidence, but not as a substitute
for milestone proof surfaces. Release-tail docs must not imply that merge-gate
success alone proves the flagship, local/Gemma, or ACIP proof stories.

## 2) Coverage Posture Gate

The current coverage gate is `adl-coverage`.

It enforces:

- workspace line coverage threshold: `90%`
- per-file line coverage threshold: `80%`
- active exclusion regex: `^$`

That means there are no active per-file exclusions in the merge gate.

Coverage enforcement is implemented by:

- `adl/tools/run_authoritative_coverage_lane.sh`
- `adl/tools/check_coverage_impact.sh`
- `adl/tools/enforce_coverage_gates.sh`

### Coverage authority rule

- Push-to-main runs use the authoritative all-features coverage lane.
- Pull requests may use a bounded PR-fast coverage lane when path policy allows
  it.
- Therefore, a green PR coverage check is valid merge-gate evidence, but it is
  not automatically a full authoritative wall-time remeasurement of the entire
  milestone.

### Live evidence used by this gate

- WP-19 PR evidence: [#2689](https://github.com/danielbaustin/agent-design-language/pull/2689)
  merged on `2026-05-03` with green `adl-ci` and `adl-coverage` checks.
- Flagship-demo PR evidence: [#2679](https://github.com/danielbaustin/agent-design-language/pull/2679)
  merged on `2026-04-29` with green `adl-ci` and `adl-coverage` checks.
- Comms-08 PR evidence: [#2676](https://github.com/danielbaustin/agent-design-language/pull/2676)
  merged on `2026-04-29` with green `adl-ci` and `adl-coverage` checks.
- Daily coverage-blocker fix PR evidence: [#2691](https://github.com/danielbaustin/agent-design-language/pull/2691)
  merged on `2026-05-03` with green `adl-ci` and `adl-coverage` checks.
- current `main` push evidence: GitHub Actions run `25272620889` for head
  `4087678b` completed on `2026-05-03` with:
  - `adl-ci`: success
  - `adl-coverage`: failure at `Enforce coverage policy gates (workspace + per-file)`

## 3) Milestone Proof-Package Gate

WP-20 covers both core governed-tools proof rows and the landed Comms / ACIP
tranche as first-level milestone work.

### Core governed-tools first-level evidence

The core Governed Tools v1.0 proof package is anchored by:

- `docs/milestones/v0.90.5/DEMO_MATRIX_v0.90.5.md`
- `docs/milestones/v0.90.5/FEATURE_PROOF_COVERAGE_v0.90.5.md`
- `docs/milestones/v0.90.5/features/MODEL_TESTING_AND_FLAGSHIP_DEMO.md`
- `docs/milestones/v0.90.5/review/model-proposal-benchmark-report.json`
- `docs/milestones/v0.90.5/review/local-gemma-model-evaluation-report.json`

This covers the landed proof rows `D1` through `D12`.

### Comms / ACIP first-level evidence

The landed Comms tranche is also first-level milestone evidence for WP-20,
even though it remains distinct from the governed-execution authority stack.

Its primary proof surfaces are:

- `docs/milestones/v0.90.5/DEMO_MATRIX_v0.90.5.md` (`D13`)
- `docs/milestones/v0.90.5/FEATURE_PROOF_COVERAGE_v0.90.5.md`
- `docs/milestones/v0.90.5/features/AGENT_COMMS_v1.md`
- `docs/milestones/v0.90.5/features/CODING_AGENT_RUNNER.md`
- `docs/milestones/v0.90.5/features/LOCAL_MODEL_PR_REVIEWER_TOOL.md`

The landed issue chain is:

- `#2628` Comms-01: promote ACIP v1 general protocol architecture
- `#2629` Comms-02: canonical ACIP envelope and identity model
- `#2630` Comms-03: invocation contract and Freedom Gate event binding
- `#2631` Comms-04: validation fixtures and conformance suite
- `#2632` Comms-05: review-agent specialization and SRP policy binding
- `#2627` Comms-06: coding-agent specialization and provider-neutral runner
- `#2633` Comms-07: trace, replay, redaction, and evidence integration
- `#2634` Comms-08: ACIP demo and proof coverage

The quality significance of that chain is:

- the milestone now has a stable message substrate rather than role-specific
  prompt hacks
- invocation is explicitly bound to policy / Freedom Gate references
- reviewer-facing conformance, redaction, and evidence surfaces are present
- coding and review specializations are bounded and do not collapse into
  same-session write-and-bless authority
- the final ACIP proof demo is deterministic and reviewable without pretending
  to prove live transport or federation

The WP-20 interpretation is:

- Comms is first-level milestone work for proof, evidence, and review posture.
- Comms is not execution authority by message alone.
- Comms does not bypass UTS, ACC, policy evaluation, Freedom Gate, or governed
  execution.
- Comms quality is evaluated through its own merged proof and conformance
  surfaces, not by silently absorbing it into the governed-tools authority
  stack.

### Comms quality posture

For WP-20, the Comms tranche is treated as substantively landed first-level
work, not as a speculative future lane.

Evidence used here:

- Comms-08 PR evidence: [#2676](https://github.com/danielbaustin/agent-design-language/pull/2676)
  merged green with `adl-ci` and `adl-coverage`
- Comms feature and proof surfaces:
  - `docs/milestones/v0.90.5/features/AGENT_COMMS_v1.md`
  - `docs/milestones/v0.90.5/features/CODING_AGENT_RUNNER.md`
  - `docs/milestones/v0.90.5/features/LOCAL_MODEL_PR_REVIEWER_TOOL.md`
  - `docs/milestones/v0.90.5/DEMO_MATRIX_v0.90.5.md`
  - `docs/milestones/v0.90.5/FEATURE_PROOF_COVERAGE_v0.90.5.md`

What WP-20 is willing to say:

- the Comms tranche has first-level milestone proof and conformance visibility
- the coding and reviewer lanes are bounded enough to participate in reviewable
  milestone quality evidence
- the ACIP proof-demo and supporting tests are part of the release-quality
  story

What WP-20 is not willing to say:

- that ACIP is a production transport layer
- that ACIP grants execution authority on its own
- that Comms has erased the need for governed execution, review, or redaction
  boundaries

### Proof-package rule

Later release-tail issues must not speak as though only Governed Tools mattered
to `v0.90.5`; the landed Comms tranche must remain visible in the same quality
story. At the same time, later docs must not erase the distinction between
first-level milestone visibility and core execution-authority semantics.

## 4) Gap Review and Rust Maintainability Watch

### Existing gap-review posture

The existing `v0.90.5` gap review correctly reported, at WP-20 time, that the
milestone was not yet ready for final closure because the release tail was
still open and closeout truth was incomplete.

First-level findings still relevant to WP-20:

- release-tail issues `#2586` through `#2591` are open
- milestone/release docs remain incomplete until later review-tail work lands
- metadata parity and automation truth still require cleanup

### Post-`#2701` records follow-up update (2026-05-04)

The post-merge records follow-up rerun under `#2706` narrowed the previous
records-risk picture:

- `bash adl/tools/check_milestone_closed_issue_sor_truth.sh --version v0.90.5`
  now passes after retroactive local closeout normalization for `#2704`
- `bash adl/tools/check_issue_metadata_parity.sh --version v0.90.5 --root <primary-root>`
  still fails, with `39` remaining issue-title/metadata mismatches
- the active records-tail issues are now `#2705`, `#2706`, and `#2707`, with
  `#2707` holding the remaining metadata-parity cleanup backlog

WP-20 does not duplicate the full gap-review report, but it now treats that
report as a first-level risk surface for release quality.

### Rust maintainability and Rustdoc tracker posture

The existing Rustdoc gap tracker reports that ADL can generate docs today but
has low public API documentation coverage and no compiler-level `missing_docs`
enforcement.

Key points from the tracker:

- approximate public-item documentation coverage is about `8.8%`
- `runtime_v2`, `godel`, `chronosense`, `trace`, and `control_plane` are among
  the least documented architectural surfaces
- maintainability/documentation debt is concentrated in the same large Rust
  subsystems that are important for onboarding and review

This is not a release blocker by itself for WP-20, but it is first-level
quality context that later docs/review and planning work must not ignore.

### Live open issue watch

The current open issue list reinforces the same release-quality story:

- release-tail work still open:
  - `#2586` `WP-21`
  - `#2587` `WP-22`
  - `#2588` `WP-23`
  - `#2589` `WP-24`
  - `#2590` `WP-25`
  - `#2591` `WP-26`
- open Rust maintainability / refactor work:
  - `#2663` split `adl/src/agent_comms.rs`
  - `#2664` split `adl/src/uts_acc_compiler.rs`
  - `#2665` / `#2688` split `adl/src/trace.rs`
  - `#2686` split `adl/src/cli/tooling_cmd/code_review.rs`
  - `#2687` split `adl/src/runtime_v2/private_state_sanctuary.rs`
  - `#2690` fix Rust PR control-plane hangs in `pr.sh` lifecycle commands

These issues are not all owned by WP-20, but they are part of the first-level
quality and maintainability posture that this gate must surface truthfully.

### Planning-package truth still visible at WP-20

The tracked planning package still matters at this stage because it defines
parallel milestone work that has to remain visible even when WP-20 is focused
on quality rather than fresh implementation.

That includes:

- the separate get-well runtime-reduction wave
- the parallel Comms sprint
- the bounded Python-reduction tranche described in
  `docs/milestones/v0.90.5/README.md` and `docs/milestones/v0.90.5/WBS_v0.90.5.md`

WP-20 does not claim the Python tranche is fully dispositioned here, but it
does treat it as milestone truth that later closeout docs must not silently
erase.

## 5) Exception Register and Get-Well Disposition

### Exception Register

| Area | State | Owner | Rationale | Disposition |
| --- | --- | --- | --- | --- |
| Current `main` authoritative coverage posture | failure | `WP-20` / follow-on quality tail | The latest push-to-main run for head `4087678b` (`25272620889`) completed with `adl-ci` green but `adl-coverage` red at the coverage-policy enforcement step. | Record as an explicit gate exception; release-tail work must not describe current `main` as fully green until the coverage failure is fixed. |
| Release-tail gap-review posture | closed by later closeout | `WP-21` through `WP-26` | The earlier gap review correctly identified the release tail as incomplete at WP-20 time. Those review, remediation, planning, and ceremony steps are now complete. | Preserve this as historical gate context; the open-gap posture was resolved by later closeout work rather than by WP-20 alone. |
| Closed-issue closeout drift class | cleared by follow-up rerun | `#2706` | The 2026-05-04 post-`#2701` rerun repaired the last active closed-issue SOR residue (`#2704`) and `check_milestone_closed_issue_sor_truth.sh --version v0.90.5` now passes. | Keep the historical gap visible in the gap review, but treat the failing gate as cleared; remaining records risk is the metadata-parity backlog routed to `#2707`. |
| Rust maintainability and Rustdoc coverage debt | explicit gap | maintainability backlog / `WP-25` planning | The Rustdoc tracker shows low public API documentation coverage, and multiple large Rust maintainability/refactor issues remain open in `v0.90.5`. | Record as first-level quality context and hand it forward into docs/review/planning rather than implying it is already resolved. |
| Release notes and public closeout wording | closed by ceremony | `WP-21` / `WP-26` | `RELEASE_NOTES_v0.90.5.md` was draft/aspirational at WP-20 time and required final closeout alignment. | Preserve as historical gate context; the release-note gap is resolved by the final ceremony package. |
| Python-reduction tranche disposition | handed forward explicitly | `WP-25` planning | The milestone planning package reserved a bounded Python-reduction tranche, and WP-20 did not yet record final disposition. | Preserve as historical gate context; the tranche remains explicit future work rather than a silently completed v0.90.5 result. |
| Post-GW runtime-cost remeasurement normalization | explicit gap | `WP-20` / `WP-25` | A post-GW authoritative push run now exists (`25272620889`), but the tracking artifact does not yet record normalized remaining hotspot counts or a reviewed runtime-effect summary, and the run itself failed the coverage gate. | Quality gate is satisfied with an explicit measurement exception; runtime-cost closure remains a follow-on unless later work captures a green authoritative remeasurement. |
| Nightly coverage watchdog stability | explicit gap | `daily blocker follow-up lane` | Recent `nightly-coverage-ratchet` runs failed before the merged fix in [#2691](https://github.com/danielbaustin/agent-design-language/pull/2691); the watchdog is not itself the canonical release gate. | Keep nightly failures visible, but do not let the watchdog replace the merge gate or milestone proof package. |

### Get-Well Disposition

Baseline source: `docs/milestones/v0.90.5/GET_WELL_TRACKING_v0.90.5.md`

- baseline authoritative coverage wall time: `660.944s`
- baseline unique tests over `45s`: `39`
- baseline tests over `60s`: `1`
- baseline deduped cumulative runtime over `45s`: `1938.389s`

Merged GW slices:

- GW-00 / [#2592](https://github.com/danielbaustin/agent-design-language/issues/2592): baseline, budget, and tracking artifact
- GW-01 / [#2593](https://github.com/danielbaustin/agent-design-language/issues/2593): external-counterparty proof-family collapse
- GW-02 / [#2594](https://github.com/danielbaustin/agent-design-language/issues/2594): private-state observatory proof-family collapse
- GW-03 / [#2595](https://github.com/danielbaustin/agent-design-language/issues/2595): delegation-subcontract proof-family collapse
- GW-04 / [#2596](https://github.com/danielbaustin/agent-design-language/issues/2596): contract-market and resource-stewardship proof-family collapse
- GW-05 / [#2597](https://github.com/danielbaustin/agent-design-language/issues/2597): CLI/demo proof-matrix tail reduction

Measured effect:

- Each merged GW slice has local proving evidence recorded in the tracking
  artifact.
- The merged PR surface is green for recent release-tail work, including
  `#2689` and `#2691`.
- A post-GW authoritative push run now exists for current `main`
  (`25272620889`), but it failed the coverage-policy gate and has not yet been
  normalized into a reviewed runtime-effect summary in the tracking artifact.

WP-20 therefore records the get-well wave as:

- **completed as a merged execution-support wave**
- **not yet authoritatively closed on runtime-cost measurement**

## WP-20 Outcome

`WP-20` is **satisfied with explicit exceptions**.

That means:

- the canonical quality gate now exists
- the repo merge gate, coverage posture, proof-package posture, and get-well
  disposition are all documented in one place
- Comms / ACIP is included as first-level milestone work in the quality story
- unresolved coverage-gate failure on current `main` and incomplete runtime-cost
  closure remain explicit rather than hidden

This outcome does **not** claim:

- that the runtime-cost problem is fully solved
- that Comms has become execution authority
- that later docs/review/release-tail work is already complete

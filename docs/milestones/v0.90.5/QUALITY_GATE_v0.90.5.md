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

The WP-20 interpretation is:

- Comms is first-level milestone work for proof, evidence, and review posture.
- Comms is not execution authority by message alone.
- Comms does not bypass UTS, ACC, policy evaluation, Freedom Gate, or governed
  execution.

### Proof-package rule

Later release-tail issues must not speak as though only Governed Tools mattered
to `v0.90.5`; the landed Comms tranche must remain visible in the same quality
story. At the same time, later docs must not erase the distinction between
first-level milestone visibility and core execution-authority semantics.

## 4) Exception Register and Get-Well Disposition

### Exception Register

| Area | State | Owner | Rationale | Disposition |
| --- | --- | --- | --- | --- |
| Current `main` authoritative coverage posture | failure | `WP-20` / follow-on quality tail | The latest push-to-main run for head `4087678b` (`25272620889`) completed with `adl-ci` green but `adl-coverage` red at the coverage-policy enforcement step. | Record as an explicit gate exception; release-tail work must not describe current `main` as fully green until the coverage failure is fixed. |
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

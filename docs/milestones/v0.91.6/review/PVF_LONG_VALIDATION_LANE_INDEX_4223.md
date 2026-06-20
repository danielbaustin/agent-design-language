# PVF Long Validation Lane Index And A/B Contract (#4223)

Status: `planning_input`
Issue: `#4223`
Version: `v0.91.6`
Date: `2026-06-20`

## Summary

This packet turns the existing PVF, slow-proof, CI-policy, and validation
inventory surfaces into one bounded long-lane index for validation work whose
observed or expected wall time exceeds `60` seconds.

The goal is not to rewrite tests in this slice. The goal is to make long
validation visible, classify it consistently, and define how validation pass
construction should split:

- `A_small`: the required short deterministic proof that can finish without
  waiting behind long work
- `B_large`: the required long proof that must remain explicit, fan out when
  safe, and join `A_small` only at an intentional barrier

`A_small + B_large` is the minimum truthful proof set for a validation pass
when both classes are required.

## Evidence Sources

This index is grounded in current tracked repo surfaces:

- `bash adl/tools/validation_inventory.sh --format json`
- `adl/config/slow_proof_families.v0.91.6.json`
- `adl/tools/skills/docs/CI_RUNTIME_POLICY_GUIDE.md`
- `docs/milestones/v0.91.4/features/PVF_INITIAL_LANE_INVENTORY_v0.91.4.md`
- `docs/milestones/v0.91.4/features/PVF_CI_RELEASE_POLICY_v0.91.4.md`
- `docs/milestones/v0.91.4/features/PARALLEL_VALIDATION_FABRIC.md`
- `docs/milestones/v0.91.2/SLOW_TEST_TIMING_DIAGNOSTICS_v0.91.2.md`
- `docs/milestones/v0.91.6/review/ci_log_archive/CI_LOG_ARCHIVE_S3_CONTRACT_4225.md`

The long-lane classifications below are therefore a mix of:

- observed evidence from existing timing diagnostics
- explicit slow-proof / release-gate policy
- expected heavy-runtime classification from current tracked lane commands

Where the table says `expected`, that is an inference from the current tracked
lane contract rather than a fresh rerun in this issue.

## Current Inventory Snapshot

`adl/tools/validation_inventory.sh --format json` currently reports:

- `242` Rust library test-bearing files with `2118` test entries
- `27` Rust binary test-bearing files with `85` test entries
- `38` Rust integration-test files with `336` test entries
- `1` Rust doc-test-bearing file with `4` doc-test blocks
- `224` shell validator surfaces
- `38` Python validator surfaces
- `97` demo/proof surfaces
- `73` release-gate surfaces
- `71` tracked slow-proof surfaces

That wider inventory is the denominator. The long-lane index in this packet is
the bounded `t > 60s` or policy-heavy subset that should not be mixed back into
the ordinary short proof stream.

## Long-Lane Index (`t > 60s`)

| Category | Current command / entrypoint | Observed or expected duration posture | Owner surface | PVF lane / proof role | Determinism posture | Resource profile | Release-gate status | Fan-out posture | Why it is slow / notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Rust unit/integration heavy proof | `cargo nextest run --features slow-proof-tests --status-level all --final-status-level slow` | observed and expected `> 60s` | runtime / tools / GitHub | `release_gate`, `slow_proof` | deterministic | high | required for slow-proof / release lanes, not ordinary PR-fast proof | split into family or shard lanes | Heavy runtime-v2 proof-materialization, golden-fixture, registry/accessor, and release-evidence tests are intentionally isolated behind `slow-proof-tests`. |
| Rust unit/integration heavy proof shards | `cargo nextest run --features slow-proof-tests --partition count:N/4 --status-level all --final-status-level slow` | expected `> 60s` for some shards | runtime / GitHub | `release_gate`, `slow_proof` | deterministic | high | required when slow-proof runs in CI or release mode | parallelize `N=1..4` whenever the lane is selected | This is the preferred B-lane fan-out shape already documented in the CI runtime guide and PVF policy. |
| Runtime family: `runtime` | `slow-proof-runtime` family from `adl/config/slow_proof_families.v0.91.6.json` | observed `> 60s` examples exist | runtime | `slow_proof` | deterministic | high | release-gated heavy proof | parallelize by shard/family; keep one authoritative barrier | Sample tests include `runtime_v2_csm_governed_episode_writes_without_path_leakage` and `runtime_v2_theory_of_mind_foundation_write_to_root_materializes_fixture`. |
| Runtime family: `private_state` | `slow-proof-private-state` family from `adl/config/slow_proof_families.v0.91.6.json` | expected `> 60s` family-level proof | runtime | `slow_proof` | deterministic | high | release-gated heavy proof | parallelize separately from ordinary fast proof | Private-state lineage, observatory materialization, and sealed-state proof should stay isolated from the short lane. |
| Runtime family: `observatory` | `slow-proof-observatory` family from `adl/config/slow_proof_families.v0.91.6.json` | observed `> 60s` examples exist in prior runtime timing evidence | runtime | `slow_proof` | deterministic | high | release-gated heavy proof | parallelize separately; keep explicit barrier before milestone/release claims | Prior timing evidence shows `observatory_flagship` tests landing well above the threshold. |
| Runtime family: `security` | `slow-proof-security` family from `adl/config/slow_proof_families.v0.91.6.json` | expected `> 60s` family-level proof | runtime | `slow_proof` | deterministic | high | release-gated heavy proof | parallelize separately from `A_small` | Security-boundary and access-control materialization proof should remain explicit and not be folded into ordinary correctness smoke. |
| Shell validation script | `bash adl/tools/run_authoritative_coverage_lane.sh` | expected `> 60s` | tools | `release_gate`, `coverage_only` | deterministic | high | authoritative on `push_main`, nightly, and policy-authority events | keep as its own B lane; do not shard inside the same build root | Full workspace `cargo llvm-cov nextest` is expensive even when bounded to default features. |
| Shell validation script | `bash adl/tools/run_local_authoritative_coverage_gate.sh` | expected `> 60s` | tools | `release_gate`, `coverage_only` | deterministic | high | local authoritative gate only | serial local lane; do not run beside another coverage lane on the same checkout | Runs the authoritative coverage lane plus local gate enforcement; intended as a heavyweight local proof surface. |
| Demo / proof command | `bash adl/tools/run_v0913_proof_validation_lane.sh` | expected `> 60s` for the full lane | tools | `integration_worktree` | deterministic | medium-high | not a generic ordinary PR lane | keep separate from `A_small`; internal steps are currently serial | This command validates multiple retained `v0.91.3` proof packets and contract suites in one pass. |
| Provider/runtime check | `bash adl/tools/run_uts_benchmark.sh` | expected `> 60s` | provider / tools | `provider_live` | non-deterministic relative to external systems | high / external | never ordinary PR-fast proof | isolate completely; no shared stream with `A_small` | Live benchmark/provider checks depend on credentials and external runtime conditions. |
| CI-only manifest-driven timing input | `adl-csdlc tooling ci-log-archive summarize ...` manifest entries with `lane_class=B_large` | future observed `> 60s` from archived CI logs | tools / GitHub | reviewable timing memory, later consumed by PVF | deterministic for summary generation | low for summarizer, evidence-backed for source lane | not itself a proof lane; input to the index | consume manifests asynchronously; do not require raw logs to stay local | `#4225` explicitly routes durable timing manifests into `#4223` so future updates do not depend on keeping raw CI log piles in Git or local temp storage. |

## Cross-Category Classification Notes

### Rust unit / integration tests

The tracked slow-proof family map is the main current source of long Rust
proof. The family split is already explicit:

- `runtime` -> `slow-proof-runtime`
- `private_state` -> `slow-proof-private-state`
- `observatory` -> `slow-proof-observatory`
- `security` -> `slow-proof-security`

These tests should stay out of the ordinary PR-fast nextest stream unless a
specific issue proves they no longer belong there.

### Shell validation scripts

Long shell lanes are primarily orchestration wrappers around:

- authoritative workspace coverage
- slow-proof family execution
- retained proof-packet validation

Short shell contracts such as path-policy, validation-manager, sprint
conductor helpers, and selector contracts remain `A_small` candidates.

### Demo / proof commands

The current long demo/proof posture is not “every demo is long.” The heavy
demo/proof class is the composed retained-proof packet lane such as
`run_v0913_proof_validation_lane.sh`, not the ordinary bounded demo smoke path.

### Provider / runtime checks

Provider-live and benchmark lanes belong in `B_large` even when their observed
duration is not freshly measured in this issue, because they are resource-heavy,
credential-bound, and not safe to hide inside ordinary deterministic local
proof.

### CI-only lanes

The `adl-slow-proof` GitHub job and authoritative coverage job are not optional
background noise. They are explicit `B_large` lanes whose status must remain
visible as `passed`, `failed`, `deferred`, `pending`, or
`release_gate_required`.

## A/B Validation-Pass Contract

### Threshold rule

- `A_small`: required validation work at or below `60` seconds, or otherwise
  classified as the ordinary short deterministic lane for the touched surface
- `B_large`: validation work above `60` seconds, or any lane explicitly tagged
  as `slow_proof`, `release_gate`, or `provider_live`, even when the exact
  timing for the current run is deferred to CI or prior evidence

### Required truth

- `A_small + B_large` is the minimum proof set when both classes are selected
- `A_small` may finish before `B_large`
- `A_small` success must not be restated as total validation success while any
  required `B_large` lane remains pending, deferred, blocked, or failed
- `B_large` must not block `A_small` by sharing the same execution stream
  unless the operator intentionally requests a single-stream run and records
  that override

### Construction rules

Build `A_small` from:

- docs/path/card/contract checks
- owner-lane checks
- ordinary PR-fast Rust lane
- focused demo smoke when the changed surface requires it
- small deterministic shell/python validators

Build `B_large` from:

- slow-proof family lanes
- authoritative coverage lanes
- provider-live / benchmark lanes
- multi-packet retained proof lanes expected to exceed the threshold
- future `#4225` CI timing manifest entries classified as `B_large`

### Fan-out rules

- Parallelize `B_large` by shard or family whenever the lane contract already
  supports it, such as `count:1/4 .. count:4/4` for `slow-proof-tests`
- Keep authoritative coverage as one explicit lane rather than pretending it is
  four independent proofs
- Keep provider-live lanes isolated from local deterministic lanes
- Use watchers or subagents to shepherd `B_large` lanes while `A_small`
  continues, especially during sprint execution and PR janitoring
- Join `A_small` and `B_large` only at an intentional barrier:
  PR merge readiness, release-mode gate, milestone closeout, or operator
  checkpoint

### Status vocabulary

`B_large` lanes must remain explicit as one of:

- `passed`
- `failed`
- `blocked`
- `pending`
- `deferred`
- `release_gate_required`
- `reused` only when the tracked lane contract proves reuse is valid

Aggregate green must not hide any other state.

## Example Validation Passes

### Example 1: docs/tooling issue with no long proof

Issue shape:

- docs/planning update
- path-policy or selector contracts only

Validation pass:

- `A_small`: `git diff --check`, milestone docs hygiene, relevant contract
  tests
- `B_large`: none

Barrier:

- none beyond ordinary issue review/publication

### Example 2: ordinary runtime PR with no slow-proof-family touch

Issue shape:

- bounded Rust/runtime change
- not classified into explicit slow-proof families

Validation pass:

- `A_small`: `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`,
  `cargo test --doc`, `bash adl/tools/run_pr_fast_test_lane.sh`, demo smoke if
  the changed surface requires it
- `B_large`: explicit `release_gate_required` slow-proof and/or authoritative
  coverage only when policy says they remain future proof rather than current PR
  proof

Barrier:

- PR publication may proceed only when the issue truth records the deferred
  `B_large` lanes explicitly

### Example 3: slow-proof or release validation pass

Issue or event shape:

- push to `main`
- release-mode proof
- explicit runtime slow-proof request

Validation pass:

- `A_small`: only the issue-local short contracts needed to confirm the
  triggering surface is intact
- `B_large`:
  - `cargo nextest run --features slow-proof-tests --partition count:1/4 ...`
  - `cargo nextest run --features slow-proof-tests --partition count:2/4 ...`
  - `cargo nextest run --features slow-proof-tests --partition count:3/4 ...`
  - `cargo nextest run --features slow-proof-tests --partition count:4/4 ...`
  - `bash adl/tools/run_authoritative_coverage_lane.sh`

Barrier:

- release or merge truth is blocked until all required B lanes settle

### Example 4: CI-log manifest refresh after `#4225`

Issue shape:

- no new runtime execution
- new durable timing manifests for prior CI runs

Validation pass:

- `A_small`: manifest/contract validation for the archive summary surface
- `B_large`: none newly executed in the local issue
- inventory update: consume `timing_summary` and `timing_entries` from the
  manifest to refresh which lanes are currently above the threshold

Barrier:

- the index update must not claim the raw CI lane passed again; it only updates
  the long-lane memory

## Follow-On Remediation Queue

This issue does not rewrite the slow surfaces, but it does identify the current
best follow-on buckets:

1. Runtime v2 registry/accessor clusters:
   route to consolidation or authoritative slow-proof reduction work
2. Proof-materialization families:
   keep one authoritative root-materialization proof and collapse repeated
   sibling setup where evidence allows
3. `observatory_flagship` and similar repeated high-runtime families:
   route to fixture/setup collapse rather than hiding them inside `A_small`
4. Retained multi-packet proof lanes such as `run_v0913_proof_validation_lane.sh`:
   split into clearer independently reportable B lanes when the retained packet
   set grows again
5. CI timing manifest ingestion:
   consume `#4225` manifests routinely so the long-lane index refreshes from
   durable timing memory rather than operator recollection

## Non-Claims

- This packet does not claim all long validation has been freshly rerun in
  `#4223`.
- This packet does not weaken slow-proof, provider-live, or authoritative
  coverage gates.
- This packet does not claim every demo/proof command is a long lane.
- This packet does not replace the validation selector, validation manager, CI
  path policy, or release-gate policy surfaces.
- This packet does not approve hiding `B_large` behind a green `A_small`.

# Get-Well Runtime Tracking - v0.90.5

## Purpose

This tracking artifact turns the v0.90.5 get-well plan into an execution queue
with a measurable baseline, runtime budget, and per-slice proof boundary.

It is not a replacement for the Governed Tools v1.0 work packages. It exists so
the GW wave can reduce validation cost before the main implementation band pays
the same expensive test setup repeatedly.

## Baseline

Source evidence:

- PR: `#2547`
- GitHub Actions run: `24914120505`
- Job: `adl-coverage`
- Nextest wall-clock summary: `660.944s`
- Tests over `60s`: `1`
- Unique tests over `45s`: `39`
- Deduped cumulative runtime for tests over `45s`: `1938.389s`

Interpretation:

- The runtime problem is concentrated in a small number of heavyweight proof
  families.
- The repeated `~47s` to `~51s` clusters indicate duplicated setup rather than
  one isolated slow assertion.
- The GW wave should reduce repeated setup while keeping at least one explicit
  end-to-end proof path for each affected feature family.

## Runtime Budget

| Metric | Baseline | v0.90.5 GW Target | Notes |
| --- | ---: | ---: | --- |
| Authoritative coverage wall time | `660.944s` | `< 540s` after GW-05 | Target is directional until GitHub runner variance is measured. |
| Unique tests over `45s` | `39` | `< 18` after GW-05 | Each family collapse should remove multiple repeated setup payments. |
| Tests over `60s` | `1` | `0` after GW-05 | The CLI proof-matrix tail is the only known over-60s test. |
| Deduped runtime for tests over `45s` | `1938.389s` | `< 900s` after GW-05 | Aggregate runtime can exceed wall time because tests run in parallel. |

Budget rules:

- A slice can be accepted with partial improvement if it records the remaining
  hotspot clearly and does not weaken proof boundaries.
- A slice should not claim global runtime success from focused local tests.
  Authoritative wall-time movement is measured from GitHub `adl-coverage`.
- If a slice discovers a different dominant hotspot, update this artifact
  before moving to the next slice.

## Slice Tracking

| Slice | Issue | Hotspot family | Baseline signal | Expected proof boundary | Focused validation |
| --- | --- | --- | --- | --- | --- |
| GW-01 | `#2593` | `runtime_v2::tests::external_counterparty` | `12` tests over `45s` | Preserve contract, golden, denial-membership, authority-drift, and materialization assertions while reducing repeated setup. | `cargo test --manifest-path adl/Cargo.toml runtime_v2_external_counterparty -- --nocapture` |
| GW-02 | `#2594` | `runtime_v2::tests::private_state_observatory` | `8` tests over `45s` | Preserve privacy, redaction, audience-view, evidence, negative-case, and materialization assertions while reducing repeated setup. | `cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_observatory -- --nocapture` |
| GW-03 | `#2595` | `runtime_v2::tests::delegation_subcontract` | `7` tests over `45s` | Preserve authority, parent accountability, explicit subcontractor-selection basis, negative-case, golden, and materialization assertions while reducing repeated setup. | `cargo test --manifest-path adl/Cargo.toml runtime_v2_delegation_subcontract -- --nocapture` |
| GW-04 | `#2596` | `runtime_v2::tests::contract_market_demo` and `runtime_v2::tests::resource_stewardship_bridge` | `9` combined tests over `45s` | Preserve D12 proof coherence, authority boundaries, resource stewardship accounting, negative-case, golden, and materialization assertions while reducing repeated setup. | `cargo test --manifest-path adl/Cargo.toml runtime_v2_contract_market_demo -- --nocapture` and `cargo test --manifest-path adl/Cargo.toml runtime_v2_resource_stewardship_bridge -- --nocapture` |
| GW-05 | `#2597` | CLI/demo proof-matrix tail | `3` named tests over `45s`, including one at `86.819s` | Keep one true CLI/demo wiring smoke and move repeated proof-matrix assertions to cheaper direct tests only when the same proof boundary remains visible. | Focused filters for `runtime_v2_feature_proof_coverage`, `runtime_v2_contract_market_demo_validates_stdout_help_and_output_path_rules`, and any moved direct proof tests. |

## Execution Rules

- Run GW-00 through GW-05 before WP-02 begins when practical.
- Execute one GW slice at a time.
- Review and close out each slice before starting the next.
- Do not weaken negative safety, authority, redaction, evidence, or denial
  proof requirements to reduce runtime.
- Do not make broad CI policy changes inside a family-collapse issue unless
  the issue explicitly requires them.

## Review Checklist

For each GW slice, confirm:

- The focused test still proves the original family contract.
- The number of heavyweight sibling tests is reduced or the residual cost is
  explicitly measured.
- At least one end-to-end proof path remains where the feature family needs it.
- The get-well plan or this tracking artifact records any changed hotspot
  picture.
- The PR is watched through green checks, merged, and closed out before the next
  GW issue starts.

## Final Disposition

WP-20 must record the final GW wave outcome before release closeout:

- completed slices and measured effect
- deferred slices and rationale
- updated authoritative coverage wall-time evidence
- remaining tests over `45s` and over `60s`

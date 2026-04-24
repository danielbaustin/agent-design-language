# Feature Proof Coverage - v0.90.4

## Status

WP-14A / D13 is landed. Every v0.90.4 contract-market feature claim now has one
explicit proof home before WP-15 docs, quality, and review convergence.

This record is a reviewer map with matching runtime evidence. It does not grant
new execution authority by itself. It binds the demo matrix to the landed proof
docs, fixtures, focused tests, the generated
`runtime_v2/feature_proof_coverage/feature_proof_coverage.json` packet, and the
explicit non-proving governed-tool handoff boundary.

## Coverage Rule

Each feature claim must have one of:

- runnable demo command
- test-backed proof packet
- fixture-backed artifact
- documented non-proving status
- explicit deferral with owner and rationale

For v0.90.4, D1 through D13 are landed and proving. Governed-tool execution
claims remain explicitly non-proving and deferred to v0.90.5; they are tracked
as boundary notes and negative-case proof, not as landed execution authority.

## Working Demo Commands

| Demo | Working command |
| --- | --- |
| D1 | `test -f docs/milestones/v0.90.4/ECONOMICS_INHERITANCE_AND_AUTHORITY_AUDIT_v0.90.4.md` |
| D2 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_contract_schema -- --nocapture` |
| D3 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_bid_schema -- --nocapture` |
| D4 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_evaluation_selection -- --nocapture` |
| D5 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_transition_authority -- --nocapture && cargo test --manifest-path adl/Cargo.toml runtime_v2_contract_lifecycle -- --nocapture` |
| D6 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_external_counterparty -- --nocapture` |
| D7 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_delegation_subcontract -- --nocapture` |
| D8 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_resource_stewardship_bridge -- --nocapture` |
| D9 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_contract_market_demo_review_surfaces_are_stable -- --nocapture` |
| D10 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_contract_market_demo -- --nocapture && cargo run --manifest-path adl/Cargo.toml -- runtime-v2 contract-market-demo --out artifacts/v0904/demo-d12-contract-market` |
| D11 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_contract_market_demo_review_surfaces_are_stable -- --nocapture` |
| D12 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_contract_market_demo -- --nocapture && cargo run --manifest-path adl/Cargo.toml -- runtime-v2 contract-market-demo --out artifacts/v0904/demo-d12-contract-market` |
| D13 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_feature_proof_coverage -- --nocapture && cargo run --manifest-path adl/Cargo.toml -- runtime-v2 feature-proof-coverage --out artifacts/v0904/feature-proof-coverage.json` |

## Coverage Table

| Demo | Owner | Status | Coverage Kind | Primary Evidence | Validation |
| --- | --- | --- | --- | --- | --- |
| D1 Economics authority inheritance audit | WP-02 | LANDED | fixture-backed audit | `ECONOMICS_INHERITANCE_AND_AUTHORITY_AUDIT_v0.90.4.md` | audit doc exists and records inherited, fixture-backed, deferred, or blocked authority surfaces |
| D2 Contract schema fixture | WP-03 | LANDED | test-backed proof packet | `features/CONTRACT_AND_BID_SCHEMA.md`, `contract_market/parent_contract.json`, `contract_market/contract_negative_cases.json` | `runtime_v2_contract_schema` focused tests |
| D3 Bid schema fixture | WP-04 | LANDED | test-backed proof packet | `features/CONTRACT_AND_BID_SCHEMA.md`, `contract_market/bid_alpha.json`, `contract_market/bid_bravo.json`, `contract_market/bid_negative_cases.json` | `runtime_v2_bid_schema` focused tests |
| D4 Evaluation artifact | WP-05 | LANDED | test-backed proof packet | `features/EVALUATION_AND_TRANSITION_AUTHORITY.md`, `contract_market/evaluation_selection.json`, `contract_market/selection_negative_cases.json` | `runtime_v2_evaluation_selection` focused tests |
| D5 Transition authority and lifecycle | WP-06 / WP-07 | LANDED | test-backed proof packet | `features/EVALUATION_AND_TRANSITION_AUTHORITY.md`, transition authority matrix/basis/negative cases, lifecycle state machine/negative cases | `runtime_v2_transition_authority`, `runtime_v2_contract_lifecycle` focused tests |
| D6 External counterparty boundary | WP-08 | LANDED | test-backed proof packet | `features/COUNTERPARTY_AND_DELEGATION.md`, `contract_market/external_counterparty_model.json`, `contract_market/external_counterparty_negative_cases.json` | `runtime_v2_external_counterparty` focused tests |
| D7 Delegation and subcontract | WP-09 | LANDED | test-backed proof packet | `features/COUNTERPARTY_AND_DELEGATION.md`, delegation subcontract, delegated output, parent integration, delegation negative cases | `runtime_v2_delegation_subcontract` focused tests |
| D8 Resource stewardship bridge | WP-10 | LANDED | test-backed proof packet | `features/RESOURCE_STEWARDSHIP_BRIDGE.md`, `contract_market/resource_stewardship_bridge.json` | `runtime_v2_resource_stewardship_bridge` focused tests |
| D9 Contract-market fixture set | WP-11 | LANDED | fixture-backed artifact set | `features/CONTRACT_MARKET_DEMO_AND_RUNNER.md`, parent contract, bids, evaluation, delegation, delegated output, parent integration fixtures | `runtime_v2_contract_market_demo_review_surfaces_are_stable` proves the packet stays coherent and manifest-linked |
| D10 Contract-market runner | WP-12 | LANDED | runnable demo command | `features/CONTRACT_MARKET_DEMO_AND_RUNNER.md`, `runtime_v2/contract_market/proof_packet.json`, `runtime_v2/contract_market/operator_report.md` | `runtime_v2_contract_market_demo` focused tests and bounded `contract-market-demo` command |
| D11 Review summary | WP-13 | LANDED | test-backed review surface | `features/CONTRACT_MARKET_DEMO_AND_RUNNER.md`, `runtime_v2/contract_market/review_summary_seed.md`, `runtime_v2/contract_market/operator_report.md` | `runtime_v2_contract_market_demo_review_surfaces_are_stable` |
| D12 Bounded contract-market proof | WP-14 | LANDED | runnable demo command | `features/CONTRACT_MARKET_DEMO_AND_RUNNER.md`, `runtime_v2/contract_market/proof_packet.json`, `runtime_v2/contract_market/negative_packet.json`, `runtime_v2/contract_market/operator_report.md` | `runtime_v2_contract_market_demo` focused tests and bounded `contract-market-demo` command |
| D13 Feature proof coverage | WP-14A | LANDED | runnable coverage packet | `FEATURE_PROOF_COVERAGE_v0.90.4.md`, `DEMO_MATRIX_v0.90.4.md`, `runtime_v2/feature_proof_coverage/feature_proof_coverage.json` | `runtime_v2_feature_proof_coverage` focused tests and bounded `feature-proof-coverage` command |

## Non-Proving Governed-Tool Handoff Claims

The following claim family is explicit but non-proving in v0.90.4:

- tool requirements may appear in contracts, bids, evaluation notes, resource
  estimates, review summaries, and the D12 operator report as constraints,
  evidence requirements, or denied attempts
- those surfaces do **not** grant execution authority
- governed-tool execution, UTS, ACC, registry binding, replay/redaction, and
  executor authority remain deferred to v0.90.5

Evidence for that non-proving boundary is still reviewable:

- `docs/milestones/v0.90.4/ideas/V0905_GOVERNED_TOOLS_HANDOFF.md`
- `docs/milestones/v0.90.4/features/CONTRACT_MARKET_DEMO_AND_RUNNER.md`
- `adl/tests/fixtures/runtime_v2/contract_market/contract_negative_cases.json`
- `adl/tests/fixtures/runtime_v2/contract_market/bid_negative_cases.json`
- `adl/tests/fixtures/runtime_v2/contract_market/resource_stewardship_bridge.json`
- `runtime_v2/contract_market/operator_report.md`
- `runtime_v2/contract_market/negative_packet.json`

## Named Deferrals

No v0.90.4 feature claim is left without a proof home. The following later-band
claims remain explicitly deferred:

- payment settlement, Lightning, x402, banking, invoicing, tax, and legal
  contracting
- governed tool execution, UTS, ACC, registry binding, and executor authority
- full inter-polis economics and open-ended autonomous markets
- v0.91 moral governance, emotional civilization, humor, and wellbeing scope
- v0.92 identity, capability rebinding, migration, and birthday semantics

## Non-Claims

This D13 record does not add new runtime behavior beyond the referenced D1
through D12 evidence surfaces. It also does not claim:

- payment settlement or financial rails
- direct tool execution authority
- production legal contracting or tax handling
- redefinition of citizen standing or private-state authority
- full inter-polis economics
- v0.91 or v0.92 scope

## WP-15 Handoff

WP-15 should consume this D13 record during docs, quality, and review
convergence. If WP-15 identifies a missing or overstated claim, it should
either link the claim to one of the proof surfaces above, correct the claim, or
create an explicit deferral with owner and rationale.

# Fixture Definition: Contract Market Baseline

## Fixture Metadata

- `fixture_id`: `contract-market-v0.90.4-baseline-01`
- `fixture_version`: `v0.90.4-01`
- `goal`: validate the first bounded contract-market packet before runner execution exists
- `source_mode`: `fixture_contract`

## Purpose

This fixture packet is the bounded source of truth for the first v0.90.4
contract-market proof.

It is intentionally small and reviewer-visible. It proves that the milestone
has a coherent artifact packet for one parent contract, two bids, one
evaluation, one subcontract, one delegated output, one parent integration
result, one completion event, one trace bundle, one review-summary seed, and
one demo manifest.

This fixture packet does not prove:

- payment settlement
- pricing or billing rails
- legal contracting
- governed tool execution
- production counterparty identity verification
- full inter-polis economics

## Packet Contents

The canonical packet is all files under `demos/fixtures/contract_market/`.

Required artifact set:

- `packet_manifest.json`
- `parent_contract.json`
- `bid_alpha.json`
- `bid_beta.json`
- `evaluation.json`
- `award_transition.json`
- `acceptance_transition.json`
- `subcontract.json`
- `delegated_output.json`
- `parent_integration_output.json`
- `completion_event.json`
- `trace_bundle.json`
- `review_summary_seed.json`
- `demo_manifest.json`
- `tool_requirement_fixture.json`

The negative packet for later denial cases lives in
`demos/fixtures/contract_market_invalid/`.

## Contract Rule

Tool requirements are allowed only as requirements or constraints.

They must not:

- grant execution authority
- imply governed-tools v1.0 is already landed
- bypass future UTS or ACC policy

## Expected Validation

- all packet JSON files parse successfully
- `packet_manifest.json` identifies each artifact and its proof purpose
- all listed artifact paths exist
- no file contains local absolute host paths
- tool requirements are represented as constraints, not execution grants
- the negative packet remains intentionally invalid and carries explicit reasons

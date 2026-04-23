# Feature Docs - v0.90.4

## Implementation-Facing Features

| Feature Doc | Purpose | Execution WPs |
| --- | --- | --- |
| features/CONTRACT_AND_BID_SCHEMA.md | parent contract and bid artifacts, required fields, validation, and examples | WP-03, WP-04 |
| features/EVALUATION_AND_TRANSITION_AUTHORITY.md | scorecards, selection, overrides, state transitions, and authority checks | WP-05, WP-06, WP-07 |
| features/COUNTERPARTY_AND_DELEGATION.md | external counterparty limits, sponsorship/gateway rules, delegation, subcontracting, and parent responsibility | WP-08, WP-09 |
| features/RESOURCE_STEWARDSHIP_BRIDGE.md | compute, memory, attention, bandwidth, artifact storage, review time, and later economics boundaries without payment rails | WP-10 |
| features/CONTRACT_MARKET_DEMO_AND_RUNNER.md | fixture set, deterministic runner, proof packet, negative cases, and review summary | WP-11 through WP-14 |

## Proof And Audit Docs

| Doc | Purpose | Execution WPs |
| --- | --- | --- |
| ECONOMICS_INHERITANCE_AND_AUTHORITY_AUDIT_v0.90.4.md | map which v0.90.3 authority surfaces are inherited, fixture-backed, or deferred for contract-market work | WP-02 / #2421 |
| WBS_v0.90.4.md | opened execution map for the contract-market substrate milestone | WP-01 / #2420 |
| DEMO_MATRIX_v0.90.4.md | live proof matrix and non-proving boundaries | WP-01 / #2420, WP-14 / #2433, WP-14A / #2434 |
| WP_ISSUE_WAVE_v0.90.4.yaml | opened issue-wave source of truth with real issue numbers | WP-01 / #2420 |
| WP_EXECUTION_READINESS_v0.90.4.md | card-authoring source for concrete WP outputs and validation | WP-01 / #2420 |

## Context / Idea Docs

| Idea Doc | Purpose | Boundary |
| --- | --- | --- |
| ideas/V0903_CITIZEN_STATE_DEPENDENCY.md | explain why economics depends on citizen-state authority | context only |
| ideas/PAYMENT_AND_INTERPOLIS_DEFERRAL.md | preserve payment and inter-polis boundaries | context only |
| ideas/V0905_GOVERNED_TOOLS_HANDOFF.md | preserve the boundary between contract-market requirements and governed tool-call execution | context only |

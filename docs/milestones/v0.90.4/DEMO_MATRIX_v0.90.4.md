# Demo Matrix - v0.90.4

## Status

Planning draft. No v0.90.4 issue wave has been opened yet.

| ID | Demo | WP | Proof Claim | Required Artifacts | Status |
| --- | --- | --- | --- | --- | --- |
| D1 | Economics authority inheritance audit | WP-02 | v0.90.4 consumes v0.90.3 standing, access, projection, and continuity authority rather than redefining it | authority dependency report | PLANNED |
| D2 | Contract schema fixture | WP-03 | Contract scope, parties, deliverables, process, constraints, evaluation, artifacts, and trace requirements are explicit | contract schema, valid fixture, invalid fixture | PLANNED |
| D3 | Bid schema fixture | WP-04 | Bids declare agent, proposal, cost, confidence, commitments, exceptions, and trace/signature requirements | bid schema, two valid bids, invalid bid case | PLANNED |
| D4 | Evaluation artifact | WP-05 | Bid selection is reviewable through mandatory checks, scorecards, aggregation, and override evidence | evaluation artifact and selection rationale | PLANNED |
| D5 | Transition authority | WP-06 / WP-07 | Contract lifecycle transitions are allowed only by authorized actors | transition matrix, lifecycle fixtures, denial cases | PLANNED |
| D6 | External counterparty boundary | WP-08 | External counterparties can participate only through explicit trust and gateway limits | counterparty fixture, denial case | PLANNED |
| D7 | Delegation and subcontract | WP-09 | Subcontracts preserve parent responsibility and trace linkage | subcontract fixture, delegated output, parent link | PLANNED |
| D8 | Contract-market fixture set | WP-10 | One coherent fixture packet contains parent contract, bids, evaluation, subcontract, output, trace, and manifest | fixture packet and manifest | PLANNED |
| D9 | Contract-market runner | WP-11 | A deterministic runner validates fixtures and emits transition/review artifacts | runner output, artifact manifest | PLANNED |
| D10 | Review summary | WP-12 | Reviewers can inspect scope, participants, selection, execution, artifacts, trace, validation, and caveats | summary artifact | PLANNED |
| D11 | Bounded contract-market proof | WP-13 | One parent contract can receive bids, award, accept, delegate, integrate, complete, and summarize | proof packet and operator report | PLANNED |
| D12 | Negative authority and trace proof | WP-14 | Unauthorized transitions, invalid bids, unsupported delegation, and missing trace links fail safely | negative test packet | PLANNED |

## Non-Proving Boundaries

- These demos do not prove payment settlement.
- These demos do not prove Lightning, x402, banking, tax, or legal contracting.
- These demos do not prove full inter-polis economics.
- These demos do not prove production counterparty identity verification.
- These demos do not redefine citizen standing or private-state authority.
- These demos prove bounded contract-market mechanics, not a complete economy.

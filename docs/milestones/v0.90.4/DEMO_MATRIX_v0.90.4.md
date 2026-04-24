# Demo Matrix - v0.90.4

## Status

WP-14A is complete. v0.90.4 demos map to execution issues #2421 through #2434,
and those proof-producing WPs are now closed. This matrix is the current
feature proof coverage record for the milestone until later review or
remediation changes land.

| ID | Demo | WP | Proof Claim | Required Artifacts | Status |
| --- | --- | --- | --- | --- | --- |
| D1 | Economics authority inheritance audit | WP-02 | v0.90.4 consumes v0.90.3 standing, access, projection, and continuity authority rather than redefining it | authority dependency report | LANDED |
| D2 | Contract schema fixture | WP-03 | Contract scope, parties, deliverables, process, constraints, evaluation, artifacts, and trace requirements are explicit | contract schema, valid fixture, invalid fixture | LANDED |
| D3 | Bid schema fixture | WP-04 | Bids declare agent, proposal, cost, confidence, commitments, exceptions, and trace/signature requirements | bid schema, two valid bids, invalid bid case | LANDED |
| D4 | Evaluation artifact | WP-05 | Bid selection is reviewable through mandatory checks, scorecards, aggregation, and override evidence | evaluation artifact and selection rationale | LANDED |
| D5 | Transition authority | WP-06 / WP-07 | Contract lifecycle transitions are allowed only by authorized actors | transition matrix, lifecycle fixtures, denial cases | LANDED |
| D6 | External counterparty boundary | WP-08 | External counterparties can participate only through explicit trust and gateway limits | counterparty fixture, denial case | LANDED |
| D7 | Delegation and subcontract | WP-09 | Subcontracts preserve parent responsibility and trace linkage | subcontract fixture, delegated output, parent link | LANDED |
| D8 | Resource stewardship bridge | WP-10 | Contract and bid artifacts can declare resource estimates without becoming payment or pricing rails | resource fixture and boundary note | LANDED |
| D9 | Contract-market fixture set | WP-11 | One coherent fixture packet contains parent contract, bids, evaluation, subcontract, output, trace, summary seed, and manifest | fixture packet and manifest | LANDED |
| D10 | Contract-market runner | WP-12 | A deterministic runner validates fixtures and emits transition/review artifacts | runner output, artifact manifest | LANDED |
| D11 | Review summary | WP-13 | Reviewers can inspect scope, participants, selection, execution, artifacts, trace, validation, and caveats | summary artifact | LANDED |
| D12 | Bounded contract-market proof | WP-14 | One parent contract can receive bids, award, accept, delegate, integrate, complete, summarize, and reject unsafe variants | proof packet, operator report, negative test packet | LANDED |
| D13 | Feature proof coverage | WP-14A | Every feature claim has a landed, skipped, failed, non-proving, or explicitly deferred proof status | this demo matrix | LANDED |

## Non-Proving Boundaries

- These demos do not prove payment settlement.
- These demos do not prove Lightning, x402, banking, tax, or legal contracting.
- These demos do not prove full inter-polis economics.
- These demos do not prove production counterparty identity verification.
- These demos do not redefine citizen standing or private-state authority.
- These demos do not prove Governed Tools v1.0, UTS, ACC, tool registry binding,
  or direct tool execution. Contracts may record tool requirements only as
  constraints and evidence until v0.90.5.
- These demos prove bounded contract-market mechanics, not a complete economy.

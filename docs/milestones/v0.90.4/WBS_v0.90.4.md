# WBS - v0.90.4

## Work Package Shape

v0.90.4 should use the standard 20-WP shape. The first sprint should inherit
v0.90.3 citizen-state authority, then lock schemas and lifecycle before the
runner and proof demo widen.

| WP | Issue | Title | Purpose | Primary Output | Depends On |
| --- | --- | --- | --- | --- | --- |
| WP-01 | planned | Promote v0.90.4 milestone package | Finalize this planning package and create the issue wave | tracked v0.90.4 docs and issue cards | v0.90.3 closeout |
| WP-02 | planned | Economics inheritance and authority audit | Compare v0.90.3 citizen-state outputs against v0.90.4 market requirements | authority dependency report | WP-01 |
| WP-03 | planned | Contract schema | Define the parent contract artifact and validation rules | contract schema, examples, negative fixtures | WP-02 |
| WP-04 | planned | Bid schema | Define bid artifact, cost, confidence, commitments, exceptions, and trace requirements | bid schema and bid fixtures | WP-03 |
| WP-05 | planned | Evaluation and selection model | Define scorecards, mandatory checks, aggregation, override, and tie-break rules | evaluation artifact and selection tests | WP-03, WP-04 |
| WP-06 | planned | Transition authority model | Define who may move a contract between lifecycle states | transition matrix and authority tests | WP-02-WP-05 |
| WP-07 | planned | Contract lifecycle state | Implement lifecycle states and transition validation | lifecycle state machine and fixtures | WP-06 |
| WP-08 | planned | External counterparty model | Bound non-citizen participation through identity, assurance, sponsor, and gateway rules | counterparty schema and denial cases | WP-02, WP-06 |
| WP-09 | planned | Delegation and subcontract model | Link subcontracts to parent responsibility without authority leakage | subcontract schema and trace-link tests | WP-03-WP-08 |
| WP-10 | planned | Contract-market fixture set | Produce canonical parent, bids, evaluation, subcontract, output, and manifest fixtures | fixture packet | WP-03-WP-09 |
| WP-11 | planned | Contract-market runner | Validate fixtures and emit transition/review artifacts deterministically | runner and proof artifacts | WP-10 |
| WP-12 | planned | Review summary shape | Produce reviewer-facing summaries of market execution and residual risk | summary schema and example | WP-10, WP-11 |
| WP-13 | planned | Bounded contract-market demo | Prove the parent contract, bids, award, delegation, completion, and summary end to end | contract-market proof packet | WP-10-WP-12 |
| WP-14 | planned | Negative authority and trace cases | Prove unauthorized transitions, invalid bids, unsupported delegation, and missing traces fail safely | negative test packet | WP-06-WP-13 |
| WP-15 | planned | Resource stewardship bridge | Connect contract-market execution to compute, memory, attention, and bandwidth without payment rails | resource bridge decision and fixture | WP-13, WP-14 |
| WP-16 | planned | Docs, feature index, and demo matrix convergence | Align docs, feature docs, demo matrix, and reviewer entry surfaces | coherent docs package | WP-03-WP-15 |
| WP-17 | planned | Internal review | Review claims, authority boundaries, fixtures, and proof packets | findings-first internal review | WP-16 |
| WP-18 | planned | External review and remediation | Run external review and fix or defer accepted findings | review packet and remediation notes | WP-17 |
| WP-19 | planned | Next milestone planning handoff | Prepare v0.91/v0.92/v0.90.5 or payment-lane handoff as appropriate | handoff docs and backlog updates | WP-18 |
| WP-20 | planned | Release ceremony | Complete release closure | release notes, ceremony result, next handoff | WP-19 |

## Compression Candidate

v0.90.4 can compress if WP-03 through WP-07 produce stable schemas and authority
fixtures early. Runtime implementation should not widen before contract, bid,
evaluation, transition, and lifecycle semantics are reviewable.

Compression must not skip:

- standing and access-control dependency checks
- external counterparty boundaries
- unauthorized transition negative tests
- delegation authority checks
- review summary truth
- release-truth checks

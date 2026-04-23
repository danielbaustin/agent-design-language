# WBS - v0.90.4

## Work Package Shape

v0.90.4 should use the standard WP-01 through WP-20 release shape, with an
explicit WP-14A demo/proof lane before quality and review convergence. The
first sprint should inherit v0.90.3 citizen-state authority, then lock schemas,
fixtures, validators, and lifecycle before the runner and proof demo widen.

| WP | Issue | Title | Purpose | Primary Output | Depends On |
| --- | --- | --- | --- | --- | --- |
| WP-01 | planned | Promote v0.90.4 milestone package | Finalize this planning package, create the issue wave, and author cards from WP_EXECUTION_READINESS_v0.90.4.md | tracked v0.90.4 docs and issue cards | v0.90.3 closeout |
| WP-02 | planned | Economics inheritance and authority audit | Compare v0.90.3 citizen-state outputs against v0.90.4 market requirements | authority dependency report | WP-01 |
| WP-03 | planned | Contract schema | Define the parent contract artifact and validation rules | contract schema, examples, negative fixtures | WP-02 |
| WP-04 | planned | Bid schema | Define bid artifact, cost, confidence, commitments, exceptions, and trace requirements | bid schema and bid fixtures | WP-03 |
| WP-05 | planned | Evaluation and selection model | Define scorecards, mandatory checks, aggregation, override, and tie-break rules | evaluation artifact and selection tests | WP-03, WP-04 |
| WP-06 | planned | Transition authority model | Define who may move a contract between lifecycle states | transition matrix and authority tests | WP-02-WP-05 |
| WP-07 | planned | Contract lifecycle state | Implement lifecycle states and transition validation | lifecycle state machine and fixtures | WP-06 |
| WP-08 | planned | External counterparty model | Bound non-citizen participation through identity, assurance, sponsor, and gateway rules | counterparty schema and denial cases | WP-02, WP-06 |
| WP-09 | planned | Delegation and subcontract model | Link subcontracts to parent responsibility without authority leakage | subcontract schema and trace-link tests | WP-03-WP-08 |
| WP-10 | planned | Resource stewardship bridge | Connect contract-market execution to compute, memory, attention, bandwidth, artifact storage, review time, and tool-resource requirements without payment rails or tool-call authority | resource bridge decision, fixture, and boundary notes | WP-03-WP-09 |
| WP-11 | planned | Contract-market fixture set | Produce canonical parent, bids, evaluation, subcontract, output, trace, review-summary seed, and manifest fixtures | fixture packet | WP-03-WP-10 |
| WP-12 | planned | Contract-market runner | Validate fixtures and emit transition/review artifacts deterministically | runner and proof artifacts | WP-11 |
| WP-13 | planned | Review summary shape | Produce reviewer-facing summaries of market execution and residual risk | summary schema and example | WP-11, WP-12 |
| WP-14 | planned | Bounded contract-market demo and negative cases | Prove the parent contract, bids, award, delegation, completion, summary, and expected denial cases end to end | contract-market proof packet and negative test packet | WP-06-WP-13 |
| WP-14A | planned | Demo matrix and feature proof demos | Verify every contract-market feature claim has a runnable demo, proof packet, fixture-backed artifact, non-proving status, or explicit deferral | demo matrix update and feature proof coverage record | WP-03-WP-14 |
| WP-15 | planned | Quality gate, docs, and review convergence | Align quality posture, docs, feature docs, reviewer entry surfaces, and completed demo/proof coverage | coherent quality gate and docs/review package | WP-14A |
| WP-16 | planned | Internal review | Run findings-first internal review over code, docs, tests, demos, issue truth, and release boundaries | internal review packet and finding register | WP-15 |
| WP-17 | planned | External review | Run bounded external review and record findings or zero-finding disposition | third-party review record | WP-16 |
| WP-18 | planned | Review findings remediation | Fix accepted internal/external findings or defer explicitly with owner and rationale | remediation PRs or deferral records | WP-16, WP-17 |
| WP-19 | planned | Next milestone planning handoff | Prepare v0.90.5 governed-tools handoff plus v0.91/v0.92/payment-lane follow-ups as appropriate | handoff docs and backlog updates | WP-18 |
| WP-20 | planned | Release ceremony | Complete release closure | release notes, ceremony result, end-of-milestone report, next handoff | WP-19 |

## Compression Candidate

v0.90.4 can compress if WP-03 through WP-07 produce stable schemas and authority
fixtures early. Runtime implementation should not widen before contract, bid,
evaluation, transition, and lifecycle semantics are reviewable.

Compression must not skip:

- standing and access-control dependency checks
- external counterparty boundaries
- unauthorized transition negative tests
- delegation authority checks
- governed-tool boundary checks when contracts or bids mention tool-mediated work
- review summary truth
- feature-by-feature demo/proof coverage before docs/review convergence
- internal review, external review, and accepted-finding remediation
- release-truth checks

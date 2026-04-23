# Decisions - v0.90.4

| ID | Decision | Status | Rationale |
| --- | --- | --- | --- |
| D-01 | Treat v0.90.4 as the citizen economics and contract-market substrate milestone | Accepted | The economics docs are substantial enough to need their own milestone rather than distorting v0.90.3 citizen-state safety. |
| D-02 | Consume v0.90.3 standing and citizen-state authority | Accepted | Economics must not redefine citizenship, private state, projection, access control, challenge, or appeal. |
| D-03 | Prove contracts before payments | Accepted | Contract authority and traceability are prerequisites for any later payment or settlement rail. |
| D-04 | Use fixture-first schemas for contracts, bids, evaluations, and review summaries | Accepted | Stable artifacts make the runner and demo reviewable. |
| D-05 | Treat external counterparties as bounded participants, not citizens by default | Accepted | External participation needs identity, assurance, sponsor/gateway, and trace limits. |
| D-06 | Keep delegation parent-responsible | Accepted | Subcontracting must not let a parent contract launder responsibility or authority. |
| D-07 | Require negative authority and trace cases | Accepted | A market proof is not credible unless invalid transitions, unsupported delegation, and missing trace links fail safely. |
| D-08 | Keep v0.90.4 readout reviewer-facing | Accepted | The proof should culminate in a summary that makes scope, evidence, warnings, and residual risk legible. |
| D-09 | Defer full inter-polis economics | Accepted | Cross-polis economics depends on stronger identity, reputation, payment, and governance layers. |
| D-10 | Defer real payment rails | Accepted | Payment settlement should wait until contract authority, lifecycle, review, and dispute evidence are stable. |
| D-11 | Preserve the demo-matrix WP before quality and review convergence | Accepted | Feature claims should not enter review without landed, skipped, failed, non-proving, or explicitly deferred proof status. |
| D-12 | Defer governed tool-call semantics to v0.90.5 | Accepted | Contracts may describe tool needs, but UTS, ACC, registry binding, executor authority, denial records, and tool-call model testing belong to the governed-tools milestone. |

# Sprint Plan - v0.90.4

## Sprint 1: Authority And Schema Foundation

- WP-01: Promote v0.90.4 milestone package
- WP-02: Economics inheritance and authority audit
- WP-03: Contract schema
- WP-04: Bid schema
- WP-05: Evaluation and selection model

Goal: make contracts and bids explicit before the market runner exists.

## Sprint 2: Lifecycle, Counterparties, And Delegation

- WP-06: Transition authority model
- WP-07: Contract lifecycle state
- WP-08: External counterparty model
- WP-09: Delegation and subcontract model
- WP-10: Resource stewardship bridge

Goal: prove that market state changes are authority-checked and traceable,
including the rule that tool-mediated requirements are constraints rather than
tool-call grants.

## Sprint 3: Runner, Proof, And Negative Cases

- WP-11: Contract-market fixture set
- WP-12: Contract-market runner
- WP-13: Review summary shape
- WP-14: Bounded contract-market demo and negative cases
- WP-14A: Demo matrix and feature proof demos

Goal: produce one reviewer-visible market proof without payment rails and
verify feature-by-feature demo/proof coverage before review convergence.

## Sprint 4: Review And Release

- WP-15: Quality gate, docs, and review convergence
- WP-16: Internal review
- WP-17: External review
- WP-18: Review findings remediation
- WP-19: Next milestone planning handoff
- WP-20: Release ceremony

Goal: close the milestone with truthful review, release, and handoff evidence.

## Parallelization Notes

WP-03 and WP-04 can begin after WP-02 if the authority audit is clear. WP-05
should wait for both schemas. WP-08 can run beside WP-07 if the transition
authority model is stable. WP-10 can begin once schemas identify resource and
tool-requirement claims. WP-14A through WP-20 should remain sequential.

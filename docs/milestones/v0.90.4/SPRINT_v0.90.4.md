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
- WP-10: Contract-market fixture set

Goal: prove that market state changes are authority-checked and traceable.

## Sprint 3: Runner, Proof, And Negative Cases

- WP-11: Contract-market runner
- WP-12: Review summary shape
- WP-13: Bounded contract-market demo
- WP-14: Negative authority and trace cases
- WP-15: Resource stewardship bridge and late authority boundary
- WP-16: Demo matrix and feature proof demos

Goal: produce one reviewer-visible market proof without payment rails and
verify feature-by-feature demo/proof coverage before review convergence.

## Sprint 4: Review And Release

- WP-17: Docs, quality, and review convergence
- WP-18: External review and remediation
- WP-19: Next milestone planning handoff
- WP-20: Release ceremony

Goal: close the milestone with truthful review, release, and handoff evidence.

## Parallelization Notes

WP-03 and WP-04 can begin after WP-02 if the authority audit is clear. WP-05
should wait for both schemas. WP-08 can run beside WP-07 if the transition
authority model is stable. WP-16 through WP-20 should remain sequential.

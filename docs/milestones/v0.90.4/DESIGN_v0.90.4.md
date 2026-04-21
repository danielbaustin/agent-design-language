# Design - v0.90.4

## Design Center

The design center is a fixture-backed contract market inside the CSM polis.

The market is governed by three boundaries:

- standing: who may act, bid, inspect, accept, delegate, or challenge
- authority: which actor may move a contract from one lifecycle state to
  another
- evidence: which artifacts prove scope, bid, evaluation, delegation,
  completion, and review

## Core Objects

v0.90.4 should introduce or formalize these implementation-facing objects:

- contract
- bid
- evaluation artifact
- award transition
- acceptance transition
- subcontract
- delegated output
- parent integration output
- lifecycle event
- external counterparty record
- contract-market review summary
- demo manifest

## Contract Lifecycle

The planned lifecycle is:

1. draft
2. open
3. bidding
4. awarded
5. accepted
6. executing
7. completed

The lifecycle must also model failed, cancelled, and disputed outcomes.

Every state transition needs:

- source state
- target state
- actor
- authority basis
- timestamp or temporal anchor
- trace link
- validation result

## Authority Model

v0.90.4 should consume v0.90.3 standing and access-control rules. It should not
invent new citizenship or private-state authority.

Minimum authority rules:

- issuer may draft and open a contract when standing allows it
- eligible bidders may submit bids only during the bidding window
- evaluator or issuer authority must be explicit for award selection
- awarded party must accept before execution starts
- delegation requires explicit parent-contract permission
- subcontractors cannot silently inherit parent authority
- disputed or failed states preserve evidence for review

## External Counterparties

External participants may appear in v0.90.4 only through explicit counterparty
records. They are not citizens by default. They may submit bids or delegated
outputs only through declared trust, sponsor, gateway, and trace rules.

The milestone should preserve the human/citizen distinction: direct
out-of-band human action is not citizen action unless mediated by an authorized
CSM identity and recorded in trace.

## Demo Design

The bounded proof should use one parent contract, two bids, one evaluation
artifact, one award, one acceptance, one subcontract, one delegated output, one
parent integration, and one review summary.

The proof should include at least one negative case:

- invalid bid timing
- unauthorized award
- unsupported delegation
- missing trace link
- external counterparty without sufficient assurance

## Compression Design

Docs and fixtures should land before runtime code widens. The runner can be
small if the schema, lifecycle, authority, and review summary are precise.

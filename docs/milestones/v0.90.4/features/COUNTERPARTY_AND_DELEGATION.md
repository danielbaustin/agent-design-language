# Counterparty And Delegation - v0.90.4

## Purpose

Bound external participation and subcontracting without leaking authority.

## External Counterparties

External counterparties may participate only through explicit records. They are
not citizens by default and do not gain private-state inspection rights merely
by participating in a contract.

Minimum counterparty fields:

- counterparty id
- type
- identity status
- trust level
- sponsor or gateway, when required
- allowed actions
- trace requirements
- revocation behavior

## Delegation

Delegation lets a parent contract create a bounded subcontract. It must preserve
the parent contractor's responsibility and maintain trace linkage.

Required subcontract properties:

- parent contract id
- delegated scope
- delegating actor
- subcontracted actor
- authority basis
- inherited constraints
- deliverables
- trace links

## Authority Boundary

Subcontractors cannot silently inherit parent authority. They may perform only
the delegated scope, and parent integration remains a separate reviewable step.

## Required Negative Cases

- subcontract exceeds parent scope
- subcontract missing parent link
- external subcontractor lacks sufficient assurance
- delegated output is integrated without parent review
- revoked counterparty continues participation

## Summary
Execute **WP-08** in the v0.92 milestone: Birthday contract and negative cases.

## Required Outcome
Produce the deterministic birth contract, disqualifying cases, negative fixtures, and claim-boundary proof.

## Dependencies
- WP-01 issue #5817
- WP-02A issue #5801

## Proof Surface
Contract, schema, valid fixture, exhaustive negative fixtures, validation report, and bounded exact-head review.

## Acceptance Criteria
- The required outcome is implemented without substituting planning prose for working behavior.
- Every dependency is verified from current repository, typed receipt, and GitHub evidence.
- Birth rejects every missing, contradictory, forged, discontinuous, or prohibited evidence class with stable reasons.
- Applicable privacy, security, portability, failure, and claim boundaries are tested.
- One bounded pre-PR review has no unresolved actionable findings.
- The implementation PR includes `Closes #5825`.

## Non-goals
- Do not absorb identity, continuity, memory, capability, witness, or governance implementation.
- Do not rewrite historical evidence or claim downstream completion.

<!-- csdlc-github-operation:v092-wp08-readiness-reconciled -->

## Summary

Complete the WP-04 architecture, security, and child-wave gate for the v0.92 distributed Guardian/polis runtime program. This issue freezes the implementation contract and exact 16-child denominator; it does not claim distributed product implementation.

## Required Outcome

- Review and approve the distributed Guardian architecture and threat model.
- Validate exactly WP-04.01 through WP-04.16 with stable outcomes, dependencies, owners, disjoint protected paths, proof boundaries, and rollback responsibilities.
- Schedule the same-milestone `WP-04-IMP` implementation umbrella against exactly that denominator.
- Require all 16 child issues to be execution-ready before `WP-04-IMP` starts.

## Dependencies

- WP-03 issue #5820 must be terminal with stable Runtime ingress, lifecycle, state, and readiness contracts.
- WP-14 issue #5832 remains blocked until `WP-04-IMP` produces stable integrated distributed contracts.

## Acceptance Criteria

- Architecture freezes node and Guardian identity, enrollment, certificate purposes, discovery, membership, authenticated transport, failure detection, epochs, leases, fencing, placement inputs, snapshot and migration semantics, rollback, observability, and failure behavior.
- Threat modeling covers partition, replay, stale leases, cloned state, wrong trust domain, certificate compromise or expiry, relocation and rollback failure, and split-brain activation.
- The retained ledger contains exactly 16 unique children with resolvable acyclic dependencies and no duplicate or overlapping protected paths.
- An issue-local validator rejects malformed identities, missing ownership or proof boundaries, dependency cycles, path collisions, and denominator drift.
- A separately scheduled v0.92 `WP-04-IMP` umbrella names exactly WP-04.01 through WP-04.16 and depends on this gate.
- Independent architecture and security review of the exact gate revision has no unresolved actionable findings.

## Non-goals

- No distributed Runtime implementation, integration, multi-node proof, or child completion credit in issue #5821.
- No v0.93 constitutional governance or polis policy redesign.
- No custom cryptography, custom framing, Runtime v2 changes, or cognition-owned placement.

<!-- csdlc-github-operation:v092-wp04-architecture-child-wave-gate -->

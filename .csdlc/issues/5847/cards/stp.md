# Structured Task Prompt

Template: 1.0.0

Issue: 5847

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver external review handoff and received review.

## Deliverables

- external review handoff and received review
- review handoff and review report

## Acceptance

1. AC-1: WP-25 is merged, terminal, claim-free, ancestral, and all blocking internal findings that prevent a coherent handoff are dispositioned.
2. AC-2: The handoff names repository/base/head/SHA, packet object manifest/digest, predecessors, included/excluded surfaces, questions, authority, return schema, and claim boundaries.
3. AC-3: The packet is publication-safe and passes link, command, identity, provenance, secret, private-path, raw-private-state, and redaction checks before dispatch.
4. AC-4: Dispatch uses an explicitly operator-approved channel; any source/digest change makes the review stale until refreshed and re-authorized.
5. AC-5: An actual reviewer-authored report is retained without favorable rewriting, and every returned finding is represented in a separate provenance-preserving WP-27 index.
6. AC-6: Exact-head review of the handoff/receive pipeline has no actionable finding and makes no product-fix or release-approval claim.

## Dependencies

- WP-25

## Inputs

- Terminal WP-25 internal report, findings register, exact target SHA, and packet manifest
- Current v0.92 source/evidence corpus and live issue/PR/typed truth
- docs/milestones/v0.91.8/review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md as format precedent only

## Non Goals

- Product remediation, release approval, or reviewer mutation of repository/GitHub/lifecycle state
- Paid/provider dispatch without explicit operator-approved channel and credentials at execution time
- Treating shadow reviews, an outbound request, or a missing response as completed formal review

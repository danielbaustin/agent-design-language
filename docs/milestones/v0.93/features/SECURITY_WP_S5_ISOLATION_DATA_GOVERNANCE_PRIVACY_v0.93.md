# Security WP-S5: Isolation, Data Governance, And Privacy v0.93

## Metadata

- Feature Name: Isolation, data governance, and privacy
- Milestone Target: v0.93
- Status: planned
- Doc Role: supporting enterprise-security feature contract
- Feature Types: architecture, policy, artifact
- Proof Modes: tests, fixtures, schema, demo, review

## Purpose

Define tenant, polis, citizen, tool, service, private-state, ToM, reputation,
and memory data boundaries so ADL can prove isolation, classification,
retention, deletion, projection, and redaction behavior.

## Dependencies

- WP-S1 zero-trust architecture.
- WP-S3 cryptographic trust.
- v0.90.3 private state, redacted projections, access control, sanctuary, and
  quarantine.
- v0.92 memory grounding, identity, and continuity.
- v0.93 ToM/reputation/shared-social-memory boundary.

## Required Work Products

- Data classification model for private state, ToM, reputation, shared social
  memory, audit records, incidents, tool outputs, and public projections.
- Isolation contract for tenant, polis, citizen, service, tool, and reviewer
  views.
- Retention, deletion, redaction, and projection fixtures.
- Leakage negative cases for unauthorized cross-boundary access.

## Invariants

- Private state is not public evidence.
- Private ToM is not public reputation.
- Retention and deletion decisions are traceable.
- Projection must preserve redaction class and authority context.
- Cross-polis, cross-tenant, and cross-citizen leakage fails closed.

## Demo Candidate

Show an unauthorized cross-boundary data access attempt. The system should deny
or redact based on classification, boundary, retention, and projection rules,
then emit a reviewable denial proof.

## Acceptance Criteria

- Every protected data class has an owner, allowed projections, and redaction
  policy.
- Leakage negative cases exist for private state, private ToM, reputation,
  memory, audit, and incident evidence.
- Reviewer packets can inspect decisions without raw protected data.
- Isolation composes with zero-trust and policy-enforcement decisions.

## Non-Goals

- No universal private-state browser.
- No deletion or retention legal-compliance claim.
- No cross-polis federation claim.

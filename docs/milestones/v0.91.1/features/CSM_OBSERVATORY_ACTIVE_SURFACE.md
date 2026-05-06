# CSM Observatory Active Surface

## Metadata

- Feature Name: CSM Observatory Active Surface
- Milestone Target: `v0.91.1`
- Status: planned
- Planned WP Home: WP-03
- Source Docs: `.adl/docs/TBD/csm_observatory/`
- Proof Modes: demo, fixtures, review

## Purpose

Move the Observatory from a static visibility surface toward an active,
operator-facing projection of governed runtime work. v0.91.1 should make agent
runs visible without exposing private state or pretending scripted transcripts
are inhabited runtime proof.

## Scope

In scope:

- Active packet shape for runtime work.
- Operator projection and redaction rules.
- Fixtures for visible, redacted, and invalid packets.
- Integration with the v0.91.1 flagship runtime proof.

Out of scope:

- Public exposure of private citizen state.
- Autonomous federation claims.
- UI polish that is not tied to proof or operator review.

## Acceptance Criteria

- Active packets project deterministically.
- Redaction rules are explicit and testable.
- Operator-visible output distinguishes proof evidence from private data.
- The flagship demo can cite Observatory artifacts as proof surfaces.

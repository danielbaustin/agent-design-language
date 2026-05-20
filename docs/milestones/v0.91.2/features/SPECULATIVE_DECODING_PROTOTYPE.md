# Speculative Decoding Prototype

## Metadata

- Feature Name: Speculative Decoding Prototype
- Milestone Target: `v0.91.2`
- Status: in_flight
- Planned WP Home: WP-11
- Source Docs: `.adl/docs/TBD/ADL_AND_GENERIC_SPECULATIVE_DECODING.md`; `.adl/docs/TBD/ADL_AND_SPECULATIVE_CODING_REPLAY.md`
- Proof Modes: design packet, deterministic prototype report, reviewer evaluation packet

## Purpose

Evaluate whether generic speculative decoding is worth carrying into ADL's
runtime as a bounded acceleration surface without weakening deterministic commit
semantics, replayability, or audit posture.

The goal is not speed theater. The goal is to decide whether the acceleration
pattern can live inside ADL honestly.

## Scope

In scope:

- A bounded architecture and executable proof posture for speculative decoding in ADL.
- Explicit separation between speculative proposal and authoritative commit.
- Runtime evidence requirements for acceptance, rejection, and fallback.
- Non-claims around side effects, Freedom Gate, ACC, and external authority.

Out of scope:

- Claiming production-grade runtime acceleration across all backends.
- Replacing deterministic commit with hidden heuristics.
- Smuggling world-changing authority through token-acceleration language.
- Benchmark theater without a clear audit and replay story.

## Acceptance Criteria

- The feature states how speculative proposal differs from authoritative commit.
- The design preserves replay, audit, and deterministic commit boundaries.
- Freedom Gate and ACC remain the authority boundary for side effects.
- The milestone leaves behind a bounded prototype/evaluation packet rather than
  a vague runtime-performance aspiration.

## Current Proof Route

- `docs/milestones/v0.91.2/review/speculative_decoding/speculative_decoding_prototype_report.json`
- `docs/milestones/v0.91.2/review/speculative_decoding/speculative_decoding_prototype_packet.md`

The current `WP-11` branch-local proof posture is:

- worth continuing for same-family local backends when acceptance stays high
- not worth continuing for poor-draft or tokenizer-mismatch pairings
- still explicitly non-proving for production provider-side acceleration claims

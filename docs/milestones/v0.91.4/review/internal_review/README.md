# v0.91.4 Internal Review Records

## Status

`public_review_records_default`

## Purpose

This directory is the tracked review-record surface for the v0.91.4 internal
review cycle. Review plans, findings registers, synthesis reports, review
handoffs, and remediation-routing summaries should live in tracked repository
paths by default so reviewers can inspect the same evidence that operators use.

This policy applies from v0.91.4 forward unless a later milestone supersedes it.

## Public Review Record Default

ADL review records are public/tracked by default when they are intended to
support milestone, issue, PR, release, or third-party review truth.

Tracked review records should include:

- review plans
- findings registers
- synthesis reports
- review handoff packets
- remediation-routing summaries
- evidence indexes and proof maps
- reviewer-facing non-claims and residual-risk notes

Tracked review records should preserve findings even after they are fixed,
superseded, or routed. Later records may add dispositions, but they should not
rewrite history to make the original review look cleaner than it was.

## Local-Only Control Artifacts

Ignored `.adl/reviews/` paths may still be used for local control artifacts,
intermediate coordination state, scratch review-lane notes, or machine-generated
working files that are not safe or useful as durable reviewer evidence.

Local-only artifacts may be referenced from tracked docs only when the tracked
doc clearly states that they are local control state and not durable public
proof.

## Redaction Rules

Do not track review records that contain:

- raw credentials, API keys, private keys, tokens, or key-file contents
- private provider logs or raw model responses that include sensitive payloads
- unredacted personal machine details or local absolute host paths
- private customer, account, or billing data
- secret environment variables or command transcripts exposing secrets
- temporary scratch paths presented as durable proof

Before publication, review records should be checked for obvious secret markers,
local absolute paths, and unsupported approval or release-readiness claims.

## Claim Boundaries

Tracked review records may say what was reviewed, what was found, what was
fixed, what was routed, and what remains risky. They must not claim:

- release approval before release ceremony
- external review approval before external review completes
- remediation completion before fixes land and are reviewed
- benchmark, demo, or provider proof beyond the recorded evidence
- local-only evidence as durable public proof

## Relationship To v0.91.4 Review

The v0.91.4 internal review owner is `#3366`. The initial review plan is:

- `V0914_INTERNAL_REVIEW_PLAN_2026-05-30.md`

The plan defines the review lanes and outputs. This README defines the public
record policy for the review packet.

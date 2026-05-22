# C-SDLC Demo Proof Contract v0.91.3

## Scope

This contract governs the bounded C-SDLC demo mini-sprint under `#3219`.

Its job is to make later demo claims reviewable before implementation broadens
the evidence surface.

## Required Packet Sections

Every demo packet must include:

1. demo identity
2. bounded purpose
3. explicit claims
4. explicit non-claims
5. run path or `not run` explanation
6. timebox truth
7. validation evidence
8. review evidence
9. result classification
10. residual risk and skipped work

## Claim Ledger Rules

Each packet must separate:

- what the demo proves
- what the demo suggests
- what the demo does not prove

Suggested value must never be worded as established proof.

## Result Classification

Allowed top-level result classes:

- `passed`: the bounded claim is supported by the recorded run, validation, and
  review evidence
- `partial`: some bounded claims are supported, but the packet records missing
  or degraded proof
- `skipped`: the packet intentionally records that the demo or part of it was
  skipped
- `failed`: the claimed behavior was attempted and did not satisfy the bounded
  proof condition
- `not run`: no executable demo run occurred, or the packet is design-only

If multiple claims exist, the packet must classify the important ones
separately instead of collapsing them into one vague success sentence.

## Timebox Truth Rules

- Measured timebox claims require explicit start and end evidence.
- Reconstructed or inferred timing must be labeled `estimated`.
- If the demo proves governance or artifact quality but misses the literal
  timebox, the packet must classify those claims separately.
- Missing time evidence must lower the timebox claim to `partial`,
  `failed`, or `not run`.

## Validation Minimums

Each demo packet must record:

- the primary demo command, or the reason it was not run
- focused validation commands for the touched surface
- validation not run, if any
- whether output paths are repo-relative

## Review Minimums

Each demo packet must record:

- the bounded review surface used
- findings fixed before publication
- residual risks that remain

No later demo may present itself as review-complete if it lacks an explicit
review note.

## Non-Claims

This contract does not allow a packet to imply:

- universal five-minute delivery
- unrestricted autonomous engineering
- hidden automation that was not actually implemented
- production readiness from demo-only evidence

## Required Packet Template

Later demos should start from:

- `C_SDLC_DEMO_PROOF_PACKET_TEMPLATE_v0.91.3.md`

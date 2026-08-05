# Structured Planning Prompt

Template: 1.0.0

Issue: 5837

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

After 5800/5820/5832/WP-18 gates, freeze one compatibility matrix, bind both existing clients to the same redacted HTTP/WSS contract, implement authenticated control and bounded reconnect/failure behavior without UI redesign, then prove real browser and Unity interactions, restart recovery, redaction/refusal, platform limits, and exact-head review.

## Plan

Revision 8

## Steps

[
  {
    "id": "S1",
    "action": "Verify all upstream gates, freeze the shared compatibility/redaction/reconnect matrix, and narrow disjoint HTML, Unity, and any necessary Runtime compatibility files.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement real HTML and Unity read/write transport bindings, explicit failure states, redaction/refusal handling, and bounded reconnect without design or authority drift.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run live browser and Unity interactions against one Runtime revision, exercise denial/failure/backpressure, restart Guardian, and verify bounded replay and unchanged authority.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Resolve exact-head review and publish compatibility evidence with closing linkage and explicit platform limits.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- One versioned schema and ordering/correlation contract serves both clients
- Runtime exposes no private state, signing key, certificate key, or raw token
- Reads never imply write authority and reconnect never widens permissions
- Retained/static packets are visibly historical and never count as live proof
- UI remains outside Runtime and Unity has no schema fork
- TLS, origin, version, stale, denied, unavailable, restart, and backpressure states are explicit

## Risks

- HTML and Unity could drift onto different schema versions
- Reconnect could duplicate events or restore stale write authority
- Fallback packets could mask live Runtime failure
- Redaction differences could expose private fields in one client
- Unity platform/network behavior could diverge from browser proof
- Shared Runtime changes could collide with upstream owners

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5837/design.md

Digest: ed0220eb8060138d3cf9d284f77a8520cdd03e7d99b95f6aa9557fb128ca7085

## Diagram

.csdlc/prepared/issues/5837/diagram.mmd

Digest: 2735fcc45f0da9aacd9372d8bc46663117662848e7775310da12795fa0824105

## Stop Conditions

- Issues 5820 or 5832 or WP-18 are not stable
- Issue 5800 browser trust is unavailable for HTML proof
- Integration requires UI code in Runtime, client-side private state, or signing keys
- The clients require incompatible schemas or ordering rules
- A live failure can be hidden by fixture or retained fallback

## Handoff

Proceed only after doctor readiness.

# Observatory and Unity Consumer Integration

## Status

Planned for WP-18A after the Runtime API, ACIP/WSS contract, and first-birthday
proof surfaces are available. The Observatory remains a separate application.

## Purpose

Make the HTML Observatory and Unity client real consumers of the same versioned
Runtime projection and event stream without embedding UI code or private state
inside Runtime v3.

## Required Behavior

- Read-only projection APIs are available to clients without a write session;
  authenticated login and explicit authority are required for writes.
- HTTP snapshots and authenticated full-duplex WSS events share versioned
  schemas, stable identifiers, ordering/correlation rules, reconnect behavior,
  and bounded backpressure.
- The Runtime exposes redacted public/operator/reviewer projections, never raw
  private citizen state, keys, or sealed checkpoints.
- HTML and Unity preserve their existing approved designs while binding every
  control, menu, proof link, packet link, and operator action to real behavior.
- Proof and packet links open independently; presentation modes never widen
  authority or data access.
- TLS trust, API discovery, WSS reconnect, stale data, unavailable services,
  authorization refusal, and Runtime restart are explicit client states.

## Proof

WP-18A must run both clients against the actual Runtime API and WSS stream,
exercise reads and authenticated writes, prove redaction and refusal cases,
verify reconnect after Guardian-owned restart, and retain browser plus native
Unity evidence without fixture substitution.

## Non-Goals

- No Observatory HTML served from Runtime.
- No design change without explicit operator approval.
- No client-side private-state access or authority bypass.
- No Unity-only fork of Runtime schemas or platform behavior.

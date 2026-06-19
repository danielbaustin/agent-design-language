# AEE, Memory/ObsMem, And ACP Bridge Accounting

## Metadata

- Feature Name: AEE, Memory/ObsMem, And ACP Bridge Accounting
- Milestone Target: `v0.91.6`
- Status: planned
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: architecture, policy
- Proof Modes: review, replay

## Purpose

Account for AEE completion, Memory/ObsMem handoff, and ACP/cognitive profile
scope before `v0.92` consumes activation or birthday claims.

## Scope

In scope:

- AEE completion boundary;
- residual runtime/provider action routing;
- ObsMem handoff;
- Memory Palace planning state;
- ACP/cognitive profile scope and privacy boundary;
- `v0.92` consumption limits.

Out of scope:

- Memory Palace implementation;
- full ACP runtime;
- provider action implementation.

## Required Decisions

- Which AEE boundaries are complete enough to consume?
- Which Memory/ObsMem handoff artifacts are required?
- Which cognitive profile fields are private, public, or blocked?
- Which provider/runtime action residuals block activation?

## Dependencies

- Identity/continuity bridge feature doc.
- Resilience and persistence feature doc.
- Security bridge feature doc.
- `v0.92` ACP and memory grounding docs.

## Validation And Review

- Review AEE completion claims against concrete artifacts.
- Separate Memory/ObsMem handoff from Memory Palace future work.
- Treat ACP profile privacy as a security boundary.
- Consume WP-07 `#4022` for bounded publication/privacy review without
  converting that review into false completion of open WP-10 work.

## v0.92 Consumption

`v0.92` may consume only named completion and handoff surfaces. It must not
rediscover AEE, Memory/ObsMem, or ACP scope during activation.

## Non-Goals

- No Memory Palace completion claim.
- No unreviewed cognitive-profile publication.
- No runtime/provider action completion claim.

## Current Security Consumption

- WP-07 packet
  `docs/milestones/v0.91.6/review/security/PUBLIC_RECORD_MEMORY_PROFILE_SECURITY_REVIEW_4022.md`
  may be consumed as the current bounded publication/privacy review for this
  bridge.
- That packet does not close WP-10. It records that memory/profile publication
  remains an open dependency until the WP-10 issue set, including the privacy
  boundary lane `#4040` and closeout lane `#4041`, lands its own reviewed
  privacy and closeout surfaces.

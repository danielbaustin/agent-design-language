# Observatory And Unity Consumption Classification

## Metadata

- Feature Name: Observatory And Unity Consumption Classification
- Milestone Target: `v0.91.6`
- Status: planned
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: policy, artifact
- Proof Modes: review, demo

## Purpose

Classify Observatory and Unity surfaces so `v0.92` birthday/demo work knows
which surfaces are proof, rehearsal, substrate, blocked, or deferred.

## Scope

In scope:

- Observatory consumption categories;
- Unity readiness categories;
- demo proof versus rehearsal boundaries;
- required artifact/proof surfaces.

Out of scope:

- Unity implementation;
- Observatory runtime productization;
- demo execution.

## Required Decisions

- Which Observatory surfaces are proof-bearing?
- Which Unity surfaces are rehearsal-only?
- Which demo claims require runtime proof before `v0.92`?
- Which blocked surfaces should be routed out of activation?

## Dependencies

- Existing Observatory readiness docs.
- Provider/model reliability feature doc.
- Security and public-record boundaries where demos expose artifacts.
- Identity continuity boundaries for inhabitant-facing display.

## Validation And Review

- Review every demo surface for proof role.
- Require artifact paths or explicit blocked/deferred status.
- Do not let rehearsal evidence prove runtime readiness.
- Consume WP-07 `#4023` for bounded inhabitant-readiness security review
  without converting that review into false WP-09 implementation closure.

## v0.92 Consumption

`v0.92` may consume Observatory/Unity surfaces only with classification. A
surface marked rehearsal or substrate cannot prove activation by itself.

## Non-Goals

- No demo execution in this doc.
- No Unity readiness claim without proof.
- No Observatory product readiness claim.

## Current Security Consumption

- WP-07 packet
  `docs/milestones/v0.91.6/review/security/UNITY_OBSERVATORY_INHABITANT_READINESS_SECURITY_REVIEW_4023.md`
  may be consumed as the current bounded security review for Unity Observatory
  inhabitant-facing and Observatory-consumption posture.
- That packet does not close WP-09. It records that inhabitant surfaces,
  ingestion security, and working Unity closeout remain open dependencies until
  the WP-09 issue set lands its own reviewed proof, and identity-safe
  inhabitant display also remains dependent on open WP-08 issue `#3973`.

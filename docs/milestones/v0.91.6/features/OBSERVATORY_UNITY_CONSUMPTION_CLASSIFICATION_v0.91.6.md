# Observatory And Unity Consumption Classification

## Metadata

- Feature Name: Observatory And Unity Consumption Classification
- Milestone Target: `v0.91.6`
- Status: child_wave_closed_and_umbrella_closed
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
  ingestion security, and working Unity closeout were open dependencies at the
  time of the review; those dependencies are now satisfied by the closed WP-09
  child wave and closed WP-08 issue `#3973`, while the packet's bounded
  security-consumption limits still apply.

## Current WP-09 Closeout Posture

WP-09 implementation proof is landed, and umbrella `#3974` is now closed. The
remaining Observatory-related work is downstream demo/runtime convergence, not
missing WP-09 implementation or umbrella closeout.

Current live issue truth on the closeout date:

| Surface | Owner | Current state | Closeout implication |
| --- | --- | --- | --- |
| Unity Observatory baseline definition | `#4030` | closed / completed | Working-baseline truth is now landed child proof rather than a live residual |
| Launchable Unity Observatory baseline | `#4031` | closed / completed | Governed launch surface is landed and may be consumed as closed child proof |
| Observatory evidence data contract | `#4032` | closed / completed | Bounded Observatory ingestion contract is landed |
| Inhabitant-readiness surfaces | `#4033` | closed / completed | Inhabitant-facing bounded surfaces are landed subject to existing identity limits |
| Logging/OTel/security consumption proof | `#4034` | closed / completed | Security/OTel consumption closure for the bounded proof slice is landed |
| Working Unity Observatory closeout proof | `#4035` | closed / completed | Retained closeout packet is landed and may be consumed as closed child proof |
| HTML Observatory mobile governed surface | `#4341` | closed / completed | Portable HTML/mobile observatory lane is landed with bounded proof |
| WP-09 umbrella | `#3974` | closed / completed | Umbrella closeout publication is complete; downstream work remains separately routed |

The authoritative closeout packet for this posture is:

- `docs/milestones/v0.91.6/review/observatory/WP09_WORKING_UNITY_OBSERVATORY_CLOSEOUT_4035.md`

## v0.92 Consumption Rule

`v0.92` may consume this feature surface only as:

- a classification and routing contract for Observatory/Unity proof posture;
- a reference to bounded security-consumption inputs from WP-07;
- a record that the WP-09 child wave and umbrella closeout are closed while
  downstream demo/runtime convergence remains separately routed.

`v0.92` may not consume this feature surface as proof that:

- the Unity Observatory is fully working and launch-ready;
- the Unity Observatory is production-ready;
- inhabitant-facing display or input is security-cleared;
- Observatory ingestion and logging/OTel consumption is fully closed;
- broader runtime or release-tail convergence is complete.

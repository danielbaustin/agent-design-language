# v0.90 Early Planning Lane Handoff

## Purpose

This handoff records the early v0.90 planning lane prepared during the v0.89.1
release tail. It gives WP-19 a tracked review surface without promoting the
entire local planning corpus prematurely.

The local planning lane remains source material. WP-19 owns the final decision
to promote, rewrite, split, or defer each planning surface before the release
ceremony.

## Current State

Owner issue: #1986

Related roadmap correction: #2006 / PR #2007

Promotion gate: v0.89.1 WP-19

Release ceremony dependency: v0.89.1 WP-20

Status: ready for WP-19 reconciliation

## Local Planning Corpus

The local planning lane has the same shape as the intended milestone package:

- root planning docs for v0.90
- feature drafts
- idea and backgrounder drafts
- a local issue-wave YAML seed
- an explicit planning inventory and WP-19 handoff note

The local source material is not release truth by itself. It exists so WP-19 can
move quickly without rediscovering the v0.90 story from chat history.

## Reconciled Roadmap Boundary

PR #2007 added the tracked Runtime v2 and birthday-boundary roadmap. WP-19
should treat that roadmap as upstream of the v0.90 handoff:

- v0.90 remains the long-lived bounded runtime milestone.
- v0.90.1 owns the Runtime v2 foundation prototype.
- v0.90.2 owns Runtime v2 hardening, invariant tests, violation artifacts,
  recovery, and security-boundary proof surfaces.
- v0.91 remains additive moral and emotional civilization work.
- v0.92 owns the first true Gödel-agent birthday.

This means v0.90 should not absorb the full Runtime v2, the full moral and
emotional civilization package, or the first birth event. It should prepare the
long-lived runtime band and make the follow-on sequence obvious.

## Strong v0.90 Candidates

These local planning surfaces are close to the v0.90 center of gravity and
should be reviewed first by WP-19:

- long-lived agent runtime feature set
- supervisor heartbeat
- agent cycle contract
- state and continuity handles
- operator control and safety
- stock league long-lived agent demo
- urgency and task prioritization, if narrowed to scheduling and supervision

## Needs Scope Decision

These are useful, but WP-19 should decide whether they belong in core v0.90,
supporting scope, or a later band:

- hypothesis engine reasoning graph
- signed trace architecture
- trace query language

The practical question is whether v0.90 can include them without weakening the
long-lived runtime milestone. If not, WP-19 should extract the minimal inspection
requirements and defer the larger architectures.

## Likely Later-Band Inputs

These planning docs remain valuable, but probably should not be promoted as
core v0.90 feature work:

- cross-agent temporal alignment
- temporal accountability
- timeline forks and counterfactuals
- later temporal and society cluster map

They should receive explicit later-milestone or backlog placement instead of
being silently carried forward.

## WP-19 Homecoming Checklist

WP-19 should complete the following before WP-20 release ceremony starts:

- confirm the v0.90 milestone thesis
- reconcile the v0.90 local planning lane with the Runtime v2 roadmap from PR #2007
- decide which local feature drafts become tracked milestone feature docs
- decide which local idea/backgrounder drafts become tracked ideas docs
- rewrite or defer material that is too large or too future-facing
- update the v0.90 WBS, sprint plan, demo matrix, checklist, release plan,
  release notes, feature index, and issue-wave YAML together
- ensure every promoted file has a work-package home
- ensure every deferred file has an explicit later milestone or backlog home

## Non-Goals

- Do not treat this handoff as the final v0.90 milestone package.
- Do not open the v0.90 issue wave from this handoff.
- Do not promote local-only planning docs without WP-19 review.
- Do not move the first true Gödel-agent birthday earlier than v0.92.

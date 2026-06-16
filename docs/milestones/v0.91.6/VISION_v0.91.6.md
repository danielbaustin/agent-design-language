# v0.91.6 Vision

## Metadata

- Project: ADL
- Milestone: `v0.91.6`
- Version: `v0.91.6`
- Date: `2026-06-16`
- Owner: ADL maintainers
- Related issue: `#3824`

## Purpose

Define the first pre-`v0.92` bridge tranche. `v0.91.6` exists to turn
load-bearing activation surfaces into reviewed documents and issue-ready proof
routes before birthday work begins.

## Overview

`v0.91.6` moves ADL from a scattered bridge ledger into a first tranche of
tracked readiness docs. It does not implement runtime behavior and it does not
claim activation readiness by itself.

The milestone focuses on:

- resilience, persistence, sleep/wake, and continuity proof;
- tooling proof-loop reliability and logging/observability consumption;
- public prompt record export and security boundaries;
- provider/model reliability for multi-agent operation;
- ACIP/A2A/provider communications decisions;
- security bridge and Continuous Adversarial Verification;
- identity, capability evidence, Observatory/Unity, AEE, Memory/ObsMem, and
  ACP bridge accounting.

## Core Goals

1. Make every first-tranche bridge surface reviewable from tracked docs.
2. Preserve exact `v0.91.7` routing for residual conceptual surfaces.
3. Keep `v0.92` activation blocked until each bridge surface is complete,
   blocked, deferred, or routed with evidence.
4. Separate planning truth from runtime implementation claims.
5. Reduce future rediscovery by giving every surface a source, decision path,
   proof expectation, and non-goal boundary.

## Strategic Pillars

- Governed cognition: resilience, continuity, AEE, Memory/ObsMem, and ACP must
  be visible before identity/birthday claims consume them.
- Reliable operation: model/provider roles, tooling proof loops, and logging
  must support multi-agent work without fragile local assumptions.
- Public trust: prompt records, ACIP, and provider communications need
  redaction, access, security, and evidence boundaries.
- Honest activation: `v0.92` may consume only reviewed bridge truth.

## Non-Goals

- Do not implement runtime features.
- Do not declare `v0.92` ready.
- Do not collapse residual `v0.91.7` work into vague future-work language.
- Do not productize Aptitude Atlas or CodeFriend in this tranche.

## Success Criteria

- The 10 planning docs exist for `v0.91.6`.
- The nine first-tranche feature docs exist and cross-link from the index.
- Reviewers can tell what `v0.92` may consume and what remains blocked,
  deferred, or routed.

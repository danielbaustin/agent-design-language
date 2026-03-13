# v0.8 Coverage and Quality Gate

This document defines the canonical v0.8 quality gate.

It is a review/release planning surface only. It does not itself implement CI behavior.

## Gate Structure

The v0.8 gate has two phases:

1. Pre-third-party review gate (before `#707`)
2. Pre-release ceremony gate (before release-tail completion)

## Required vs Recommended

- **Required**: must pass for the gate phase to be considered complete.
- **Recommended**: high-value checks that should pass when feasible; failures are triaged explicitly.

## Required Checks: Pre-Third-Party Review (`#707`)

1. Milestone docs convergence:
   - canonical v0.8 docs exist and cross-links resolve.
2. Demo matrix completeness:
   - required demo rows in `DEMOS_V0.8.md` have evidence surfaces defined.
3. Determinism contract consistency:
   - schema/spec docs preserve deterministic ordering and explicit boundaries.
4. Security/privacy hygiene:
   - no absolute host paths, no secret/token/prompt/tool-argument leakage in milestone artifacts.
5. Issue/plan alignment:
   - execution order, schema order, and handoff boundaries are mutually consistent.

## Required Checks: Pre-Release Ceremony

1. Required review findings resolved or explicitly deferred with owner+issue.
2. Release docs updated to shipped state:
   - checklist, release plan, release notes.
3. Canonical required demos marked complete with evidence references.
4. Quality command suite green at agreed milestone baseline (check/fmt/clippy/test or documented equivalent).
5. No unresolved blocker-grade findings in milestone review output.

## Recommended Checks

1. Coverage trend check (workspace-level), with rationale for known exclusions.
2. Demo smoke verification for representative entries across major workstreams.
3. Replay-oriented spot checks for deterministic schema/example surfaces.
4. External reviewer dry-run prior to final third-party pass.

## Deterministic Validation Surface

Use deterministic, repo-local checks for doc/spec gate verification, including:

- explicit issue-anchor checks (`rg`)
- canonical-link presence checks
- host-path/secrets hygiene scans

Where runtime validation is referenced, commands and acceptance thresholds must be documented explicitly in release-tail cards.

## Deferral Policy

A required gate item may be deferred only when all are true:

1. deferral is explicit,
2. risk is documented,
3. an owner and follow-up issue exist,
4. milestone scope remains bounded.

## Out of Scope

- Introducing new feature scope to satisfy quality wording.
- Redesigning CI architecture in this planning doc.
- Reclassifying deferred post-v0.8 items as required v0.8 deliverables.

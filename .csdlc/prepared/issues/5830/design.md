# Issue 5830 Design: Evidence-Grounded Cognitive Profiles

## Outcome And Sources

Define WP-13's bounded ACP profile from `docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md` and landed memory, capability, Theory-of-Mind, intelligence, and governed-learning evidence under `adl/src/runtime_v2/`.

## Owned Surface

Candidate protected paths are the ACP feature contract, a narrowly named Runtime v2 profile contract, matching tests/fixtures, and `.csdlc/evidence/5830/`. The profile contains ID/schema, identity and continuity bindings, allowed evidence refs, update reason/actor, privacy/redaction policy, projections, and explicit non-claims.

## Contract

Profiles are deterministic evidence maps, not free-form personality labels. Every field must cite an allowed source category and current digest. Updates preserve prior revision linkage and explain additions/removals. Missing evidence, stale or forbidden refs, private-state leakage, unsupported label inference, identity mismatch, and attempts to derive reputation, standing, rights, personhood, or consciousness fail closed.

## Dependencies And Invariants

WP-10/#5827, WP-11/#5828, and WP-12/#5829 must be terminal; the v0.91.1 ToM/intelligence/governed-learning inputs remain bounded prerequisites. Public projection is strictly narrower than the internal evidence map.

## Validation And Rollback

Focused schema and update tests prove canonical records and revision linkage. Negative/privacy lanes cover unsupported labels, stale evidence, root mismatch, forbidden paths, redaction failure, and reputation/standing inference. Rollback removes the v0.92 profile layer without mutating source evidence.

## Non-Goals

Diagnosis, scalar moral verdicts, reputation, public standing, rights allocation, citizenship, raw private-state access, and autonomous profile mutation are excluded.

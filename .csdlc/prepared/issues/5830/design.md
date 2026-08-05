# Issue 5830 Design: Evidence-Grounded Cognitive Profiles

## Outcome And Sources

Define WP-13's bounded ACP profile in current Runtime v3 authority from `docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md` and landed memory, capability, Theory-of-Mind, intelligence, and governed-learning evidence. Retained Runtime v2 evidence may be consumed only through explicit versioned references.

## Owned Surface

Protected implementation paths are `adl-runtime-kernel/src/cognitive_profile.rs`, `adl-runtime-kernel/src/lib.rs` (module registration only), `adl-runtime-kernel/tests/cognitive_profile.rs`, `adl-runtime-kernel/tests/fixtures/cognitive_profile/`, `docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md`, and `.csdlc/evidence/5830/`. Existing evidence under `adl/src/runtime_v2/` is read-only input authority. The profile contains ID/schema, identity and continuity bindings, allowed evidence refs, update reason/actor, privacy/redaction policy, projections, and explicit non-claims.

## Contract

Profiles are deterministic evidence maps, not free-form personality labels. Every field must cite an allowed source category and current digest. Updates preserve prior revision linkage and explain additions/removals. Missing evidence, stale or forbidden refs, private-state leakage, unsupported label inference, identity mismatch, and attempts to derive reputation, standing, rights, personhood, or consciousness fail closed.

## Dependencies And Invariants

WP-10/#5827, WP-11/#5828, and WP-12/#5829 must be terminal; the v0.91.1 ToM/intelligence/governed-learning inputs remain bounded prerequisites. Public projection is strictly narrower than the internal evidence map.

## Validation And Rollback

The exact `cognitive_profile` Runtime v3 integration-test target must run a nonzero count proving canonical records, revision linkage, unsupported-label rejection, stale evidence, root mismatch, forbidden paths, redaction failure, and reputation/standing non-inference. Native Linux CI and a retained native macOS receipt use the same fixture digest before portability is claimed. Rollback removes the v0.92 profile layer without mutating source evidence.

## Non-Goals

Diagnosis, scalar moral verdicts, reputation, public standing, rights allocation, citizenship, raw private-state access, and autonomous profile mutation are excluded.

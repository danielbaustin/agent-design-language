# Structured Task Prompt

Template: 1.0.0

Issue: 5830

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver only the WP-13 ACP schema, update semantics, fixtures, validator, privacy/negative proof, and retained exact-revision report.

## Deliverables

- Versioned ACP schema and deterministic update/revision contract
- Canonical internal and public projection fixtures
- Negative fixtures for stale/forbidden evidence, root mismatch, leakage, and unsupported labels
- Retained focused, privacy, and non-reputation report

## Acceptance

1. The WP-13 profile deterministically binds identity and continuity to allowed evidence digests, update actor/reason, revision linkage, privacy policy, bounded projections, and explicit non-claims.
2. WP-10/#5827, WP-11/#5828, WP-12/#5829, and current ToM/intelligence/governed-learning evidence are verified before implementation.
3. Implementation is confined to adl-runtime-kernel/src/cognitive_profile.rs, lib.rs module registration, tests/cognitive_profile.rs, tests/fixtures/cognitive_profile/, the ACP feature contract, and .csdlc/evidence/5830/; Runtime v2 is read-only evidence.
4. Canonical profile creation and revision updates replay identically and retain exact-revision internal and public projection proof.
5. Stale or forbidden evidence, identity mismatch, unexplained mutation, private-state leakage, unsupported labels, reputation, standing, rights, diagnosis, personhood, and consciousness inferences fail closed.
6. One bounded exact-head SRP review records no unresolved actionable findings.
7. The implementation PR targets the intended base and includes Closes #5830 without claiming completion of downstream Birthday work.
8. The exact cognitive_profile nextest target runs a positive test count on native GitHub Actions macOS and Linux at exact candidate HEAD; issue-local producers retain hashed source manifests, complete command logs, and canonical semantic outputs, and independent validation recomputes every digest and requires semantic equivalence.

## Dependencies

- WP-10 / issue #5827 terminal proof
- WP-11 / issue #5828 terminal proof
- WP-12 / issue #5829 terminal proof
- Current v0.91.1 Theory-of-Mind, intelligence, and governed-learning evidence

## Inputs

- docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md
- adl-runtime-kernel/src/identity_memory.rs
- adl-runtime-kernel/src/cognition.rs
- adl-runtime-kernel/src/governance.rs
- adl/src/runtime_v2/ (read-only retained evidence only)

## Non Goals

- Diagnosis, scalar moral verdicts, reputation, public standing, rights, citizenship, personhood, or consciousness
- Raw private-state access or public projection of internal evidence
- Autonomous or unexplained profile mutation

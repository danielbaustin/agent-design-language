# Structured Task Prompt

Template: 1.0.0

Issue: 5829

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver only the WP-12 capability envelope contract, validation fixtures, and exact-revision evidence while preserving #4761.

## Deliverables

- Versioned capability-envelope schema and deterministic canonicalization
- Complete provider/model/tool/skill/grant/deny/limit fixture
- Negative fixtures for stale provenance, escalation, omitted limits, secrets, and host paths
- Retained focused, security, and portability report

## Acceptance

1. The WP-12 envelope deterministically binds identity and evidence revision to explicit provider/model identifiers, tools, skills, grants, denials, resource limits, provenance, and unsupported claims.
2. WP-08/#5825, WP-09/#5826, and retained #4761 evidence are verified before implementation begins.
3. Implementation is confined to adl-runtime-kernel/src/capability_envelope.rs, lib.rs module registration, tests/capability_envelope.rs, tests/fixtures/capability_envelope/, the feature contract, and .csdlc/evidence/5829/.
4. Equivalent capability inputs serialize canonically and produce reproducible exact-revision validation evidence.
5. Unknown or stale provider/model evidence, undeclared tools/skills, authority escalation, omitted limits, secret-like content, private paths, and host paths fail closed.
6. One bounded exact-head SRP review records no unresolved actionable findings.
7. The implementation PR targets the intended base and includes Closes #5829 without claiming completion of downstream Birthday work.
8. WP-08/#5825 and WP-09/#5826 are terminal before execution, and the serialized adl-runtime-kernel/src/lib.rs registration waits for WP-11/#5828 to land and release that path.
9. The exact capability_envelope nextest target runs a positive test count on native GitHub Actions macOS and Linux at exact candidate HEAD; issue-local producers retain hashed source manifests, complete command logs, and canonical semantic outputs, and independent validation recomputes every digest and requires semantic equivalence.

## Dependencies

- WP-08 / issue #5825 terminal proof
- WP-09 / issue #5826 terminal proof
- Retained issue #4761 capability-envelope evidence

## Inputs

- docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md
- adl/src/provider/ (read-only provider inventory)
- adl/src/provider_adapter.rs
- .csdlc/evidence/4761/capability-envelope/

## Non Goals

- Provider execution, credential setup, remote deployment, or unlimited-capacity claims
- Granting authority, proving invocation, identity creation, Memory Palace, or birthday approval
- Mutating or silently copying retained #4761 evidence

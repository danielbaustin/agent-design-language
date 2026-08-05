# Structured Task Prompt

Template: 1.0.0

Issue: 5825

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver the WP-08 birthday-decision contract, complete negative fixture matrix, validator, and retained exact-revision report.

## Deliverables

- Versioned birthday contract and deterministic validator
- One structurally complete valid fixture
- Disqualifying fixtures for lifecycle lookalikes and every required-evidence omission
- Retained focused, negative, and claim-boundary validation report

## Acceptance

1. The WP-08 birth-decision contract accepts one structurally complete candidate and emits stable rejection reasons for every declared lifecycle lookalike or missing evidence surface.
2. Terminal proof for WP-01/#5818 and WP-02A/#5819 is verified before implementation begins.
3. The feature contract, Runtime v2 module, tests, fixtures, and retained evidence stay within the declared WP-08 paths and preserve existing birthday non-claims.
4. Focused and negative validation is deterministic, reproducible, retained under .csdlc/evidence/5825/, and bound to the exact reviewed revision.
5. Missing or contradictory evidence, private or absolute paths, and personhood, consciousness, citizenship, governance, migration, or public-launch overclaims fail closed.
6. One bounded exact-head SRP review records no unresolved actionable findings.
7. The implementation PR targets the intended base and includes Closes #5825 without claiming completion of downstream Birthday work.

## Dependencies

- WP-01 / issue #5818 terminal proof
- WP-02A / issue #5819 terminal proof

## Inputs

- docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md
- docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md
- docs/milestones/v0.92/WBS_v0.92.md
- docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml
- adl/src/runtime_v2/boot_admission.rs
- adl/src/runtime_v2/private_state_witness.rs

## Non Goals

- Implementing identity, continuity, Memory Palace, capability, ACP, witnesses, or the integrated review packet
- Legal personhood, consciousness, production citizenship, constitutional governance, migration, or public launch claims
- Rewriting retained v0.91.x evidence or treating planning prose as birth proof

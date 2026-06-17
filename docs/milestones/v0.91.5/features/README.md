# v0.91.5 Feature Plans

## Metadata

- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Date: `2026-05-29`
- Owner: ADL maintainers
- Status: `sprint_4_release_tail_active`
- Planning template set: `docs/templates/planning/1.0.0`

## Template Rules

This index follows the planning feature-doc shape. It is an index, not
implementation evidence.

## Purpose

List the tracked v0.91.5 feature contracts that the release tail should use as
reviewable scope baselines.

## Context

v0.91.5 is a bridge milestone. Its feature docs define the operational
surfaces that must be ready before v0.92 first birthday opens.

## Coverage / Ownership

This index owns package navigation. Each linked feature doc owns its own scope,
validation, risks, and future-work boundary.

## Overview

The package covers AEE completion, multi-agent C-SDLC operation,
provider/model matrix, public prompt records, demo/Unity Observatory readiness,
security/CAV scheduling, enterprise-security organization boundaries, and
v0.92 activation readiness.

## Design

Feature docs should stay evidence-bound, template-valid, and linked from the
milestone README, WBS, sprint plan, and issue wave.

## Execution Flow

1. The issue wave seeds feature scope.
2. Execution and review packets land evidence across Sprint 1 through Sprint 4.
3. Release-tail WPs align feature docs only to landed evidence or explicit
   blocker truth.

## Determinism and Constraints

The package must not claim implementation completion before v0.91.5 work lands.

## Integration Points

- [../README.md](../README.md)
- [../WBS_v0.91.5.md](../WBS_v0.91.5.md)
- [../WP_ISSUE_WAVE_v0.91.5.yaml](../WP_ISSUE_WAVE_v0.91.5.yaml)

## Validation

Each linked feature doc should pass the active `feature_doc` template
validator and link checks.

## Acceptance Criteria

- Every major v0.91.5 bridge track has a linked feature doc.
- The package validates structurally.
- WP-01 can consume the package without chat reconstruction.

## Risks

- The index may drift from the live release tail. Mitigation: WP-15 and later
  closeout work must reconcile it against the issue wave, quality gate, and
  review packets.

## Future Work

v0.92 consumes this package for first-birthday activation readiness.

## Notes

This index intentionally keeps v0.91.5 bridge work separate from v0.92
first-birthday implementation.

## Feature Documents

- [AEE_COMPLETION_TRANCHE_v0.91.5.md](AEE_COMPLETION_TRANCHE_v0.91.5.md)
- [CAV_THREAT_MODEL_AND_CODEFRIEND_SECURITY_SCHEDULING_v0.91.5.md](CAV_THREAT_MODEL_AND_CODEFRIEND_SECURITY_SCHEDULING_v0.91.5.md)
- [CAV_THREAT_MODEL_AND_CODEFRIEND_SECURITY_SOURCE_PACKET_v0.91.5.md](CAV_THREAT_MODEL_AND_CODEFRIEND_SECURITY_SOURCE_PACKET_v0.91.5.md)
- [ENTERPRISE_SECURITY_ORGANIZATION_BOUNDARY_v0.91.5.md](ENTERPRISE_SECURITY_ORGANIZATION_BOUNDARY_v0.91.5.md)
- [MULTI_AGENT_C_SDLC_OPERATION_v0.91.5.md](MULTI_AGENT_C_SDLC_OPERATION_v0.91.5.md)
- [PROVIDER_MODEL_MATRIX_v0.91.5.md](PROVIDER_MODEL_MATRIX_v0.91.5.md)
- [PUBLIC_PROMPT_RECORDS_v0.91.5.md](PUBLIC_PROMPT_RECORDS_v0.91.5.md)
- [DEMO_AND_UNITY_OBSERVATORY_READINESS_v0.91.5.md](DEMO_AND_UNITY_OBSERVATORY_READINESS_v0.91.5.md)
- [V092_ACTIVATION_READINESS_v0.91.5.md](V092_ACTIVATION_READINESS_v0.91.5.md)

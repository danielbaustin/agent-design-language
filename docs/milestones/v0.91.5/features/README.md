# v0.91.5 Feature Plans

## Metadata

- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Date: `2026-05-29`
- Owner: ADL maintainers
- Status: `active_wp_01_opening`
- Planning template set: `docs/templates/planning/1.0.0`

## Template Rules

This index follows the planning feature-doc shape. It is an index, not
implementation evidence.

## Purpose

List the tracked v0.91.5 feature contracts that WP-01 should consume.

## Context

v0.91.5 is a bridge milestone. Its feature docs define the operational
surfaces that must be ready before v0.92 first birthday opens.

## Coverage / Ownership

This index owns package navigation. Each linked feature doc owns its own scope,
validation, risks, and future-work boundary.

## Overview

The package covers AEE completion, multi-agent C-SDLC operation,
provider/model matrix, public prompt records, demo/Unity Observatory readiness,
and v0.92 activation readiness.

## Design

Feature docs should stay evidence-bound, template-valid, and linked from the
milestone README, WBS, sprint plan, and issue wave.

## Execution Flow

1. WP-01 reviews this index and linked feature docs.
2. WP-01 reconciles live issue labels and the v0.91.5 issue wave.
3. Later WPs update feature docs only with landed evidence.

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

- The index may drift from the issue wave. Mitigation: WP-01 must reconcile
  both before opening or executing issues.

## Future Work

v0.92 consumes this package for first-birthday activation readiness.

## Notes

This index intentionally keeps v0.91.5 bridge work separate from v0.92
first-birthday implementation.

## Feature Documents

- [AEE_COMPLETION_TRANCHE_v0.91.5.md](AEE_COMPLETION_TRANCHE_v0.91.5.md)
- [MULTI_AGENT_C_SDLC_OPERATION_v0.91.5.md](MULTI_AGENT_C_SDLC_OPERATION_v0.91.5.md)
- [PROVIDER_MODEL_MATRIX_v0.91.5.md](PROVIDER_MODEL_MATRIX_v0.91.5.md)
- [PUBLIC_PROMPT_RECORDS_v0.91.5.md](PUBLIC_PROMPT_RECORDS_v0.91.5.md)
- [DEMO_AND_UNITY_OBSERVATORY_READINESS_v0.91.5.md](DEMO_AND_UNITY_OBSERVATORY_READINESS_v0.91.5.md)
- [V092_ACTIVATION_READINESS_v0.91.5.md](V092_ACTIVATION_READINESS_v0.91.5.md)

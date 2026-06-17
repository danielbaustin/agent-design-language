# v0.92 Activation Readiness v0.91.5

## Metadata

- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Date: `2026-05-29`
- Owner: ADL maintainers
- Status: `sprint_4_release_tail_active`
- Related issues: `#3377`, `#3502`

## Template Rules

This is an activation-readiness baseline, not v0.92 launch approval.

## Purpose

Define the activation-readiness work required before v0.92 first birthday.

## Context

Several ADL features were developed in earlier milestones but will come alive
together during v0.92. They need explicit test coverage before the birthday
milestone opens.

## Coverage / Ownership

This feature owns the bridge-level activation map and readiness handoff to
`#3377`.

## Overview

The activation surface includes memory v2, ACP/cognitive profiles,
aptitude/capability selector, identity/continuity, affect and happiness,
Gödel mechanics, economics context, Observatory, Unity readiness, ACIP,
provider/model matrix, public prompt records, and multi-agent workcell evidence.

## Design

Use [../V092_ACTIVATION_TEST_MAP_v0.91.5.md](../V092_ACTIVATION_TEST_MAP_v0.91.5.md)
as the checklist for surfaces that must be tested, blocked, or deferred before
v0.92 WP-01 opens.

## Execution Flow

1. Inventory activation surfaces.
2. Map each surface to v0.92 candidate WPs.
3. Record required tests or proof packets.
4. Feed final go/no-go into `#3377`.

## Determinism and Constraints

Activation readiness must be evidence-bound. Unknown readiness becomes a gap,
not a positive claim.

## Integration Points

- [../V092_ACTIVATION_TEST_MAP_v0.91.5.md](../V092_ACTIVATION_TEST_MAP_v0.91.5.md)
- [../WP_ISSUE_WAVE_v0.91.5.yaml](../WP_ISSUE_WAVE_v0.91.5.yaml)
- [../../v0.92/WP_ISSUE_WAVE_v0.92.yaml](../../v0.92/WP_ISSUE_WAVE_v0.92.yaml)

## Validation

Validation should confirm every activation row has owner issue, v0.92 WP
mapping, test/proof posture, and blocked/deferred state if needed.

## Acceptance Criteria

- Activation map covers all known v0.92 live surfaces.
- `#3377` consumes the map.
- v0.92 WP-01 can use the map without reconstructing intent from chat.

## Risks

- Feature surfaces may be missed because they were developed in older
  milestones.
- Some features may need more testing than v0.91.5 can finish.

## Future Work

v0.92 implements and validates the birthday surfaces. v0.93 consumes the
resulting identity evidence for governance.

## Notes

This feature is the answer to “what comes alive in v0.92?”

Sprint 4 should use it as a handoff/evidence surface for final preflight, not
as implied proof that activation readiness is already complete.

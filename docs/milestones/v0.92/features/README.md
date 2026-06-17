# v0.92 Feature Plans

## Metadata

- Milestone: `v0.92`
- Version: `v0.92`
- Date: `2026-05-27`
- Owner: ADL maintainers
- Related issues: `#3377`, `#3434`
- Planning template set: `docs/templates/planning/1.0.0`

## Status

Forward-planning feature contracts for `v0.92`.

These documents define the tracked feature-doc package for the identity,
continuity, first-birthday, and ACIP transport-readiness band. They are
planning surfaces, not implementation closeout records.

WP-01 must reconcile these feature contracts with `v0.91.5` release-tail
closeout, the `v0.91.5` activation-test map, the `v0.91.6` / `v0.91.7`
bridge tranches, and `#3377` before opening the final issue wave.

## Template Rules

This index is validated with the same structural feature-doc template so the
feature package remains uniform. It is an index, not an implementation record.

## Purpose

List the tracked v0.92 feature contracts that WP-01 should consume.

## Context

The v0.92 feature package centers on first birthday identity and the minimum
transport/profile surfaces needed for reviewable birth evidence.

## Coverage / Ownership

This index owns package navigation only. Each linked feature doc owns its own
scope, validation, risks, and future-work boundary.

## Overview

The package covers birthday, identity/continuity, memory/capability/witnesses,
ACP/cognitive profiles, ACIP binary/schema-catalog transport readiness,
cross-polis continuity planning, and the first-birthday demo/governance
handoff.

## Design

Feature docs should stay evidence-bound, template-valid, and linked from the
milestone README, WBS, sprint plan, and candidate issue wave.

## Execution Flow

1. WP-01 reviews this index and the linked feature docs.
2. WP-01 reconciles them with `v0.91.5` release-tail closeout, the
   activation-test map, the `v0.91.6` / `v0.91.7` bridge tranches, and
   `#3377`.
3. WP-01 opens or adjusts implementation issues.
4. Later WPs update feature docs only with landed evidence.

## Determinism and Constraints

The package must not claim implementation completion before v0.92 work lands.

## Integration Points

- [../README.md](../README.md)
- [../WBS_v0.92.md](../WBS_v0.92.md)
- [../WP_ISSUE_WAVE_v0.92.yaml](../WP_ISSUE_WAVE_v0.92.yaml)
- `#3377`
- [../../v0.91.5/V092_ACTIVATION_TEST_MAP_v0.91.5.md](../../v0.91.5/V092_ACTIVATION_TEST_MAP_v0.91.5.md)

## Validation

Each linked feature doc should pass the active `feature_doc` template
validator and link checks.

## Acceptance Criteria

- Every planned v0.92 feature has a linked feature doc.
- The package validates structurally.
- WP-01 can consume the package without chat reconstruction.

## Risks

- The index may drift from the issue wave. Mitigation: WP-01 must reconcile
  both before opening issues.

## Future Work

Future milestones may add governance, transport-security, signed-trace, and
MVP hardening feature packages.

## Notes

This index intentionally keeps `#3377` and the v0.91.5 activation-test map
visible as launch-readiness sources.

## Feature Documents

- [ACP_COGNITIVE_PROFILES_v0.92.md](ACP_COGNITIVE_PROFILES_v0.92.md)
- [ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md](ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md)
- [CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md](CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md)
- [FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md](FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md)
- [IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md](IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md)
- [MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md](MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md)
- [MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md](MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md)
- [FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md](FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md)

## WP Coverage Map

| Candidate WPs | Feature coverage |
| --- | --- |
| WP-02, WP-09, WP-10 | [FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md](FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md) |
| WP-03, WP-04 | [IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md](IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md) |
| WP-05, WP-06, WP-09 | [MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md](MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md) |
| WP-05, WP-10, WP-16 | [MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md](MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md) |
| WP-07 | [ACP_COGNITIVE_PROFILES_v0.92.md](ACP_COGNITIVE_PROFILES_v0.92.md) |
| WP-08 | [ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md](ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md) |
| WP-11 | [CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md](CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md) |
| WP-12, WP-13, WP-14 | [FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md](FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md) |

Review, quality, docs, remediation, next-milestone planning, and ceremony WPs
are release/process work rather than standalone product features.

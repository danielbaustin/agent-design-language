# v0.92 Feature Plans

## Metadata

- Milestone: `v0.92`
- Version: `v0.92`
- Date: `2026-08-04`
- Owner: ADL maintainers
- Related issues: `#3377`, `#3434`
- Planning template set: `docs/templates/planning/1.0.0`

## Status

Active WP-01 feature-contract index for `v0.92`.

These documents define the tracked feature-doc package for the identity,
continuity, first-birthday, and ACIP transport-readiness band. They are
planning surfaces, not implementation closeout records.

The final issue wave is open. These documents remain planned contracts until
their owning issues land reviewed implementation and proof.

## Template Rules

This index is validated with the same structural feature-doc template so the
feature package remains uniform. It is an index, not an implementation record.

## Purpose

List the tracked v0.92 feature contracts that WP-01 should consume.

## Context

The v0.92 feature package covers the reliable Runtime and distributed polis
substrate, first-birthday identity, and the transport/profile/consumer surfaces
needed for reviewable birth evidence.

## Coverage / Ownership

This index owns package navigation only. Each linked feature doc owns its own
scope, validation, risks, and future-work boundary.

## Overview

The package covers Runtime launch/resilience, distributed Guardian/polis,
birthday, identity/continuity, memory/capability/witnesses, ACP/cognitive
profiles, Adaptive Learning DAG implementation, ACIP/A2A transport readiness,
cross-polis continuity, Observatory/Unity consumers, provider-neutral proof,
and the first-birthday demo/governance handoff.

The tracked external-launch surface under `../external_launch/` supplies
claim-bounded public copy, reviewer FAQ, and publication-gate language for
WP-24 and the release tail without asserting that the birthday event or
publication approval is complete.

## Design

Feature docs should stay evidence-bound, template-valid, and linked from the
milestone README, WBS, sprint plan, and opened issue wave.

## Execution Flow

1. WP-01 reconciles this index, the linked contracts, the final v0.91.8
   handoff, `#3377`, and the opened issue wave.
2. WP-01B updates the canonical `docs/planning/ADL_FEATURE_LIST.md` and current
   documentation/version surfaces before substantive implementation.
3. Owning WPs implement and validate their bounded contracts.
4. WP-23 updates feature status only from landed evidence before review and
   release.

## Determinism and Constraints

The package must not claim implementation completion before v0.92 work lands.

## Integration Points

- [../README.md](../README.md)
- [../WBS_v0.92.md](../WBS_v0.92.md)
- [../WP_ISSUE_WAVE_v0.92.yaml](../WP_ISSUE_WAVE_v0.92.yaml)
- [../external_launch/README.md](../external_launch/README.md)
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

- The index may drift from the issue wave. Mitigation: WP-01 validates exact WP
  ownership now, and WP-23 reconciles final landed truth.

## Future Work

Future milestones may add governance, transport-security, signed-trace, and
MVP hardening feature packages.

## Notes

This index intentionally keeps `#3377` and the v0.91.5 activation-test map
visible as launch-readiness sources.

## Feature Documents

- [ACP_COGNITIVE_PROFILES_v0.92.md](ACP_COGNITIVE_PROFILES_v0.92.md)
- [ADAPTIVE_LEARNING_DAG_v0.92.md](ADAPTIVE_LEARNING_DAG_v0.92.md)
- [ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md](ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md)
- [DISTRIBUTED_GUARDIAN_POLIS_v0.92.md](DISTRIBUTED_GUARDIAN_POLIS_v0.92.md)
- [CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md](CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md)
- [FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md](FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md)
- [IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md](IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md)
- [MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md](MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md)
- [MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md](MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md)
- [OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92.md](OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92.md)
- [PROVIDER_NEUTRAL_MULTI_AGENT_PROOF_v0.92.md](PROVIDER_NEUTRAL_MULTI_AGENT_PROOF_v0.92.md)
- [RUNTIME_LAUNCH_AND_RESILIENCE_v0.92.md](RUNTIME_LAUNCH_AND_RESILIENCE_v0.92.md)
- [FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md](FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md)

## WP Coverage Map

| WPs | Feature coverage |
| --- | --- |
| WP-08, WP-09, WP-10 | [FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md](FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md) |
| WP-03 | [RUNTIME_LAUNCH_AND_RESILIENCE_v0.92.md](RUNTIME_LAUNCH_AND_RESILIENCE_v0.92.md) |
| WP-04 | [DISTRIBUTED_GUARDIAN_POLIS_v0.92.md](DISTRIBUTED_GUARDIAN_POLIS_v0.92.md) |
| WP-09, WP-10, WP-17 | [IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md](IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md), [CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md](CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md) |
| WP-11, WP-12, WP-15 | [MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md](MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md) |
| WP-11, WP-16 | [MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md](MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md) |
| WP-13 | [ACP_COGNITIVE_PROFILES_v0.92.md](ACP_COGNITIVE_PROFILES_v0.92.md) |
| WP-13A | [ADAPTIVE_LEARNING_DAG_v0.92.md](ADAPTIVE_LEARNING_DAG_v0.92.md) |
| WP-14 | [ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md](ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md) |
| WP-18, WP-18B, WP-19 | [FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md](FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md) |
| WP-18A | [OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92.md](OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92.md) |
| WP-18B | [PROVIDER_NEUTRAL_MULTI_AGENT_PROOF_v0.92.md](PROVIDER_NEUTRAL_MULTI_AGENT_PROOF_v0.92.md) |

## v0.92 Completion Gate

Every feature document in this index must have a landed owning issue and
exact-revision implementation, validation, review, and integration evidence
before the milestone can pass WP-22 or enter WP-25 internal review. A feature
that remains `planned`, lacks real proof, or is replaced by fixtures or
synthetic success is a release blocker. Deferral requires an explicit milestone
scope change approved before review; silence or an open issue is not deferral.

## Supporting Work Tracks

WP-01/WP-01B planning and docs, WP-02 repository migration, WP-02A CI,
WP-05 through WP-07 workflow tooling, WP-20 proof coverage, WP-21/WP-21A code
quality, WP-22/WP-23 quality and docs, WP-24/WP-24A publication, and WP-25
through WP-30 review/release work support the feature package but are not
standalone product features. Their omission from the feature table is
intentional, not forgotten scope.

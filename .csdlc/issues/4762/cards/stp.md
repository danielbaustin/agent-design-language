# Structured Task Prompt

Template: 1.0.0

Issue: 4762

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement the #4762 auditable birth-witness register, receipt package, negative-case dispositions, deterministic validator, handoff consumption links, lifecycle truth, and reviewed ready PR without merging or closeout.

## Deliverables

- .csdlc/prepared/issues/4762/birth-witness-receipt-schema.v1.json
- .csdlc/prepared/issues/4762/birth-witness-receipt-negative-cases.v1.json
- .csdlc/prepared/issues/4762/birth-witness-receipt-design.md
- .csdlc/prepared/issues/4762/birth-witness-receipt-validation.md
- .csdlc/prepared/issues/4762/validate_birth_receipt_package.rb
- docs/milestones/v0.91.8/review/v092_handoff_4762/
- Updated v0.91.8/v0.92 handoff references and retained validation/review evidence.

## Acceptance

1. AC1: Birth witness register and birth receipt artifacts exist at the retained v0.91.8 handoff path with required schema, issue, status, evidence, witness, receipt, and handoff fields.
2. AC2: Negative-case dispositions cover the required startup, wake, restore, snapshot, copied-state, fixture, simulation, migration, forced-suspension, missing-evidence, and unsupported-label cases and reject them fail-closed.
3. AC3: v0.91.8 activation/handoff docs and the v0.92 first-birthday launch packet cite the #4762 package by exact path for downstream consumption.
4. AC4: The package explicitly preserves birth_event_status: not_claimed and does not claim the birthday occurred, legal personhood, production citizenship, completed constitutional governance, or v0.93 governance completion.
5. AC5: Focused validation evidence, exact-head review evidence, and ready PR publication truth are retained without merge or closeout.

## Dependencies

- GitHub issue #4762 and parent WP-21 #5362.
- Live execution claim claim-4762-birth-witness-receipt-execution.
- v0.91.8 activation map and v0.92 first-birthday planning sources.

## Inputs

- .csdlc/prepared/issues/4762/design.md
- .csdlc/prepared/issues/4762/diagram.mmd
- docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md
- docs/milestones/v0.91.8/features/V092_HANDOFF_v0.91.8.md
- docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md
- docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md
- docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md
- docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md

## Non Goals

- Do not claim the v0.92 birthday occurred.
- Do not implement runtime birthday activation, legal personhood, production citizenship, or v0.93 governance.
- Do not merge, close the issue manually, or perform post-merge closeout in this session.

# v0.91.1 Feature Index

## Status

First-pass feature index. Feature docs may be promoted or split when the issue
wave opens, but this index records the intended implementation homes.

| Feature Area | Feature Doc | Source Cluster | Planned WP Home | Required Outcome |
| --- | --- | --- | --- |
| Runtime/polis architecture | [RUNTIME_POLIS_ARCHITECTURE.md](RUNTIME_POLIS_ARCHITECTURE.md) | `.adl/docs/TBD/runtime_v2/` | WP-02 | source-grounded architecture package aligned with current code |
| Agent lifecycle state model | [AGENT_LIFECYCLE_STATE_MODEL.md](AGENT_LIFECYCLE_STATE_MODEL.md) | `.adl/docs/TBD/ADL_AND_SLEEP.md` | WP-03 | lifecycle state contract, transitions, ACIP eligibility, and proof fixtures |
| CSM observatory active surface | [CSM_OBSERVATORY_ACTIVE_SURFACE.md](CSM_OBSERVATORY_ACTIVE_SURFACE.md) | `.adl/docs/TBD/csm_observatory/` | WP-04 | active packet, projection, redaction, and operator visibility |
| Citizen standing | [CITIZEN_STANDING_MODEL.md](CITIZEN_STANDING_MODEL.md) | `.adl/docs/TBD/citizen_standing/` | WP-05 | standing contract, transitions, and naked-actor rejection |
| Citizen state | [CITIZEN_STATE_SUBSTRATE.md](CITIZEN_STATE_SUBSTRATE.md) | `.adl/docs/TBD/citizen_state/` | WP-06 | secure state format, projection, validation, and review boundary |
| Memory/identity architecture | [MEMORY_IDENTITY_ARCHITECTURE.md](MEMORY_IDENTITY_ARCHITECTURE.md) | `.adl/docs/TBD/memory_identity/` | WP-07 | memory/identity architecture without birthday claims |
| Theory of Mind | [THEORY_OF_MIND_FOUNDATION.md](THEORY_OF_MIND_FOUNDATION.md) | `.adl/docs/TBD/ToM/` | WP-08 | ToM schemas, update event, and evidence constraints |
| Capability/aptitude testing | [CAPABILITY_APTITUDE_TESTING.md](CAPABILITY_APTITUDE_TESTING.md) | `.adl/docs/TBD/capability_testing/` | WP-09 | executable harness slice and report shape |
| Intelligence metric architecture | [INTELLIGENCE_METRIC_ARCHITECTURE.md](INTELLIGENCE_METRIC_ARCHITECTURE.md) | `.adl/docs/TBD/intelligence/` | WP-10 | evidence-bound metric architecture and limitations |
| Governed learning | [GOVERNED_LEARNING_SUBSTRATE.md](GOVERNED_LEARNING_SUBSTRATE.md) | `.adl/docs/TBD/learning_model/` | WP-11 | learning update, feedback, and policy boundary |
| ANRM/Gemma | [ANRM_GEMMA_PLACEMENT.md](ANRM_GEMMA_PLACEMENT.md) | `.adl/docs/TBD/anrm/` | WP-12 | ANRM placement, trace extractor, and dataset mapping |
| ACIP hardening | [ACIP_HARDENING.md](ACIP_HARDENING.md) | `.adl/docs/TBD/acip/` | WP-13 | local encrypted/authenticated envelope, redaction, conformance |
| A2A adapter | [A2A_ADAPTER_BOUNDARY.md](A2A_ADAPTER_BOUNDARY.md) | `.adl/docs/TBD/a2a/` | WP-14 | adapter over ACIP, not a parallel comms architecture |
| Runtime inhabitant proof | [RUNTIME_INHABITANT_PROOF.md](RUNTIME_INHABITANT_PROOF.md) | `.adl/docs/TBD/v0.91_1_runtime_observatory_dependency_note.md` | WP-15-WP-16 | observatory-visible agent-shaped run |

## Promotion Rule

Each row now has a tracked feature doc. When this milestone opens, WP cards
should consume these docs directly and tighten them with issue numbers,
validation commands, and implementation-specific proof surfaces.

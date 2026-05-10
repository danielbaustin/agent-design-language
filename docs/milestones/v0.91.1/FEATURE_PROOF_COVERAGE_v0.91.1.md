# Feature Proof Coverage - v0.91.1

## Status

Active milestone coverage map. Some early runtime surfaces are landed; the
broader proof and demo convergence band is still pending.

## Coverage Rule

Each feature should have one truthful proof route:

- landed implementation and focused validation
- integrated demo route
- explicit pending owner in the issue wave

## Feature Coverage Map

| Feature | WP | Current Route | Status |
| --- | --- | --- | --- |
| Runtime/polis architecture | WP-02 | [RUNTIME_POLIS_ARCHITECTURE_PACKAGE_v0.91.1.md](RUNTIME_POLIS_ARCHITECTURE_PACKAGE_v0.91.1.md) | landed |
| Agent lifecycle state model | WP-03 | feature doc + landed runtime/tests | landed |
| CSM observatory active surface | WP-04 | feature doc + landed runtime/tests | landed |
| Citizen standing | WP-05 | feature doc + landed runtime/tests | landed |
| Citizen state | WP-06 | feature doc + landed citizen-state substrate packet, fixtures, and focused runtime/private-state tests | landed |
| Memory/identity architecture | WP-07 | feature doc + landed memory/identity architecture packet, fixtures, and focused runtime/private-state tests | landed |
| Theory of Mind foundation | WP-08 | feature doc + landed ToM packet, fixtures, and focused runtime tests | landed |
| Capability/aptitude testing | WP-09 | feature doc + landed fixture-mode harness, review bundle, and focused harness/demo tests | landed |
| Intelligence metric architecture | WP-10 | feature doc + landed intelligence metric architecture packet, fixture bundle, and focused runtime tests | landed |
| Governed learning | WP-11 | feature doc + landed governed-learning packet, tracked review fixtures, and focused runtime tests | landed |
| ANRM/Gemma placement | WP-12 | feature doc + landed deterministic extractor, tracked dataset/spec/placement artifacts, and focused tooling test | landed |
| ACIP hardening | WP-13 | `adl/src/runtime_v2/acip_hardening.rs`; `adl/src/runtime_v2/tests/acip_hardening.rs`; `adl/src/agent_comms/orchestrate/conformance.inc`; `docs/milestones/v0.91.1/features/ACIP_HARDENING.md` | focused runtime packet + conformance/lifecycle proof |
| A2A adapter boundary | WP-14 | `adl/src/runtime_v2/a2a_adapter_boundary.rs`; `adl/src/runtime_v2/tests/a2a_adapter_boundary.rs`; `adl/src/agent_comms/a2a.inc`; `adl/tests/fixtures/runtime_v2/comms/a2a_adapter_boundary.json` | focused runtime packet + adapter-boundary fixture proof |
| Runtime inhabitant proof | WP-15 | [RUNTIME_INHABITANT_PROOF.md](features/RUNTIME_INHABITANT_PROOF.md), `adl/src/runtime_v2/runtime_inhabitant_integration.rs`, `adl/src/runtime_v2/tests/runtime_inhabitant_integration.rs`, `adl/tests/fixtures/runtime_v2/inhabitant/runtime_inhabitant_integration.json`, `adl/tests/fixtures/runtime_v2/inhabitant/runtime_inhabitant_operator_report.md` | landed |
| Observatory-visible flagship demo | WP-16 | [RUNTIME_INHABITANT_PROOF.md](features/RUNTIME_INHABITANT_PROOF.md), `adl/src/runtime_v2/observatory_flagship.rs`, `adl/src/runtime_v2/tests/observatory_flagship.rs`, `docs/milestones/v0.91.1/review/observatory_flagship_demo/flagship_proof_packet.json`, `docs/milestones/v0.91.1/review/observatory_flagship_demo/flagship_operator_report.md`, `docs/milestones/v0.91.1/review/observatory_flagship_demo/flagship_walkthrough.jsonl`; the tracked flagship packet explicitly maps demo coverage for WP-02 through WP-16 | landed |

## Explicit Deferrals

- `v0.91.2`: tooling, benchmark, productization, publication, and workflow
  recovery surfaces
- `v0.92`: birthday and identity-continuity work

## Non-Claims

- full milestone proof coverage is not complete yet
- review and release convergence have not happened yet

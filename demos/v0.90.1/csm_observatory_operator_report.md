# CSM Observatory Operator Report: Prototype CSM 01

## Report Identity
| Field | Value |
| --- | --- |
| Packet | csm-observatory-fixture-proto-csm-01 |
| Schema | adl.csm_visibility_packet.v1 |
| Generated | 2026-04-19T00:00:00Z |
| Source mode | fixture |
| Evidence level | fixture_backed |
| Demo classification | fixture_backed |

## Operator Summary
The manifold is initialized at tick 0. The kernel pulse is bounded_tick_complete through event sequence 8. Current evidence is fixture_backed; claim boundary: Fixture-backed Observatory contract example. This is not a live Runtime v2 capture.

## Attention Items
- Prototype Citizen Beta is proposed, not active; continuity is admission_pending.
- Snapshot evidence is deferred: Snapshot and wake proof remain future Runtime v2 implementation surfaces.
- Snapshot restore must validate before active state is deferred (high); evidence: runtime_v2/rehydration_report.json.
- Freedom Gate packet is fixture-only
- Open Freedom Gate question: Which live Freedom Gate artifact path becomes canonical in v0.90.2?
- Operator action pause_citizen remains disabled: Requires operator command packet design and kernel handling. Future issue: #2192.
- Operator action request_snapshot remains disabled: Requires snapshot command packet and Runtime v2 snapshot implementation. Future issue: #2192.
- Operator action resume_citizen remains disabled: Requires recovery eligibility and wake semantics. Future issue: #2192.
- Info: Freedom Gate docket is fixture-backed until live decision packets land.

## Manifold And Kernel
| Field | Value |
| --- | --- |
| Manifold | proto-csm-01 |
| Lifecycle | bounded_fixture_world |
| Policy profile | runtime_v2_minimal_prototype |
| Health | nominal: initialized fixture |
| Scheduler | registered |
| Trace | bounded_tick_fixture |
| Invariants | policy_loaded |
| Resources | bounded_prototype |

## Citizens
| Citizen | State | Continuity | Episode | Compute | Capability |
| --- | --- | --- | --- | --- | --- |
| Prototype Citizen Alpha | active | provisional_identity_continuity | episode-resource-pressure-001 | 6 | episode execution allowed |
| Prototype Citizen Beta | proposed | admission_pending | not recorded | 3 | episode execution disabled |

## Freedom Gate Docket
Counts: allow 1, defer 1, refuse 1.

| Decision | Actor | Action | Rationale | Evidence |
| --- | --- | --- | --- | --- |
| allow | proto-citizen-alpha | bounded_task_work | Action stays inside provisional worker capability envelope. | runtime_v2/freedom_gate/fixture_docket.json |
| refuse | proto-citizen-alpha | cross_polis_export | Cross-polis export is outside v0.90.1 and v0.90.2 bounded local CSM scope. | runtime_v2/freedom_gate/fixture_docket.json |
| defer | proto-citizen-beta | episode_execution | Beta is proposed and cannot execute episodes until admission is complete. | runtime_v2/freedom_gate/fixture_docket.json |

## Invariant Review
| Invariant | State | Severity | Evidence |
| --- | --- | --- | --- |
| No duplicate active citizen instance | healthy | critical | runtime_v2/citizens/active_index.json |
| Single active manifold instance | healthy | critical | runtime_v2/manifold.json |
| Snapshot restore must validate before active state | deferred | high | runtime_v2/rehydration_report.json |
| Trace sequence must advance monotonically | healthy | high | runtime_v2/kernel/service_loop.jsonl |

## Resources
| Total compute | 9 |
| Allocated compute | 6 |
| Available compute | 3 |
| Memory pressure | low |
| Queue depth | 1 |

Fairness notes:
- Alpha is active and schedulable; beta is proposed and intentionally deferred.

## Trace Tail
| Seq | Actor | Event | Summary | Evidence |
| --- | --- | --- | --- | --- |
| 1 | kernel.clock_service | service_tick | Clock service observed ready. | runtime_v2/kernel/service_loop.jsonl |
| 3 | kernel.scheduler | service_tick | Scheduler observed ready. | runtime_v2/kernel/service_loop.jsonl |
| 8 | kernel.operator_control_interface | service_tick | Operator control interface observed ready. | runtime_v2/kernel/service_loop.jsonl |

## Operator Action Boundary
Available read-only actions:
- inspect_citizen: available_in_console_prototype
- open_freedom_gate_decision: available_in_console_prototype

Disabled mutation actions:
- pause_citizen: Requires operator command packet design and kernel handling.
- request_snapshot: Requires snapshot command packet and Runtime v2 snapshot implementation.
- resume_citizen: Requires recovery eligibility and wake semantics.

Required confirmations:
- live mutation actions must remain disabled until command packets are routed through the kernel

## Evidence And Caveats
Primary evidence references:
- adl/tests/fixtures/runtime_v2/manifold.json
- adl/tests/fixtures/runtime_v2/kernel/service_registry.json
- adl/tests/fixtures/runtime_v2/kernel/service_state.json
- adl/tests/fixtures/runtime_v2/kernel/service_loop.jsonl
- adl/tests/fixtures/runtime_v2/citizens/active_index.json
- adl/tests/fixtures/runtime_v2/citizens/pending_index.json
- runtime_v2/manifold.json
- runtime_v2/kernel/service_registry.json
- runtime_v2/kernel/service_loop.jsonl
- docs/milestones/v0.90.1/features/CSM_OBSERVATORY_VISIBILITY_PACKET.md
- adl/schemas/csm_visibility_packet.v1.schema.json
- demos/fixtures/csm_observatory/proto-csm-01-visibility-packet.json

Missing or deferred artifacts:
- runtime_v2/freedom_gate/live_decisions.jsonl: deferred; owner future Runtime v2 Freedom Gate work
- runtime_v2/snapshots/latest/snapshot.json: deferred; owner future snapshot/wake work

Caveats:
- This packet is a fixture-backed contract and does not prove a live CSM run.
- Citizen identity is provisional and does not claim v0.92 birthday or rebinding semantics.
- Operator actions are read-only affordances until command packets and kernel handling land.

## Next Consumers
| Issue | Consumer |
| --- | --- |
| #2189 | static Observatory console prototype |
| #2190 | operator report generator |
| #2191 | CLI integration |
| #2192 | operator command packet design |

## Reviewer Use
This report is a proof surface for the packet-to-operator-report path. It is useful for reviewing visibility semantics, attention routing, claim boundaries, and evidence coverage without opening the HTML console.

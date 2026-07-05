# CSM Observatory Operator Report: Runtime / Ops Soak #1

## Report Identity
| Field | Value |
| --- | --- |
| Packet | v0916-runtime-soak-observatory-packet-0001 |
| Schema | adl.csm_visibility_packet.v1 |
| Generated | 2026-07-05T03:09:55.004005+00:00 |
| Source mode | captured_artifacts |
| Evidence level | bounded_local_runtime_capture |
| Demo classification | captured_artifacts |

## Operator Summary
The manifold is Stopped_after_reviewable_capture at tick 3. The kernel pulse is bounded_review_tick_complete through event sequence 3. Current evidence is bounded_local_runtime_capture; claim boundary: This packet is a bounded local capture produced by the v0.91.6 integrated runtime soak. It is derived from runtime-owned artifacts, suitable for Unity Observatory consumption, and explicitly does not claim full product completion, live telemetry streaming, or v0.92 coherence.

## Attention Items
- Observatory export lane is awake, not active; continuity is unity_contract_export_ready.
- Runtime lane alpha is paused, not active; continuity is restart_proved_then_Stopped.
- Artifact capture is bounded and local; no always-on or remote streaming claim is made.
- Direct runtime mutation remains out of scope for the Unity consumer surface.
- Operator action mutate_runtime_from_unity_surface remains disabled: The Unity Observatory consumer remains fail-closed and packet-driven in v0.91.6. Future issue: #4555.
- Operator action promote_to_live_streaming remains disabled: The Soak #1 observatory handoff is a bounded retained capture, not a live streaming bridge. Future issue: #4555.
- Unity contract and operator report are generated from the same soak-owned packet.

## Citizens
| Citizen | State | Continuity | Episode | Capability |
| --- | --- | --- | --- | --- |
| Runtime lane alpha | paused | restart_proved_then_Stopped | episode-runtime-cadence | episode execution allowed |
| Resilience review lane | active | classification_proofs_captured | episode-resilience-classification | episode execution allowed |
| Observatory export lane | awake | unity_contract_export_ready | episode-unity-handoff | episode execution disabled |

## Freedom Gate Docket
Counts: allow 1, defer 1, refuse 1.

| Decision | Actor | Action | Rationale | Evidence |
| --- | --- | --- | --- | --- |
| allow | runtime-lane-alpha | honor_stop_between_cycles | Stop requests are allowed only after a bounded cycle completes and the cadence window is safe. | docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/long_lived_agent_stop_probe/stop_probe.json |
| defer | runtime-lane-beta | attempt_parallel_provider_work_under_saturation | Bulkhead saturation remains visible and reviewable instead of being hidden behind automatic queue growth. | docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/resilience/bulkhead_execution.json |
| refuse | runtime-lane-beta | treat_hanging_local_endpoint_as_success | Remote timeout failures must remain classifiable and retry-aware rather than being treated as success. | docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/remote_exec/timeout_probe.json |

## Invariant Review
| Invariant | State | Severity | Evidence |
| --- | --- | --- | --- |
| Unity consumer surface remains packet-driven and read only | healthy | critical | docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/runtime_v2/observatory/operator_report.md |
| Stop requests settle between cadence cycles | healthy | high | docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/long_lived_agent_stop_probe/stop_probe.json |
| Timeout and degraded fallback traces remain reviewer-readable | healthy | high | docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/resilience/timeout_execution.json |

## Operator Action Boundary
Available read-only actions:
- inspect_runtime_soak_packet: available_from_bounded_capture
- open_unity_operator_report: available_from_same_packet_bundle
- stage_unity_contract: available_for_local_unity_consumption

Disabled mutation actions:
- promote_to_live_streaming: The Soak #1 observatory handoff is a bounded retained capture, not a live streaming bridge.
- mutate_runtime_from_unity_surface: The Unity Observatory consumer remains fail-closed and packet-driven in v0.91.6.

## Evidence And Caveats
Primary evidence references:
- docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/long_lived_agent/state/status.json
- docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/long_lived_agent/state/cycle_ledger.jsonl
- docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/inspection/latest.json
- docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/long_lived_agent_stop_probe/stop_probe.json
- docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/resilience/timeout_execution.json
- docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/resilience/bulkhead_execution.json
- docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/resilience/degraded_fallback_execution.json
- docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/remote_exec/timeout_probe.json
- docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/obsmem/transition_memory_request.json
- docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/runtime_v2/observatory/visibility_packet.json
- docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/runtime_v2/observatory/operator_report.md
- docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/runtime_v2/observatory/unity_observatory_contract.json

Caveats:
- This is a bounded local capture and not a live telemetry stream.
- Unity consumes a deterministic contract produced after the soak run; it is not co-executing the runtime.
- Inhabitant identity, profile, and v0.92 rebinding semantics remain out of scope.

## Reviewer Use
This report is a proof surface for the packet-to-operator-report path. It is useful for reviewing visibility semantics, attention routing, claim boundaries, and evidence coverage without opening the HTML console.

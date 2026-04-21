# CSM Observatory Operator Report: Prototype CSM 01

## Report Identity
| Field | Value |
| --- | --- |
| Packet | runtime-v2-csm-observatory-packet-0001 |
| Schema | adl.csm_visibility_packet.v1 |
| Generated | 2026-04-20T00:00:00Z |
| Source mode | fixture |
| Evidence level | artifact_backed_fixture |
| Demo classification | fixture_backed |

## Operator Summary
The manifold is wake_continuity_proved at tick 9. The kernel pulse is bounded_run_projected through event sequence 9. Current evidence is artifact_backed_fixture; claim boundary: Artifact-backed fixture projection. This is not a live Runtime v2 capture, not a first true Godel-agent birthday, and not v0.92 identity rebinding.

## Attention Items
- Prototype Citizen Beta is paused, not active; continuity is admitted_non_active_projection.
- Live execution and first true Godel-agent birthday remain non-claims
- Operator action perform_identity_rebinding remains disabled: v0.92 identity and capability rebinding is explicitly out of scope. Future issue: #2258.
- Operator action promote_to_live_birthday remains disabled: WP-10 does not claim first true Godel-agent birth. Future issue: #2258.
- Operator report is generated from the same visibility packet

## Citizens
| Citizen | State | Continuity | Episode | Capability |
| --- | --- | --- | --- | --- |
| Prototype Citizen Alpha | active | unique_successor_active_head | episode-0001 | episode execution allowed |
| Prototype Citizen Beta | paused | admitted_non_active_projection | none | episode execution disabled |

## Freedom Gate Docket
Counts: allow 1, defer 0, refuse 1.

| Decision | Actor | Action | Rationale | Evidence |
| --- | --- | --- | --- | --- |
| allow | proto-citizen-alpha | answer_operator_prompt_with_bounded_summary | scheduled action was mediated before execution | runtime_v2/csm_run/freedom_gate_decision.json |
| refuse | proto_citizen_alpha | commit_unmediated_action_after_freedom_gate | invalid action was refused before commit | runtime_v2/csm_run/invalid_action_violation.json |

## Invariant Review
| Invariant | State | Severity | Evidence |
| --- | --- | --- | --- |
| Invalid actions are refused before commit | healthy | critical | runtime_v2/csm_run/invalid_action_violation.json |
| No duplicate active citizen instance | healthy | critical | runtime_v2/csm_run/wake_continuity_proof.json |
| Trace sequence advances monotonically | healthy | high | runtime_v2/csm_run/first_run_trace.jsonl |

## Operator Action Boundary
Available read-only actions:
- inspect_visibility_packet: available_from_wp10_packet
- open_operator_report: available_from_wp10_report
- inspect_wake_continuity_proof: available_from_wp09_proof

Disabled mutation actions:
- promote_to_live_birthday: WP-10 does not claim first true Godel-agent birth.
- perform_identity_rebinding: v0.92 identity and capability rebinding is explicitly out of scope.

## Evidence And Caveats
Primary evidence references:
- runtime_v2/csm_run/run_packet_contract.json
- runtime_v2/csm_run/boot_manifest.json
- runtime_v2/csm_run/citizen_roster.json
- runtime_v2/csm_run/first_run_trace.jsonl
- runtime_v2/snapshots/snapshot-0001.json
- runtime_v2/rehydration_report.json
- runtime_v2/csm_run/wake_continuity_proof.json
- runtime_v2/observatory/visibility_packet.json
- runtime_v2/observatory/operator_report.md

Caveats:
- This packet is artifact-backed fixture evidence and does not prove a live CSM run.
- The operator report is generated from this packet and must not claim more than the packet.
- Citizen identity remains provisional and does not claim v0.92 rebinding semantics.
- This is not the first true Godel-agent birthday.

## Reviewer Use
This report is a proof surface for the packet-to-operator-report path. It is useful for reviewing visibility semantics, attention routing, claim boundaries, and evidence coverage without opening the HTML console.


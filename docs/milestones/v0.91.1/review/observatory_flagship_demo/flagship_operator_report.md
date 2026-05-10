# D12 Inhabited CSM Observatory Flagship

Proof classification: `proving`

Primary proof packet: `runtime_v2/observatory/flagship_proof_packet.json`

Reviewer command: `adl runtime-v2 observatory-flagship-demo --out artifacts/v0911/demo-d12-observatory-flagship`

Citizen continuity basis:
- witness set: `runtime_v2/private_state/continuity_witnesses.json`
- citizen receipt set: `runtime_v2/private_state/citizen_receipts.json`
- redacted projection: `runtime_v2/observatory/private_state_projection_packet.json`
- continuity challenge: `runtime_v2/challenge/challenge_artifact.json`
- sanctuary/quarantine: `runtime_v2/private_state/sanctuary_quarantine_artifact.json`

Sprint 3 runtime/comms bindings:
- lifecycle state contract: `runtime_v2/agent_lifecycle/state_contract.json`
- lifecycle transition matrix: `runtime_v2/agent_lifecycle/transition_matrix.json`
- ACIP hardening packet: `runtime_v2/acip/acip_hardening_packet.json`
- A2A adapter boundary packet: `runtime_v2/acip/a2a_adapter_boundary_packet.json`
- runtime inhabitant integration packet: `runtime_v2/inhabitant/runtime_inhabitant_integration_packet.json`

Operator-facing result: the Observatory can explain why the citizen-state scenario is reviewable, which authority paths are refused, and which ambiguous continuity transition is frozen without exposing canonical private state.

Non-claims: personhood, first true Godel-agent birthday, raw private-state inspection, and unbounded live Runtime v2 execution remain outside this proof.

Feature demo coverage:
- `runtime-polis-architecture` Runtime/polis architecture [WP-02 / dedicated_demo]
  surfaces: docs/milestones/v0.91.1/RUNTIME_POLIS_ARCHITECTURE_PACKAGE_v0.91.1.md, docs/milestones/v0.91.1/DEMO_MATRIX_v0.91.1.md
  summary: Architecture inspection demo proves the runtime/polis package and artifact layout stay aligned.
- `agent-lifecycle-state-model` Agent lifecycle state model [WP-03 / dedicated_demo]
  surfaces: runtime_v2/agent_lifecycle/state_contract.json, runtime_v2/agent_lifecycle/transition_matrix.json
  summary: Lifecycle state demo proves receipt, queue, reject, and invoke eligibility remain explicit.
- `csm-observatory-active-surface` CSM observatory active surface [WP-04 / dedicated_demo]
  surfaces: runtime_v2/observatory/visibility_packet.json, runtime_v2/observatory/operator_report.md
  summary: Observatory active packet demo proves operator-visible projection and redaction without raw state leakage.
- `citizen-standing-model` Citizen standing [WP-05 / dedicated_demo]
  surfaces: runtime_v2/standing/standing_transitions.json, runtime_v2/standing/standing_events.json
  summary: Standing demo proves mediated authority transitions and denied escalation paths.
- `citizen-state-substrate` Citizen state [WP-06 / dedicated_demo]
  surfaces: runtime_v2/citizen_state/citizen_state_substrate.json, runtime_v2/private_state/private_state_observatory_proof.json
  summary: Citizen-state demo proves stale-state awareness, private-state boundaries, and safe projection.
- `memory-identity-architecture` Memory/identity architecture [WP-07 / dedicated_demo]
  surfaces: runtime_v2/memory_identity/memory_identity_architecture.json, runtime_v2/private_state/continuity_witnesses.json
  summary: Memory demo proves witness-backed continuity and observatory-linked identity state.
- `theory-of-mind-foundation` Theory of Mind foundation [WP-08 / dedicated_demo]
  surfaces: runtime_v2/theory_of_mind/theory_of_mind_foundation.json, runtime_v2/memory_identity/memory_identity_architecture.json
  summary: ToM demo proves bounded agent-model updates from explicit evidence rather than spoofed mind-reading.
- `capability-aptitude-testing` Capability/aptitude testing [WP-09 / dedicated_demo]
  surfaces: docs/milestones/v0.91.1/review/capability_aptitude_testing_fixture/scorecard.json, docs/milestones/v0.91.1/review/capability_aptitude_testing_fixture/final_report.md
  summary: Capability demo proves fixture-mode execution and bounded internal evaluation with explicit limitations.
- `intelligence-metric-architecture` Intelligence metric architecture [WP-10 / dedicated_demo]
  surfaces: runtime_v2/intelligence/intelligence_metric_architecture.json, docs/milestones/v0.91.1/review/intelligence_metric_architecture_fixture/scorecard.json
  summary: Intelligence demo proves evidence-bound metrics layered over capability and ToM artifacts.
- `governed-learning-substrate` Governed learning substrate [WP-11 / dedicated_demo]
  surfaces: runtime_v2/learning/governed_learning_substrate.json, docs/milestones/v0.91.1/review/governed_learning_fixture/accepted_feedback_update.json
  summary: Governed learning demo proves accepted, rejected, and rollback-aware update boundaries.
- `anrm-gemma-placement` ANRM/Gemma placement [WP-12 / dedicated_demo]
  surfaces: docs/milestones/v0.91.1/review/anrm_gemma_trace_dataset/anrm_trace_dataset.json, docs/milestones/v0.91.1/review/anrm_gemma_trace_dataset/anrm_trace_extractor_spec.json
  summary: ANRM/Gemma demo proves deterministic trace extraction and dataset/spec parity.
- `acip-hardening` ACIP hardening [WP-13 / dedicated_demo]
  surfaces: runtime_v2/acip/acip_hardening_packet.json, runtime_v2/access_control/access_events.json
  summary: ACIP hardening demo proves authenticated local communication remains state-gated and reviewable.
- `a2a-adapter-boundary` A2A adapter boundary [WP-14 / dedicated_demo]
  surfaces: runtime_v2/acip/a2a_adapter_boundary_packet.json, runtime_v2/acip/acip_hardening_packet.json
  summary: A2A adapter demo proves compatibility stays layered over ACIP rather than becoming a second transport model.
- `runtime-inhabitant-proof` Runtime inhabitant proof [WP-15 / integrated_demo_dependency]
  surfaces: runtime_v2/inhabitant/runtime_inhabitant_integration_packet.json, runtime_v2/inhabitant/runtime_inhabitant_operator_report.md
  summary: WP-15 integrates standing, state, lifecycle, memory, capability, learning, access, and comms into one agent-shaped proof surface.
- `observatory-visible-flagship-demo` Observatory-visible flagship demo [WP-16 / flagship_demo]
  surfaces: runtime_v2/observatory/flagship_proof_packet.json, runtime_v2/observatory/flagship_operator_report.md, runtime_v2/observatory/flagship_walkthrough.jsonl, runtime_v2/challenge/challenge_artifact.json, runtime_v2/operator/control_report.json
  summary: D12 flagship demo proves the inhabited observatory route and explicitly aggregates the earlier feature demos through 7 standing/access refs.

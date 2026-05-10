# Demo Matrix - v0.91.1

## Status

Candidate demo matrix. These demos should be implemented only after the
supporting work packages land.

| Demo | Owning WP | Purpose | Proving Surface | Non-Claims |
| --- | --- | --- | --- | --- |
| Runtime/polis architecture inspection | WP-02 | Show runtime/polis docs match current code and artifact layout. | architecture report and source-linked checklist | does not prove inhabited runtime |
| Agent lifecycle state demo | WP-03 | Show lifecycle states, transitions, and ACIP receipt/invocation eligibility. | transition matrix, lifecycle fixtures, and denied-invocation report | does not claim consciousness or birthday |
| Observatory active packet demo | WP-04 | Show operator-visible projection from active runtime packet data. | projection artifact and redaction check | does not expose private state |
| Citizen standing/state fixture demo | WP-05, WP-06 | Show standing and state transitions with safe projection. | valid/invalid fixtures, citizen-state substrate packet, and validation output | does not claim constitutional citizenship |
| Memory/ToM evidence demo | WP-07, WP-08 | Show bounded memory and agent-model updates from explicit evidence. | memory/identity architecture packet, ToM packet, fixtures, and deterministic update report | does not claim mind-reading or final identity |
| Capability/intelligence report demo | WP-09, WP-10 | Show capability and intelligence signals from executable fixtures. | WP-09 lands the fixture-mode capability bundle and report artifact with limitations; WP-10 lands the intelligence metric architecture packet, golden fixture, and bounded scorecard/report overlays | does not create reputation scoreboard |
| Governed learning demo | WP-11 | Show feedback/update boundaries under policy. | governed-learning substrate packet, accepted/rejected/unsafe review fixtures, and rollback audit case | does not allow hidden self-modification |
| ANRM/Gemma trace dataset demo | WP-12 | Show trace extraction and dataset mapping. | `adl/tools/build_v0911_anrm_trace_dataset.py`, `docs/milestones/v0.91.1/review/anrm_gemma_trace_dataset/anrm_trace_dataset.json`, `docs/milestones/v0.91.1/review/anrm_gemma_trace_dataset/anrm_trace_extractor_spec.json`, and limitations report | does not claim model training success |
| ACIP/A2A hardening demo | WP-13, WP-14 | Show authenticated local comms, redaction, adapter boundary, and state-gated invocation. | `adl/src/runtime_v2/acip_hardening.rs`, `adl/src/runtime_v2/a2a_adapter_boundary.rs`, `adl/src/agent_comms/a2a.inc`, and the ACIP/A2A proof packets; packet paths are generated as `runtime_v2/acip/acip_hardening_packet.json` and `runtime_v2/acip/a2a_adapter_boundary_packet.json` | does not claim external transport readiness |
| Runtime inhabitant integration proof | WP-15 | Show standing, state, lifecycle, memory, comms, capability, learning, and observatory surfaces integrated into one agent-shaped run surface. | integrated run packet, dependency checklist, trace/projection fixture, and redaction check | does not claim birthday, full identity continuity, or flagship completeness |
| Observatory-visible agent flagship demo | WP-16 | Show an agent-shaped run inside the CSM boundary with operator projection. | run artifacts, trace, lifecycle state, comms evidence, projection | does not claim birthday or autonomous federation |
| Multi-agent podcast pilot | supplemental demo follow-on | Turn the earlier multi-agent transcript wave into one recurring transcript-first episode format. | episode contract, pilot transcript, role register, and proof note | does not claim long-term identity continuity, native audio production, or a broad media platform |

## Flagship Acceptance

The flagship demo must include:

- a governed runtime identity or standing record
- a state transition or runtime packet
- lifecycle-state evidence showing whether the agent may receive, queue,
  reject, or invoke ACIP messages
- one authenticated local communication or invocation event
- a trace/projection visible to the observatory surface
- a redaction proof for private material
- an operator-facing report explaining what was proven and what was not

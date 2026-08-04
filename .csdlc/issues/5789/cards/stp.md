# Structured Task Prompt

Template: 1.0.0

Issue: 5789

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Fix the existing Runtime v3 HTML Observatory and the minimal runtime/control-plane paths required by its documented controls; do not broaden into unrelated runtime redesign.

## Deliverables

- Default live Runtime v3 Observatory wiring
- Truthful live/read/stream/fallback state model
- Working or fail-closed operator-to-agent messaging controls
- Resolved links/export/events/control behavior
- Focused tests and browser proof artifacts
- Ready PR with Closes #5789

## Acceptance

1. AC-1: The default https://localhost:8765/demos/html-observatory/ route uses the active Runtime v3 API when available and does not leave live-ready fields pending.
2. AC-2: The explicit Runtime v3 query route and the default route agree on live feed identity, agent count, event count, process ids, and freshness.
3. AC-3: WebSocket connect/live mode works for public read or reports a precise actionable failure while live GET feed remains visibly authoritative.
4. AC-4: Runtime process id, vector process id, and port liveness shown in the UI match repo-native adl process status probes.
5. AC-5: Packet, Proof Report, Runtime, Agents, Events, Evidence, Export, and mode controls resolve to correct live or retained surfaces with no stale hidden v2 assumptions.
6. AC-6: Operator-to-agent messaging supports target selection, compose, authenticated submit, delivery/receipt display, event-tail visibility, and fail-closed diagnostics for missing auth, bad envelope, stale agent, unavailable write channel, or denied policy.
7. AC-7: Unauthenticated writes cannot silently send messages, and retained AWS/CloudWatch evidence is never presented as live proof.
8. AC-8: Browser-level and CLI validation exercise the checked-in page/routes and negative/offline cases, not only generated fixtures.

## Dependencies

- Current Runtime v3 service running locally on https://localhost:20997
- Current Observatory static server on https://localhost:8765
- Existing runtime process-status helper

## Inputs

- GitHub issue #5789
- demos/html-observatory/index.html
- demos/html-observatory/app.js
- demos/html-observatory/runtime-v3.config.json
- demos/html-observatory/README.md
- adl/src/csm_runtime_api.rs
- adl/src/csm_shepherd_agent.rs
- adl/src/csm_resident_agents.rs
- docs/tooling/PERMISSION_SAFE_PROCESS_STATUS.md

## Non Goals

- AWS or CloudWatch live operations
- mock-only demo replacement
- unrelated Runtime v3 redesign
- production deployment claims
- tracked edits on main

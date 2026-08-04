# Structured Task Prompt

Template: 1.0.0

Issue: 5764

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Fix only the observed overnight monitoring ambiguity for Runtime v3 and the HTML Observatory.

## Deliverables

- Canonical readiness/probe route behavior or explicit docs truth
- Weather freshness surfaced as an unambiguous readiness/degradation signal
- Updated Observatory README/operator check recipe
- Focused Runtime v3 tests proving route and stale-weather behavior

## Acceptance

1. AC-1 Docs name the correct versioned Runtime v3 probe endpoints and do not direct users to unversioned /health or /ready paths
2. AC-2 Either /v1/ready exists as a bounded readiness endpoint, or docs explicitly state readiness is derived from /v1/health plus /v1/observatory fields
3. AC-3 Stale weather is unambiguous in live checks: refreshed under normal live operation or surfaced as explicit degraded readiness with tests and docs
4. AC-4 Watcher recipe uses canonical endpoints and reports changed blockers only
5. AC-5 Focused Runtime v3 route tests and docs/diff validation pass
6. AC-6 PR body includes Closes #5764

## Dependencies

- Runtime v3 ControlService and Axum router
- Existing Runtime v3 observatory feed
- Existing HTML Observatory README run instructions

## Inputs

- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/tests/control.rs
- docs/api/runtime-v3/v1/runtime.openapi.json
- docs/api/runtime-v3/v1/observatory.openapi.json
- demos/html-observatory/README.md

## Non Goals

- Default Runtime v3 cutover
- Runtime v2 decommission
- Browser mutation authority expansion
- AWS or external deployment work

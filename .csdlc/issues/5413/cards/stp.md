# Structured Task Prompt

Template: 1.0.0

Issue: 5413

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Repair only the five findings in #5413 and the minimum coupled tests, proof tooling, demo client, and evidence packet required to prove them.

## Deliverables

- truthful parity classifications with real cross-runtime proof where equivalence is claimed
- authenticated Runtime v3 Observatory feed for local and remote clients
- real live HTTPS Observatory client proof
- bounded weather refresh/staleness implementation and regressions
- complete #5277-#5286 release child/PR/check inventory

## Acceptance

1. AC-1 parity groups use real cross-runtime comparisons or truthful non-equivalence classifications
2. AC-2 Observatory proof drives a real browser or client against a running HTTPS Runtime v3 endpoint
3. AC-3 local and remote feed access is authenticated and documented consistently
4. AC-4 weather refresh and staleness semantics are bounded and tested
5. AC-5 release packet includes the complete #5277-#5286 child, PR, and check wave with corrected classifications

## Dependencies

- #5412 exact reviewed Runtime v3 state-authenticity baseline
- source review #5276
- child issues #5277-#5286 and their merged PR/check evidence

## Inputs

- docs/reviews/v0.91.7/remaining-sprints-5403/RUNTIME_V3_LIVE_PARITY_REVIEW_5276.md
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/parity.rs
- adl-runtime-kernel/src/weather.rs
- adl/tools/validate_v0917_html_observatory.py
- demos/v0.91.7/html-observatory/app.js
- docs/architecture/runtime_v3_observatory_consumption_5286.v1.json

## Non Goals

- default Runtime v3 cutover
- unrelated Runtime v2 removal
- Unity Observatory remediation outside the #5276 findings
- broad UI redesign

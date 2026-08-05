# Structured Review Prompt

Template: 1.0.0

Issue: 5701

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

## Scope

demos/html-observatory
adl/tools/validate_v0917_html_observatory.py
adl-runtime-kernel/tests/parity.rs
adl-runtime/tests/runtime_api_wss.rs
adl/config/validation_lane_selector.v0.91.6.json
adl/tools/test_select_validation_lanes.sh
adl/tools/test_v0917_html_observatory_integrated_proof.sh
docs/architecture/runtime_v3_observatory_consumption_5286.v1.json
docs/milestones/v0.91.7/DEMO_MATRIX_v0.91.7.md
docs/milestones/v0.91.7/FEATURE_PROOF_COVERAGE_v0.91.7.md
docs/milestones/v0.91.7/review/V0917_WP15_DEMO_CONVERGENCE_4642.md
docs/milestones/v0.91.7/review/demo_matrix_4691/4691-birthday-visible-demo-matrix-proof.md
docs/milestones/v0.91.7/review/wp15_demo_convergence_4642/feature_proof_coverage_4642.json
docs/milestones/v0.91.7/review/wp16_quality_gate_4643/quality_gate_4643.py
docs/reviews/v0.91.7/remaining-sprints-5403/RUNTIME_V3_LIVE_PARITY_REVIEW_5276.md

## Prompts

- Do the OpenAPI contracts cover every real production Runtime v3 and Observatory endpoint without documenting unavailable behavior?
- Does route-parity validation prevent undocumented real routes and documented phantom routes?
- Are WSS authentication, inbound frames, outbound frames, close/error behavior, and correlation identifiers documented accurately?
- Are constants such as port and base URL represented through config/server variables rather than hard-coded runtime behavior?
- Does the implementation avoid #5344 protected paths unless there is explicit typed transfer or release?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:db41b249277a91140d4fd67bfc5bf898f4565774:17d3919116b2dcfd1fba8bb4152b670f3b2155c0a3fbbed2bc189c7d9808a9c0")

Reviewer: Some("codex:019fbbff-9fb2-72f2-8b81-2f03f6ae2e1e")

Result: pass

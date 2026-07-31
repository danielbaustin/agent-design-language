# Structured Review Prompt

Template: 1.0.0

Issue: 5701

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

demos/html-observatory
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

Revision: Some("git-blake3:7801b941d0f00a89c8e8ebe4bc70eb93a73a8359:1d726d40a3ece4218b082358250bced33bde150f73d220bea8af825fc5975f35")

Reviewer: Some("subagent:019fafe9-2cac-77d2-8aa9-fcbd0157746d")

Result: pass

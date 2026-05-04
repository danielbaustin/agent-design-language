# Feature Docs - v0.90.5

## Implementation-Facing Features

| Feature Doc | Purpose | Execution WPs |
| --- | --- | --- |
| features/TOOL_CALL_THREAT_MODEL_AND_SEMANTICS.md | proposal/action boundary, tool-call threat model, side-effect taxonomy, dangerous-category denial expectations, and governed-tools non-goals | WP-02 |
| features/UTS_PUBLIC_SPEC_AND_CONFORMANCE.md | Universal Tool Schema v1.0 public-compatible schema, examples, invalid examples, extension rules, and conformance | WP-03-WP-05 |
| features/ACC_AUTHORITY_AND_VISIBILITY.md | ADL Capability Contract v1.0 authority, identity, delegation, privacy, visibility, redaction, trace, replay, and policy semantics | WP-06-WP-07 |
| features/TOOL_REGISTRY_AND_COMPILER.md | registered tools, binding rules, UTS-to-ACC compiler, normalization, and rejection behavior | WP-08-WP-10 |
| features/GOVERNED_EXECUTION_AND_TRACE.md | policy injection, Freedom Gate integration, governed executor, trace, replay, and redaction contract | WP-11-WP-14 |
| features/MODEL_TESTING_AND_FLAGSHIP_DEMO.md | multi-model benchmark, local/Gemma evaluation, dangerous negative suite, and Governed Tools v1.0 flagship demo | WP-15-WP-18 |
| features/LOCAL_MODEL_PR_REVIEWER_TOOL.md | operator and agent usage guide for the demo-grade pre-PR code review tool, fixture backend, and direct Ollama HTTP path | #2603 |
| features/CODING_AGENT_RUNNER.md | provider-neutral coding-agent runner contract, provider-lane matrix, review-handoff boundary, and fixture-mode proof path | #2627 |

## Proof And Audit Docs

| Doc | Purpose | Execution WPs |
| --- | --- | --- |
| WBS_v0.90.5.md | execution plan for Governed Tools v1.0 | WP-01 |
| DEMO_MATRIX_v0.90.5.md | proof matrix, flagship-demo proof boundary, and non-proving classifications | WP-01, WP-19, WP-21 |
| FEATURE_PROOF_COVERAGE_v0.90.5.md | reviewer map from every governed-tools proof row to its landed evidence home plus explicit adjacent non-claims | WP-19 |
| WP_ISSUE_WAVE_v0.90.5.yaml | opened issue-wave source of truth for the tracked v0.90.5 band | WP-01 |
| WP_EXECUTION_READINESS_v0.90.5.md | card-authoring source for concrete WP outputs, validation, non-goals, and proof expectations | WP-01 |
| GET_WELL_PLAN_v0.90.5.md | milestone-root get-well pointer for validation-cost recovery, the separate GW wave, and runtime-reduction disposition | WP-01, GW-00-GW-05, WP-20, WP-25 |
| RELEASE_PLAN_v0.90.5.md | release evidence, public-spec guardrails, and review-handoff expectations | WP-21-WP-26 |
| RELEASE_READINESS_v0.90.5.md | reviewer entry surface for the active docs/review/release tail after proof and quality convergence | WP-21-WP-26 |
| DECISIONS_v0.90.5.md | accepted planning baseline for governed-tools scope and boundaries | WP-01, WP-21 |

## Context / Idea Docs

| Idea Doc | Purpose | Boundary |
| --- | --- | --- |
| ideas/TOOLS_ARE_GOVERNED_CAPABILITIES.md | explain why ADL treats tools as authority-bearing capabilities, not model-callable functions | context only |
| ideas/GEMMA4_UTS_ACC_MODEL_BENCHMARK_PLAN.md | place the Gemma/local/remote benchmark plan across WP-16, WP-17, WP-18, and v0.91 follow-on work | v0.90.5 does bounded demo only; full comparison deferred to v0.91 |
| ideas/TEST_RUNTIME_REDUCTION_PLAN_v0.90.5.md | execution-support plan for reducing heavyweight proof-family runtime cost without weakening governed-tools proof claims | separate GW wave; not a canonical WP |

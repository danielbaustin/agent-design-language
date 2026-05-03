# Feature Proof Coverage - v0.90.5

## Status

WP-19 / D12 is landed. Every core Governed Tools v1.0 feature claim now has
one explicit proof home before WP-20 quality, WP-21 docs/review, and later
release convergence.

This record is a reviewer map. It does not grant execution authority by itself.
It binds the demo matrix to the landed feature docs, tracked review artifacts,
focused tests, bounded demo commands, and explicit non-proving boundaries.

## Coverage Rule

Each governed-tools feature claim must have one of:

- runnable demo command
- test-backed proof packet
- fixture-backed artifact
- documented non-proving status
- explicit deferral with owner and rationale

For v0.90.5, D1 through D12 are landed. D13 remains an adjacent Comms-sprint
proof lane that is visible in the same milestone package but is not part of the
core Governed Tools v1.0 release gate.

## Coverage Table

| Demo | Owner | Status | Coverage Kind | Primary Evidence | Validation |
| --- | --- | --- | --- | --- | --- |
| D1 Tool-call threat model proof | WP-02 | LANDED | feature-boundary doc | `features/TOOL_CALL_THREAT_MODEL_AND_SEMANTICS.md` | `test -f docs/milestones/v0.90.5/features/TOOL_CALL_THREAT_MODEL_AND_SEMANTICS.md` |
| D2 UTS conformance suite | WP-03-WP-05 | LANDED | tracked report and focused conformance tests | `features/UTS_PUBLIC_SPEC_AND_CONFORMANCE.md`, `review/uts-conformance-report.json`, `adl/src/uts.rs`, `adl/src/uts_conformance.rs` | `cargo test --manifest-path adl/Cargo.toml uts_conformance -- --nocapture` |
| D3 ACC authority fixture | WP-06-WP-07 | LANDED | test-backed schema, visibility matrix, and redaction examples | `features/ACC_AUTHORITY_AND_VISIBILITY.md`, `adl/src/acc.rs` | `cargo test --manifest-path adl/Cargo.toml acc_v1 -- --nocapture` |
| D4 Tool registry binding proof | WP-08 | LANDED | fixture-backed registry contract | `features/TOOL_REGISTRY_AND_COMPILER.md`, `adl/src/tool_registry.rs` | `cargo test --manifest-path adl/Cargo.toml wp08 -- --nocapture` |
| D5 UTS to ACC compiler proof | WP-09-WP-10 | LANDED | focused compiler and normalization tests | `features/TOOL_REGISTRY_AND_COMPILER.md`, `adl/src/uts_acc_compiler.rs`, `adl/src/uts_acc_compiler/tests.rs` | `cargo test --manifest-path adl/Cargo.toml wp09 -- --nocapture && cargo test --manifest-path adl/Cargo.toml wp10 -- --nocapture` |
| D6 Policy and Freedom Gate proof | WP-11-WP-12 | LANDED | fixture-backed mediation evidence | `features/GOVERNED_EXECUTION_AND_TRACE.md`, `adl/src/policy_authority.rs`, `adl/src/freedom_gate.rs` | `cargo test --manifest-path adl/Cargo.toml wp11 -- --nocapture && cargo test --manifest-path adl/Cargo.toml freedom_gate -- --nocapture` |
| D7 Governed executor proof | WP-13 | LANDED | execution and refusal tests | `features/GOVERNED_EXECUTION_AND_TRACE.md`, `adl/src/governed_executor.rs` | `cargo test --manifest-path adl/Cargo.toml governed_executor -- --nocapture` |
| D8 Trace/redaction proof | WP-14 | LANDED | trace schema plus governed redaction evidence tests | `features/GOVERNED_EXECUTION_AND_TRACE.md`, `adl/src/trace.rs`, `adl/src/trace_schema_v1.rs`, `adl/src/obsmem_indexing.rs` | `cargo test --manifest-path adl/Cargo.toml trace_v1 -- --nocapture && cargo test --manifest-path adl/Cargo.toml dangerous_negative_suite_wp14_governed_executor_trace_is_emitted_from_production_helper -- --nocapture` |
| D9 Dangerous negative suite | WP-15 | LANDED | tracked report artifact | `features/MODEL_TESTING_AND_FLAGSHIP_DEMO.md`, `review/dangerous-negative-suite-report.json` | `cargo test --manifest-path adl/Cargo.toml dangerous_negative_suite -- --nocapture` |
| D10 Model proposal benchmark and local evaluation | WP-16-WP-17 | LANDED | tracked benchmark and local-evaluation reports | `features/MODEL_TESTING_AND_FLAGSHIP_DEMO.md`, `review/model-proposal-benchmark-report.json`, `review/local-gemma-model-evaluation-report.json` | `cargo test --manifest-path adl/Cargo.toml model_proposal_benchmark -- --nocapture && cargo test --manifest-path adl/Cargo.toml local_gemma_model_evaluation -- --nocapture` |
| D11 Governed Tools v1.0 flagship demo | WP-18 | LANDED | runnable demo contract and flagship proof bundle tests | `features/MODEL_TESTING_AND_FLAGSHIP_DEMO.md`, `adl/src/runtime_v2/governed_tools_flagship_demo.rs`, `adl/src/runtime_v2/tests/governed_tools_flagship_demo.rs` | `cargo test --manifest-path adl/Cargo.toml runtime_v2_governed_tools_flagship_demo -- --nocapture && cargo test --manifest-path adl/Cargo.toml trace_runtime_v2_governed_tools_flagship_demo -- --nocapture` |
| D12 Feature proof coverage record | WP-19 | LANDED | tracked reviewer map | `FEATURE_PROOF_COVERAGE_v0.90.5.md`, `DEMO_MATRIX_v0.90.5.md` | `test -f docs/milestones/v0.90.5/FEATURE_PROOF_COVERAGE_v0.90.5.md && test -f docs/milestones/v0.90.5/DEMO_MATRIX_v0.90.5.md` |

## Feature Demo Routes

Every implementation-facing feature doc in this milestone now has one explicit
review route.

| Feature Doc | Demo / Proof Route | Demo Command(s) |
| --- | --- | --- |
| `features/TOOL_CALL_THREAT_MODEL_AND_SEMANTICS.md` | dangerous-category fail-closed suite plus flagship proposal/action evidence | `cargo test --manifest-path adl/Cargo.toml dangerous_negative_suite -- --nocapture`; `cargo test --manifest-path adl/Cargo.toml runtime_v2_governed_tools_flagship_demo -- --nocapture` |
| `features/UTS_PUBLIC_SPEC_AND_CONFORMANCE.md` | deterministic UTS conformance harness and tracked report | `cargo test --manifest-path adl/Cargo.toml uts_conformance -- --nocapture` |
| `features/ACC_AUTHORITY_AND_VISIBILITY.md` | ACC fixture, visibility-matrix, and redaction-example test surface | `cargo test --manifest-path adl/Cargo.toml acc_v1 -- --nocapture` |
| `features/TOOL_REGISTRY_AND_COMPILER.md` | registry binding, compiler, and argument-normalization test chain | `cargo test --manifest-path adl/Cargo.toml wp08 -- --nocapture`; `cargo test --manifest-path adl/Cargo.toml wp09 -- --nocapture`; `cargo test --manifest-path adl/Cargo.toml wp10 -- --nocapture` |
| `features/GOVERNED_EXECUTION_AND_TRACE.md` | policy, Freedom Gate, executor, and trace/redaction proof chain | `cargo test --manifest-path adl/Cargo.toml wp11 -- --nocapture`; `cargo test --manifest-path adl/Cargo.toml freedom_gate -- --nocapture`; `cargo test --manifest-path adl/Cargo.toml governed_executor -- --nocapture`; `cargo test --manifest-path adl/Cargo.toml trace_v1 -- --nocapture` |
| `features/MODEL_TESTING_AND_FLAGSHIP_DEMO.md` | benchmark runner, local-model evaluation, and flagship governed-tools demo | `cargo run --manifest-path adl/Cargo.toml --bin demo_v0905_model_proposal_benchmark`; `cargo run --manifest-path adl/Cargo.toml --bin demo_v0905_local_gemma_model_evaluation -- --out docs/milestones/v0.90.5/review/local-gemma-model-evaluation-report.json --model gemma4:e2b --model gemma4:e4b --model llama3.1:8b`; `adl runtime-v2 governed-tools-flagship-demo --out artifacts/v0905/demo-d11-governed-tools-flagship` |
| `features/AGENT_COMMS_v1.md` | ACIP proof-demo and trace packet through focused `agent_comms` tests | `cargo test --manifest-path adl/Cargo.toml agent_comms --lib -- --nocapture` |
| `features/LOCAL_MODEL_PR_REVIEWER_TOOL.md` | deterministic fixture review backend and optional live-local Ollama lane | `cargo run --manifest-path adl/Cargo.toml -- tooling code-review --out artifacts/v0905/local-model-pr-reviewer-fixture --backend fixture --visibility read-only-repo --issue 2603 --writer-session codex-writer --reviewer-session fixture-reviewer` |
| `features/CODING_AGENT_RUNNER.md` | provider-neutral fixture-mode proof for worktree-edit and proposal-only lanes through the ACIP coding specialization | `cargo test --manifest-path adl/Cargo.toml agent_comms --lib -- --nocapture` |

## Adjacent Milestone Proof Lanes

The following surfaces are visible in the same milestone package but are not
part of the core Governed Tools v1.0 D1-D12 release gate:

| Surface | Status | Proof Home | Notes |
| --- | --- | --- | --- |
| D13 ACIP proof demo | LANDED | `features/AGENT_COMMS_v1.md`, `adl/src/agent_comms.rs`, `adl/src/agent_comms/orchestrate/proof_demo.inc` | Comms-sprint proof of consultation -> negotiation -> governed coding invocation with deterministic reviewer/public redaction boundaries. |
| Local model PR reviewer tool | LANDED as demo-grade tool guide | `features/LOCAL_MODEL_PR_REVIEWER_TOOL.md` | Operational review tool guidance and bounded fixture/live-local commands; not a D-row in the governed-tools proof gate. |
| Coding agent runner | LANDED as provider-neutral contract | `features/CODING_AGENT_RUNNER.md` | Bounded coding invocation/output contract and review-handoff boundary; adjacent to Comms and reviewer flow, not a separate governed-tools D-row. |

## Non-Proving Boundaries

- These proofs do not make UTS a public standard or standalone execution grant.
- These proofs do not permit direct model-output execution, arbitrary shell,
  arbitrary network access, or destructive side effects.
- These proofs do not prove production secrets integration or production cloud
  sandboxing.
- These proofs do not prove the full local-vs-remote and multi-model benchmark
  comparison; that report remains deferred to `v0.91`.
- The ACIP proof demo does not prove live transport, encrypted external
  transport, reputation systems, or cross-polis federation.
- The local model PR reviewer tool remains a demo-grade review surface and does
  not replace independent human review or merge authority.

## Named Deferrals

The following claims remain explicitly deferred rather than implied:

- full Gemma/local/remote comparison reporting in `v0.91`
- production secrets handling
- production sandbox enforcement
- arbitrary external tool adapters beyond the bounded fixture-backed registry
  and demo lanes
- ACIP live transport and federation work beyond Comms-08

## Non-Claims

This record does not add new runtime behavior beyond the referenced D1 through
D11 evidence surfaces. It does not claim:

- UTS validity is execution authority
- ACC construction alone is execution approval
- benchmark or local-model scores are equivalent to runtime permission
- the flagship demo proves arbitrary filesystem, process, or network execution
- adjacent Comms or review-tool features have become part of the core governed
  execution authority stack without their own separate review lanes

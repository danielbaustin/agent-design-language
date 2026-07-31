# Structured Task Prompt

Template: 1.0.0

Issue: 5526

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Build an execution-ready preparation packet for #5526 and stop before product implementation.

## Deliverables

- Validated six-card C-SDLC v2 packet
- Issue-local design and architecture diagram
- Focused preparation validation lane
- Future provider/model validation plan and review focus

## Acceptance

1. AC-1: every provider retains a distinct vendor identity even when using a shared wire protocol
2. AC-2: provider secrets are read only from approved sources and never emitted to stdout, logs, cards, traces, fixtures, or retained artifacts
3. AC-3: model aliases cannot silently change retained execution identity; resolved provider/model/version truth is recorded
4. AC-4: model discovery is bounded, cached or snapshotted, timeout-controlled, and not required for deterministic replay
5. AC-5: tool-call, JSON, reasoning, multimodal, context-window, streaming, and caching capabilities are evidence-backed per model
6. AC-6: authentication, quota, unsupported-feature, malformed-response, timeout, cancellation, and provider-unavailable paths are classified consistently
7. AC-7: direct-provider proof is distinct from OpenRouter proof and local Ollama/vLLM profiles remain distinct from hosted vendors
8. AC-8: provider matrix feeds scheduler/model-role selection without granting lifecycle authority
9. AC-9: focused tests and one bounded pre-PR review pass before publication

## Dependencies

- Parent WP-09 provider/governed-tool adapter issue #5349 live-merged and ancestral on current origin/main
- Runtime v3 adapter #5341
- Portable bounded execution engine #5340
- Records/signing contracts #5342
- WP-10A distributed workcell #5497, especially #5499 and #5501

## Inputs

- GitHub issue #5526
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/PARALLEL_EXECUTION_PLAN_v0.91.8.md
- docs/milestones/v0.91.8/WP_EXECUTION_READINESS_v0.91.8.md
- adl/src/provider_adapter.rs
- adl/src/provider_adapter_cli.rs
- adl/src/provider_substrate.rs
- adl/src/provider_communication.rs
- adl/src/provider
- adl/src/model_identity.rs
- adl/src/cli/provider_cmd.rs
- adl/tests/provider_tests.rs
- adl/tests/provider_tests
- adl/tests/mock_provider_tests.rs
- adl/tests/local_ollama_provider_tests.rs
- adl/tests/remote_ollama_provider_tests.rs

## Non Goals

- No separate transport stack for every OpenAI-compatible provider
- No OpenRouter-as-direct-provider validation
- No production-readiness claim from one completion
- No autonomous workflow authority from a model or benchmark
- No credential requirement for deterministic adapter closure when live proof is separately gated
- No AWS-based provider routes

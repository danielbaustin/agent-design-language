# Capability-Aware Local Model Execution and Semantic Tool Fallback

## Metadata
- Feature Name: `capability_aware_local_model_execution_and_semantic_tool_fallback`
- Milestone Target: `v0.87.1`
- Status: `complete` (draft | planned | in-progress | complete)
- Owner: `tools`
- Doc Role: `primary` (primary | supporting)
- Supporting Docs: `demos/v0.87.1/codex_ollama_operational_skills_demo.md`, `adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md`
- Feature Types: `runtime`, `artifact`, `architecture` (list: runtime | artifact | policy | architecture)
- Proof Modes: `demo`, `tests`, `review` (list: demo | tests | schema | replay | review)

## Template Rules

- Every section must be completed or explicitly marked `N/A` with a brief justification.
- Sections such as Execution Flow, Demo, Tests, or Schema Validation may be `N/A` for non-runtime or non-artifact features, but must state why.

## Purpose

This feature makes the local-model operational-skills demo capability-aware instead of assuming every local model supports native tool calling. It exists because the Codex CLI + Ollama demo now spans models with different behavior profiles: some local models can participate in the direct Codex tool path, while others are still useful reasoning models but need the runtime to control file edits.

The immediate problem it solves is the mismatch exposed by the demo: `gpt-oss:latest` could participate in the direct tool path, while `deepseek-r1:latest` failed specifically at provider-side tool-calling capability. This feature turns that mismatch into explicit, reviewable runtime behavior within the bounded demo surface.

## Context

- Related milestone: `v0.87.1`
- Related issues: `#1467`, `#1478`
- Dependencies: `#1467`

This feature is a follow-on from the local-model operational-skills demo. The demo proved that tracked skills can be installed and exercised through Codex CLI with a local provider, but it also showed that model capability is provider-specific rather than universal. The current implementation is intentionally bounded to the local-model operational-skills demo path; it does not claim general runtime-wide capability routing yet.

## Coverage / Ownership

Describe how this document participates in the coverage model.

- Primary owner doc: this document
- Covered surfaces:
  - `adl/src/provider_substrate.rs`
  - `adl/tools/local_model_capabilities.v1.json`
  - `adl/tools/demo_codex_ollama_operational_skills.sh`
  - `adl/tools/test_demo_codex_ollama_semantic_fallback.sh`
  - `demos/v0.87.1/codex_ollama_operational_skills_demo.md`
- Related / supporting docs:
  - `demos/v0.87.1/codex_ollama_operational_skills_demo.md`
  - `adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md`

## Overview

This feature adds an explicit capability layer for the local-model operational-skills demo and uses that layer to choose between a native tool-calling path and a semantic fallback path.

Key capabilities:
- classify provider/model combinations by native tool support, semantic fallback eligibility, and structured output mode
- record the chosen capability profile, selection reason, and execution mode in the demo manifest
- route non-tool local models through a bounded JSON-proposal path that the runtime validates and applies deterministically

## Design

### Core Concepts

Describe the main concepts, abstractions, or entities introduced by this feature.

- `ProviderCapabilitiesV1`: normalized capability metadata attached to provider substrate and invocation-target views
- `local_model_capabilities.v1`: tracked capability manifest that maps provider/model patterns to demo runtime behavior
- `structured_output_mode`: the expected response contract for the chosen path, such as native JSON/tool surfaces or prompt-driven strict JSON emission
- `semantic_tool_fallback`: a demo-bounded path where the model does not execute tools and does not write files; instead, it emits one strict JSON proposal containing bounded replacement content for the allowed fixture files, and the runtime alone performs validation and file writes

### Architecture

Explain how the feature is structured and how it integrates with existing systems.

- Inputs (explicit sources / triggers):
  - local provider selection, local model selection, and the tracked capability manifest
- Outputs (artifacts / side effects):
  - demo manifest metadata, fallback prompt/response artifacts, and deterministically updated fixture cards
- Interfaces (APIs, CLI, files, schemas):
  - `provider_substrate_v1`, `provider_invocation_target_v1`, `adl/tools/local_model_capabilities.v1.json`, `adl/tools/demo_codex_ollama_operational_skills.sh`
- Invariants (must always hold):
  - native tool calling is never assumed without an explicit capability match
  - if a model has neither native tool support nor semantic fallback eligibility, the demo must fail with a clear, operator-visible error
  - in fallback mode, the model only emits a bounded JSON proposal; it never executes tools and never writes files directly
  - the runtime is the only component that performs file writes
  - fallback writes are limited to the declared fixture targets: `stp.md` and `sip.md`
  - fallback JSON must be strictly parsed and validated before any write is accepted
  - runtime behavior must be recorded truthfully in the generated manifest

### Data / Artifacts

Describe any artifacts, schemas, or persistent data structures.

- `adl/tools/local_model_capabilities.v1.json`: tracked capability profiles keyed by provider and model glob
- `demo_manifest.json`: run artifact recording capability profile, selection reason, and execution mode
- `semantic_tool_fallback_prompt.md`: generated bounded prompt used only in fallback mode
- `semantic_tool_fallback_parsed.json`: strictly parsed and validated fallback output used for deterministic runtime application
- `semantic_tool_fallback_raw.json`: raw model response captured only in fallback mode

## Execution Flow

If this is a runtime or artifact-bearing feature, describe the execution flow. If not, state "N/A" and explain why.

1. Resolve the provider/model capability profile from `adl/tools/local_model_capabilities.v1.json`.
2. Record the selected capability profile, selection reason, and execution mode in the demo manifest.
3. If the model is native-tool-capable, run the existing Codex CLI tool path against the bounded fixture workspace.
4. If the model is not native-tool-capable but is marked fallback-eligible:
   - build the bounded semantic fallback prompt
   - request one strict JSON object from the model
   - parse the returned JSON
   - validate the contract deterministically:
     - required fields must be present for both `stp.md` and `sip.md`
     - required content must be non-empty and well-formed
     - unexpected keys are rejected
     - target scope is fixed to the allowed fixture files only
   - reject invalid output without applying writes
   - accept valid output and apply writes only to the declared files
5. If the model is neither native-tool-capable nor fallback-eligible, fail with a clear, operator-visible error.
6. Validate the resulting cards and surface either a truthful success state or a bounded operator-visible failure such as timeout.

## Determinism and Constraints

- Determinism guarantees (what must be repeatable and how):
  - mode selection is deterministic because it is resolved from the tracked capability manifest and explicit runtime inputs
  - fallback model generation is not guaranteed deterministic, because the model may emit different valid proposals across runs
  - runtime acceptance and application are deterministic: identical accepted JSON yields identical file writes
  - runtime behavior is deterministic even when model output is not
- Constraints (performance, ordering, limits):
  - fallback execution is currently implemented only for the Ollama local provider
  - fallback is bounded to the card-cleanup fixture in the operational-skills demo, not generalized runtime-wide tool orchestration
  - Ollama fallback calls are time-bounded through `ADL_OLLAMA_GENERATE_TIMEOUT_SECS`

## Integration Points

List integrations generically; include only what applies.

| System / Surface | Integration Type | Description |
| --- | --- | --- |
| Providers | read/observe | Provider substrate now exposes capability metadata for the demo's local-model execution surfaces. |
| Authoring | read/write | The demo consumes a tracked fixture bundle and writes only the bounded `stp.md` and `sip.md` targets. |
| Trace | observe | The runtime writes mode-specific demo artifacts and records the chosen execution mode in the manifest. |
| Tooling | trigger/read/write | The demo script selects native tool calling or semantic fallback and executes the bounded workflow accordingly. |

## Validation

Describe all validation modes required for this feature. Not all features require demos.

### Demo (if applicable)
- Demo script(s): `adl/tools/demo_codex_ollama_operational_skills.sh`
- Expected behavior: tool-capable local models use native Codex tool calling; non-tool local models either complete through semantic fallback or fail clearly with bounded timeout/error messaging

### Deterministic / Replay
- Replay requirements: semantic fallback must accept or reject one parsed JSON payload against the fixed contract for the bounded fixture surfaces
- Determinism guarantees: identical accepted JSON yields identical `stp.md` and `sip.md` writes

### Schema / Artifact Validation
- Schemas involved: `local_model_capabilities.v1` manifest shape; provider substrate v1 JSON shape
- Artifact checks:
  - demo manifest must always record capability profile, selection reason, and execution mode
  - fallback artifacts must exist only in fallback mode
  - fallback parsed output must satisfy the bounded JSON contract before writes are applied

### Tests
- Test surfaces: `cargo test --manifest-path adl/Cargo.toml provider_substrate_ -- --nocapture`, `adl/tools/test_demo_codex_ollama_operational_skills.sh`, `adl/tools/test_demo_codex_ollama_semantic_fallback.sh`

### Review / Proof Surface
- Review method (manual/automated): automated tests plus manual live local-model probes
- Evidence location: `demo_manifest.json`, `semantic_tool_fallback_parsed.json`, demo logs under `artifacts/v0871/codex_ollama_skills/`

## Acceptance Criteria

- Functional correctness (what must work): the demo runtime must choose the correct local-model execution mode and preserve the existing native path for tool-capable models
- Determinism / replay correctness: fallback writes must be runtime-controlled, deterministic after acceptance, and limited to the explicitly declared file set
- Validation completeness (tests/schema/demo/review as applicable):
  - if there is no native tool support and no fallback eligibility, the demo fails with a clear, operator-visible error
  - fallback never writes outside the declared file set
  - fallback artifacts exist only in fallback mode
  - the demo manifest always records capability profile, execution mode, and reason for selection

## Risks

- Primary risks (failure modes): stale capability declarations, misleading fallback success claims, local-model latency causing silent hangs, or documentation that overstates the current scope
- Mitigations: tracked capability manifest, strict fallback contract validation, explicit timeout/error handling, and repeated documentation that the implementation is demo-bounded

## Future Work

- Follow-ups / extensions: promote capability-aware routing beyond the demo path into broader ADL runtime execution surfaces
- Known gaps / deferrals: semantic fallback is currently bounded to the local-model operational-skills demo and does not yet generalize across every provider/runtime surface

## Notes

This feature intentionally does not try to force native tool calling into models that do not support it. The current implementation is bounded to the local-model operational-skills demo path, and the important architectural move is making capability differences explicit while keeping the runtime, not the model, responsible for accepting and applying edits.


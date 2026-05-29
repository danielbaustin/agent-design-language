# Provider Substrate Reconciliation Plan

Status: proposed
Issue: #3477
Milestone: v0.91.4
Audience: ADL and UTS maintainers

## Summary

ADL and UTS now have overlapping provider execution surfaces. ADL owns the canonical typed architecture for provider substrate, model identity, traces, governance, and benchmark identity. UTS has the most operationally mature benchmark runner for hosted and local models, including retry handling, provider status artifacts, and portable model identity rows.

The right next step is not to pull a large runtime component out immediately. The safer path is to stabilize a shared provider evidence contract first, then make ADL and UTS conform to that contract with parity fixtures. Once both repos depend on the same contract shape, extraction into a reusable package can be considered from evidence rather than instinct.

## Problem

The two repos are solving the same provider problem from different directions:

- ADL needs provider identity to be part of trace, runtime, governance, continuity, and benchmark evidence.
- UTS needs provider execution to be simple, portable, standalone, and scientifically credible for repeated benchmark runs.
- The UTS runner has already moved faster than ADL's provider path in practical hosted/local benchmark behavior.
- If the surfaces continue to evolve independently, benchmark evidence will become hard to compare and provider behavior will drift by repo.

The reconciliation objective is to prevent semantic divergence without making UTS depend on ADL internals.

## Source Inventory

Source baseline notes:

- ADL observations are based on the issue #3477 worktree provider and model-identity source files listed below.
- UTS observations are based on a local source inspection of the UTS repository's current benchmark runner and benchmark helper files before this plan was written.
- The UTS-side claims in this document are planning inputs, not a frozen external release audit. The follow-on UTS issue should confirm them against the exact UTS commit used for implementation.

### ADL Provider Surface

Source paths:

- `adl/src/provider_substrate.rs`
- `adl/src/provider/mod.rs`
- `adl/src/provider/http_family.rs`
- `adl/src/provider/local.rs`
- `adl/src/provider/profiles.rs`
- `adl/src/model_identity.rs`

Current strengths:

- Typed Rust provider substrate with `ProviderSubstrateV1`, `ProviderInvocationTargetV1`, provider transports, and capability inference.
- `ModelIdentityV1`, `EvaluatorIdentityV1`, `LaneIdentityV1`, and `BenchmarkIdentityV1` already exist as ADL-level identity concepts.
- Provider profile handling distinguishes OpenAI, Anthropic, Ollama, mock, and generic HTTP profiles.
- HTTP provider paths include bounded provider error excerpts and invocation artifact writing.
- Local provider path supports Ollama CLI execution with timeouts and process cleanup.

Current gaps relative to UTS:

- Provider invocation metadata is not yet aligned with UTS benchmark provider status artifacts.
- Hosted retry/error classification is less explicit than the UTS runner's operational taxonomy.
- Local model digest discovery is not yet the shared execution norm across benchmark artifacts.
- The Python benchmark path and Rust provider path do not share one portable provider contract.
- UTS cannot consume ADL's Rust implementation directly without losing its standalone value.

### UTS Provider Surface

Source paths in the UTS repository:

- `tools/uts_benchmark_runner.py`
- `tools/benchmark/uts_benchmark_panel.py`
- `tools/benchmark/test_uts_benchmark_model_identity.py`
- `MODELS_TESTED.md`

Current strengths:

- One Python benchmark runner handles regular, UTS-only, and governed lanes.
- Hosted providers include OpenAI, Gemini, and Anthropic with retry attempts, provider error metadata, and text extraction.
- Local Ollama execution uses HTTP generation parameters, keep-alive behavior, digest discovery, and portable model identity rows.
- Benchmark artifacts include detailed rows, summary rows, provider status, attempts, provider failures, raw response excerpts, and `model_identity`.
- Ad-hoc model testing is possible without changing the canonical model panel.

Current gaps relative to ADL:

- Provider behavior is embedded in the benchmark runner rather than isolated as a reusable provider substrate module.
- Model identity shape is compatible in spirit with ADL but not generated from a shared schema or parity fixture.
- Evaluator, lane, and benchmark identity are implemented for benchmark utility rather than as first-class ADL trace/runtime concepts.
- UTS should remain standalone, so it needs a portable contract subset rather than an ADL runtime dependency.

## Reconciliation Principle

Define the shared contract before extracting code.

The reusable asset should first be a small provider evidence contract with tests and parity fixtures. Implementation can remain Rust in ADL and Python in UTS until the contract proves stable. Extraction should happen only after both repos repeatedly use the same contract without churn.

## Proposed Shared Contract

The contract should define portable JSON shapes and semantics. ADL should own the canonical design and schema. UTS should consume the compatible subset as a standalone benchmark evidence format.

### `ProviderRouteV1`

Identifies where a provider call is routed.

Required fields:

- `provider_kind`: `hosted`, `local`, `mock`, or `unknown`
- `provider`: provider family such as `openai`, `anthropic`, `gemini`, `ollama`, or `mock`
- `runtime_surface`: `hosted_api`, `ollama_http`, `ollama_cli`, `mock`, or another explicit surface
- `provider_model_id`: provider-native model identifier

Optional fields:

- `endpoint_ref`: redacted endpoint family or named profile, not a secret-bearing URL when avoidable
- `credential_ref`: environment/key-file reference only, never secret material
- `source_registry`: model panel, profile file, or ad-hoc source label

### `ProviderInvocationRequestV1`

Captures the governed request envelope without storing secrets.

Required fields:

- `route`
- `prompt_contract_ref`
- `lane_ref`
- `timeout_seconds`
- `attempt_policy`

Optional fields:

- `inference_parameter_fingerprint`
- `tool_surface`
- `governance_surface`
- `evaluator_ref`
- `benchmark_ref`

### `ProviderAttemptV1`

Records one attempt inside a provider invocation.

Required fields:

- `attempt_index`
- `started_at`
- `duration_ms`
- `status`: `ok`, `error`, or `timeout`
- `retryable`

Optional fields:

- `http_status`
- `provider_error_kind`
- `provider_error_excerpt`
- `raw_response_excerpt`

### `ProviderFailureV1`

Normalizes failure classification across repos.

Recommended error kinds:

- `provider_auth_missing`
- `provider_auth_error`
- `provider_rate_limited`
- `provider_timeout`
- `provider_transient_http`
- `provider_empty_text_output`
- `provider_model_unavailable`
- `provider_billing_blocked`
- `local_runtime_unavailable`
- `local_runtime_busy`
- `provider_error`
- `unknown`

Failure records should preserve the provider-native excerpt in bounded redacted form when available, but should not treat provider prose as a stable machine contract.

### `ProviderInvocationResultV1`

Captures the final provider-call result.

Required fields:

- `route`
- `model_identity`
- `attempts`
- `final_status`: `ok`, `failed`, `skipped`, or `blocked`
- `duration_ms`

Optional fields:

- `output_text_excerpt`
- `failure`
- `artifact_ref`
- `trace_ref`

### `ModelIdentityV1`

ADL's existing model identity object should remain the anchor concept.

Required fields:

- `provider_kind`
- `provider`
- `model_ref`
- `provider_model_id`
- `runtime_surface`
- `identity_strength`
- `observed_at`

Allowed identity strengths:

- `pinned`
- `provider_asserted`
- `tag_only`
- `ad_hoc`
- `unknown`

Rules:

- Hosted models are `provider_asserted` unless the provider exposes immutable revision metadata.
- Local Ollama models are `pinned` only when a digest is observed from the runtime.
- Local Ollama models without a digest are `tag_only`.
- Ad-hoc rows remain allowed but must disclose their source and identity strength.
- No implementation may invent a digest.

### `ProviderStatusArtifactV1`

Benchmark evidence should expose provider status separately from scored correctness.

Required fields:

- `run_id`
- `created_at`
- `runner_version`
- `provider_rows`

Each provider row should include:

- model display name
- provider route
- model identity
- lane status
- attempt count
- final provider status
- failure kind when applicable
- source artifact reference

### `ProviderRunLogV1`

Long-running provider and benchmark execution must produce logs that are easy to watch while the run is active. This is a hard operational requirement, not just a convenience, because hosted and local model runs commonly fail due to connection resets, auth problems, rate limits, stalled local runtimes, missing models, and provider-side empty outputs.

Required behavior:

- Write a line-oriented log file as the run progresses.
- Flush each log event immediately or near-immediately so `tail -f` is useful.
- Include ISO-8601 UTC timestamps on every event.
- Include `run_id`, provider, model, lane, task id, attempt index, and event type when applicable.
- Record provider-call lifecycle events: run start, model start, lane start, task start, attempt start, retry scheduled, attempt success, attempt failure, task result, lane result, model result, and run finish.
- Record normalized failure kind and bounded provider excerpt for errors.
- Keep secrets, authorization headers, raw credentials, and full prompt bodies out of the watch log.
- Keep watch logs separate from scored summary artifacts so a failed provider connection cannot be mistaken for a scored task failure.

Recommended outputs:

- `run.log.jsonl`: machine-readable line-delimited JSON events.
- `run.log`: optional human-readable mirror for quick terminal watching.
- `provider_status.json`: final summarized provider status artifact.
- `details.json`: scored task detail artifact.
- `summary.json`: scored summary artifact.

The JSONL log should be the canonical watch surface. Human-readable logs may be derived from it, but should not be the only source of operational truth.

#### Operator Notes For Watching Provider Runs

Every provider-backed benchmark or smoke run should print or record the run artifact directory before provider calls begin. Operators should be able to watch progress without reading scored artifacts or waiting for the run to finish.

Expected files:

- `run.log.jsonl`: canonical tail-friendly operational event stream.
- `run.log`: optional human-readable mirror.
- `provider_status.json`: final provider health/status summary.
- `details.json`: scored per-task detail output.
- `summary.json`: scored aggregate output.

Primary watch command:

```bash
tail -f <run-dir>/run.log.jsonl
```

Useful filtered watches:

```bash
tail -f <run-dir>/run.log.jsonl | rg 'provider_auth|provider_rate_limited|provider_timeout|local_runtime|retry'
tail -f <run-dir>/run.log.jsonl | rg 'attempt_failure|retry_scheduled|model_result|run_finish'
```

Operators should look for:

- missing or invalid credentials
- provider rate limits
- provider connection resets
- provider timeouts
- provider empty text output
- missing local models
- local runtime busy or hung states
- retries that continue across many tasks or lanes

Operational provider failures are not the same thing as scored task failures. A task should only be counted as an evaluated benchmark failure when the runner records that a valid provider response was evaluated against the task contract. Auth errors, connection failures, missing models, and runtime stalls should remain provider/run-status failures unless the benchmark contract explicitly says otherwise.

The runner should emit the artifact directory and the watch command near run start, for example:

```text
run_dir=docs/milestones/v0.91.4/review/.../runs/<run-id>
watch=tail -f docs/milestones/v0.91.4/review/.../runs/<run-id>/run.log.jsonl
```

## Execution Boundary

### Hosted Providers

Hosted provider calls should standardize on:

- explicit provider family
- provider-native model ID
- timeout policy
- retry policy
- response text extraction semantics
- provider-asserted identity
- bounded redacted error excerpts

The contract should not require hosted providers to expose immutable model digests unless they actually provide them.

### Local Providers

Local benchmark execution should prefer Ollama HTTP for deterministic benchmark control and metadata discovery. Ollama CLI can remain useful for legacy ADL local-provider behavior, but benchmark-grade identity should use HTTP metadata when possible.

Local rules:

- Query runtime metadata before or during benchmark execution when safe.
- Capture a digest when the runtime exposes one.
- Record `pinned` only with an observed digest.
- Record `tag_only` if the local tag exists but no digest can be observed.
- Do not run multiple local models concurrently for benchmark evidence.

## Ownership Model

ADL owns:

- canonical provider identity design
- trace/runtime/governance identity semantics
- Rust schema structs and validation
- cross-repo parity fixtures
- governed adapter compatibility requirements

UTS owns:

- standalone benchmark runner usability
- Python implementation of the compatible provider evidence subset
- canonical UTS benchmark panels and benchmark artifacts
- ad-hoc model testing ergonomics

Shared expectation:

- Contract names and fields should stay compatible.
- UTS should not depend on ADL internals to run.
- ADL should not silently redefine provider evidence semantics that UTS has already published.

## Proposed Migration Sequence

1. Freeze this reconciliation plan as the v0.91.4 design input.
2. Add a dedicated ADL follow-up issue for provider evidence contract schemas and parity fixtures.
3. Add ADL schema tests for `ModelIdentityV1`, provider route, attempts, failures, and status artifacts.
4. Align ADL's Python benchmark runner or explicitly retire it in favor of the UTS runner for standalone UTS benchmark execution.
5. Align ADL Rust provider invocation artifacts with the same provider status and failure taxonomy.
6. Add a UTS follow-up issue to consume the stable portable subset in `tools/uts_benchmark_runner.py` without adding an ADL runtime dependency.
7. Add cross-repo fixture comparison: one hosted success, one hosted failure, one local pinned Ollama row, and one local tag-only row.
8. Reconsider extraction only after both repos pass parity checks across at least one milestone.

## Follow-On Issue Candidates

### ADL: Define provider evidence contract schemas and parity fixtures

Repository: ADL.

Depends on: this reconciliation plan.

Scope:

- Add JSON-schema-like contract docs or Rust-serializable schema fixtures for provider route, invocation result, provider failure, and provider status artifact.
- Include `ModelIdentityV1` as the canonical identity anchor.
- Add parity fixture files under a tracked test/evidence location.

Acceptance criteria:

- Contract fields are documented with required/optional status.
- At least four fixture rows exist: hosted success, hosted failure, local pinned Ollama row, and local tag-only row.
- Provider run logging contract includes tail-friendly JSONL events, immediate flushing expectations, normalized failure kinds, and secret-redaction rules.
- Fixture validation can run without live provider calls.

Validation expectation:

- Focused schema/fixture validation only; no benchmark run required.

Non-goal:

- No provider execution rewrite.

### ADL: Align provider invocation artifacts with shared provider evidence contract

Repository: ADL.

Depends on: ADL provider evidence contract schemas and parity fixtures.

Scope:

- Update Rust provider invocation artifact shape to include provider route, attempts, model identity, and normalized failure kind.
- Preserve compatibility fields during the transition.

Acceptance criteria:

- Rust provider invocation artifacts include route, attempts, model identity, final status, and normalized failure kind where applicable.
- Provider execution writes tail-friendly operational log events without leaking secrets or full prompt bodies.
- Existing compatibility fields remain present during the migration window.
- Fixture or unit tests prove the new fields serialize as expected.

Validation expectation:

- Focused Rust serialization/unit tests; no live provider call required.

Non-goal:

- No live benchmark expansion.

### ADL: Decide benchmark runner ownership

Repository: ADL.

Depends on: ADL provider evidence contract schemas and parity fixtures.

Scope:

- Decide whether ADL's `adl/tools/uts_benchmark_runner.py` remains supported, delegates to UTS, or is retired.
- Document the chosen path clearly.

Decision criteria:

- Which runner is simpler for repeated UTS benchmark operation?
- Which runner emits the provider evidence contract with less duplicate maintenance?
- Which path keeps UTS standalone for non-ADL users?
- Which path avoids two active benchmark semantics for the same lanes?

Acceptance criteria:

- The decision records one of three outcomes: keep-and-align, delegate-to-UTS, or retire-ADL-runner.
- The decision includes migration steps for any deprecated entrypoint.
- Documentation names the supported command surface.

Validation expectation:

- Command-surface documentation check and focused no-provider smoke validation if an entrypoint changes.

Non-goal:

- No new benchmark claims.

### UTS: Consume provider evidence contract subset

Repository: UTS.

Depends on: ADL provider evidence contract schemas and parity fixtures.

Scope:

- Keep `tools/uts_benchmark_runner.py` standalone.
- Add or update tests proving UTS rows emit the compatible provider route, model identity, attempts, failure, and provider status shapes.
- Confirm the UTS source baseline against the exact UTS commit used for implementation.

Acceptance criteria:

- UTS benchmark artifacts include compatible provider route, model identity, attempts, failure, provider run log, and provider status shapes.
- Ad-hoc model rows disclose model source and identity strength.
- Logs can be watched with `tail -f` during hosted/local runs and expose connection, auth, rate-limit, timeout, retry, and local-runtime failures as separate operational events.
- Tests cover hosted provider-asserted rows, local pinned rows, local tag-only rows, and provider failure rows without requiring live providers.

Validation expectation:

- UTS Python fixture/unit tests only; live benchmark runs remain separate evidence work.

Non-goal:

- No ADL runtime dependency.

### ADL and UTS: Evaluate extraction after parity

Repository: ADL should own the architecture decision; UTS should own the standalone usability review. A paired issue may be opened in UTS if extraction becomes likely.

Depends on: ADL and UTS parity fixtures passing against the same contract shape.

Scope:

- If the contract stabilizes, evaluate a small shared package or generated schema source.
- Prefer extraction only if it reduces duplicate maintenance without making UTS harder to run.

Acceptance criteria:

- A decision record states whether extraction is accepted, rejected, or deferred.
- The decision names the package/repo owner if extraction is accepted.
- The decision records why standalone UTS usability is preserved.

Validation expectation:

- Architecture review and fixture parity evidence; no live provider calls required.

Non-goal:

- No premature framework split.

## Validation Strategy

Initial validation should be contract-level and fixture-level, not benchmark-level.

Recommended checks:

- ADL unit tests for valid and invalid `ModelIdentityV1` values.
- ADL fixture tests for provider route, attempts, failure kinds, and provider status artifact shape.
- UTS tests for hosted ad-hoc rows, local pinned rows, local tag-only rows, and provider failure rows.
- Cross-repo fixture comparison that ignores timestamps but checks required field compatibility.
- No live provider call should be required to prove contract compatibility.

## Non-Goals For #3477

This issue does not:

- rewrite ADL provider execution
- rewrite the UTS benchmark runner
- create a new shared package
- run live hosted or local benchmarks
- publish benchmark evidence
- make UTS depend on ADL internals

## Recommendation

Proceed in two phases.

First, stabilize the provider evidence contract and parity fixtures in ADL. Second, port the compatible subset into UTS while keeping the runner standalone. This gives us reuse without coupling, and it gives reviewers a concrete way to see that ADL and UTS are describing the same provider event when they run the same model through the same benchmark lane.

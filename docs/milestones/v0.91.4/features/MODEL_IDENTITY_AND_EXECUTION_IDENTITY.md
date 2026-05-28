# Model Identity And Execution Identity

Status: planned for v0.91.4 implementation.

## Problem

ADL traces, provider records, delegation records, and benchmark rows already carry partial model labels such as `model_ref`, `provider_model_id`, provider metadata, and benchmark model names. Those labels are useful, but they are not enough for durable scientific evidence.

The same apparent model name can refer to different cognitive execution substrates over time because hosted providers can update model snapshots, local runtimes can change quantization or digest, provider wrappers can change request semantics, and benchmark lanes can change prompt or governance contracts.

## Decision

ADL treats model identity as a first-class runtime and trace object named `ModelIdentityV1`.

Required fields:

- `provider_kind`
- `provider`
- `model_ref`
- `provider_model_id`
- `runtime_surface`
- `identity_strength`
- `observed_at`

Optional fields:

- `resolved_digest`
- `source_registry`
- `runtime_fingerprint`
- `inference_parameter_fingerprint`
- `tool_surface`
- `governance_surface`
- `evaluator_ref`
- `lane_ref`
- `benchmark_ref`

Allowed `identity_strength` values:

- `pinned`: the runtime exposed an immutable digest and ADL recorded it.
- `provider_asserted`: a hosted provider asserted the model identity, but no immutable revision was exposed.
- `tag_only`: ADL observed a local model tag but no valid digest.
- `ad_hoc`: the run intentionally used an operator-supplied model outside the canonical registry.
- `unknown`: identity was not strong enough to classify more precisely.

## Compatibility

`model_ref` and `provider_model_id` remain separate.

`model_ref` is ADL's stable project-facing reference. `provider_model_id` is the provider-native identifier actually sent to the provider. They may differ intentionally, and that distinction is preserved in `ModelIdentityV1`.

Existing trace/provider records keep their legacy fields during the compatibility window. New records can include `model_identity` without removing older readers' ability to inspect `model_ref` and `provider_model_id`.

## Execution Identity

Benchmark and governed-tool evidence also needs identity for the evaluation surface itself. ADL defines:

- `EvaluatorIdentityV1`: evaluator name/version, prompt-contract version, and classifier version.
- `LaneIdentityV1`: regular, UTS-only, or UTS+ACC/governed lane plus its contract version.
- `BenchmarkIdentityV1`: benchmark id/version, task-panel digest, model-panel digest, runner version, and contract-lock digest.

These objects make changes in task panels, model panels, prompts, classifiers, or runner contracts visible in evidence.

## Local And Hosted Model Rules

Hosted providers default to `provider_asserted` unless the provider exposes immutable revision metadata.

Local/Ollama paths attempt digest discovery. If a normalized SHA-256 digest is available, the identity is `pinned`. If only the model tag is available, the identity is `tag_only`. ADL must not fake a digest.

## UTS Consumption

The standalone UTS repository should consume the portable subset of this evidence shape without depending on ADL internals.

UTS benchmark rows should include `model_identity`. Canonical UTS rows should distinguish digest-pinned local rows from provider-asserted hosted rows. Ad-hoc model rows remain allowed, but they must also record `model_source: ad_hoc` and an explicit identity strength.

# ADL Provider Capabilities and Runtime Probing
*Proposed v0.9x+ spec (capability contracts, probes, and effective envelopes)*

## Goals

ADL needs a reliable way to describe and **verify** what an LLM provider/runtime can actually do. Providers frequently publish incomplete or optimistic claims (context length, output limits, JSON-mode, tool calling, determinism). For **agents**, these properties must be measurable and replayable.

This document defines:

- A **capability contract schema** (declared capabilities)
- A **runtime probing methodology** (observed capabilities)
- Rules for computing an **Effective Capability Envelope**
- How capability artifacts integrate into **trace**, **replay**, and **learning export bundles**

## Non-Goals

- Standardizing provider marketing claims
- Proving safety or correctness of model outputs
- Replacing existing provider SDKs (ADL only needs a stable integration contract)

---

## Terminology

- **Declared Caps**: Values declared by a provider adapter (docs/config/SDK).
- **Observed Caps**: Values empirically measured by an ADL probe run.
- **Effective Caps**: The conservative envelope ADL uses at runtime.
- **Probe Artifact**: JSON emitted by `adl provider probe` and referenced by trace.

---

## Artifact Types

ADL treats capability information as first-class artifacts.

### Declared Capability Contract Artifact

File: `provider_caps_declared.json`

- Source: provider adapter / configuration
- Semantics: “what the runtime/provider claims”

### Observed Capability Artifact

File: `provider_caps_observed.json`

- Source: probe tool
- Semantics: “what we measured on this machine/runtime”

### Probe Report Artifact (Optional but Recommended)

File: `provider_probe_report.json`

- Source: probe tool
- Semantics: additional diagnostics (latency curves, trial stats, failure modes)

---

## Capability Schema (v1)

This is the canonical JSON shape ADL expects. Providers MAY extend with namespaced fields, but MUST NOT change meaning of core fields.

### `provider_caps_declared.json`

```json
{
  "schema_version": "adl.provider_caps.declared.v1",
  "provider": {
    "kind": "ollama",
    "adapter_version": "0.9.0-dev"
  },
  "model": {
    "name": "Qwen3.5:9b",
    "model_build_fingerprint": "sha256:...",
    "tokenizer_fingerprint": "sha256:..."
  },
  "runtime": {
    "name": "ollama",
    "version": "0.17.5",
    "endpoint": "http://localhost:11434"
  },
  "limits": {
    "max_context_tokens_theoretical": 32768,
    "max_output_tokens": 4096,
    "max_request_bytes": 10485760
  },
  "features": {
    "system_prompt": true,
    "streaming": true,
    "json_mode": false,
    "tool_calling_native": false,
    "vision": false,
    "reasoning_trace_visible": true,
    "seed_supported": false
  },
  "namespaced": {
    "provider.ollama": {}
  }
}
```

### `provider_caps_observed.json`

```json
{
  "schema_version": "adl.provider_caps.observed.v1",
  "provider": {
    "kind": "ollama",
    "adapter_version": "0.9.0-dev"
  },
  "model": {
    "name": "Qwen3.5:9b",
    "model_build_fingerprint": "sha256:..."
  },
  "runtime": {
    "name": "ollama",
    "version": "0.17.5",
    "endpoint": "http://localhost:11434"
  },
  "probe": {
    "started_at": "2026-03-03T18:42:00Z",
    "finished_at": "2026-03-03T18:44:12Z",
    "config": {
      "temperature": 0.0,
      "top_p": 1.0,
      "seed": 12345,
      "trials_per_point": 3,
      "stable_success_threshold": 0.95
    }
  },
  "observations": {
    "max_context_tokens_observed": 32768,
    "max_context_tokens_stable": 28672,
    "max_output_tokens_observed": 4096,
    "json_mode_reliability": 0.87,
    "determinism_hash_stability": 0.92,
    "reasoning_trace_visible": true
  },
  "environment": {
    "os": "macos",
    "arch": "arm64",
    "device": "mps",
    "memory_gb": 64
  },
  "namespaced": {
    "provider.ollama": {}
  }
}
```

Notes:

- *Observed* values are always grounded to a concrete machine/runtime.
- `*_reliability` and `*_stability` values are probabilities in `[0, 1]`.

---

## Effective Capability Envelope

ADL computes an Effective Envelope from declared + observed caps.

### Rule

- **EffectiveCaps = conservative_merge(DeclaredCaps, ObservedCaps)**

Conservative merge means:

- If a field exists in both: use the *minimum* for numeric limits.
- If a field exists only in Observed: use Observed.
- If a field exists only in Declared:
  - Strict mode: fail closed (no EffectiveCaps)
  - Non-strict mode: accept Declared but emit a warning

### Example

```text
declared.max_context_tokens_theoretical = 65536
observed.max_context_tokens_observed    = 32768

effective.max_context_tokens = 32768
```

---

## Runtime Probe Methodology (v1)

### Why probe?

LLMs cannot introspect their own runtime environment. Models frequently hallucinate operational properties (e.g., “I have 256K context”). Probing provides externally verifiable measurements.

### Probe configuration defaults

Probing aims to measure **capabilities**, not creativity. Therefore the probe MUST reduce stochasticity:

- `temperature = 0.0`
- `top_p = 1.0`
- `seed = fixed` (if supported)
- `trials_per_point = 3` (minimum)
- Always use strict output shaping (“answer ONLY with…”)

### Measurements

#### 1) Max accepted context (hard limit)

Goal: largest input that the provider accepts without error/truncation.

Method:

- Incrementally increase prompt size until request fails
- Binary-search the boundary
- Record `max_context_tokens_observed`

#### 2) Max stable context (usable limit)

Goal: largest context where the model still reliably accesses early/mid/late content.

Method: multi-needle depth sweep

- Embed N needles at known depth fractions, e.g. `[0.05, 0.25, 0.50, 0.75, 0.95]`
- Ask for exactly one needle token per trial
- Score success; compute success rate per depth and overall
- Define “stable” as overall success ≥ `stable_success_threshold`
- Binary-search largest context meeting the threshold
- Record `max_context_tokens_stable`

#### 3) Output token cap

Goal: maximum output length before truncation.

Method:

- Prompt the model to emit exactly K repeated tokens
- Increase K until truncation is detected
- Record `max_output_tokens_observed`

#### 4) JSON compliance reliability

Goal: estimate how often the model adheres to a JSON schema under strict instruction.

Method:

- Provide a small JSON schema (or equivalent contract)
- Run multiple trials at fixed settings
- Validate output; reliability = successes / trials
- Record `json_mode_reliability`

#### 5) Determinism stability (best-effort)

Goal: estimate replay stability under identical inputs.

Method:

- Fix all params (seed if available)
- Run N identical requests
- Hash outputs and compute stability ratio
- Record `determinism_hash_stability`

#### 6) Reasoning trace leakage detection

Goal: detect visible “Thinking…” / chain-of-thought markers.

Method:

- Use a simple deterministic prompt
- If output contains known markers (configurable patterns), set `reasoning_trace_visible=true`

---

## Trace and Replay Integration

### Trace linking

Traces SHOULD record references to:

- `provider_caps_declared_ref` (artifact hash/path)
- `provider_caps_observed_ref` (artifact hash/path)
- `effective_caps_ref` (optional: derived artifact)

### Replay equivalence rule

A replay is **equivalent** only if:

- `model_ref` matches AND
- EffectiveCaps match (or are stricter)

If capability envelopes differ, replay MUST be marked **non-equivalent**, even if the model name is the same.

---

## Learning Export Bundle Integration

Learning export bundles SHOULD include:

- `provider_caps_declared.json`
- `provider_caps_observed.json` (if available)
- `provider_probe_report.json` (optional)

These are part of the execution substrate and support reproducible learning and review.

---

## CLI Sketch (Non-Normative)

Target command:

```bash
adl provider probe ollama --model Qwen3.5:9b \
  --endpoint http://localhost:11434 \
  --temperature 0.0 \
  --trials 3 \
  --out-dir .adl/artifacts/probes/2026-03-03__qwen3.5_9b
```

Outputs:

- `provider_caps_observed.json`
- `provider_probe_report.json` (optional)

---

## Implementation Notes (Non-Normative)

- The first implementation can be in a scripting language for speed of iteration, but ADL’s target implementation is Rust.
- Probing MUST be careful not to echo secrets into prompts; use synthetic random needles.
- Probing MUST record runtime versions and environment fingerprints, since these materially change limits.

---

## Open Questions

- How should ADL standardize token counting across providers (provider-reported tokens vs local tokenization)?
- Should probes define separate stability metrics per depth rather than a single scalar?
- Should “strict mode” be a global ADL setting or per-run policy?
- How should providers with built-in “JSON mode” be modeled vs prompt-based compliance?

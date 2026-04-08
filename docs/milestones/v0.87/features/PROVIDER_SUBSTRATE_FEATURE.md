

# PROVIDER SUBSTRATE FEATURE v1

## Metadata
- Owner: `adl`
- Status: `promoted`
- Target milestone: `v0.87`
- Work package: `WP-04`
- Purpose: Define the canonical provider substrate for ADL in `v0.87`, including declarative capability, deterministic invocation, and trace-integrated execution.

## Purpose

Define the **provider substrate** for ADL.

This document specifies:
- the provider abstraction contract
- the declarative capability model (v1)
- the transport/invocation boundary
- how provider execution integrates with trace
- determinism and portability requirements

This is the **canonical feature doc** for provider/transport in `v0.87`.

## Core Principle

> Providers in `v0.87` are declarative, deterministic execution endpoints with explicit capability surfaces.

They are NOT:
- dynamic capability discovery systems
- adaptive routing systems
- optimization engines

They are a **stable substrate boundary**.

## Why This Matters in `v0.87`

`v0.87` is a substrate milestone.

Provider/transport must be:
- stable
- inspectable
- trace-integrated

Without this:
- trace cannot reliably attribute execution
- memory (ObsMem) cannot ground observations
- review surfaces cannot assign responsibility

## Scope

### In scope
- provider abstraction contract
- declarative capability definition (static)
- deterministic invocation interface
- transport normalization (bounded)
- trace integration (`model_ref`, `provider_ref`)

### Out of scope
- dynamic capability inference
- provider arbitration or routing logic
- optimization strategies
- multi-provider orchestration policies
- adaptive or learning-based provider selection

## Provider Definition

A provider is a **named execution backend** capable of performing model inference.

Examples:
- OpenAI API
- Anthropic API
- local Ollama runtime
- CLI-based model invocation

In ADL, providers are treated uniformly via a **declarative contract**.

## Provider Identity

Each provider must define:
- `provider_id`
- `provider_type` (e.g., `http`, `local`, `cli`)
- `models` (set of supported model identifiers)

Each invocation must resolve to:
- `provider_ref`
- `model_ref`

These MUST be present in trace.

## Declarative Capability Model (v1)

Capabilities are declared statically.

They are NOT inferred at runtime.

### Minimal capability fields (v1)

Each model may declare:
- `max_context_tokens`
- `supports_streaming` (bool)
- `supports_tools` (bool)
- `supports_system_prompt` (bool)

Optional (bounded):
- `max_output_tokens`

These fields exist to support:
- validation
- planning
- trace interpretation

They are NOT used for dynamic optimization in `v0.87`.

## Transport Model

Transport defines **how a provider is invoked**, not what it can do.

### Supported transport types (v1)

- `http`
- `local`
- `cli`

Each provider must define:
- transport type
- endpoint or execution path
- required configuration (auth, headers, etc.)

### Normalization requirement

All transports must normalize into a **single invocation interface**.

This ensures:
- deterministic execution
- trace consistency
- portability across providers

## Profile Compatibility Layer

`v0.87` profile-based providers may carry bounded runtime config overrides where
the substrate still needs concrete execution details from the deployment surface.

This is especially important for HTTP-backed common-provider profiles:
- the profile should carry the bounded canonical provider family + default model
- runtime config may still supply concrete endpoint, auth, headers, timeout, and
  explicit `provider_model_id` compatibility values
- canonical `model_ref` remains the ADL-stable identity surface and must stay
  distinct from raw provider-native model identifiers

Compatibility overrides are acceptable when they are explicit and reviewable.
They are not a license to collapse back into provider-native canonical config.

## Invocation Contract

Provider invocation must be:
- explicit
- deterministic
- fully traceable

### Required inputs

- `provider_id`
- `model_id`
- prompt/messages (structured)
- invocation parameters (temperature, etc.)

### Required outputs

- response content
- usage metadata (if available)
- error information (if applicable)

## Trace Integration

Every provider invocation MUST emit trace events.

### Required trace fields

- `event_type = MODEL_INVOCATION`
- `provider_ref`
- `model_ref`
- `input_ref` (artifact or inline)
- `output_ref` (artifact or inline)
- `duration_ms`
- `status`

This ensures:
- full attribution of execution
- replay compatibility
- linkage to ObsMem

## Determinism Requirements

Provider execution is deterministic when:
- invocation inputs are fully specified
- provider + model are explicitly identified
- parameters are explicitly defined

Allowed variability:
- stochastic model outputs

Not allowed:
- implicit provider selection
- hidden parameter defaults
- missing provider/model identity in trace

## Portability Contract

The provider substrate must ensure:

- ADL workflows do not depend on a single provider
- provider differences are normalized at the boundary
- trace remains consistent across providers

This enables:
- swapping providers without breaking contracts
- reproducible execution surfaces

## Relationship to Other Substrates

### Trace
- provider invocations are primary trace events
- provider identity anchors execution provenance

### ObsMem
- model calls generate observation records (`obs.model_call`)
- provider/model identity must be preserved in memory

### Review Surface
- findings may reference provider behavior
- provider identity must be stable for review credibility

### Skills
- skills may invoke providers
- they must not bypass the provider abstraction

## Acceptance Criteria

The provider substrate is acceptable for `v0.87` when:

- a canonical provider abstraction exists
- capabilities are declarative and static
- invocation is deterministic and explicit
- all model calls emit proper trace events
- provider/model identity is preserved across trace, memory, and review
- transport differences are normalized behind the interface

## Open Questions

- what is the minimal configuration format for provider definitions?
- how are credentials injected in a deterministic but secure way?
- what is the initial set of supported providers in-repo?

## Non-Goals (v1)

- intelligent routing across providers
- cost/performance optimization layers
- dynamic capability learning
- provider reputation systems

## Next Steps

Derive or align the following from this doc:
- provider schema/config doc
- provider implementation issues
- trace instrumentation validation for provider calls
- demo coverage using at least two providers

The provider substrate in `v0.87` establishes a clean, deterministic boundary between ADL and model execution backends.

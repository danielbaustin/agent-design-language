# Provider Capability and Transport Architecture

## Metadata
- Feature Name: `Provider Capability and Transport Architecture`
- Milestone Target: `v0.92`
- Status: `planned`
- Owner: `ADL core / Layer 8`
- Doc Role: `primary`
- Supporting Docs:
  - `PROVIDER_AND_TRANSPORT_MODEL.md`
  - `ADL_PROVIDER_CAPABILITIES.md`
- Feature Types:
  - `architecture`
  - `runtime`
  - `artifact`
- Proof Modes:
  - `schema`
  - `review`
  - `tests`

## Template Rules

- Every section is completed.
- No sections are `N/A` for this feature because it defines both runtime structure and artifact shape.
- Feature Types affect emphasis, but do not remove the requirement to define each section concretely.

## Purpose

Define a stable provider model for ADL that cleanly separates vendor identity,
transport, stable ADL model references, and provider-native model strings.

This feature exists to replace the current compressed provider surface, where:

- transport and provider identity are conflated
- exact model names are hardcoded as unstable strings
- profiles look like providers
- capability work has no clean structural home

The goal is to make OpenAI, Anthropic/Claude, Gemini, and Ollama first-class
provider families while preserving deterministic configuration, replay, and
future capability-aware routing.

## Context

- Related milestone: `v0.92`
- Related issues:
  - `#341`
- Dependencies:
  - `PROVIDER_AND_TRANSPORT_MODEL.md`
  - `ADL_PROVIDER_CAPABILITIES.md`

This feature sits at the intersection of Layer 8 provider integration,
capability contracts, and future agent-routing policy. It does not replace the
provider-capability work; it gives that work a stable runtime and configuration
home.

## Coverage / Ownership

- Covered surfaces:
  - provider/vendor identity
  - transport taxonomy
  - stable `model_ref` addressing
  - provider-native model identifier isolation
  - provider catalogs and default selection
  - Gödel/AEE-facing provider selection shape
- Related / supporting docs:
  - `PROVIDER_AND_TRANSPORT_MODEL.md`
  - `ADL_PROVIDER_CAPABILITIES.md`

## Overview

ADL should model provider execution with four explicit layers:

1. transport
2. provider/vendor
3. stable ADL `model_ref`
4. provider-native `provider_model_id`

Key capabilities:
- define first-class provider adapters for OpenAI, Anthropic, Gemini, Ollama, compatible HTTP, and mock
- isolate unstable provider model names behind stable ADL-facing references
- provide a configuration surface that lets agents target vendors and models without brittle string coupling
- integrate cleanly with declared, observed, and effective capability envelopes

Current runtime clarification:
- `local_ollama` is the explicit local CLI-backed Ollama surface
- `ollama` may resolve to either local CLI compatibility mode or first-class HTTP transport when `base_url` or `config.endpoint` is configured
- remote Ollama intentionally permits explicit `http://` endpoints because native Ollama deployments commonly run on loopback or LAN-local plaintext HTTP

## Design

### Core Concepts

- `transport`
  - how ADL communicates with a backend
- `vendor`
  - the provider/runtime family whose semantics ADL is integrating
- `model_ref`
  - the stable ADL identifier used by agents, policies, and configuration
- `provider_model_id`
  - the provider-native model string used at the integration boundary
- `model_catalog`
  - the mapping from stable ADL refs to provider-native identifiers and baseline capabilities
- `provider_policy`
  - a higher-level selection surface that may later resolve to provider/model choices

### Architecture

The architecture distinguishes configuration, runtime execution, and capability
evidence:

- Inputs (explicit sources / triggers):
  - ADL provider configuration
  - provider catalogs
  - runtime capability probe artifacts
  - agent/provider policy configuration
- Outputs (artifacts / side effects):
  - resolved provider selections
  - normalized runtime provider bindings
  - declared and effective capability envelopes
- Interfaces (APIs, CLI, files, schemas):
  - ADL config/provider schema
  - provider catalog schema
  - capability artifact schema
  - runtime provider adapter surface
- Invariants (must always hold):
  - transport and vendor remain distinct concepts
  - ADL-facing configuration uses stable `model_ref` values
  - provider-native model strings remain isolated in catalogs or adapters
  - capability-aware selection remains explicit and inspectable

### Data / Artifacts

- provider definition artifact
  - describes vendor, transport, endpoint/auth profiles, and default `model_ref`
- model catalog artifact
  - maps `model_ref` to provider-native ids, family, tier, and declared capabilities
- capability artifacts
  - declared capabilities
  - observed capabilities
  - effective capability envelope

## Execution Flow

1. ADL loads a provider definition with explicit `vendor`, `transport`, and `model_catalog`.
2. ADL resolves a stable `model_ref` to the corresponding provider-native `provider_model_id`.
3. The selected transport adapter executes the request against the chosen vendor/runtime.
4. Capability artifacts and policy constraints determine what ADL is allowed to trust and select.

## Determinism and Constraints

- Determinism guarantees:
  - provider selection must be explicit and config-driven
  - `model_ref` to `provider_model_id` resolution must be stable for a given catalog version
  - capability-aware routing must remain policy-bound and replayable
- Constraints:
  - no hidden runtime vendor switching
  - no hard dependency on unstable provider model strings outside catalogs/adapters
  - compatibility adapters must not silently claim first-party semantics they cannot verify

## Integration Points

| System / Surface | Integration Type | Description |
| --- | --- | --- |
| Providers | read/write/trigger | Runtime adapter selection and request execution depend on the vendor/transport split. |
| Gödel | read | Gödel should target stable `model_ref` values or provider policies, not vendor-native strings. |
| AEE | read | AEE should reason over capability envelopes and provider policies rather than raw provider names. |
| Trace | observe | Provider resolution and effective capability envelopes should be traceable and replayable. |
| Identity | read | Stable model references make long-lived identity and continuity bindings less brittle. |
| Governance | read/write | Governance policy should constrain allowed vendors, transports, and capability classes. |

## Validation

Declared Proof Modes for this feature:

- `schema`
  - provider definition and catalog shapes must be representable as stable ADL artifacts
- `review`
  - the design must clearly separate vendor, transport, `model_ref`, and provider-native ids
- `tests`
  - provider resolution and catalog mapping logic should be unit-testable and deterministic

### Demo
- Demo script(s): none required at this stage
- Expected behavior: this feature is validated primarily through schema shape, review clarity, and deterministic resolution tests rather than a standalone demo

### Deterministic / Replay
- Replay requirements:
  - provider selection must be reconstructable from config, catalog, and capability artifacts
  - a trace should be able to record the resolved vendor, transport, `model_ref`, and `provider_model_id`
- Determinism guarantees:
  - the same config and catalog version must resolve to the same provider binding

### Schema / Artifact Validation
- Schemas involved:
  - provider definition schema
  - model catalog schema
  - provider capability artifact schema
- Artifact checks:
  - `model_ref` entries are unique within a catalog
  - provider definitions reference valid vendors, transports, and catalog entries
  - provider-native ids are isolated to catalog or adapter boundaries

### Tests
- Test surfaces:
  - deterministic provider resolution from config + catalog
  - vendor/transport separation enforcement
  - invalid catalog reference rejection
  - compatibility-adapter behavior staying distinct from first-party adapters

### Review / Proof Surface
- Review method (manual/automated):
  - manual design review plus schema/test review
- Evidence location:
  - this feature doc
  - `PROVIDER_AND_TRANSPORT_MODEL.md`
  - `ADL_PROVIDER_CAPABILITIES.md`

## Acceptance Criteria

- Functional correctness:
  - ADL has a documented provider model where vendor, transport, stable `model_ref`, and provider-native model identifiers are separate and well-defined
  - the model supports first-class OpenAI, Anthropic, Gemini, Ollama, compatible HTTP, and mock adapter families
- Determinism / replay correctness:
  - provider selection can be reconstructed from config, catalogs, and capability artifacts without relying on ad hoc string interpretation
- Validation completeness:
  - the feature defines concrete schema expectations, review criteria, and deterministic test surfaces for implementation

## Risks

- Primary risks:
  - over-designing the taxonomy before adapter implementation catches up
  - preserving backward compatibility with current profile-based config
  - letting compatibility adapters blur into first-party adapters again
- Mitigations:
  - keep the first-class adapter set small and explicit
  - preserve current profiles as compatibility indirection during migration
  - isolate provider-native ids in catalogs and test the resolution path directly

## Future Work

- Follow-ups / extensions:
  - provider policy resolution for agent classes such as Gödel and AEE
  - richer capability catalogs and runtime probing integration
- Known gaps / deferrals:
  - this document does not define the full final catalog schema in JSON detail
  - this document does not yet specify dynamic fallback policy semantics

## Notes

This is the implementation-facing owner document for the provider redesign.
`PROVIDER_AND_TRANSPORT_MODEL.md` remains the broader rationale and design note.

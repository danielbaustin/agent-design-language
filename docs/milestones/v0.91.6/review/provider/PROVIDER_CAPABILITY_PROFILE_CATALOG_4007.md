# Provider And Capability Profile Catalog Proof Note for #4007

## Scope

This note records the bounded proof surface for `#4007`. It defines the first
WP-05 provider/capability profile catalog boundary and does not claim role
suitability completion, Gemma/OpenRouter reliability proof, failure-mode
closure, or final provider/model closeout.

## Source evidence

- [PROVIDER_MODEL_RELIABILITY_v0.91.6.md](../../features/PROVIDER_MODEL_RELIABILITY_v0.91.6.md)
- [ADR 0004 provider profiles](../../../adr/0004-provider-profiles.md)
- `adl/src/provider/profiles.rs`

## Catalog split established here

`#4007` establishes one explicit split:

1. provider profiles
2. capability profiles

This split is the guardrail that prevents later role-provider work from
confusing:

- vendor/service infrastructure
- model capability
- role authority
- identity/citizen/institution state

## Provider profile boundary

Provider profiles are infrastructure/service descriptors.

They own:

- provider family
- deterministic profile id
- service substrate
- default model binding
- endpoint/locality class
- qualitative economics, latency, cost, tool-support, and lane class
- operator-managed auth surface expectations

They do not own:

- role authority
- actor identity
- citizen records
- institution records

## Capability profile boundary

Capability profiles are provider-independent behavioral descriptors.

They own:

- interaction modes
- structured-output posture
- tool-orchestration posture
- context class
- reasoning posture
- determinism posture
- safety-limit notes

They do not own:

- vendor identity
- endpoint configuration
- credential location
- civil or institutional identity

## Current mapping evidence

The current tracked provider-profile sources already prove deterministic
provider families for:

- OpenAI/ChatGPT-facing HTTP profiles
- Anthropic/Claude-facing HTTP profiles
- Google/Gemini-facing HTTP profiles
- Ollama local profiles
- OpenRouter aggregator profile
- DeepSeek hosted profile
- mock/test profile

This issue uses that live registry evidence to define the first catalog
boundary rather than inventing a new untracked profile source.

## Input drift captured for remediation

The authored issue input references:

- `.adl/docs/TBD/ADL_PROFILES_PROVIDERS_V2.MD`

That path is not present in the current tracked repository state.

The live tracked inputs used instead are:

- `docs/adr/0004-provider-profiles.md`
- `adl/src/provider/profiles.rs`

This is not a blocker for `#4007`, but it is authoring/input drift that should
be corrected in follow-on remediation rather than copied forward silently.

## What this proof note does not claim

This note does not claim:

- final role-provider suitability decisions
- Gemma or OpenRouter reliability proof
- provider failure-mode closure
- provider/model closeout matrix completion
- identity/citizen/institution design completion

Those remain owned by `#4008`, `#4009`, `#4010`, `#4012`, and later work.

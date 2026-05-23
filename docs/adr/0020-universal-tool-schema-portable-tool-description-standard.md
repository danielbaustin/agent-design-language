# ADR 0020: Universal Tool Schema As Portable Tool Description Standard

- Status: Accepted
- Date: 2026-05-23
- Accepted in: v0.91.3
- Candidate source: docs/architecture/adr/0020-universal-tool-schema-portable-tool-description-standard.md
- Target milestone: v0.91.2
- Related issues: #3076, #3104, #3107, #3108, #3124
- Related ADRs: supersedes the active UTS interpretation in ADR 0015

## Context

ADR 0015 recorded the v0.90.5 governed-tools stack as one coherent umbrella:
model proposals, Universal Tool Schema (`UTS`), ACC, registry binding,
compiler behavior, policy mediation, governed execution, and trace evidence.
That was the right shape while ADL was making the whole tool-governance stack
legible.

The v0.91.2 work separates two concerns that now have different audiences:

- UTS is becoming a portable, provider-neutral, potentially standalone
  tool-description standard.
- ACC remains ADL's runtime authority and governance contract.

UTS v1.1 evidence from the local-model and hosted-frontier benchmark issues is
strong enough that ADL should record its active architectural stance clearly,
without making ADL the only home of the standard.

## Decision

ADL adopts UTS as the portable, provider-neutral tool-description standard for
model-facing tool surfaces.

UTS describes a tool. It does not authorize tool execution.

ADL may project UTS definitions into provider-native tool/function-call
surfaces, including OpenAI-style tool definitions, and may normalize
provider-native outputs back into UTS proposal records. That compatibility is a
schema and adapter boundary, not a trust boundary.

The normative UTS package should move toward the standalone
`universal-tool-schema` repository. ADL remains an origin, adopter, benchmark
user, and governance-companion example, not the exclusive owner of UTS as a
standard.

## Requirements

- UTS must remain provider-neutral and transport-neutral.
- UTS must preserve compatibility with provider-native tool/function-call
  shapes where that can be done without weakening authority boundaries.
- UTS records should include enough metadata for runtimes to reason about
  side effects, determinism, idempotence, replay posture, observability,
  security posture, and error surfaces.
- A valid UTS record is only a valid description or proposal input.
- Runtime permission, actor identity, delegation, visibility, policy, Freedom
  Gate decisions, redaction, and trace authority remain outside UTS.

## Consequences

### Positive

- Developers can reason about UTS as a drop-in or adapter-backed replacement
  for naked tool schemas without inheriting ADL runtime machinery.
- ADL can test and compare provider-native calls against a portable schema.
- The UTS standardization path can move into a separate repository without
  losing ADL provenance or benchmark evidence.

### Negative

- ADL docs must be careful not to call UTS an execution authority.
- Provider-native compatibility needs explicit projection and normalization
  language.
- ACC and the UTS-to-ACC compiler boundary must stay clear enough that future
  work cannot smuggle permission into schema validity.

## Alternatives Considered

### Keep UTS bundled inside the governed-tools umbrella

This preserves ADR 0015 as-is, but it obscures the public-standard path and
makes UTS look more ADL-specific than it now is.

### Treat UTS as the authority boundary

This is simpler externally, but false to ADL. Tool description is not
permission, and valid JSON is not a governed action.

## Validation Notes

This ADR was reviewed against:

- the UTS v1.1 local-model benchmark evidence from #3076
- the UTS v1.1 hosted-frontier benchmark evidence from #3104
- the standalone UTS repository plan from #3107
- the canonical UTS conformance panel and fixture work from #3108

## Non-Claims

- This ADR does not accept or publish UTS as an external standard by itself.
- This ADR does not move UTS artifacts into the standalone repo.
- This ADR does not grant execution authority from UTS validity.

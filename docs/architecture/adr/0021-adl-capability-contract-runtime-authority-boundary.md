# ADR 0021 Candidate: ADL Capability Contract As Governed Runtime Authority Boundary

- Status: Candidate
- Target milestone: v0.91.2
- Related issues: #3076, #3104, #3124
- Related ADRs: supersedes the ACC and governed-execution portions of ADR 0015 after acceptance

## Context

ADR 0015 made ADL's governed-tools stack explicit. v0.91.2 sharpens that stack
by separating portable description from runtime authority.

UTS describes tool shape. ACC decides whether a proposed capability exercise
can become an authorized ADL action.

The distinction matters across v0.91.2:

- benchmark work compares proposal behavior without treating proposals as
  execution
- modernization and other write-capable integrations need a governed execution
  boundary
- speculative decoding must not turn generated candidate text into hidden
  side-effect authority

## Decision

ADL treats the ADL Capability Contract (`ACC`) as the runtime-facing authority
contract for capability exercise.

A model proposal, provider-native tool call, or valid UTS record becomes
actionable only after ACC construction, policy mediation, Freedom Gate review,
and required trace/redaction constraints.

ACC is ADL-native. Unlike UTS, it is not intended to be a provider-neutral
schema standard. It is the governed authority boundary that binds actor,
grantor, delegation, visibility, policy, replay, failure posture, and evidence.

## Requirements

- ACC records must preserve actor identity, grantor attribution, delegation,
  standing, authority scope, visibility, privacy, replay posture, failure
  posture, and trace requirements.
- Provider-native calls and UTS records are untrusted inputs until normalized
  into the governed proposal-to-ACC path.
- Missing authority, unsafe ambiguity, destructive intent, exfiltration risk,
  adapter mismatch, under-attributed proposals, or policy failure must fail
  closed.
- Policy and Freedom Gate mediation must happen before execution.
- Trace/redaction evidence must make proposal, normalization, ACC construction,
  decision, execution or denial, and redaction posture reviewable.

## Consequences

### Positive

- Keeps ADL's authority model explicit as UTS becomes more portable.
- Gives write-capable integrations and speculative-decoding work a common
  authority boundary.
- Prevents provider-native tool calling from bypassing ADL governance.

### Negative

- ACC changes now carry significant architecture weight.
- Tool adapters must preserve the description-versus-authority split.
- Docs must avoid treating ACC as a generic public schema when it is really an
  ADL governance contract.

## Alternatives Considered

### Treat provider-native tool calls as executable intent

This is incompatible with ADL's governed execution model. Provider output is
proposal evidence, not permission.

### Fold ACC into UTS

This would make UTS less portable and blur schema validity with governed
authority.

## Validation Notes

This candidate should be reviewed against the v0.91.2 UTS/ACC benchmark
evidence, provider-native comparison evidence, and code-modernization authority
notes.

## Non-Claims

- This ADR does not claim every ACC field is final.
- This ADR does not replace Freedom Gate or operator confirmation.
- This ADR does not authorize real destructive tool execution.

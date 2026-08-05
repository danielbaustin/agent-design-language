# Structured Task Prompt

Template: 1.0.0

Issue: 5795

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver real local model-backed Shepherd dialogue through governed Runtime v3 and Observatory surfaces.

## Deliverables

- Runtime provider adapter and governed command/response path for configured local MLX/Gemma
- Observatory interaction, truthful status, real smoke, deterministic regression, and negative-case evidence

## Acceptance

1. A configured local MLX/Gemma model produces a real Shepherd response through Runtime v3
2. The Observatory sends the bounded command and renders response/result evidence
3. Status distinguishes unavailable, deterministic test double, and real local-model execution
4. No-model, timeout, malformed command, and unsigned or unauthorized mutation cases fail truthfully
5. Deterministic adapter regressions and a real local model smoke both pass
6. Freedom Gate, CAV, constitutional policy, and advisory-only authority remain enforced
7. The global default model is unchanged and no AWS or cloud path is used
8. One exact-revision review has no unresolved actionable findings

## Dependencies

- WP-03
- issue-5800
- WP-14 contract stability before final Observatory integration

## Inputs

- adl-runtime/src/runtime_api.rs
- adl-runtime/src/resident_agent.rs
- adl-runtime-kernel/src/assembly.rs
- demos/html-observatory

## Non Goals

- Full v0.95 Shepherd or Gemma program
- Training or evaluator productization
- Global default model switch
- AWS or cloud provider execution
- Success credit from a deterministic fake alone

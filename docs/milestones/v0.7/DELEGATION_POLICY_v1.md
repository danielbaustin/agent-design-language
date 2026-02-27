# Delegation Policy v1

## Scope

Delegation Policy v1 is the minimal deterministic allow/deny surface for delegated runtime actions in v0.7.

It exists to make decisions explicit, auditable, and replayable without introducing a full policy DSL or governance engine.

## Supported action kinds

Policy rules may match these action kinds:

- `tool_invoke`
- `provider_call`
- `remote_exec`
- `filesystem_read`
- `filesystem_write`

In the current runtime, audit/enforcement is emitted only for action kinds exercised by implemented execution paths.

## Fields

`run.delegation_policy` supports:

- `default_allow: bool`
- `rules: []`

Each rule supports:

- `id: string`
- `action: enum`
- `target_id: string | null`
- `effect: allow | deny`
- `require_approval: bool`

## Evaluation semantics

Delegation Policy v1 uses explicit first-match semantics:

1. Rules are evaluated in declared order.
2. The first matching rule wins.
3. If no rule matches, `default_allow` decides the outcome.

Decision outcomes are:

- `allowed`
- `denied`
- `needs_approval`

## Audit / trace behavior

Policy evaluation emits a minimal `DelegationPolicyEvaluated` event through the existing trace infrastructure. The event records:

- `action_kind`
- `target_id`
- `decision`
- `rule_id` when a rule matched

No raw secrets, prompts, tool arguments, or absolute host paths are logged.

## Non-goals

Out of scope for v1:

- regex or glob matching
- conditional policy language
- multi-party approvals
- learning-driven policy mutation
- bypassing security envelope or sandbox checks

Security and sandbox invariants remain enforced independently by the remote envelope and sandbox layers.

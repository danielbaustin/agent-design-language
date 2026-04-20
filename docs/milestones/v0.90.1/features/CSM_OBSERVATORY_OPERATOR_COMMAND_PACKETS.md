# CSM Observatory Operator Command Packets

## Purpose

Define the safe packet contract for moving CSM Observatory from read-only
visibility toward governed operator interaction.

Schema name: adl.csm_operator_command_packet.v1

The Observatory UI must never mutate Runtime v2 state directly. It may only
submit bounded command packets to the Runtime v2 kernel/control plane. The
kernel validates identity, policy, invariants, traceability, and command
availability before any state transition is allowed.

## Design Goal

Every operator action must answer four questions before it can affect the CSM:

- Who requested the action?
- What exact target and intent were declared?
- Which kernel service is responsible for accepting, deferring, or refusing it?
- Which operator event proves the decision and any later effect?

The packet is intentionally more formal than a UI button payload. It is a small
constitutional instrument for a governed world.

## Packet Contract

Required top-level fields:

- schema
- command_id
- command_kind
- requested_by
- requested_at
- target
- intent
- availability
- safety
- confirmation
- kernel_handoff
- event_logging
- evidence_refs
- claim_boundary

### Command Identity

The command_id must be stable for retries and must not encode a host path,
secret, prompt, or private operator note. The recommended shape is:

```text
csm-cmd-<manifold-or-citizen>-<action>-<sequence>
```

The requested_by field records an accountable operator or service principal. It
must not contain personal secrets, local usernames, private hostnames, or raw
session transcript text.

### Command Kinds

Allowed command_kind values for the first design pass:

- inspect_manifold
- inspect_citizen
- inspect_episode
- open_freedom_gate_decision
- annotate_trace
- ask_shepherd
- pause_citizen
- resume_citizen
- request_snapshot
- request_quiesce
- request_wake
- request_recovery_review

Read-only commands may be available in the console before live mutation exists.
State-changing commands remain disabled until Runtime v2 kernel handlers and
operator-event append guarantees exist.

### Target

The target object must include:

- target_kind
- target_id
- manifold_id
- evidence_ref

Allowed target_kind values:

- manifold
- citizen
- episode
- trace
- invariant
- freedom_gate
- shepherd
- kernel_service
- snapshot

All evidence references must be repository-relative artifact references. Public
packets must not contain absolute host paths, local endpoints, secrets, raw
prompts, or tool arguments.

### Availability

The availability object records whether the Observatory may surface the action:

```json
{
  "state": "disabled",
  "reason": "Runtime v2 has no live command handler for this action yet.",
  "enabled_in": "future Runtime v2 kernel/control-plane work"
}
```

Allowed availability states:

- available
- disabled
- deferred
- requires_confirmation
- submitted
- refused

The UI may render disabled and deferred commands, but it must explain why the
operator cannot execute them.

### Safety Classification

The safety object must include:

- classification
- policy_checks
- side_effect_scope
- operator_risk
- citizen_risk

Allowed classification values:

- read_only
- advisory
- reversible_state_change
- guarded_mutation
- destructive_or_irreversible

Rules:

- read_only commands must not change Runtime v2 state.
- advisory commands may call a shepherd model or deterministic summarizer but
  must not mutate state.
- reversible_state_change commands require an operator event and kernel policy
  checks.
- guarded_mutation commands require confirmation and invariant validation.
- destructive_or_irreversible commands remain disabled unless a later milestone
  explicitly accepts them.

### Confirmation

The confirmation object must include:

- required
- prompt
- required_phrase
- expires_after_seconds

Dangerous or unavailable commands must either require explicit confirmation or
remain disabled. A disabled command may set required to false only because it
cannot be submitted yet.

### Kernel Handoff

The kernel_handoff object must include:

- service
- operation
- policy_checks
- trace_append_required
- invariant_check_required
- ui_direct_mutation_allowed

The ui_direct_mutation_allowed field must always be false for CSM Observatory
commands.

The handoff sequence is:

1. receive command packet
2. assign temporal anchor
3. validate operator identity and target identity
4. validate policy and invariants
5. append operator event
6. route to responsible kernel service
7. append outcome event

No service may bypass trace append. No UI path may write the target Runtime v2
state directly.

### Event Logging

Every command requires an operator event, even read-only inspection. The
event_logging object must include:

- event_schema
- event_ref
- required_fields
- redact_prompt_or_tool_args
- append_before_effect

Required event fields:

- event_schema
- event_id
- command_id
- command_kind
- requested_by
- requested_at
- target
- availability_state
- safety_classification
- decision
- decision_reason
- kernel_service
- evidence_refs

Allowed decision values:

- recorded
- accepted
- deferred
- refused
- blocked

## Initial Action Table

| Command | Safety | Initial state | Kernel owner | Event required |
| --- | --- | --- | --- | --- |
| inspect_manifold | read_only | available | operator_control_interface | yes |
| inspect_citizen | read_only | available | operator_control_interface | yes |
| inspect_episode | read_only | available | operator_control_interface | yes |
| open_freedom_gate_decision | read_only | available | freedom_gate | yes |
| annotate_trace | advisory | deferred | trace_system | yes |
| ask_shepherd | advisory | deferred | shepherd_interface | yes |
| pause_citizen | guarded_mutation | disabled | scheduler | yes |
| resume_citizen | guarded_mutation | disabled | scheduler | yes |
| request_snapshot | guarded_mutation | disabled | snapshot_manager | yes |
| request_quiesce | guarded_mutation | disabled | scheduler | yes |
| request_wake | guarded_mutation | disabled | scheduler | yes |
| request_recovery_review | advisory | deferred | invariant_enforcement | yes |

## Disabled Action Explanation Model

Disabled actions must render an explanation with:

- unavailable action
- current blocker
- required kernel capability
- required evidence before enablement
- safe alternative action

Example:

```json
{
  "action": "request_snapshot",
  "current_blocker": "Snapshot command handling is not live in the CSM Observatory.",
  "required_kernel_capability": "snapshot_manager.request_snapshot",
  "required_evidence_before_enablement": [
    "operator event append before effect",
    "invariant checks pass before snapshot",
    "snapshot artifact path is repository-relative"
  ],
  "safe_alternative_action": "inspect_manifold"
}
```

## Shepherd Boundary

The ask_shepherd command is advisory. It may produce a recommendation, concern,
or explanation, but it must not silently mutate state. Shepherd output must cite
visibility packet evidence or trace artifacts and must report uncertainty.

ANRM or a dedicated shepherd model may later provide this role, including a
trainable Gemma-backed local shepherd, but the command packet contract does not
depend on training being complete.

## Deferred Runtime Work

This issue does not implement live pause, resume, snapshot, quiesce, wake,
recovery, or shepherd execution. Those require Runtime v2 handlers that can
prove:

- operator-event append before effect
- identity and target validation
- invariant validation before mutation
- replay-safe outcome events
- no private path, endpoint, prompt, tool-argument, or secret leakage

## Proof Surfaces

- Schema: adl/schemas/csm_operator_command_packet.v1.schema.json
- Example command packet set: demos/fixtures/csm_observatory/proto-csm-01-operator-command-packets.json
- Example operator event: demos/fixtures/csm_observatory/proto-csm-01-operator-event.json
- Validator: adl/tools/validate_csm_operator_command_packets.py
- Focused test: adl/tools/test_csm_operator_command_packets.sh

## Non-Goals

- Do not implement live Runtime v2 mutations.
- Do not make the static console submit commands.
- Do not bypass Freedom Gate, kernel policy, identity validation, invariant
  checks, or trace append.
- Do not turn citizen or shepherd interaction into uncontrolled chat.

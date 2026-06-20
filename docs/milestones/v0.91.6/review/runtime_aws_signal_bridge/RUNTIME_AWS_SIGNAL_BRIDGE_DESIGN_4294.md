# Runtime AWS Signal Bridge Design For `#4294`

## Status

- Issue: `#4294`
- Milestone: `v0.91.6`
- Status: `design_contract`
- Scope: bounded design packet for the first runtime AWS signal bridge
- Proof mode: design review plus sample event packets; no live AWS mutation

## Purpose

Define one shared AWS-facing signal contract that both:

- `#4295` runtime heartbeat publication; and
- `#4296` ACIP-to-SNS publication

can implement without collapsing operational heartbeat telemetry into ACIP
protocol delivery semantics.

The design must stay boring, fail-closed, tail-friendly, and explicit about
what remains approval-gated or deferred.

## Inputs Used

- `docs/milestones/v0.91.6/RUNTIME_FIRE_UP_PLAN_v0.91.6.md`
- `docs/milestones/v0.91.6/RUNTIME_INTEGRATION_SOAK_SPRINT_v0.91.6.md`
- `docs/milestones/v0.91.6/RUNTIME_OBSERVABILITY_COMPLETION_SCHEDULE_v0.91.6.md`
- `docs/milestones/v0.91.6/review/logging_observability/HEARTBEAT_TIMEOUT_PROGRESS_PROOF_3998.md`
- `docs/milestones/v0.91.6/review/logging_observability/LOGGING_VALIDATION_REDACTION_PROOF_4000.md`
- `docs/milestones/v0.91.6/review/AGENT_LOGIC_AWS_ACCOUNT_DECISION_RECORD_3902.md`
- `docs/milestones/v0.91.6/review/issue_resource_telemetry/ISSUE_RESOURCE_TELEMETRY_V1_AND_S3_ARCHIVE_PLAN_4280.md`
- `docs/milestones/v0.91.6/review/security/LOCAL_POLIS_SSM_OPERATIONS_BRIDGE_4109.md`
- `docs/milestones/v0.91.6/features/ACIP_A2A_PROVIDER_COMMUNICATIONS_v0.91.6.md`
- `adl/src/long_lived_agent.rs`
- `adl/src/cli/observability.rs`
- `adl/src/agent_comms.rs`
- `adl/src/cli/run_artifacts/runtime/trace_envelope.rs`

## Design Decision Summary

The first runtime AWS signal bridge is an operations bridge, not a new
authority layer.

It has two explicitly separate lanes:

1. `heartbeat`
   - operational liveness/progress publication
   - first AWS target: CloudWatch Logs
2. `acip_projection`
   - bounded ACIP event projection for external subscribers
   - first AWS target: SNS

These lanes share one envelope and redaction policy, but they do not share
meaning:

- heartbeat is not an ACIP message;
- ACIP SNS projection is not the runtime's liveness authority; and
- AWS does not become the canonical source of runtime, ACIP, identity, private
  state, or governance truth.

## Non-Goals

- no live AWS resource creation
- no account bootstrap or IAM rollout beyond documented assumptions
- no publication of raw private-state payloads
- no promotion of SNS into canonical ACIP authority
- no claim that EventBridge, S3 archive, or OTEL export are settled by this
  design
- no claim that live AWS proof is complete in `v0.91.6`

## Shared Envelope Contract

The first shared envelope schema is:

```text
adl.runtime.aws_signal.v1
```

Required top-level fields:

- `schema_version`
- `signal_kind`
- `runtime_id`
- `agent_id`
- `cycle_id`
- `heartbeat_seq`
- `status`
- `timestamp`
- `capabilities`
- `failure_class`
- `correlation_id`
- `projection_level`
- `transport`
- `payload`

### Field Rules

| Field | Rule |
| --- | --- |
| `schema_version` | Exact value `adl.runtime.aws_signal.v1`. |
| `signal_kind` | Exact first-lane values: `heartbeat` or `acip_projection`. |
| `runtime_id` | Stable runtime instance id or bounded rehearsal id. |
| `agent_id` | Stable agent/workflow identity for the emitting runtime surface. |
| `cycle_id` | Current runtime cycle id when one exists; otherwise explicit string `not_applicable`. |
| `heartbeat_seq` | Monotonic sequence number for heartbeat events; `null` for non-heartbeat projection events. |
| `status` | Bounded status only, such as `started`, `heartbeat`, `completed`, `failed`, `degraded`, `publish_blocked`, or `publish_skipped`. |
| `timestamp` | RFC3339 UTC timestamp. |
| `capabilities` | Bounded list of capability labels or route classes; no provider credentials, prompt contents, or private-state blobs. |
| `failure_class` | Explicit failure classification or `null`; do not hide publish failures behind generic success. |
| `correlation_id` | Stable correlation link to runtime/ACIP traces when available. |
| `projection_level` | One of the allowed projection levels below. |
| `transport` | Transport descriptor with mode and AWS target kind, not private identifiers. |
| `payload` | Lane-specific bounded payload object whose contents are constrained by `signal_kind` and `projection_level`. |

### Projection Levels

The first approved projection levels are:

- `operations_safe`
  - heartbeat-safe liveness/progress data only
- `delivery_metadata`
  - ACIP route/delivery metadata only
- `content_summary`
  - explicitly redacted summary text or approved content digest only

Forbidden in all first-lane projection levels:

- raw private state
- raw memory payloads
- provider request/response bodies
- credentials
- account ids
- private ARNs or private endpoints
- unredacted ACIP message bodies unless a later tracked design explicitly
  approves them

## Lane-Specific Semantics

### Heartbeat Lane

Heartbeat is an operations/liveness signal.

Required payload fields for `signal_kind=heartbeat`:

- `state`
- `elapsed_ms`
- `next_cycle_hint`
- `stop_requested`
- `lease_state`

Allowed examples:

- runtime started
- cycle running
- sleep until next heartbeat
- degraded because external signal publication failed
- runtime completed or stopped

Not allowed:

- ACIP message content
- provider prompt/response bodies
- private-state snapshots

### ACIP Projection Lane

ACIP SNS publication is a bounded delivery/projection bridge.

Required payload fields for `signal_kind=acip_projection`:

- `message_kind`
- `route_class`
- `sender_class`
- `recipient_class`
- `delivery_outcome`
- `message_ref`

Optional bounded fields:

- `summary`
- `trace_ref`
- `content_sha256`

Not allowed:

- raw ACIP conversation content by default
- provider-native invocation payloads
- private-state envelopes
- identity/governance authority claims that bypass ACIP policy

Default route posture for the first SNS bridge:

- local runtime semantics remain the authority
- SNS projection is approval-gated external delivery
- cross-boundary delivery should be treated as `deferred` until the subscriber
  class and target are explicitly approved
- mock proof may demonstrate the event shape without claiming that cross-boundary
  delivery authority is already open

## AWS Target Ownership

### First implementation targets

| Surface | First AWS target | Why |
| --- | --- | --- |
| Heartbeat | CloudWatch Logs | Append-only, tail-friendly, operationally boring, and aligned with liveness/progress consumption. |
| ACIP projection | SNS | Fan-out delivery surface fits bounded external subscriber notification better than a log sink. |

### Explicitly deferred

| Surface | Status | Why deferred |
| --- | --- | --- |
| EventBridge | deferred | Adds routing/event-bus semantics beyond the first bounded bridge. |
| S3 archive | deferred | Durable raw archive belongs with later evidence/archive work rather than first publication. |
| OTEL exporter | deferred | Current observability contract keeps OTEL optional and not yet the first runtime AWS bridge. |

## Mode Contract

All first-lane publishers must support:

- `disabled`
- `mock`
- `live`

### Disabled

- no AWS client construction
- no credential requirement
- emits explicit local observability showing publication was skipped
- must not silently masquerade as successful external delivery

### Mock

- no live AWS mutation
- no credential requirement
- writes bounded local JSON event shapes or uses a mock publisher trait
- primary proof mode for `v0.91.6`

### Live

- explicit operator approval required
- explicit environment configuration required
- fail closed if required configuration is missing
- must not invent defaults for credentials, region, topic, or log group

## Configuration Contract

First-lane configuration should stay environment-driven and private-value safe.

Allowed configuration names:

- `ADL_AWS_SIGNAL_MODE`
- `ADL_AWS_REGION`
- `ADL_AWS_HEARTBEAT_TARGET`
- `ADL_AWS_HEARTBEAT_LOG_GROUP`
- `ADL_AWS_HEARTBEAT_LOG_STREAM`
- `ADL_AWS_SNS_TOPIC_ARN`
- `ADL_AWS_SIGNAL_APPROVED`

Rules:

- values are private operator/runtime configuration and must not be committed
- logs may name env-var keys, but not their sensitive values
- live mode must require `ADL_AWS_SIGNAL_APPROVED=1`
- missing live-mode config is a fail-closed publication error

## IAM And Security Assumptions

This design assumes later operator-approved IAM surfaces with the smallest
needed action sets.

### Heartbeat minimum posture

- `logs:CreateLogStream`
- `logs:PutLogEvents`

### SNS minimum posture

- `sns:Publish`

### Shared rules

- no wildcard administrative posture in the runtime bridge
- no credential material in logs, docs, samples, or tracked fixtures
- no assumption that account bootstrap, topic creation, or log-group creation is
  done by these issues
- no assumption that AWS is the authority for runtime or ACIP semantics

## Failure And Retry Semantics

## Idempotency Contract

Retries are allowed only when they preserve the same logical signal event.

Rules:

- retries must reuse the same envelope identity fields
- retries must not mint a new `correlation_id`
- retries must not change `projection_level`
- retries must not widen payload content
- retries may change only bounded transport-attempt metadata that remains local
  to the publisher implementation and is not part of the reviewer-facing signal
  contract

`correlation_id` is trace correlation, not the sole dedupe key.

### Heartbeat idempotency

Heartbeat is expected to be at-least-once publishable.

The logical dedupe key is:

```text
signal_kind + runtime_id + agent_id + cycle_id + heartbeat_seq + status
```

Rules:

- `heartbeat_seq` must be monotonic within the active runtime instance
- retrying one heartbeat publish must keep the same `heartbeat_seq`
- a later heartbeat must advance `heartbeat_seq`; it must not reuse the prior
  sequence number to hide duplication
- consumers may collapse duplicate deliveries with the same dedupe key

### ACIP SNS idempotency

ACIP-to-SNS projection is also at-least-once publishable, but duplicate
delivery must not create new protocol meaning.

The logical dedupe key is:

```text
signal_kind + correlation_id + message_ref + projection_level + payload.content_sha256
```

Rules:

- retrying one ACIP projection must preserve `correlation_id`, `message_ref`,
  `projection_level`, and `content_sha256`
- changing the redacted content summary or hash creates a different projection
  event and therefore requires a new logical delivery
- SNS publish retry must not be used to upgrade a deferred/censored projection
  into a broader disclosure class
- consumers may treat duplicate deliveries with the same dedupe key as one
  logical delivery attempt

### First implementation posture

- `#4295` should implement stable heartbeat dedupe behavior
- `#4296` should implement stable ACIP projection dedupe behavior
- neither issue needs distributed exactly-once infrastructure in `v0.91.6`
- both issues must document that publication is at-least-once with bounded
  duplicate-collapse semantics

## Failure And Retry Semantics

### Heartbeat

- heartbeat publish failure must be observable
- heartbeat publish failure must set explicit `failure_class`
- heartbeat publish failure may degrade the signal lane without pretending the
  external heartbeat succeeded
- runtime execution itself may continue if local policy allows, but it must not
  report successful publication when publication failed

Recommended first failure classes:

- `config_missing`
- `approval_missing`
- `transport_init_failed`
- `publish_timeout`
- `publish_denied`
- `publish_failed`

### ACIP SNS

- if SNS delivery is selected and publish fails, delivery result must fail
  closed for that external bridge attempt
- local ACIP authority remains local; SNS success does not create new authority
- retry policy must be bounded and explicit

Recommended first failure classes:

- `config_missing`
- `approval_missing`
- `projection_denied`
- `publish_timeout`
- `publish_denied`
- `publish_failed`

## Logging And Tail-Friendliness Rules

- machine-readable payloads remain stdout-owned only where the existing command
  contract already says so
- operator-facing bridge observability remains stderr/default `adl_event`
  territory
- every publish attempt should have one bounded event line rather than a noisy
  multiline dump
- no full stack traces in ordinary success paths
- no raw AWS SDK debug dumps in tracked proof

## Local Validation Plan

Required local proof for first implementation slices:

- envelope-shape tests
- disabled-mode tests
- mock-mode publisher tests
- failure-class tests
- redaction/no-secret logging tests
- `git diff --check`

Optional later live proof, only if explicitly approved:

- one bounded heartbeat publish to an approved CloudWatch surface
- one bounded SNS publish to an approved topic
- redacted evidence packet only; no secret values or private identifiers

## Child-Issue Ownership

### `#4295`

Owns:

- shared runtime AWS signal config and mode seam
- shared envelope type used by both lanes
- heartbeat publisher
- heartbeat mock proof

Recommended first touched surfaces:

- one new shared `adl/src/...aws_signal...` contract module
- one heartbeat-specific publisher module
- focused tests for heartbeat mode and envelope behavior

### `#4296`

Owns:

- ACIP projection builder on top of the shared envelope seam
- SNS publisher
- ACIP-to-SNS mock proof

Recommended first touched surfaces:

- ACIP projection module layered on the shared signal contract
- SNS-specific publisher module
- focused tests for projection level, redaction, and fail-closed delivery

## Sequencing Decision

`#4295` and `#4296` should be treated as sequential by default unless the
shared envelope/config seam is already landed.

Reason:

- both implementation issues need the same shared contract and mode/config
  boundary
- letting both issues invent that seam independently would create avoidable file
  collisions and truth drift

That means the honest first execution order is:

1. `#4294` design contract
2. `#4295` shared seam plus heartbeat publisher
3. `#4296` ACIP projection plus SNS publisher on top of the landed seam

If a later stacked-branch plan proves the shared seam is isolated and stable,
parallel execution can be reconsidered. It is not the default for this
mini-sprint.

## Prerequisites And Follow-On Boundaries

Required before any live proof:

- operator-approved AWS target account from the `#3902` boundary
- operator-approved region
- operator-approved log group or SNS topic target
- explicit live-mode approval
- narrow IAM posture

If live proof becomes necessary and these prerequisites are still unresolved,
open a separate remediation issue rather than widening `#4295` or `#4296`.

## Samples

Tracked samples live next to this packet:

- `runtime_heartbeat_signal_envelope_4294.json`
- `acip_sns_projection_envelope_4294.json`

They are contract examples only, not proof of live AWS delivery.

## Non-Claims

- this packet does not create AWS resources
- this packet does not approve live publication
- this packet does not claim EventBridge, S3 archive, or OTEL export are first
  lane requirements
- this packet does not permit raw ACIP/private-state publication
- this packet does not claim `#4295` and `#4296` are conflict-free for parallel
  execution

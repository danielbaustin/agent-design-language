# Agent Communication And Invocation Protocol v1

## Status

Tracked feature boundary for the v0.90.5 Comms sprint.

This document promotes the prior TBD protocol note into a milestone-facing
feature surface. It defines the parent communication model that later Comms
issues implement and specialize. It does not, by itself, grant execution
authority, create provider transport, or replace Governed Tools.

## Purpose

ADL needs a general communication substrate for agents, not a review-only or
coding-only side protocol.

ACIP v1 exists so agents can:

- converse naturally
- consult one another
- invoke bounded work
- delegate or hand off responsibility
- negotiate with multiple parties
- return reviewable, traceable results

The core rule is stable:

> Agents communicate through messages, not through prompts or cards.

STP, SIP, SOR, SRP, evidence packets, demo packets, and later structured
artifacts are payloads or references inside messages. They are not the message
primitive itself.

## Core Boundary

ACIP v1 defines communication and invocation structure.

It does not define:

- execution authority by message alone
- encrypted external transport as a v1 requirement
- reputation or karma systems
- cross-polis federation
- same-session write-and-bless workflows

Governed action still depends on UTS, ACC, policy evaluation, Freedom Gate, and
governed execution. A message may request work; it does not authorize the work
by itself.

## Architectural Layers

### 1. Message Substrate

Every interaction is modeled as:

`message -> interpretation -> optional invocation -> response`

The message substrate must support free-form text, structured payload refs, and
traceable identity without forcing ordinary conversation into prompt-card form.

### 2. Canonical Envelope

ACIP messages should conform to a stable envelope so they can be traced,
validated, replayed, and redacted safely.

The minimum stable envelope includes:

- `message_id`
- `conversation_id`
- `sender_id`
- `recipient_id`
- `timestamp_utc`
- `monotonic_order`
- `intent`
- `visibility`
- `content`
- `payload_refs`
- `artifact_refs`
- `memory_refs`
- `authority_scope`
- `trace_required`
- `attachments`

Envelope design rules:

- natural-language content is first-class
- large or sensitive payloads should be referenced instead of inlined
- identity and ordering must be reconstructable
- the envelope must not require STP, SIP, SOR, or SRP to exist

### 3. Interaction Modes

ACIP v1 uses one substrate across a small stable mode vocabulary:

- `conversation`
- `consultation`
- `invocation`
- `review`
- `delegation`
- `negotiation`
- `handoff`
- `broadcast`

Modes may require additional payloads or validation rules, but they should not
fork the communication universe.

### 4. Invocation Contract

Structured work is represented as an invocation contract embedded in or
referenced by a message.

The minimum stable invocation shape includes:

- `invocation_id`
- `conversation_id`
- `causal_message_id`
- `caller_id`
- `target_id`
- `intent`
- `purpose`
- `input_refs`
- `constraints`
- `expected_outputs`
- `stop_policy`
- `authority_scope`
- `policy_refs`
- `decision_event_ref`
- `response_channel`
- `trace_required`

Governed invocation rules:

- invocation must be explicit or reconstructable
- governed invocation must link to a Freedom Gate or equivalent policy decision
- outputs must satisfy the declared output contract or emit refusal/failure
  evidence
- invocation does not imply repository, merge, or tool authority

### 5. Evidence, Trace, And Redaction

ACIP must be accountable without becoming a privacy leak.

The trace model must be able to represent:

- message creation
- invocation request
- policy or Freedom Gate decision
- response, refusal, failure, or partial completion
- output refs
- redaction decisions

Audience-specific views should remain possible for actor, operator, reviewer,
public, and Observatory-style consumers where applicable.

### 6. Specializations

Specializations inherit the core message, identity, visibility, invocation, and
trace rules instead of redefining transport.

Expected first specializations are:

- reviewer-agent invocation with SRP policy refs
- coding-agent invocation with patch or proposal outputs
- delegation and handoff
- demo or operator invocation
- multi-agent negotiation

## Relationship To Governed Tools

ACIP is adjacent to, but distinct from, the governed-tools stack.

### UTS

UTS describes tool-call shape and compatibility. It does not define agent
communication.

### ACC

ACC carries authority, identity, and execution policy for proposed actions. It
does not replace messages or conversations.

### Freedom Gate

Freedom Gate governs whether a candidate action may proceed. ACIP messages may
carry invocation requests and decision refs, but they do not bypass the gate.

### Governed Execution And Trace

Governed execution consumes approved authority surfaces. ACIP contributes the
message and invocation trace primitives that later trace and redaction issues
can reuse.

## Relationship To Review And Prompt Artifacts

SRP remains a durable review-policy artifact. It is a payload or policy ref
used by a review specialization, not the transport.

Likewise:

- STP is a structured task payload
- SIP is a structured input payload
- SOR is a structured result payload

These artifacts are useful when precision and replayability matter, but ACIP v1
must still support ordinary conversation without them.

## Security And Privacy Posture

Inside a single polis, ACIP should be treated as local, identity-bound,
traceable communication.

The v1 posture is:

- communication is private by default and visible by policy
- sender and recipient identity must be explicit and authenticated
- local envelopes should be integrity-protected so message identity, ordering,
  and visibility cannot drift silently
- sensitive local payloads should travel through encrypted payload refs or
  encrypted attachments rather than being dumped into public transcript form
- raw private reasoning should not be made a required envelope field

External or cross-polis transport remains out of scope until stronger transport
and identity guarantees are accepted.

## v0.90.5 Scope Boundary

Within v0.90.5, ACIP is a governed-tools-adjacent prerequisite feature.

This tranche is intended to stabilize:

- ACIP terminology and message-not-card boundary
- the parent envelope and invocation shape
- the local intra-polis security stance
- the relationship to UTS, ACC, Freedom Gate, review, coding, and trace

This tranche should not claim that the secure communication substrate is fully
implemented in `v0.90.5`. The default allocation still treats secure envelope,
identity binding, invocation records, local routing semantics, and conformance
fixtures as downstream implementation work that may remain in `v0.91` and
`v0.91.1` unless the milestone boundary deliberately keeps additional Comms
slices in `v0.90.5`.

## Consumer Map

This feature doc is the parent terminology and scope surface for:

- Comms-02 envelope and identity schema work
- Comms-03 invocation contract and Freedom Gate linkage
- Comms-04 conformance fixtures
- Comms-05 review-agent specialization
- Comms-06 coding-agent specialization
- Comms-07 trace, replay, redaction, and evidence integration
- Comms-08 demo and proof coverage
- future cross-polis and external-transport planning once TLS or
  mTLS-equivalent transport is accepted

Later consumers should reference this feature rather than copying protocol prose
into role-specific docs.

## Non-Proving Statements

ACIP v1 does not prove:

- production-ready external transport
- TLS or mTLS-equivalent federation
- proof that encrypted payload or attachment handling has been implemented
  end-to-end
- cross-polis routing
- reputation or social-contract systems

Those are future-work or later-milestone concerns unless explicitly implemented
and separately proved.

## Current Implementation Tranche

For v0.90.5, this feature doc establishes the architecture and boundary needed
for the Comms wave to proceed in a reviewable way. It exists so later issues can
implement bounded envelope, invocation, conformance, review, coding, trace, and
demo slices under one parent contract, while preserving the option to defer the
secure-local substrate implementation back to `v0.91` if the milestone review
boundary requires it.

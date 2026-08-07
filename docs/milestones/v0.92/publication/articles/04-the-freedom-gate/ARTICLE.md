# The Freedom Gate

The most consequential word in an agent system is not “intelligence.” It is “may.”

A model may understand a request. It may generate a useful plan. It may even identify the exact command that would accomplish the goal. None of those facts answers whether the command may run.

ADL calls the component that answers that question the **Freedom Gate**. It is the runtime boundary between proposed action and authorized effect.

## Proposal and authority are different data

Many agent architectures treat tool use as a continuation of text generation. The model emits a function name and arguments, and the application executes them. Safety then depends heavily on prompt wording, tool-specific checks, and whatever context the application remembered to carry.

Freedom Gate begins from a stricter premise:

> A valid proposal is not a valid permit.

The proposal describes what an actor wants to do. Authority describes why that actor is allowed to do it. Policy describes the constraints. Resource state determines whether the action can be admitted now. All of those inputs must agree before execution.

This separation also creates a clean trust boundary. Models can remain untrusted generators. The gate is ordinary software with explicit validation and fail-closed behavior.

## What the implemented gate checks

ADL's Runtime v3 implements a focused Freedom Gate contract in Rust. A request is bound to a principal, action, resource, unit ceiling, policy identity, and expiry. Commitments and authority grants are signed with trusted Ed25519 keys.

Delegation is possible, but it must attenuate. A child grant references its signed parent and cannot silently gain more units or delegation depth. Revoked, expired, forged, stale, escalated, or replayed authority is refused.

The gate checks trusted time and resource availability. It reserves the required units and consumes the request identity in the same critical section that emits a signed one-shot permit. That atomic relationship matters: two concurrent attempts should not both spend the same authority or resource reservation.

The permit is not a general credential. It authorizes one bounded actuation. The execution layer verifies its signature and consumes it once.

## Refusal is a first-class result

Systems often treat denial as an error string. Freedom Gate treats refusal as governance evidence.

A refusal retains its reason, request and policy identities, prior audit relationship, and a canonical evidence hash. An operator decision can be signed independently. An appeal preserves both the original refusal and the later decision rather than rewriting history to make the final answer look inevitable.

This is valuable even when the gate is correct. A human reviewer needs to understand whether a request failed because authority was missing, policy was stale, a resource was exhausted, a commitment was revoked, or a replay was detected. Those are operationally and ethically different events.

Refusal evidence also supports correction. A policy can be changed through an accountable path. A grant can be reissued. A resource can become available. The gate can admit a later request without pretending the earlier request was valid.

## The actuation boundary

Once a permit is issued, ADL's Adaptive Execution Engine invokes an injected actuation shell. The shell might eventually wrap a provider, tool, or external service; the runtime kernel does not reimplement every SDK.

The engine verifies the permit before calling the shell. Result bytes are bounded and hashed. Successful outcomes become canonical audit events. Tool errors and oversized results are quarantined rather than released as if they were normal trusted outputs.

This is another important separation. The gate decides whether an action may be attempted. It does not guarantee that the tool is correct, the external service is available, or the result is desirable. Authorization and outcome are different facts, and the audit trail preserves both.

## Continuity and replay

Governance state cannot disappear whenever the process restarts. Otherwise a restart could restore revoked authority, forget consumed permits, or break the audit chain.

Freedom Gate and the actuation engine therefore participate in Runtime v3 checkpoint and restore. Snapshots retain resource balances, revocations, consumed request identities, refusal and appeal evidence, permits, results, and audit continuity. Restore validates schema and evidence integrity before execution continues.

That design connects runtime continuity to security. Persistent identity without persistent authority state would be unsafe. Persistent authority state without integrity checks would be untrustworthy.

## Why call it freedom?

The name may sound paradoxical. A gate constrains action. How can it represent freedom?

In ADL's framing, meaningful agency requires more than unconstrained output. An agent should be able to form proposals, receive delegated authority, act within a capability envelope, encounter reasoned refusal, and participate in accountable review. Constraints make the scope of action legible to the agent and everyone affected by it.

The alternative is not pure freedom. It is opaque power distributed across prompts, application code, credentials, and accidents.

## The honest boundary

The current implementation proves a focused kernel contract through tests covering allowed mediation, forged or missing authority, stale policy, attenuating delegation, resource exhaustion, revocation, replay, appeal, quarantine, and checkpoint recovery.

It does not prove a universal policy language, formally verified governance, distributed authority, production provider integration, or a complete operator interface. Cryptographic signatures establish integrity and provenance within a trust model; they do not make a harmful policy wise.

That boundary is exactly why Freedom Gate is useful as an architecture. It identifies the place where intelligence becomes power, then gives engineers concrete contracts for deciding whether that transition should occur.

## Repository Sources

- [`docs/milestones/v0.86/features/FREEDOM_GATE.md`](../../../../v0.86/features/FREEDOM_GATE.md)
- [`docs/architecture/RUNTIME_V3_GOVERNED_EXECUTION_ARCHITECTURE.md`](../../../../../architecture/RUNTIME_V3_GOVERNED_EXECUTION_ARCHITECTURE.md)
- [`docs/adr/0015-governed-tools-execution-authority-architecture.md`](../../../../../adr/0015-governed-tools-execution-authority-architecture.md)
- [`docs/adr/0021-adl-capability-contract-runtime-authority-boundary.md`](../../../../../adr/0021-adl-capability-contract-runtime-authority-boundary.md)
- [`adl-runtime-kernel/src/governance.rs`](../../../../../../adl-runtime-kernel/src/governance.rs)

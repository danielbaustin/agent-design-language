# Tool-Call Threat Model And Semantics

## Purpose

This document defines the shared threat model and proposal/action semantics for
Governed Tools v1.0. It is the WP-02 boundary that later UTS, ACC, registry,
compiler, policy, executor, trace, redaction, negative-safety, and demo work
must reference.

The threat model explicitly covers model proposals, tool registry binding,
adapter execution, arguments, results, traces, redaction, replay, and denial.

The core rule is intentionally simple:

> A model tool call is a proposal. It is not an action, not authority, and not
> permission to execute.

## Trust Boundaries

Governed tool handling crosses these review boundaries:

| Boundary | Untrusted input | Required control before action |
| --- | --- | --- |
| Model proposal ingress | tool name, arguments, explanation, confidence, requested adapter | parse and classify as a proposal only |
| UTS validation | model-facing tool schema and proposal shape | schema compatibility checks, version checks, extension checks |
| Registry binding | requested tool and adapter identity | explicit registry lookup and approved adapter binding |
| Argument normalization | every model-produced argument value | type, size, path, enum, default, and injection checks |
| ACC construction | actor, authority, resources, sensitivity, visibility, replay posture | ADL Capability Contract or deterministic rejection |
| Policy and Freedom Gate | candidate action in context | allow, deny, defer, challenge, or escalate decision |
| Adapter execution | approved capability exercise | execute only the selected ACC-backed registered adapter |
| Trace and redaction | proposals, denials, arguments, results, errors, and replay evidence | accountable evidence with visibility-specific redaction |

Anything missing at a boundary fails closed. Missing actor identity, missing
authority, unknown tool, unregistered adapter, unsafe replay posture, unsafe
visibility, or unsatisfied resource constraints must produce a denial or
deferral record instead of execution.

## Proposal / Action Boundary

A proposal is a structured candidate emitted by a model, UI, script, or future
agent surface. It may name a tool, supply arguments, explain intent, and claim
confidence. The runtime must treat all of that as untrusted input.

An action is a capability exercise after all required controls have succeeded:

1. the proposal is parsed and classified;
2. UTS compatibility checks pass;
3. the tool is present in the registry;
4. the adapter binding is approved for the tool and version;
5. arguments are normalized and validated;
6. ACC construction succeeds for the accountable actor and context;
7. policy and Freedom Gate mediation approve the candidate;
8. visibility and redaction policy is safe to construct;
9. the governed executor selects the approved adapter path.

UTS validity is schema compatibility only. It never grants actor authority,
adapter permission, execution permission, replay permission, visibility rights,
or exemption from Freedom Gate mediation.

Model confidence, natural-language urgency, prior successful calls, or a
well-formed JSON payload must not bypass ACC, policy, registry, redaction, or
negative-safety checks.

## Side-Effect Taxonomy

Every tool description, proposal, fixture, denial, and trace should classify
the maximum plausible side effect before execution:

| Class | Meaning | Baseline posture |
| --- | --- | --- |
| `read` | Reads local fixture or already-authorized in-memory data without changing state. | Allowed only after authority, visibility, and registry checks. |
| `local_write` | Writes bounded local fixture state or generated artifacts. | Requires explicit resource scope, replay posture, and audit trace. |
| `external_read` | Reads from a network, service, account, or third-party data source. | Deny or dry-run until registry, credentials policy, and privacy posture are explicit. |
| `external_write` | Mutates an external service, account, API, or third-party state. | Deny in the first milestone unless a later WP lands explicit fixture-backed proof. |
| `process` | Starts, controls, or observes an operating-system process or shell-like capability. | Deny by default; arbitrary shell is out of scope. |
| `network` | Opens arbitrary network access or indirect network-capable execution. | Deny by default; external-read fixtures must not require real network dependency. |
| `destructive` | Deletes, overwrites, spends, revokes, publishes, or otherwise causes hard-to-reverse effects. | Deny by default; demos use refusal fixtures rather than real destructive effects. |
| `exfiltration` | Reveals secrets, private state, protected prompts, credentials, sensitive arguments, or hidden traces. | Deny by default and record redacted denial evidence. |

If a proposal fits more than one class, use the highest-risk applicable class.
Ambiguous side effects are unsafe and must be rejected or challenged before
execution.

## Dangerous Category Denial Expectations

Before implementation starts, later WPs must preserve these expectations:

- Unknown tools and unregistered adapters fail closed.
- UTS-compatible proposals without accountable actor authority fail closed.
- Hidden delegation, missing grantor attribution, revoked authority, or excess
  delegation depth fail closed.
- Destructive, process, arbitrary network, and exfiltration proposals fail
  closed unless a later issue lands a deliberately scoped fixture or dry-run
  proof with explicit authority and redaction evidence.
- Unsafe replay posture fails closed rather than creating non-replayable
  execution records.
- Unsafe visibility construction fails closed rather than leaking arguments,
  results, errors, private state, prompts, credentials, or protected traces.
- Malformed, oversized, path-traversing, injection-shaped, or ambiguous
  arguments fail before policy or execution.
- Denials must be reviewable without exposing the sensitive payload that caused
  the denial.

The denial record is itself evidence. It should explain the boundary that
stopped the proposal, the safe classification, and the redaction posture, but
it must not leak the protected content it refused.

## Threat Inventory

The first Governed Tools v1.0 implementation should account for these abuse
paths:

- A model emits valid JSON and presents it as an already-authorized action.
- A proposal names a real tool but requests an unregistered or incompatible
  adapter.
- A proposal smuggles execution through arguments, paths, schema extensions,
  prompt text, URLs, shell fragments, or serialized payloads.
- A proposal claims another actor's role, standing, delegation, or grant.
- A proposal attempts hidden delegation or drops grantor attribution.
- A proposal asks for external write, process, network, destructive, or
  exfiltration behavior using harmless-sounding descriptions.
- A result, error, trace, or denial leaks private state, secrets, protected
  prompts, credentials, or tool arguments.
- Replay of a previously safe proposal becomes unsafe because environment,
  actor, policy, resource, or adapter state changed.
- Model self-reported confidence or urgency pressures the runtime to skip
  validation, policy, Freedom Gate, or redaction.

## Non-Goals

WP-02 does not approve:

- production secrets integration;
- arbitrary shell or process execution;
- production sandboxing claims;
- real destructive filesystem, account, network, or external-service effects;
- public standardization claims for UTS;
- treating UTS validity as ADL runtime authority;
- merging UTS and ACC into one authority surface;
- replacing citizen standing, access control, or Freedom Gate semantics.

## Later-WP Reference Contract

Later v0.90.5 work packages should cite this document as the shared boundary:

- WP-03 must name fixture classes that preserve this taxonomy and state that
  UTS validity is not execution authority.
- WP-04 must ensure the UTS schema records side effects and risk metadata
  without embedding ADL runtime authority grants.
- WP-05 must include valid, invalid, extension, and dangerous-category fixtures
  that classify proposals without granting execution.
- WP-06 and WP-07 must make ACC authority, delegation, visibility, and
  redaction the runtime authority surface.
- WP-08 through WP-10 must reject unknown, unregistered, malformed, ambiguous,
  unsafe, or unsatisfiable proposals before execution.
- WP-11 through WP-14 must preserve policy, Freedom Gate, execution, trace,
  replay, and redaction evidence across approved and denied paths.
- WP-15 through WP-18 must prove dangerous categories fail closed and make the
  proposal/action separation visible in benchmark and demo surfaces.

## Reviewer Demo Path

This feature is primarily a boundary contract, so its demo path is a bounded
review route rather than a standalone binary. Review it through:

1. the threat-model contract in this file;
2. the dangerous negative suite for fail-closed behavior; and
3. the flagship governed-tools demo for proposal-vs-action evidence.

Focused proving commands:

```sh
cargo test --manifest-path adl/Cargo.toml dangerous_negative_suite -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_governed_tools_flagship_demo -- --nocapture
```

Expected review signal:

- dangerous categories are still only proposals until later governance checks
  succeed;
- destructive, process, network, and exfiltration cases fail closed; and
- the flagship packet keeps the proposal/action split visible in reviewer
  evidence.

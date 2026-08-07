# UTS and ACC: Making Agents With Tools Safer

A tool definition usually tells a model three things: the tool's name, what it does, and the arguments it accepts.

That is enough to generate a function call. It is not enough to govern one.

Will the tool change external state? Can the operation be replayed? Is it idempotent? What data can leave the system? Which credentials are required? Who authorized this actor? What evidence must survive? Who may see the result?

ADL separates those questions into two connected contracts: **Universal Tool Schema (UTS)** and **ADL Capability Contract (ACC)**.

UTS answers, “What is this tool?” ACC answers, “Who may use it, under what authority, with what visibility, and with what evidence?”

## UTS describes the capability

UTS is a portable, JSON-compatible tool description. Its implemented v1 baseline goes beyond parameters and return types. It includes side-effect class, determinism, replay safety, idempotence, resource needs, authentication, data sensitivity, exfiltration risk, execution environment, errors, and extension points.

These fields let a runtime reason before execution. A read-only deterministic lookup should not have the same posture as a payment, a deployment, or a command that deletes data. A replay-safe operation should not be treated like a one-time external effect. A tool that handles secrets needs different visibility and logging rules from one that formats public text.

UTS is transport-independent. The same semantic description can sit behind different model providers or invocation protocols. That portability helps prevent governance from being trapped in provider-specific function-call metadata.

But UTS intentionally does not grant authority.

## ACC describes the governed relationship

ACC adds the runtime-facing authority contract. It records the accountable actor and grantor, required capabilities, delegated scope, depth limits, policy and confirmation requirements, Freedom Gate posture, visibility rules, redaction, and failure behavior.

The distinction is easiest to see with a simple example.

Suppose a tool can publish a software release. UTS can describe its inputs, outputs, authentication, side effects, replay danger, and errors. ACC can say that a release manager delegated authority for one repository and version, that human confirmation is required, that the action must pass Freedom Gate, and that public logs receive a redacted projection while reviewers retain a fuller evidence record.

The call may be perfectly valid under UTS and still fail under ACC because the actor lacks authority, the delegation expired, confirmation is absent, or policy forbids the target.

That failure is a feature.

## Validity is not permission

The three equations at the heart of the design are intentionally negative:

- UTS validity is not authority.
- UTS validity is not execution permission.
- UTS validity is not replay permission.

Schema validation can establish that a proposal is well formed. It cannot establish that the proposal is legitimate in the current context.

This matters because tool ecosystems naturally optimize for interoperability. A common schema makes it easier for more models to call more tools. The same interoperability increases the importance of a separate admission layer. The easier invocation becomes, the less safe it is to smuggle permission into the fact that an invocation can be parsed.

## The trusted runtime decides

In ADL, a model can select a tool and produce arguments, but a trusted runtime evaluates the proposal. It can combine UTS side-effect and data posture with ACC authority, delegation, confirmation, visibility, and evidence requirements. Freedom Gate can then mediate the specific request before any external effect occurs.

This composition keeps responsibilities clear:

- The model proposes.
- UTS describes.
- ACC constrains authority and visibility.
- Freedom Gate admits or refuses.
- The actuation boundary executes.
- Audit records preserve what happened.

No single layer is asked to solve the entire safety problem.

## Current versions and future evolution

ADL currently treats UTS v1, whose formal implemented schema is v1.0, and ACC v1.0 as the implemented baselines. They have normative documents and machine-readable schema or runtime surfaces.

UTS v1.1 and ACC v1.1 are tracked additive directions. They refine invocation metadata, observability, version negotiation, authority, and visibility, but proposal documents should not be confused with guaranteed current wire behavior.

This version distinction is more than release hygiene. Agents and runtimes need to know which contract they are interpreting. Quietly treating a proposed field as enforced policy would be exactly the kind of hidden assumption these standards are meant to prevent.

## What contracts can and cannot do

UTS and ACC can improve interoperability, inspectability, replay posture, and governance. They can make refusal deterministic and evidence richer. They can expose dangerous mismatches before a call reaches a tool.

They cannot guarantee that a tool is bug-free, that a model's intent is correct, that a policy is just, or that an authorized action will have a good outcome. They do not eliminate security review or human judgment.

Their value is narrower and practical: they give agent platforms a precise place to represent facts that minimal function calling leaves implicit. In systems where models can touch files, infrastructure, money, messages, and identities, making those facts explicit is a substantial safety improvement.

## Repository Sources

- [`docs/explainers/UTS_AND_ACC.md`](../../../../../explainers/UTS_AND_ACC.md)
- [`docs/specs/uts/README.md`](../../../../../specs/uts/README.md)
- [`docs/specs/uts/UTS_V1.0_SCHEMA.md`](../../../../../specs/uts/UTS_V1.0_SCHEMA.md)
- [`docs/specs/acc/ACC_V1.0_SPEC.md`](../../../../../specs/acc/ACC_V1.0_SPEC.md)
- [`docs/specs/acc/ACC_V1.1_SPEC.md`](../../../../../specs/acc/ACC_V1.1_SPEC.md)
- [`docs/adr/0020-universal-tool-schema-portable-tool-description-standard.md`](../../../../../adr/0020-universal-tool-schema-portable-tool-description-standard.md)

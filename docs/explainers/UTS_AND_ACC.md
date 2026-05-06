# UTS + ACC

Universal Tool Schema (UTS) and ADL Capability Contract (ACC) are the two
halves of ADL's governed-tools model.

UTS answers: "What is this tool?"

ACC answers: "Who may use it, under what authority, with what visibility, and
what evidence?"

The split is intentional. A model can propose a tool call, and a tool can have a
valid portable schema, but neither fact grants execution authority. ADL keeps
tool shape, actor authority, policy checks, Freedom Gate mediation, trace
posture, replay posture, and redaction separate enough to review.

## UTS

UTS is the portable, JSON-compatible description of a tool. It records:

- tool name and version
- input and output schema
- side-effect class
- determinism and replay posture
- authentication and resource requirements
- data sensitivity and exfiltration risk
- error model and extension rules

UTS is useful for compatibility with tool-call ecosystems, but UTS validity is
not permission to execute.

## ACC

ACC is the runtime-facing authority contract. It records:

- accountable actor and grantor
- authority scope and required capabilities
- delegation chain and depth limits
- policy and confirmation requirements
- Freedom Gate requirements
- visibility rules for actor, operator, reviewer, public report, and
  Observatory projection
- redaction and failure posture

ACC makes it possible to say no deterministically, explain why, and keep safe
views for different audiences.

## Why This Matters

The UTS + ACC model lets ADL treat untrusted model output as a proposal, not an
action. The trusted runtime decides whether a proposed tool call is allowed,
records the decision, and preserves reviewable evidence.

That is the core of governed tool execution in ADL: expressive enough for modern
agents, bounded enough for security review.

## Deeper References

- [UTS public spec and conformance](../milestones/v0.90.5/features/UTS_PUBLIC_SPEC_AND_CONFORMANCE.md)
- [ACC authority and visibility](../milestones/v0.90.5/features/ACC_AUTHORITY_AND_VISIBILITY.md)
- [Governed Tools v1.0 release evidence](../milestones/v0.90.5/RELEASE_EVIDENCE_v0.90.5.md)
- [UTS + ACC multi-model benchmark plan](../milestones/v0.91.2/features/UTS_ACC_MULTI_MODEL_BENCHMARK.md)


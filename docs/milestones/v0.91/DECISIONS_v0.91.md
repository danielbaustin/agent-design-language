# Decisions - v0.91

## Status

These are the accepted architecture and milestone-boundary decisions for the
`v0.91.0` release line after the core implementation and proof lanes.

Durable ADR records:

- [ADR 0016: Moral Evidence And Cognitive-Being Substrate](../../adr/0016-moral-evidence-and-cognitive-being-substrate.md)
- [ADR 0017: Secure Local Agent Comms And A2A Boundary](../../adr/0017-secure-local-agent-comms-and-a2a-boundary.md)
- [ADR 0018: Structured Planning And Review Policy Artifacts](../../adr/0018-structured-planning-and-review-policy-artifacts.md)

| ID | Decision | Status | ADR / Evidence | Rationale | Consequences |
| --- | --- | --- | --- | --- | --- |
| D-01 | Treat v0.91 as moral governance and wellbeing foundations | Accepted | ADR 0016 | Moral trace and wellbeing evidence must land before birthday and constitutional governance can be credible. | v0.91 implements bounded moral evidence rather than vague ethics prose. |
| D-02 | Moral events are evidence records, not moral verdicts | Accepted | ADR 0016 | Reviewers need durable choice evidence without pretending the runtime has final moral judgment. | Event records include alternatives, reasons, constraints, authority, trace, and temporal anchors. |
| D-03 | Metrics are signals, not scores | Accepted | ADR 0016 | Scalar goodness, karma, or happiness scores would distort the model. | Moral metrics and wellbeing diagnostics stay decomposed and reviewable. |
| D-04 | Citizen self-access to wellbeing diagnostics is required | Accepted | ADR 0016 | A citizen should be able to inspect its own wellbeing state. | Operator, public, and governance views are mediated and redacted by policy. |
| D-05 | Anti-harm must be trajectory-aware | Accepted | ADR 0016 | Harm can be decomposed across benign-looking steps or delegated actors. | v0.91 includes synthetic delegated-harm proof surfaces. |
| D-06 | First birthday remains v0.92 | Accepted boundary | ADR 0016 | v0.91 supplies moral conditions but not the identity/birth event. | Birthday language stays downstream. |
| D-07 | Constitutional citizenship remains v0.93 | Accepted boundary | ADR 0016 | v0.91 trace evidence feeds governance but does not complete it. | Rights, duties, social contract, ToM, reputation, and shared social memory stay in v0.93. |
| D-08 | Runtime v2 is inherited, not reopened | Accepted boundary | ADR 0016 | Runtime v2 and citizen-state surfaces are prerequisites from v0.90.x. | v0.91 consumes runtime evidence rather than relitigating the substrate. |
| D-09 | Cognitive-being features are first-class | Accepted | ADR 0016 | Kindness, humor/absurdity, affect, cultivated intelligence, and moral resources are required before birthday readiness is credible. | Each feature has implemented contracts, fixtures, and proof surfaces. |
| D-10 | Agent Comms stays local and secure before external transport | Accepted | ADR 0017 | Intra-polis messages do not cross an external network, but still need identity, integrity, visibility policy, redaction, and sensitive-payload protection. | External or cross-polis comms remain unsupported or TLS/mTLS-gated. |
| D-11 | v0.91.1 is the adjacent-systems milestone | Accepted boundary | v0.91.1 planning package | Capability/aptitude testing, intelligence metric architecture, ANRM/Gemma, ToM, memory/identity, and runtime-v2/polis docs are too broad to absorb into v0.91. | v0.91 remains focused on the moral/cognitive-being substrate; v0.91.1 carries adjacent-system implementation, proof, and hardening. |
| D-12 | Structured planning should be durable before execution | Accepted | ADR 0018 | `/plan` is useful, but chat-only plans are too easy to lose or skip. | v0.91 adds saved planning artifacts, validator support, editor support, and plan-review direction. |
| D-13 | `SRP` should land early as the durable review-policy artifact | Accepted | ADR 0018 | Reviewer invocation should bind to durable policy rather than packet-only or chat-only instructions. | v0.91 promotes `SRP` into the issue bundle and reviewer-readiness surface. |
| D-14 | A2A is an adapter over Agent Comms, not a separate comms model | Accepted boundary | ADR 0017 | External-agent interoperability should consume the same identity, invocation, policy, and trace substrate rather than inventing a parallel one. | v0.91 records the bounded A2A adapter boundary; v0.91.1 carries broader adapter implementation and hardening. |

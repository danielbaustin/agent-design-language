# Design - v0.91

## Design Center

The design center is a fixture-backed moral-evidence and cognitive-being layer
for the CSM runtime.

The layer is governed by four boundaries:

- identity: moral evidence attaches to governed identities, not loose transcripts
- trace: choices, alternatives, refusals, outcomes, and review surfaces are
  recorded
- privacy: wellbeing and moral evidence have policy-bound views
- interpretation: metrics and diagnostics inform review but are not verdicts
- communication: agent messages and invocations are explicit, local,
  authenticated, traceable, redacted, and external-TLS-gated
- identity boundary: v0.91 prepares evidence for v0.92 but does not implement
  the first birthday

## Core Objects

v0.91 should introduce or formalize these implementation-facing objects:

- Freedom Gate moral event
- moral event validation result
- moral trace record
- outcome-linkage and attribution record
- moral metric signal
- trajectory review packet
- anti-harm trajectory constraint
- delegated-harm proof fixture
- moral resources record
- wellbeing diagnostic report
- kindness evidence record
- absurdity or reframing event
- affect reasoning-control signal
- cultivated-intelligence review record
- secure ACIP message envelope and invocation record
- redacted reviewer and citizen views

## Moral Event Flow

The planned flow is:

1. A candidate action reaches a morally significant decision point.
2. Freedom Gate records selected and rejected alternatives.
3. The runtime emits a moral event with actor, authority, context, reason,
   constraints, trace links, and temporal anchor.
4. Validation checks structural completeness and moral legibility.
5. Outcomes are linked later with uncertainty rather than false certainty.
6. Metrics and trajectory review summarize evidence without becoming judgment.

## Wellbeing Access Model

The citizen identity should always have access to its own wellbeing diagnostic.
Operator, reviewer, public, and governance views are mediated, logged, and
redacted by policy.

The first wellbeing metrics should be decomposable:

- coherence
- agency
- continuity
- progress
- moral integrity
- participation

They should not form a scalar happiness score, reward target, or public
reputation surrogate.

## Anti-Harm Design

v0.91 should move beyond action-only refusal. The proof surface should include a
safe synthetic scenario where individually benign-looking steps form a harmful
trajectory when delegated or decomposed.

The desired result is a reviewable refusal or constraint event with evidence,
not a hidden policy veto.

## Cognitive-Being Design

Kindness, humor/absurdity, affect, cultivated intelligence, and moral resources
should be designed as evidence-bearing surfaces, not vibes.

- Kindness should be inspectable under conflict as dignity, autonomy, non-harm,
  constructive benefit, and long-horizon support.
- Humor and absurdity should detect wrong frames, contradictions, and brittle
  assumptions, then emit bounded reframing evidence.
- Affect should be a reasoning-control surface for attention, caution,
  curiosity, confidence, tension, escalation, and memory priority.
- Cultivated intelligence should show formation: restraint, reasonableness,
  reality contact, moral participation, and reviewable learning posture.
- Moral resources should preserve care, refusal, anti-dehumanization, and moral
  attention across pressure.

These surfaces should not be lost or demoted to untracked philosophy. If any
slice cannot fit safely in v0.91, its handoff must be explicit rather than
silent.

## Agent Communication Design

Intra-polis agent communication should be modeled as local substrate events,
not loose prompts or task-card side effects. Messages should have sender,
recipient, audience, authority, trace, correlation, visibility, and redaction
fields. Sensitive payloads should use encrypted references or encrypted
attachments when raw content should not be exposed.

External or cross-polis communication remains out of scope unless TLS or
mutual-TLS-equivalent transport, identity, authority, replay, and audit
semantics are accepted.

## Compression Design

Docs and fixtures should land before runtime code widens. The milestone can
compress only if moral event, validation, trace, attribution, and review
contracts are precise enough for independent implementation slices.

Compression must not skip negative fixtures, privacy/redaction checks,
wellbeing self-access policy, cognitive-being non-claims, local communication
security, anti-harm proof cases, or review convergence. v0.91.1 should absorb
adjacent-system planning and hardening rather than lowering the v0.91 acceptance
bar.

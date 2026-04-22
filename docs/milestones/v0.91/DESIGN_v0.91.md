# Design - v0.91

## Design Center

The design center is a fixture-backed moral-evidence layer for the CSM runtime.

The layer is governed by four boundaries:

- identity: moral evidence attaches to governed identities, not loose transcripts
- trace: choices, alternatives, refusals, outcomes, and review surfaces are
  recorded
- privacy: wellbeing and moral evidence have policy-bound views
- interpretation: metrics and diagnostics inform review but are not verdicts

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

## Compression Design

Docs and fixtures should land before runtime code widens. The milestone can
compress only if moral event, validation, trace, attribution, and review
contracts are precise enough for independent implementation slices.

Compression must not skip negative fixtures, privacy/redaction checks,
wellbeing self-access policy, anti-harm proof cases, or review convergence.

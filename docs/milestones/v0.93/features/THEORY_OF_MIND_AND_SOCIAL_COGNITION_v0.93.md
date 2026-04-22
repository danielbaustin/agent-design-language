# v0.93 Theory Of Mind And Social Cognition

## Status

Forward-planning feature contract for v0.93. This document does not claim that
Theory of Mind, reputation, or shared social memory is implemented in v0.90.3.

The local ToM source packet previously carried older late-roadmap targeting. The
current scheduling decision is:

- v0.90.3 supplies citizen standing, access control, projection, and private
  state boundaries.
- v0.91 supplies moral trace, wellbeing evidence, and moral trajectory review.
- v0.92 supplies durable identity, memory grounding, capability envelopes, and
  first-birthday evidence.
- v0.93 owns the first bounded Theory of Mind, reputation-boundary, shared
  social memory, and constitutional-citizenship integration pass.

## Purpose

Theory of Mind in ADL is a bounded, evidence-grounded model of another
participant's beliefs, intentions, capabilities, uncertainty, and likely
coordination posture.

It is not prompt intuition. It is also not public reputation, standing,
citizenship, consciousness, aptitude, or a scalar trust score.

v0.93 should make Theory of Mind useful for a polis without letting it become a
hidden authority surface. ToM may inform arbitration, Freedom Gate evaluation,
delegation, communication, and constitutional review, but it must remain
bounded by standing, access control, redaction, signed trace, and policy.

## Source Dependencies

| Source | Role in ToM |
| --- | --- |
| CSM_CITIZENS_AND_STANDING.md source | Defines citizens, guests, service actors, external actors, and prohibited naked actors. ToM must consume this standing model rather than redefining actor classes. |
| [STANDING_COMMUNICATION_BOUNDARY_v0.90.3.md](../../v0.90.3/STANDING_COMMUNICATION_BOUNDARY_v0.90.3.md) | Tracked v0.90.3 proof that standing classes, communication examples, and negative cases have landed. |
| [ACCESS_CONTROL_SEMANTICS_v0.90.3.md](../../v0.90.3/ACCESS_CONTROL_SEMANTICS_v0.90.3.md) | Tracked v0.90.3 proof that sensitive access paths require authority and auditable denial. |
| [REDACTED_OBSERVATORY_PROJECTIONS_v0.90.3.md](../../v0.90.3/REDACTED_OBSERVATORY_PROJECTIONS_v0.90.3.md) | Projection and redaction boundary for reviewer/operator views. |
| [MORAL_GOVERNANCE_ALLOCATION_v0.91.md](../../v0.91/MORAL_GOVERNANCE_ALLOCATION_v0.91.md) | Moral trace, wellbeing, outcome, and trajectory evidence that ToM must not replace. |
| [IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md](../../v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md) | Durable identity, capability envelope, memory grounding, and birth evidence consumed by v0.93 social cognition. |
| [CONSTITUTIONAL_CITIZENSHIP_AND_POLIS_GOVERNANCE_PLAN_v0.93.md](../CONSTITUTIONAL_CITIZENSHIP_AND_POLIS_GOVERNANCE_PLAN_v0.93.md) | Governance layer that consumes ToM without collapsing it into public verdicts. |

## Core Boundary

ToM is private, evidence-grounded social cognition. Reputation is a public or
governance-facing summary. Standing is recognition by the polis. Constitutional
review is policy interpretation over evidence.

Those four surfaces must not collapse into one another.

| Surface | Purpose | Authority boundary |
| --- | --- | --- |
| Theory of Mind | Model another participant's beliefs, intentions, capabilities, uncertainty, and expected behavior. | Private or mediated cognitive state; inspection and projection require authority. |
| Reputation | Public or governance-facing summary derived from trace and reviewable evidence. | Must be redacted, challengeable, and distinct from private ToM. |
| Standing | Determine who may participate and under what class of recognition. | Supplied by v0.90.3 standing/access work and later governance rules. |
| Constitutional review | Interpret conduct under rights, duties, policy, trace, standing, and appeal context. | Consumes ToM only as evidence input or contextual signal, never as unreviewable verdict. |

## Privacy And Standing Rules

1. A ToM model may be created only for an actor with declared standing or for a
   refused/prohibited actor as part of a bounded security or denial record.
2. A guest may be modeled only within the guest scope, duration, channel, and
   privacy class.
3. A service actor may be modeled only as a delegated runtime surface, not as a
   social citizen, unless later policy explicitly elevates it.
4. A naked actor may not be normalized into the polis by ToM modeling. The model
   may record only refusal, quarantine, or security evidence.
5. ToM inspection, projection, migration, export, or use in governance requires
   explicit authority and an auditable access or denial event.
6. Communication does not imply inspection. Receiving a message from a citizen
   or guest does not grant access to that actor's private state or private ToM
   model.
7. Citizen-facing access to self-relevant governance records must not imply
   unrestricted access to another citizen's private ToM model.

## Minimum Agent Model Contract

A later v0.93 implementation should define a versioned agent-model schema with
at least:

- target actor identity and standing class
- model owner or actor that maintains the model
- policy and authority context for creation and inspection
- beliefs, intentions, capabilities, and uncertainty entries
- evidence references for every entry
- confidence value and confidence basis for every entry
- freshness and temporal anchor for every entry
- conflict group or contradiction reference where relevant
- redaction class for projection
- model version and previous version reference

Every belief, intention, capability, and uncertainty entry must be traceable to
evidence. Assumptions may be recorded only when they are explicitly labeled as
assumptions, carry low or bounded confidence, and cite the reason the assumption
is being held.

## Minimum ToM Update Event Contract

A later v0.93 implementation should define a signed update-event schema with at
least:

- event id and schema version
- actor maintaining the model
- target actor being modeled
- model id, prior model version, and new model version
- update type
- update mode
- policy reference and authority disposition
- observation, trace, or ObsMem evidence references
- changed entries with typed prior and new states
- confidence delta and confidence basis
- conflict status, conflict group, and resolution state where relevant
- decay policy, freshness horizon, and decay reason where relevant
- redaction and projection class
- timestamp or temporal anchor
- signature or signed-trace envelope reference

Greenfield update events must be replayable from evidence. They must not be
arbitrary object blobs that only a human can interpret after the fact.

## Signed Trace Posture

ToM update events are governance-relevant. In v0.93 they should be treated as
signed-trace records whenever they can affect:

- arbitration
- Freedom Gate evaluation
- delegation
- reputation
- standing
- constitutional review
- public, operator, reviewer, or citizen-facing projections

Unsigned local rehearsal fixtures may exist during development, but any
milestone claim must distinguish rehearsal data from signed governance evidence.

## Conflict And Decay

Conflicts must never be silently overwritten.

A later implementation should model:

- the two or more entries in conflict
- the evidence supporting each side
- current confidence per side
- who detected the conflict
- whether the conflict is unresolved, deferred, superseded, or resolved
- any action restriction caused by the conflict
- any appeal or challenge path when the conflict affects governance

Temporal decay must also be explicit. A stale ToM entry should not keep its
original force merely because it remains stored. Decay events should record:

- freshness horizon
- decay policy
- prior confidence
- new confidence
- evidence age or missing reinforcement
- operational consequence of the decay

## Runtime Placement

ToM should not be inserted as a single step in the older cognitive loop and then
treated as done.

In Runtime v2 terms, ToM participates in a cycle as:

1. governed observation or interaction produces evidence
2. evidence is recorded or made durably referenceable
3. ToM update policy decides whether a model update is allowed
4. signed ToM update events record the model change
5. updated summaries become available to affect, arbitration, delegation, and
   Freedom Gate evaluation through bounded projections
6. any governance-facing use records which ToM projection was used and under
   which authority

The authoritative source remains evidence and signed trace, not an unreviewed
prompt summary.

## Shared Social Memory

Shared social memory is the polis-facing evidence layer that allows multiple
citizens or reviewers to reason about social facts without exposing raw private
state.

It may include:

- public reputation summaries
- standing transitions
- open challenges and appeals
- governed communication facts
- delegation outcomes
- constitutional review findings
- redacted ToM-derived signals when policy allows projection

It must not include:

- raw private citizen state
- private model-of-other-agent content without authority
- private wellbeing details beyond the allowed projection class
- prompt leakage
- hidden operator annotations that affect governance without trace

## State-Space Compression Rationale

ToM is a compression layer over another participant's hidden, high-dimensional
state. The compression is useful only if it stays predictive, evidence-grounded,
bounded, and challengeable.

The goal is not to know another citizen completely. The goal is to maintain a
small, inspectable, uncertainty-aware social model good enough to support
coordination and governance without violating privacy.

Good compression should:

- preserve uncertainty rather than turn guesses into facts
- prefer evidence links over fluent summaries
- degrade when stale
- expose conflict instead of smoothing it away
- be cheap enough to update during ordinary cycles
- be redacted before public or reviewer projection

This connects ToM to the broader CSM and state-space compression story without
turning a mathematical framing into an implementation claim.

## v0.93 Work Package Shape

Later v0.93 WP planning should split this work into bounded pieces:

1. ToM schema and update-event contract.
2. Standing, privacy, access-control, and projection rules for ToM.
3. Private ToM versus public reputation and shared social memory boundary.
4. Conflict, decay, and signed-trace proof fixtures.
5. ToM-informed arbitration or Freedom Gate fixture that shows influence
   without override.
6. Reviewer-facing demo packet showing evidence, redaction, challengeability,
   and non-claims.

## Demo Candidates

| Candidate | What it proves | Proof surface |
| --- | --- | --- |
| Private ToM update with signed evidence | A model update is evidence-grounded and replayable. | Observation fixture, signed update event, model diff. |
| Guest-scope ToM boundary | Guest participation can be modeled without creating citizen rights or inspection rights. | Guest communication fixture, model projection, denied inspection. |
| Reputation projection from bounded evidence | Public reputation is a redacted projection, not the private ToM model. | Private model fixture, projection policy, public summary, redaction report. |
| Conflict and decay | Contradictory or stale model entries are retained, marked, and reduced in force. | Conflict fixture, decay event, arbitration impact. |
| ToM-informed Freedom Gate | ToM can influence action evaluation without bypassing policy. | Candidate action packet, ToM projection, Freedom Gate disposition. |

## Non-Claims

- This document does not implement ToM.
- This document does not create a v0.93 issue wave.
- This document does not claim ToM proves consciousness or sentience.
- This document does not turn private ToM into public reputation.
- This document does not grant inspection rights over private citizen state.
- This document does not replace v0.90.3 standing, access control, or projection
  work.
- This document does not replace v0.91 moral trace or v0.92 identity and
  birthday work.

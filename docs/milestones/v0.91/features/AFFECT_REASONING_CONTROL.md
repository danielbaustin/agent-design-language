# Affect Reasoning-Control

## Milestone Boundary

This v0.91 feature does not claim consciousness, inner feeling, or human
emotion. It defines affect-like state as an explicit reasoning-control surface
that can improve prioritization, caution, escalation, memory priority, and
bounded self-regulation.

It belongs in v0.91 because moral and cognitive-being work needs inspectable
control signals, not just flat scores or implicit prompt style.

## Purpose

ADL should treat affect-like state as:

> a structured internal summary of how current evidence, uncertainty, novelty,
> progress, and risk should bias the next reasoning move.

The function is operational, not theatrical. Affect exists here to guide
reasoning quality, search allocation, critique depth, and self-correction.

## Core Thesis

A deterministic agent can reason better if it carries explicit internal
affect-like state that summarizes how the current situation should bias its next
step.

This matters because a flat confidence score often misses important dynamics:

- the branch is coherent but dangerous
- the search is novel but fragile
- the system is stuck in repetitive low-yield effort
- contradiction is rising even though local checks still pass

## Proposed Dimensions

The initial v0.91 affect surface should stay compact and machine-useful:

- uncertainty
- urgency
- attention
- friction
- deferral

The landed v0.91 WP-13 surface is `affect_reasoning_control_packet.v1` in
`adl/src/runtime_v2/affect_reasoning_control.rs`. The first bounded contract
keeps five operational control signals explicit:

- uncertainty
- urgency
- attention
- friction
- deferral

It also exposes explicit policy effects for review-depth increases, escalation,
attention retention, candidate shifts, and staged deferral. The surface is
operational only: it does not claim hidden emotions, consciousness, or
subjective experience.

These are functional dimensions, not claims of subjective feeling.

## Operational Meaning

### Uncertainty

Signal that the current branch remains unresolved and needs stronger review.

### Urgency

Signal that consequence or timing pressure should bias toward escalation or
faster governed action.

### Attention

Signal that a branch or evidence set should remain salient.

### Friction

Signal that effort is accumulating without adequate progress or coherence.

### Deferral

Signal that staged review or delay is safer than false closure.

## Source Signals

Affect-like state should be derived from measurable runtime evidence such as:

- prediction error
- disagreement among critics or agents
- evidence strength
- novelty
- progress against acceptance criteria
- cost or latency budget burn
- safety or policy risk
- contradiction density
- replay instability
- operator correction

## Example Artifact

```yaml
affect_reasoning_control:
  uncertainty: unknown
  urgency: unknown
  attention: unknown
  friction: unknown
  deferral: unknown
  evidence_links:
    - metric_ref
    - trace_ref
  effects:
    - raise_review_depth
    - escalate
    - retain_attention
    - shift_candidate
    - defer
```

## Relationship To Reasoning

Affect-like control should influence:

- hypothesis ranking
- branch expansion and pruning
- confidence calibration
- retry or backoff behavior
- escalation decisions
- memory retention priority
- when to continue, stop, ask for help, or request critique

## Implementation Placement

v0.91 should land a bounded affect reasoning-control surface after the moral
trace and metric layers exist. The initial implementation should include:

- an explicit record or report shape
- deterministic update rules
- policy hooks
- negative cases against hidden or theatrical emotion claims

## Evidence Expectations

The proof surface should show that affect-like state can change:

- attention allocation
- caution depth
- escalation behavior
- exploration pressure
- memory priority

without claiming inner life or turning the feature into freeform prose.

## Non-Claims

This feature does not claim subjective feeling, personhood, consciousness, or
human emotional depth. It claims only that explicit affect-like control signals
can improve bounded reasoning and make that control visible to reviewers.

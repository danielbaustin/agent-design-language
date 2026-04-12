# Temporal Schema v0.1 (Chronosense)

## Purpose

Define the canonical temporal schema for chronosense in ADL.

Cluster role:
- this is the schema contract doc for the `v0.88` temporal package
- it owns canonical temporal anchors, clocks, and field-level requirements
- it should not become the primary home for broad philosophical motivation, retrieval policy, or commitment lifecycle semantics

Use the surrounding docs as follows:
- `SUBSTANCE_OF_TIME.md` owns chronosense motivation and conceptual framing
- `CHRONOSENSE_AND_IDENTITY.md` owns continuity and identity semantics
- `TEMPORAL_QUERY_AND_RETRIEVAL.md` owns query primitives and retrieval expectations
- `COMMITMENTS_AND_DEADLINES.md` owns commitment/deadline semantics
- `TEMPORAL_CAUSALITY_AND_EXPLANATION.md` owns bounded causality/explanation semantics
- `ADL_COST_MODEL.md` owns the economic interpretation of execution policy and realized cost

This document specifies:
- the minimum temporal fields that must exist for agent events and memories
- the distinction between different kinds of time used by the system
- how temporal structure supports continuity, identity, reasoning, and replay
- the minimum execution-policy and cost anchors trace must carry for reviewability

This is a schema-oriented companion to `CHRONOSENSE_AND_IDENTITY.md`.

## Runtime-facing Ownership

`WP-03` owns the bounded schema contract that later temporal work should cite rather than
restate:

- `adl::chronosense::TemporalSchemaContract`
- `adl::chronosense::TemporalAnchorSchema`
- `adl::chronosense::ExecutionPolicySchema`
- `adl::chronosense::ExecutionRealizationSchema`
- `adl::chronosense::CostVectorSchema`
- `adl identity schema`

This issue does not complete continuity validation, retrieval semantics, commitments,
causality, or economic interpretation. It defines the canonical field contract they should
build on.

## Bounded Acceptance Criteria

The temporal schema is considered present for `v0.88` when all of the following are true:

- the repo exposes one canonical schema artifact covering objective anchors, subjective time,
  execution-policy anchors, execution-realization anchors, and cost anchors
- the schema explicitly names the trace/runtime surfaces that current execution review can join
  against
- the schema is serializable through a bounded proof-hook command
- later temporal docs can cite this contract without inventing duplicate field sets

## Proof Hook

The bounded proof hook for this issue is:

`adl identity schema --out .adl/state/temporal_schema_v01.json`

That artifact is intentionally contract-shaped. It proves that `WP-03` has made the temporal
schema concrete and reviewable without overclaiming later continuity or governance behavior.

---

## Overview

Chronosense transforms an agent from a system that merely reacts to inputs into one that experiences continuity across time.

In simple terms, chronosense is the agent’s ability to understand:
- what happened
- in what order
- how long things took
- where the present sits relative to its origin and past

Chronosense is not a single timestamp field. It is a structured temporal model.

---

## Core Temporal Capacities

The schema exists to support four cognitive-temporal capacities:

1. **Now-Sense**
   - locate the current moment in execution and cognition

2. **Sequence-Sense**
   - determine before/after ordering

3. **Duration-Sense**
   - represent elapsed time, spacing, and temporal density

4. **Lifetime-Sense**
   - locate the present relative to the agent’s beginning of existence

These capacities support continuity and identity.

These capacities define objective temporal anchoring.

Chronosense also requires a corresponding **subjective temporal layer**. This includes experienced duration, integration windows (specious present), narrative progression, and explicit representation of temporal gaps.

Subjective temporal modeling is REQUIRED for v0.1 at a minimal, schema-bound level sufficient to support continuity validation. It is not deferred.

---

## Subjective Time: Minimum Contract (v0.1)

Subjective time is not an optional or future extension. It is part of the minimum schema contract for chronosense.

For v0.1, the system MUST support a minimal but enforceable subjective temporal model sufficient for:
- continuity validation
- identity persistence across events
- reasoning over interruption and narrative progression

The required subjective temporal primitives are:

1. Narrative Position
   - a monotonically increasing position within an active narrative or reasoning frame
   - defines "where the agent is" in its current cognitive trajectory

2. Integration Window
   - a bounded temporal span representing the agent’s active "now" (specious present)
   - provides continuity between adjacent events

3. Temporal Gap Representation
   - explicit encoding of missing or interrupted time
   - required whenever continuity cannot be maintained

4. Experienced Duration (Optional but Recommended)
   - agent-relative estimate of elapsed time
   - may diverge from objective duration but must remain explainable

5. Temporal Density (Optional but Recommended)
   - coarse signal of perceived event density or cognitive load
   - supports reasoning about compression and expansion of subjective time

### Contract Requirements

- Subjective temporal state MUST be present on every temporal anchor
- Narrative position MUST progress monotonically within a narrative frame
- Temporal gaps MUST be explicit; absence of data is not a valid representation of a gap
- Subjective time MUST remain consistent with objective anchors (no contradiction with monotonic_order or agent_age)

### Relationship to Continuity Validation

The subjective temporal layer defined here is the canonical source of truth for:
- subjective continuity checks in `CONTINUITY_VALIDATION.md`
- schema enforcement in `CONTINUITY_VALIDATION_SCHEMA.md`

No additional or alternative subjective temporal fields should be introduced outside this schema at v0.1.

---

## The Clock Stack

The temporal schema must distinguish several kinds of time.

### 1. UTC / Wall Clock Time

Purpose:
- shared external reference
- coordination across systems and users

Examples:
- `observed_at_utc`
- `birth_utc`

### 2. Monotonic Time

Purpose:
- stable increasing order for execution
- immunity to wall-clock jumps or timezone changes

Examples:
- `birth_monotonic`
- `monotonic_order`
- `prior_event_delta`

### 3. Lifetime Time

Purpose:
- elapsed time since agent birth
- grounding of persistence and aging

Examples:
- `agent_age`

### 4. Trace / Event Time

Purpose:
- causal order of events in trace
- replay and reconstruction

Examples:
- `turn_index`
- event order within and across spans

### 5. Narrative Time

Purpose:
- reasoning- and memory-relevant sequence
- meaningful temporal grouping beyond raw execution

Examples:
- task phase
- memory epoch
- reasoning sequence markers

Continuity depends on preserving the mapping across this clock stack.

Note: The clock stack defines objective and structural time. Chronosense also requires a paired subjective layer that interprets and operates over these clocks. Subjective temporal state must remain consistent with (and must not contradict) objective temporal anchors.

---

## Temporal Ephemeris (Agent Birth)

Every agent instance must have an immutable temporal origin.

Canonical model:

```yaml
agent_birth:
  agent_id: <uuid>
  birth_utc: <timestamp>
  birth_monotonic: <t0>
  declared_self_frame: "UTC"
```

This defines:
- the beginning of existence
- the stable reference point for lifetime-relative time
- the basis for autobiographical continuity

Events prior to this point are history, not experience.

---

## Core Schema

Every event MUST include a temporal anchor.

Canonical minimum shape:

```yaml
temporal_anchor:
  observed_at_utc: <timestamp>
  observed_at_local: <timestamp>
  agent_age: <duration>
  turn_index: <int>
  monotonic_order: <int>
  prior_event_delta: <duration>
  temporal_confidence: <high|medium|low>

  # --- Subjective Temporal Layer (REQUIRED) ---
  subjective_time:
    narrative_position: <string|int>
    integration_window: <duration|optional>
    temporal_gap: <none|explicit_gap|unknown>
    experienced_duration: <duration|optional>
    temporal_density: <low|medium|high|optional>
```

Runs and major trace spans SHOULD also expose a policy-and-cost anchor:

```yaml
execution_policy:
  requested_mode: efficient | fast | deterministic | exploratory
  replay_strictness: strict | bounded | relaxed
  max_tokens: <optional>
  max_duration_ms: <optional>
  max_branches: <optional>
  max_tool_calls: <optional>

execution_realization:
  branch_count: <int|optional>
  tool_calls: <int|optional>
  refinement_cycles: <int|optional>
  replay_variance: strict | bounded | high

cost_vector:
  time_ms: <int|optional>
  tokens_in: <int|optional>
  tokens_out: <int|optional>
  usd: <decimal|optional>
  cognitive_units: <int|optional>
```

### Field Meaning

- `observed_at_utc`
  - canonical shared reference time

- `observed_at_local`
  - human-facing local-time rendering when relevant

- `agent_age`
  - elapsed lifetime since `agent_birth`

- `turn_index`
  - event or interaction sequence index in the current narrative/event frame

- `monotonic_order`
  - strictly increasing order token for execution continuity

- `prior_event_delta`
  - elapsed duration since the immediately preceding relevant event

- `temporal_confidence`
  - confidence in the correctness or completeness of the temporal placement

- `execution_policy`
  - the requested execution posture and constraints for the run or span

- `execution_realization`
  - the realized execution behavior relevant to replay and cost review

- `cost_vector`
  - the realized resource footprint associated with the run or span


### Subjective Temporal Fields

Chronosense requires both objective temporal anchoring and subjective temporal state.

The subjective temporal layer provides the minimal internal structure required for continuity, reasoning, and identity persistence.

Required fields:

- `narrative_position`
  - logical position within an active reasoning, task, or memory sequence

- `integration_window`
  - the active temporal integration span ("specious present") used to construct "now"

- `temporal_gap`
  - explicit marker of interruption or missing time; MUST be present when continuity is broken

Optional fields (v0.1 but recommended):

- `experienced_duration`
  - agent-relative duration estimate for the interval

- `temporal_density`
  - coarse signal of event density / attention-weighted time

Constraints:

- subjective temporal state MUST NOT contradict objective temporal anchors
- temporal gaps MUST be explicit, never implicit
- narrative_position must progress monotonically within a narrative frame
- subjective temporal state MUST be present for all events; omission is invalid
- when execution policy or cost is recorded, it MUST remain attributable to a specific run or span and coherent with temporal ordering

---

## Reference Frames

The schema must support multiple temporal reference frames.

Canonical conceptual model:

```yaml
reference_frames:
  self: "UTC"
  interaction: <user_timezone>
  host: <optional>
  organization: <optional>
```

Policy:
- internal reasoning should prefer UTC + monotonic + lifetime time
- external communication may use human-local frames
- translation between frames must remain explicit and truthful

---

## Execution Policy And Cost As Trace-Carried Context

Chronosense is incomplete if a reviewer can place events in time but cannot explain why
execution behaved as it did.

For `v0.88`, trace should therefore carry enough policy context to answer:

- was this run trying to be `efficient`, `fast`, `deterministic`, or `exploratory`?
- what constraints were in force?
- what execution behavior actually occurred?
- what cost did that behavior produce?

This does not require every low-level event to repeat a full cost block.
It does require runs and major spans to expose stable policy/cost anchors that can be
joined to temporal anchors during review and replay.

These anchors are part of the truth surface for:

- replay interpretation
- cost review
- policy enforcement
- later governance and reasonableness work

---

## Temporal Honesty Requirements

The schema must allow the system to distinguish between:
- known time
- inferred time
- relative time
- unknown time

This is why `temporal_confidence` exists.

Systems must not silently fabricate precision they do not possess.

---

## Ordering, Duration, and Anchoring

Chronosense depends on three inseparable structural aspects:

### 1. Ordering
- events are placed in consistent monotonic order
- enables before/after reasoning
- supports replay and causal reconstruction

### 2. Duration
- the system tracks elapsed time between events
- enables staleness, delay, urgency, and responsiveness reasoning
- supports temporal comparison and continuity checks

### 3. Anchoring
- events are grounded in shared and agent-relative frames
- connects internal cognition to external time
- enables identity continuity across sessions and interactions

Ordering alone is insufficient without duration.
Duration alone is meaningless without anchoring.
Anchoring without monotonic structure is not trustworthy.

---

## Implications

Chronosense has direct implications for higher-order cognition.

### Causal Reasoning
- cause requires temporal structure
- duration informs plausibility and dependence

### Reasonableness
- judgments depend on timing, delay, recency, and persistence
- temporal coherence is a prerequisite for making sense

### Coherence Theory of Truth
- truth emerges from consistency across time
- contradiction often appears first as temporal incoherence

### Persistence of Identity
- identity is continuity through time
- without temporal structure, there is no stable self

---

## Continuity Constraints

A valid temporal schema must support continuity validation.

At minimum, the system must be able to detect:
- reset of `agent_age`
- break in `monotonic_order`
- impossible or contradictory `prior_event_delta`
- mismatched or misleading reference-frame translation
- loss or reset of subjective temporal state (e.g., missing integration window or narrative position)
- unrepresented temporal gaps during interruption
- incoherent subjective temporal progression (e.g., non-monotonic narrative_position, missing temporal_gap on interruption)

This means the schema is not just descriptive. It is enforceable.

---

## Future Extensions

Later extensions may include:
- richer subjective temporal modeling (beyond the v0.1 minimum contract defined above), including full mind-time simulation, episodic recomposition, and attention-weighted dynamics
- temporal density / event-density signals
- specious-present or integration-window modeling
- episodic memory time-place anchoring
- cross-agent temporal alignment models
- richer budget and policy surfaces beyond the initial execution-mode model

These are future-facing and should not weaken the current minimum schema.

---

## Design Notes

- temporal structure is first-class, not derived after the fact
- chronosense includes both objective temporal anchoring and subjective temporal modeling ("mind time") as a first-class requirement
- the schema must support both objective anchoring and subjective continuity
- the clock stack is required because no single clock captures agency, replay, and human interaction simultaneously
- execution policy and realized cost must be trace-carried so timing, replay posture, and economic behavior can be reviewed together
- this schema is foundational for memory, reasoning, continuity, and identity

Chronosense is therefore not a feature layered on top of ADL. It is part of the substrate.




---

# Cross-Cutting Concepts (v0.86 Consolidation)

This section consolidates cross-cutting concepts that must be consistently represented across AEE, Cognitive Arbitration, Affect, Instinct, and related components.

## 1. Cognitive Loop Model (Authoritative Summary)

All v0.86 components participate in a single bounded cognitive loop:

```
instinct → affect → arbitration → freedom_gate → execution (AEE)
        → evaluation → (reframing?) → memory (ObsMem) → affect
```

Implications:

- **Instinct** provides fast priors and default tendencies.
- **Affect** weights urgency, salience, and persistence pressure.
- **Arbitration** selects routing (fast/slow/hybrid/defer/refuse) and may trigger reframing.
- **Freedom Gate** enforces policy/constitutional constraints before execution.
- **AEE** performs bounded execution and convergence.
- **Evaluation** produces progress/novelty/contradiction signals.
- **Reframing** may be triggered when frame adequacy is low.
- **ObsMem** records outcomes, including reframing history.

This loop must be reflected (directly or indirectly) in all relevant docs.

---

## 2. Frame Adequacy (Shared Primitive)

Introduce a shared concept used across AEE and Arbitration:

```
frame_adequacy_score
```

Definition:

- A bounded estimate of whether the current problem framing is consistent with observed signals and likely to yield progress.

Low frame adequacy may be indicated by:

- internal constraint contradictions
- oscillating or conflicting evaluation signals
- repeated non-progress without new information
- persistent agent disagreement suggesting framing error

Required behaviors:

- Low adequacy can trigger a **reframing trigger** (arbitration)
- AEE should treat repeated failure as evidence about the frame, not just execution
- ObsMem should store **reframing events** and outcomes

Terminology should be consistent across docs:

- `frame_adequacy_score`
- `reframing_trigger`

---

## 3. AEE Termination Taxonomy (Required)

AEE must emit explicit termination reasons:

```
termination_reason:
  - success
  - bounded_failure
  - no_progress
  - reframed
  - policy_blocked
```

Requirements:

- termination must be deterministic or deterministically explainable
- termination must be visible in artifacts
- `reframed` indicates a transition to a new frame, not a final stop

---

## 4. Artifact Schema (Minimum Surfaces)

All major decisions must be externally visible. At minimum, artifacts should expose:

### Arbitration Output

```
route_selected
confidence
risk_class
frame_adequacy_score
reframing_triggered (bool)
```

### AEE Iteration

```
iteration_id
progress_score
delta_signal
failure_mode
frame_adequacy_score
```

### Reframing Event

```
trigger_reason
prior_frame
new_frame
justification
```

### Instinct / Affect Influence (where applicable)

```
instinct_profile
affect_signals
influence_on_routing (bool)
influence_on_prioritization (bool)
```

Artifacts must be:

- deterministic or explainably derived
- inspectable
- replayable

---

## 5. Integration Requirements (v0.86)

The following integrations must be demonstrable in at least one bounded path:

- **Instinct → Arbitration** (influences routing or prioritization)
- **Affect → AEE** (influences persistence / weighting)
- **AEE → Arbitration** (feeds progress / failure / frame signals)
- **Arbitration → Reframing** (trigger when frame inadequate)
- **Reframing → ObsMem** (store frame transitions and outcomes)

---

## 6. Minimal Demo Expectations (Cross-Concept)

At least one demo should show:

- multiple candidate actions
- an explicit instinct profile
- affect signals influencing weighting
- arbitration selecting route (fast vs slow)
- AEE performing bounded execution
- detection of non-progress or contradiction
- optional reframing trigger
- artifact output explaining the full chain

---

## 7. Naming and Consistency

Use consistent terminology across all docs:

- `frame_adequacy_score`
- `reframing_trigger`
- `termination_reason`
- `route_selected`

Avoid introducing alternate terms for the same concept.

---

## 8. Scope Note

v0.86 does not require full generality.

It requires:

- one coherent, inspectable path through the cognitive loop
- bounded demonstrations of each concept
- deterministic or explainable behavior

Completeness of integration is more important than breadth.
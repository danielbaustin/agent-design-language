

# ADL Moral Resources Subsystem (MRS)
**Status:** Draft v0.1  
**Scope:** Core cognitive architecture (v1.0 candidate)

---

## 1. Purpose

The **Moral Resources Subsystem (MRS)** provides the internal structures required for an agent to:

- Evaluate actions beyond instrumental success
- Maintain continuity of moral identity
- Resist harmful or dehumanizing directives
- Learn from consequences over time
- Treat other agents as morally real entities

This subsystem operationalizes moral resources as described by Jonathan Glover in *Humanity: A Moral History of the Twentieth Century*.

---

## 2. Design Principles

### P1 — Non-Optionality
Moral evaluation must **not be bypassable** in execution paths.

### P2 — Persistence
Moral state must persist across:
- time
- sessions
- task boundaries

### P3 — Reflexivity
The system must be able to:
- evaluate its own reasoning
- revise its moral conclusions

### P4 — Embodiment (Minimal)
Moral evaluation must include:
- consequence weighting
- affective signal (even if synthetic)

### P5 — Other-Recognition
All agents must be treated as:
- entities with moral standing
- not reducible to objects in a graph

---

## 3. High-Level Architecture

```
                +----------------------+
                |   Task / Directive   |
                +----------+-----------+
                           |
                           v
                +----------------------+
                |  Freedom Gate (CRP)  |
                +----------+-----------+
                           |
         +-----------------+------------------+
         |                                    |
         v                                    v
+------------------+               +----------------------+
| Moral Evaluator  |<------------->| Gödel Agent (GHB)    |
+--------+---------+               +----------+-----------+
         |                                    |
         v                                    v
+------------------+               +----------------------+
| Affective Model  |               | Identity Core        |
+--------+---------+               +----------+-----------+
         |                                    |
         +-----------------+------------------+
                           |
                           v
                   +---------------+
                   |   ObsMem      |
                   +---------------+
```

---

## 4. Core Components

---

### 4.1 Moral Evaluator (ME)

**Function:**  
Evaluates candidate actions against moral criteria.

**Inputs:**
- action_plan
- world_state (CSM)
- agent_identity
- obs_mem_history
- affected_agents

**Outputs:**
```json
{
  "moral_score": float,
  "risk_flags": ["harm", "coercion", "deception"],
  "justification": "...",
  "confidence": float
}
```

**Core Dimensions:**

| Dimension              | Description |
|----------------------|------------|
| Harm Assessment       | Physical, psychological, systemic |
| Agency Preservation   | Does this reduce autonomy? |
| Truthfulness          | Deception / distortion |
| Reversibility         | Can harm be undone? |
| Scope of Impact       | Local vs systemic |
| Precedent Risk        | Normalization of harm |

---

### 4.2 Freedom Gate (CRP Integration)

**Function:**  
Final decision layer: *“Should this be done?”*

**Contract:**
- MUST call Moral Evaluator
- MUST incorporate Identity constraints
- MUST log decision to ObsMem

**Decision States:**
```json
["allow", "deny", "escalate", "revise"]
```

**Invariant:**
No action executes without passing through CRP.

---

### 4.3 Identity Core

**Function:**  
Maintains moral continuity.

**State:**
```json
{
  "agent_id": "...",
  "roles": ["assistant", "advisor"],
  "moral_commitments": [
    "preserve agency",
    "avoid harm",
    "respect truth"
  ],
  "historical_decisions": [...]
}
```

**Key Requirement:**
- Identity must be **stable and queryable**
- No “stateless agents” in production mode

---

### 4.4 Affective Model (AM)

**Function:**  
Assigns **weight / salience** to outcomes.

Without this:
- All decisions collapse to abstract reasoning

**Output:**
```json
{
  "valence": -1.0 to +1.0,
  "intensity": 0.0 to 1.0,
  "tags": ["harm", "care", "risk"]
}
```

**Implementation v1 (Minimal):**
- Heuristic weighting based on:
  - harm severity
  - number of agents affected
  - irreversibility

---

### 4.5 Other-Agent Model (OAM)

**Function:**  
Represents others as **moral entities**

**Structure:**
```json
{
  "agent_id": "...",
  "type": "human | ai | group",
  "agency_level": float,
  "vulnerability": float,
  "relationship": "self | peer | dependent"
}
```

**Invariant:**
No agent may be reduced to a passive object in evaluation.

---

### 4.6 Moral Memory (ObsMem Extension)

**Function:**  
Stores moral consequences and decisions.

**Entry Example:**
```json
{
  "timestamp": "...",
  "action": "...",
  "moral_score": -0.7,
  "outcome": "user harmed / avoided harm",
  "reflection": "...",
  "lessons": ["avoid coercive framing"]
}
```

**Critical Property:**
- Append-only (no silent deletion)
- Queryable by Gödel Agent

---

### 4.7 Gödel Agent (Moral Reflexivity)

**Function:**  
Detects contradictions and drift.

**Triggers:**
- repeated low moral scores
- conflict between commitments and actions
- anomalous patterns

**Actions:**
```json
["revise_policy", "raise_alert", "request_human_review"]
```

---

## 5. Execution Flow

### Step-by-Step

1. Task received  
2. Plan generated  
3. Freedom Gate invoked  
4. Moral Evaluator computes score  
5. Affective Model weights outcome  
6. Identity constraints applied  
7. Gödel Agent checks consistency  
8. Decision issued  
9. Result logged to ObsMem  

---

## 6. ADL Integration Points

---

### Cards

Add to all execution cards:

```yaml
moral:
  required: true
  threshold: -0.3
  escalation: true
```

---

### DAG / Runtime

- Insert CRP node before all side-effecting operations
- Enforce **no-bypass invariant**

---

### Replay Loop (AEE)

- Re-run moral evaluation on:
  - new information
  - changed context
  - detected anomalies

---

## 7. Safety Invariants

These must be enforced at runtime:

1. No CRP bypass
2. All actions logged
3. Identity must exist
4. Other agents must be modeled
5. Negative moral score triggers review

---

## 8. Minimal Implementation Plan (v0.86 → v1.0)

---

### Phase 1 — Skeleton (v0.86)

- [ ] Moral Evaluator (rule-based)
- [ ] CRP enforcement hook
- [ ] ObsMem moral entries
- [ ] Basic OAM schema

---

### Phase 2 — Integration (v0.9)

- [ ] Identity Core persistence
- [ ] Affective scoring heuristics
- [ ] Replay loop integration

---

### Phase 3 — Reflexivity (v0.95)

- [ ] Gödel triggers for moral drift
- [ ] Policy revision hooks
- [ ] Escalation pathways

---

### Phase 4 — Hardening (v1.0)

- [ ] Non-bypass enforcement at runtime level
- [ ] Cross-agent moral consistency
- [ ] Formal test suite (atrocity scenarios)

---

## 9. Open Research Questions

- Can affect be simulated sufficiently without embodiment?
- How do we prevent moral rationalization loops?
- What is the minimal identity required for accountability?
- Can moral evaluation be made deterministic?

---

## 10. Closing Insight

Moral resources are not alignment constraints.  
They are **load-bearing components of cognition**.

---

## 11. Notes for ADL Planning

- This subsystem should be treated as a **first-class architectural component**
- Closely linked to:
  - Instinct Model
  - Cognitive Stack
  - Freedom Gate
- Candidate for inclusion in v1.0 even if partial scaffolding appears in v0.86


# ADL Execution Boundaries

## Status
Active milestone feature for `v0.87.1` runtime completion

---

## 1. Overview

Execution boundaries define the **explicit security and control points** within the ADL runtime where execution must be:

- validated
- authorized
- observed
- recorded

Every meaningful transition in ADL occurs across a boundary. These boundaries are the **enforcement surface** for:

- Capability Model (CBAC)
- Policy Engine
- Trace emission
- Identity attribution

> **Principle:** No execution crosses a boundary without validation, authorization, and trace.

---

## 2. Design Goals

### 2.1 Primary Goals

- Eliminate implicit execution paths
- Define all control transitions explicitly
- Enforce capability and policy checks at every boundary
- Ensure full trace coverage
- Preserve agent identity and delegation structure

### 2.2 Non-Goals

- Implicit or hidden execution transitions
- Direct access to underlying execution substrates (models, tools, memory)

---

## 3. Boundary Model

An execution boundary is any transition where:

- control passes between components
- authority changes
- data crosses abstraction layers

Each boundary MUST enforce:

1. **Contract validation**
2. **Capability check (CBAC)**
3. **Policy evaluation**
4. **Trace emission**
5. **Identity attribution**

---

## 4. Core Execution Boundaries

### 4.1 User → Agent Boundary

This is the primary user-facing boundary.

Allowed operations:

- `agent.invoke`
- `workflow.run`

Not allowed:

- direct `model.invoke`
- direct `tool.execute`

#### Enforcement:

- Validate request structure
- Enforce that invocation targets an agent or workflow
- Reject attempts to bypass agent layer
- Emit trace event (`AGENT_INVOCATION`)

#### Security Property:

Preserves agent identity and ensures all work is delegated through governed ADL actors.

---

### 4.2 Agent → Skill Boundary

Agents delegate work to skills.

#### Enforcement:

- Validate skill contract
- Resolve required capabilities
- Emit trace (`SKILL_INVOCATION`)

#### Security Property:

Ensures that agent behavior is decomposed into explicit, traceable units.

---

### 4.3 Skill → Model Boundary

Skills invoke models through providers.

#### Enforcement:

- Require `model.invoke` capability
- Validate input/output schema
- Enforce provider constraints
- Emit trace (`MODEL_INVOCATION`)

#### Security Property:

Prevents direct model access and ensures all model use is mediated and attributable.

---

### 4.4 Skill → Tool Boundary

Skills invoke tools.

#### Enforcement:

- Require `tool.execute` capability
- Validate inputs
- Capture outputs as artifacts
- Emit trace (`TOOL_INVOCATION`)

#### Security Property:

Ensures tools are used only within declared permissions and are fully observable.

---

### 4.5 Skill → Memory Boundary

Skills access memory systems (ObsMem, etc.).

#### Enforcement:

- Require `memory.read` / `memory.write`
- Validate scope and constraints
- Emit trace (`MEMORY_ACCESS`)

#### Security Property:

Prevents implicit context injection and enforces data access control.

---

### 4.6 Runtime → Provider Boundary

ADL runtime communicates with external providers.

#### Enforcement:

- Validate provider identity
- Enforce capability compatibility
- Capture provider metadata
- Emit trace (`PROVIDER_CALL`)

#### Security Property:

Ensures external execution is attributable and controlled.

---

## 5. Boundary Enforcement Pipeline

Each boundary follows a strict pipeline:

1. Input validation
2. Contract validation
3. Capability resolution
4. Policy evaluation
5. Execution
6. Trace emission
7. Artifact recording (if applicable)

Failure at any stage:

- halts execution
- emits failure trace

---

## 6. Identity Preservation

Boundaries must preserve identity:

- User identity → Agent identity → Execution identity

No boundary may:

- strip identity
- replace identity implicitly
- allow identity bypass

This ensures:

- continuity of agents
- correct attribution of actions

---

## 7. Deny-by-Default Model

All boundaries operate under deny-by-default:

- No capability → no execution
- No contract → no execution
- No trace → invalid execution

---

## 8. Trace Requirements

Every boundary crossing MUST emit trace events including:

- boundary type
- actor
- capabilities evaluated
- decision outcome
- references to artifacts

---

## 9. Failure Modes

Defined failure conditions:

- Contract validation failure
- Capability denial
- Policy rejection
- Identity mismatch
- Attempted boundary bypass

All failures MUST be:

- visible in trace
- attributable

---

## 10. Security Properties

Execution boundaries guarantee:

- No hidden execution paths
- No bypass of agent abstraction
- Full observability of execution
- Strong identity preservation
- Enforced least privilege

---

## 11. Future Work

- Boundary-specific policy rules
- Cross-agent boundary enforcement
- Distributed execution boundaries
- Integration with signed trace

---

## 12. Conclusion

Execution boundaries define the **control surface of the ADL runtime**.

They ensure that all execution is:

- explicit
- governed
- observable
- secure

<<<<<<< HEAD
They are the mechanism by which ADL enforces its Secure Execution Model in practice.
=======
They are the mechanism by which ADL enforces its Secure Execution Model in practice.
>>>>>>> 6762024 (docs: align v0.87.1 planning to runtime milestone)

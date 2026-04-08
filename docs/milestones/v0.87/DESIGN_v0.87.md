# Design: v0.87 Coherent Cognitive Substrate

## Metadata
- Milestone: `v0.87`
- Version: `0.87`
- Date: `2026`
- Owner: `adl`
- Related issues: `#1292-#1302`, `#1345-#1350`, `#1354`

## Purpose
Define the integrated design for v0.87 as the milestone that **consolidates, aligns, and stabilizes** the ADL cognitive substrate following the expansion of v0.86.

This milestone ensures that all major system surfaces—contracts, execution, review, trace, and documentation—form a **coherent, deterministic, and externally credible system**.

v0.87 is not about introducing new cognitive capabilities. It is about making the existing system **correct, consistent, inspectable, and trustworthy**.

---

## Problem Statement

v0.86 successfully introduced a bounded cognitive system with:
- cognitive loop
- arbitration
- fast/slow reasoning
- Freedom Gate
- evaluation and bounded execution

However, the system now risks:

- divergence between docs and implementation
- inconsistent contract enforcement
- partial or informal review outputs
- tooling fragility (worktrees, PR flows, automation boundaries)
- lack of system-level coherence across surfaces

Without consolidation, ADL risks becoming:
- powerful but unreliable
- expressive but inconsistent
- difficult to evaluate externally

v0.87 addresses this by defining a **coherent substrate design** that aligns all system surfaces.

---

## Goals

- Ensure **end-to-end system coherence** across execution, contracts, trace, and review
- Strengthen **contract and validation integrity**
- Formalize **review and verification surfaces** as structured outputs
- Align **canonical documentation with actual system behavior**
- Stabilize **tooling and developer workflows**
- Eliminate inconsistencies and undefined behavior
- Prepare the system for **external (3rd party) evaluation**

## Non-Goals

- Introducing new cognitive subsystems (Gödel agent, GHB loop, identity)
- Expanding affect or instinct models
- Implementing full ObsMem or reasoning graph systems
- Implementing delegation or Freedom Gate v2
- Expanding scope beyond v0.86 feature surface

## Scope
### In scope
- system-wide contract alignment
- validation consistency across inputs/outputs
- structured review surfaces and outputs
- trace alignment with actual execution
- canonical documentation updates
- tooling stabilization (PR flow, worktrees, automation)
- system-level coherence across all surfaces

### Out of scope
- new cognitive architecture components
- identity and persistence systems
- delegation frameworks
- advanced reasoning graph systems

## Requirements
### Functional
- Define one coherent execution truth across contracts, execution, trace, review, and documentation.
- Standardize review outputs so findings, system-level assessment, and action plans are structurally consistent.
- Make trace sufficient to reconstruct actual major control decisions.
- Tighten validation so strict and loose modes are explicit, bounded, and predictable.
- Canonicalize documentation so milestone docs describe implemented behavior truthfully.
- Stabilize tooling behavior across PR flows, worktrees, and automation boundaries.

### Non-functional
- Deterministic behavior and reproducible outputs.
- Clear failure semantics and observability.
- Tooling and review surfaces must be reliable enough for credible external evaluation.

## Proposed Design
### Overview

v0.87 defines ADL as a **closed-loop, deterministic, inspectable system**:

contracts → execution → trace → review → documentation

All five surfaces must agree.

No surface is allowed to diverge silently.

### 1. System Coherence Model

All system components must align along a single execution truth.

#### Requirement
Every system surface must reflect the same underlying behavior:
- contracts define allowed structure
- execution follows contracts
- trace records actual execution
- review evaluates trace + behavior
- documentation describes actual behavior

#### Constraint
No component may:
- invent behavior not present in execution
- omit behavior that materially affects correctness
- reinterpret contracts inconsistently

### 2. Contract and Validation Layer

#### Objective
Make contracts **strict, consistent, and reliable**.

#### Design
- All inputs and outputs must pass through validation
- Unknown or undefined fields must be rejected in strict mode
- Loose mode must be explicitly defined and bounded
- Error reporting must be structured and deterministic

#### Required outputs
Validation failures must include:
- location
- reason
- expected vs actual

#### Constraint
Contracts are the **source of truth**, not optional hints.

### 3. Execution and Trace Alignment

#### Objective
Ensure trace reflects **actual execution**, not approximations.

#### Design
- Every major control decision must produce a trace event
- Trace must include:
  - inputs
  - decisions (arbitration, routing, gate)
  - outputs
- Trace must be ordered and reproducible

#### Constraint
Trace must be sufficient to:
- reconstruct system behavior
- support review and debugging

### 4. Review and Verification Surface

#### Objective
Define review as a **first-class system output**, not an ad hoc process.

#### Required structure
Each review must include:

**Findings**
- Severity (P1–P4)
- Location
- Description
- Impact
- Trigger
- Evidence
- Fix Direction

**System-Level Assessment**
- dominant risk themes
- clustering of issues
- system maturity implications

**Recommended Action Plan**
- fix now
- fix before milestone close
- defer to future milestone

#### Additional requirements
- must analyze trust boundaries
- must detect test misalignment
- must explain failure modes causally

### 5. Documentation Canonicalization

#### Objective
Ensure documentation is a **reliable interface to the system**.

#### Design
- canonical docs must match implementation
- remove duplicate or outdated docs
- enforce consistent structure across milestones
- link design ↔ implementation ↔ demos

#### Constraint
Documentation must not:
- describe unimplemented behavior as real
- contradict runtime behavior

### 6. Tooling and Workflow Stability

#### Objective
Stabilize development workflow for reliable iteration.

#### Design
- standardize PR tooling behavior (`pr.sh` flows)
- ensure worktree-safe operations
- reduce environment-sensitive failures
- enforce predictable automation boundaries

#### Constraint
Tooling must be:
- reproducible
- deterministic
- minimally surprising

### Interfaces / Data contracts

**Validation Output**
- field
- error_type
- expected
- actual

**Trace Event**
- event_type
- timestamp
- inputs
- decision
- outputs

**Review Output**
- findings[]
- system_assessment
- action_plan

### Execution semantics

1. Inputs enter through explicit validated contract boundaries.
2. Execution proceeds according to canonical runtime behavior.
3. Major control decisions emit trace events.
4. Review consumes code, behavior, and trace/proof surfaces to produce structured findings.
5. Documentation is reconciled against implemented truth and validated proof surfaces.
6. Tooling and workflow surfaces preserve reproducibility across daily development and milestone closeout.

## Risks and Mitigations
- Risk: hidden inconsistencies remain across surfaces.
  - Mitigation: enforce trace + review + doc alignment against implemented truth.
- Risk: contracts become too rigid for practical use.
  - Mitigation: keep strict vs loose modes explicit and bounded.
- Risk: review becomes superficial or formulaic.
  - Mitigation: require structured findings plus causal explanations and system-level synthesis.
- Risk: tooling instability blocks progress or undermines trust.
  - Mitigation: prioritize reproducibility, worktree safety, and simple automation boundaries.

## Alternatives Considered
- Option: continue capability expansion immediately after v0.86.
  - Tradeoff: would increase feature surface area while leaving correctness, review, and tooling drift unresolved.
- Option: do a docs-only cleanup milestone.
  - Tradeoff: would improve language but fail to strengthen the actual contract, trace, review, and tooling substrate.

## Validation Plan
- Checks/tests:
  - verify contracts reject invalid inputs deterministically
  - verify trace captures all major decisions needed for reconstruction
  - verify review outputs follow the required structure
  - verify documentation matches runtime behavior and proof surfaces
  - verify tooling works consistently across environments and worktrees
- Success metrics:
  - system surfaces are aligned (contracts, execution, trace, review, docs)
  - validation is consistent and reliable
  - trace is complete and reconstructable
  - review is structured and actionable
  - documentation is accurate and canonical
  - tooling is stable and predictable
- Rollback/fallback:
  - if full cross-surface convergence is not achieved, preserve a truthful reduced v0.87 scope that still improves contract integrity, review structure, and tooling stability without claiming full substrate coherence.

## Exit Criteria
- All system surfaces are consistent and mutually reinforcing.
- No known contradictions remain between docs and implementation.
- Review and validation are structured and enforced.
- Tooling is stable for daily development use.
- The system is ready for credible external evaluation.

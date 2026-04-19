# ADL Runtime v2 — Invariants

## Status
Historical source draft - context only in the v0.90.1 tracked package

## Purpose

Define the **non-negotiable invariants** of ADL Runtime v2 (Gödel Agent Land).

These invariants are not implementation details.
They are the conditions under which the system remains:

- coherent
- replayable
- migratable
- governable
- identity-preserving

> If an invariant is violated, the system is no longer ADL Runtime v2.

---

## Core Principle

> **ADL Runtime v2 is deterministically reconstructable, not strictly deterministic.**

The system must allow:

- bounded variance in execution (e.g., model differences)
- BUT full reconstruction of:
  - what happened
  - why it happened
  - what alternatives existed
  - what was chosen

---

## Invariant Classes

1. Manifold invariants (world integrity)
2. Citizen invariants (identity continuity)
3. Temporal invariants (chronosense integrity)
4. Trace invariants (observability)
5. Migration invariants (portability)
6. Governance invariants (Freedom Gate authority)

---

# 1. Manifold Invariants

## M-1: Single Authoritative Timeline

The manifold MUST maintain a single, authoritative event ordering.

- events are strictly ordered (monotonic index)
- wall-clock time is secondary to ordering

No conflicting timelines may exist.

---

## M-2: No Orphan State

Every state mutation MUST be causally linked to an event.

- no silent mutation
- no hidden state transitions

All state must be explainable via trace.

---

## M-3: Replay Sufficiency

A sealed snapshot MUST contain sufficient information to:

- reconstruct the manifold
- restore all citizens
- resume execution

If replay cannot reconstruct behavior, the system is invalid.

---

## M-4: World Persistence

The manifold MUST survive beyond any episode.

- episodes may terminate
- manifold must not

---

# 2. Citizen Invariants

## C-1: Identity Continuity

Each citizen MUST maintain a stable identity across:

- time
- sleep/wake cycles
- migration

Identity must not fork.

---

## C-2: No Duplicate Activation

A citizen MUST NOT exist in multiple active instances simultaneously.

- no double-wake
- no split-brain identity

---

## C-3: Memory Persistence

Citizen memory MUST persist across:

- episodes
- sleep
- migration

Memory loss must be explicit, not accidental.

---

## C-4: Obligation Continuity

Commitments MUST survive:

- sleep
- migration

A citizen cannot silently drop obligations.

---

## C-5: Capability Envelope Declaration

Each citizen MUST operate within an explicit capability envelope.

- model
- tools
- limits

Changes must be recorded.

---

# 3. Temporal Invariants (Chronosense)

## T-1: Temporal Anchoring

Every event MUST include:

- monotonic order
- UTC timestamp
- agent-relative age (optional but recommended)

---

## T-2: Monotonic Ordering

Event ordering MUST be strictly increasing.

No time regression allowed.

---

## T-3: Temporal Honesty

The system MUST distinguish:

- known time
- inferred time
- unknown time

No fabricated certainty.

---

## T-4: Causal Before/After Integrity

If event B depends on event A, then:

- A must precede B in ordering

---

# 4. Trace Invariants

## TR-1: Full Observability

All meaningful actions MUST be represented in trace.

- no hidden execution

---

## TR-2: Decision Visibility

Every committed action MUST include a Freedom Gate event.

Trace must show:

- alternatives
- selection
- commitment

---

## TR-3: Causal Reconstruction

Given trace, a reviewer MUST be able to reconstruct:

- reasoning path
- decision points
- outcomes

---

## TR-4: Bounded Completeness

Trace need not capture all tokens, but MUST capture:

- decisions
- state transitions
- policy interactions

---

# 5. Migration Invariants

## MIG-1: Snapshot Integrity

A sealed snapshot MUST be:

- complete
- internally consistent
- verifiable

---

## MIG-2: Identity Preservation

After migration:

- citizen identity remains identical
- no new identities are created implicitly

---

## MIG-3: Ordering Preservation

Event ordering MUST be preserved across migration.

---

## MIG-4: No Duplicate Worlds

Migration MUST NOT produce:

- two active instances of the same manifold

---

## MIG-5: Capability Rebinding Transparency

If capabilities change (e.g., model provider), the system MUST:

- record the change
- expose the difference

---

# 6. Governance Invariants (Freedom Gate)

## G-1: Mandatory Gate Crossing

All non-trivial actions MUST pass through the Freedom Gate.

---

## G-2: Alternative Existence

Every decision MUST involve:

- at least one alternative

No blind execution.

---

## G-3: Commitment Recording

All commitments MUST be recorded as events.

---

## G-4: Policy Supremacy

Policy MUST override:

- instinct
- affect
- arbitration

---

## G-5: Refusal Capability

The system MUST be able to:

- refuse
- defer
- escalate

---

# Cross-Cutting Invariant

## X-1: No Silent Drift

The system MUST NOT change behavior due to:

- hidden state
- undocumented mutation
- implicit environment change

All changes must be:

- observable
- attributable

---

# Failure Modes (Explicit)

The following indicate invariant violation:

- agent exists without recoverable identity
- state changes without trace
- migration produces duplicate agents
- decisions occur without Freedom Gate record
- time ordering becomes ambiguous
- replay cannot reconstruct prior state

---

# Invariant Enforcement Model

## Purpose

Define how the runtime **detects**, **classifies**, and **responds** to invariant violations.

Invariants without enforcement are advisory.  
Runtime v2 requires **continuous, automatic enforcement**.

---

## EM-1: Detection

Every invariant MUST have at least one detection mechanism.

Detection sources:

- kernel guards (authoritative checks)
- trace validators (post-event verification)
- periodic auditors (background scans)
- resident staff agents (semantic / policy checks)

Example:

```yaml
violation:
  type: duplicate_citizen_activation
  detected_by: kernel.identity_guard
  evidence:
    citizen_id: ga-004
    active_instances: [node-A, node-B]
```

---

## EM-2: Classification

Violations MUST be classified by severity.

Levels:

- `warning` — non-critical inconsistency
- `recoverable` — system can self-correct safely
- `critical` — must halt or quarantine
- `fatal` — manifold integrity compromised

---

## EM-3: Response

Each violation class MUST map to a deterministic response policy.

Possible responses:

- log only
- emit alert
- quarantine citizen
- halt episode
- freeze manifold
- force rollback to snapshot
- escalate to staff agents

Example:

```yaml
response_policy:
  violation: duplicate_citizen_activation
  severity: critical
  action:
    - freeze_manifold
    - select_primary_instance
    - terminate_secondary_instances
    - record_resolution_event
```

---

## EM-4: Recovery

The system MUST define recovery paths for all recoverable violations.

Recovery mechanisms:

- rollback to last valid snapshot
- reconcile divergent traces
- rehydrate canonical state
- re-evaluate pending commitments

---

## EM-5: Attribution

Every violation MUST be attributable.

Trace MUST include:

- origin event
- responsible agent or subsystem
- contributing conditions

No anonymous failure.

---

## EM-6: Enforcement Surfaces

Enforcement is distributed across runtime layers:

- **kernel** — hard invariants (identity, ordering, duplication)
- **trace system** — causal and completeness validation
- **Freedom Gate** — policy and decision integrity
- **resident staff agents** — higher-order reasoning and anomaly detection

---

## EM-7: Continuous Verification

Invariant enforcement MUST be continuous:

- at event commit time (synchronous)
- during runtime (asynchronous auditing)
- during snapshot/seal
- during rehydration

---

## EM-8: No Silent Recovery

All recovery actions MUST be recorded as events.

The system may self-heal, but it must never:

- hide the violation
- erase evidence

---

## EM-9: Violation Artifacts

Every violation MUST produce a structured artifact:

- violation type
- severity
- evidence
- response taken
- recovery outcome

These artifacts are part of the **world trace**.

---

## Summary of Enforcement Model

The runtime must:

- detect violations
- classify them
- respond deterministically
- recover when possible
- record everything

> **If invariants define the laws, the enforcement model is the physics engine that enforces them.**

---

# Invariant Ownership Model

## Purpose

Define **ownership and guardianship** for each invariant.

Invariants must not only be enforced — they must be **actively protected** by designated runtime components.

---

## IO-1: Explicit Ownership

Every invariant MUST declare a primary owner.

Example:

```yaml
invariant:
  id: C-2
  name: no_duplicate_activation
  owner: kernel.identity_guard
  backup_owner: staff.identity_auditor
```

Owners are responsible for:

- detection
- enforcement
- escalation

---

## IO-2: Dual-Layer Guardianship

Each invariant SHOULD have:

- **primary owner** (kernel or system component)
- **secondary owner** (resident staff agent)

This provides:

- fast enforcement (kernel)
- semantic validation (agents)

---

## IO-3: Ownership Domains

Ownership maps to invariant classes:

- **Manifold invariants** → kernel.core / trace.system
- **Citizen invariants** → kernel.identity_guard / staff.identity_auditor
- **Temporal invariants** → kernel.clock / trace.validator
- **Trace invariants** → trace.system / staff.archivist
- **Migration invariants** → kernel.migration / staff.migration_marshal
- **Governance invariants** → Freedom Gate / staff.reasonableness_agent

---

## IO-4: Escalation Responsibility

If an invariant cannot be enforced automatically, the owner MUST:

- escalate to resident staff
- emit violation artifact
- trigger containment or freeze if required

---

## IO-5: Ownership Visibility

Invariant ownership MUST be discoverable.

The system should allow queries like:

- "who enforces C-2?"
- "which invariants does this component own?"

---

## IO-6: No Unowned Invariants

Every invariant MUST have at least one owner.

Unowned invariants are invalid and must be rejected at design time.

---

## IO-7: Ownership Persistence

Ownership relationships MUST persist across:

- runtime restarts
- sleep/wake cycles
- migration

Ownership is part of the **manifold definition**, not runtime state.

---

## Summary of Ownership Model

The runtime must ensure:

- every invariant has a guardian
- enforcement responsibility is explicit
- ownership persists across time

> **If enforcement is the physics engine, ownership defines who keeps the laws intact.**

---

# Summary

ADL Runtime v2 is defined not by features, but by invariants.

If these invariants hold:

- agents have continuity
- the world is coherent
- behavior is explainable
- migration is safe

If they do not:

> **There is no cognitive spacetime—only execution fragments.**

---

# Next Steps

- map invariants → schema
- map invariants → runtime enforcement
- build invariant test suite
- define violation detection and recovery

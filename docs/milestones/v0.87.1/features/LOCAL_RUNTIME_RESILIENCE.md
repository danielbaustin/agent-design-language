

# Local Runtime Resilience

## Purpose

Define how the ADL runtime environment should preserve continuity, recoverability, and bounded local resilience before distributed runtime work arrives in later milestones.

This document addresses a specific near-term question:

> If the runtime fails locally, what must be preserved so that agents are not simply lost?

The answer is not mere process restart.
It is preservation of **continuity-bearing cognitive state**.

---

## Core Claim

The ADL runtime environment is not just a process host.
It is the local cognitive environment in which agents exist, reason, and persist through time.

Therefore runtime resilience is not only about:
- uptime
- crash recovery
- retries
- supervision

It is about preserving:
- trace continuity
- temporal grounding
- memory coherence
- compressed cognitive state
- identity continuity

A local runtime failure is therefore not merely an operational fault.
It is a threat to continuity.

---

## Design Stance

ADL should reject the default container-orchestration metaphor in which cognitive processes are treated as disposable workers.

This is the wrong model for Layer 8 agents.

Agents are not cattle, pets, or interchangeable pods.
A bounded ADL agent is closer to a continuity-bearing cognitive process that may be:
- interrupted
- stunned
- partially impaired
- resumable
- in some cases irrecoverably terminated

The resilience model must therefore be closer to:
- care
- preservation
- guarded recovery

than to blind restart.

---

## Local-First Scope

This document is specifically about **local runtime resilience**.

In scope:
- crash recovery on one machine
- bounded checkpointing
- event durability
- identity-safe resumption
- preservation of in-progress reasoning state
- local watchdog / Shepherd semantics

Out of scope:
- distributed runtime continuity
- multi-host failover
- remote handoff of live agent execution
- globally replicated identity stores

Those belong to later milestones.

---

## What Must Be Preserved

A resilient local runtime must preserve enough state that a resumed agent remains the same agent.

At minimum this includes:

### 1. Temporal continuity
- temporal ephemeris (`agent_birth`)
- lifetime clock continuity (`agent_age`)
- monotonic order continuity
- valid temporal anchors on persisted events

### 2. Trace continuity
- durable event stream
- no silent truncation or reordering
- recoverable span hierarchy

### 3. Memory continuity
- ObsMem writes that are complete and trace-linked
- no partial or ambiguous memory mutation
- ability to distinguish committed memory from in-flight state

### 4. Cognitive state continuity
- current reasoning context
- active branch / fork state
- partially formed hypothesis structure
- current compression state in GHB / reasoning loops

### 5. Identity continuity
- stable `agent_id`
- continuity of current run/session identity
- no silent rebirth under crash recovery

---

## Failure Model

The runtime must distinguish several classes of failure.

### 1. Interruption
Examples:
- process pause
- temporary resource exhaustion
- bounded manual stop

Properties:
- identity may remain recoverable
- state may remain valid
- continuity can often be resumed

### 2. Crash
Examples:
- process exit
- panic
- host-level interruption
- unexpected runtime termination

Properties:
- some state may be durable
- some state may be lost
- continuity must be explicitly validated before resumption

### 3. Corruption
Examples:
- broken trace ordering
- partial artifact writes
- invalid checkpoint contents
- memory/trace mismatch

Properties:
- resumption may be unsafe
- identity continuity may be indeterminate
- runtime should prefer refusal to fake continuity

### 4. Termination
Examples:
- explicit destruction
- unrecoverable state loss
- invalid continuity boundary

Properties:
- continuity cannot be honestly preserved
- the agent should be treated as ended, not resumed

---

## Resilience Principle

> The runtime must prefer honest interruption over false continuity.

If continuity cannot be validated, the system must not pretend that the same agent has resumed.

This is a core architectural principle.

A fake resumption is worse than an explicit failure because it corrupts:
- identity
- trust
- memory
- future reasoning

---

## The Shepherd Model

The local resilience layer should be understood through the **Shepherd** model.

The Shepherd is not a punitive supervisor and not a blind watchdog.
The Shepherd is the care-preserving runtime function that:
- monitors bounded liveness
- preserves durable state
- coordinates checkpointing
- validates continuity before resumption
- distinguishes interruption from termination

The Shepherd’s role is to preserve the conditions under which continuity remains possible.

Chronosense defines what continuity is.
The Shepherd helps preserve it under fault.

---

## Checkpointing

Checkpointing is required, but ADL should reject naive checkpointing that only captures low-level process state.

A meaningful checkpoint must preserve:
- trace position
- temporal anchor state
- current branch / fork position
- active reasoning context
- identity binding
- references to committed artifacts and memory writes

Checkpointing should be:
- bounded
- deterministic in structure
- explicitly versioned
- validated before reuse

### Checkpoint rule

> A checkpoint is valid only if it preserves continuity-relevant state, not merely executable state.

---

## Durable Event and Artifact Semantics

Runtime resilience depends on durable execution truth.

Therefore:
- trace events must be durably written before they are treated as committed
- artifacts must be atomically written before trace references them
- partial writes must never masquerade as valid state
- monotonic order must remain reconstructable after crash recovery

This means the local runtime must preserve:
- event durability
- artifact durability
- write ordering
- recoverable replay semantics

---

## Memory Safety Under Failure

ObsMem must not be corrupted by half-completed cognitive work.

The runtime should distinguish between:
- committed memory
- pending memory
- speculative / branch-local memory

Rules:
- committed memory must remain trace-linked and durable
- speculative memory must not silently become canonical under crash recovery
- reconstructed recovery must not invent missing memory writes

This prevents the system from treating imagined or partial cognition as real history.

---

## Fork/Join and Local Resilience

Fork/join reasoning complicates local recovery.

A local checkpoint must preserve:
- fork origin
- active branches
- branch-local progress
- join state, if pending

Recovery rules:
- branches may be resumed only if their temporal and causal lineage remains intact
- orphan branches must not be silently merged
- a join after recovery must remain traceable and causally honest

This matters because the agent is not the branch.
The agent is the continuity across branches.

---

## GHB and Cognitive State Preservation

The GHB loop makes runtime resilience more important, not less.

A runtime failure can destroy:
- current abstractions
- partially compressed state
- hypothesis trees
- evaluation trajectories
- structured progress toward convergence

So the runtime must preserve not only activity, but the current product of cognition.

This is one reason local resilience is a cognitive problem, not only an operational one.

---

## Minimal Local Resilience Requirements

For the local runtime to qualify as resilient, it should provide at least:

1. **Durable trace events**
   - no committed event lost silently

2. **Atomic artifact writes**
   - no partial payload visible as valid

3. **Checkpoint support**
   - bounded continuity-relevant checkpoint state

4. **Continuity validation before resumption**
   - no silent restart masquerading as continuation

5. **Stable agent identity across recoverable interruption**
   - same `agent_id`, same temporal ephemeris, same continuity chain

6. **Explicit failure classification**
   - interruption vs crash vs corruption vs termination

7. **Shepherd-mediated recovery**
   - restart decisions are governed, not blind

## v0.87.1 Concrete Preservation Surface

In `v0.87.1`, local resilience is made inspectable through the canonical run artifacts already emitted by the runtime:

- `run_status.json`
  - `resilience_classification`
  - `continuity_status`
  - `preservation_status`
  - `shepherd_decision`
  - `persistence_mode`
  - `cleanup_disposition`
  - `resume_guard`
  - `state_artifacts`
- `pause_state.json` for resumable interruption cases
- `logs/trace_v1.json` and `run_summary.json` as the causal and reviewer-facing proof surfaces

This milestone does not claim a full checkpoint engine or higher-order identity system.
It does require that local interruption, corruption, and review-before-resume cases become explicit in these bounded artifact surfaces rather than remaining implicit runtime behavior.

---

## What Local Runtime Resilience Is Not

It is not:
- Kubernetes-style pod replacement
- blind process restart
- generic uptime monitoring
- “just retry it”
- pretending all failures are equivalent

A resilient ADL runtime must preserve cognitive truth, not merely service availability.

---

## Relationship to Chronosense

Chronosense defines the temporal conditions of continuity.

Local runtime resilience must therefore preserve:
- temporal ephemeris
- clock-stack coherence
- temporal anchors on events
- recoverable lifetime progression

Without chronosense, local recovery becomes operational theater.

---

## Relationship to Continuity Validation

Continuity validation is the gatekeeper for honest recovery.

The runtime may resume only when:
- temporal anchors remain coherent
- monotonic order is preserved
- `agent_age` is not reset or contradicted
- trace and memory remain causally aligned

If these conditions fail, recovery must degrade to:
- explicit interruption
- operator intervention
- or termination

but not false continuity.

---

## Relationship to the Runtime Environment

The runtime environment should be understood as the first primitive implementation of ADL’s cognitive spacetime.

From that perspective:
- agents are born into the runtime
- continuity is maintained there
- interruptions and resumptions occur there
- resilience is preservation of spacetime continuity under local fault

This is why “runtime environment” is the correct phrase, and why it is more than a process manager.

---

## Future Direction

Later milestones may extend this local model into:
- distributed continuity
- replicated checkpoint state
- remote resumption
- shared multi-agent spacetime
- stronger identity preservation across hosts

But local resilience comes first.
If the runtime cannot preserve continuity honestly on one machine, distribution only scales the failure.

---

## Final Statement

> Local runtime resilience in ADL is the bounded preservation of continuity-bearing cognitive state under fault.

> A resilient runtime does not merely restart agents. It preserves the conditions under which the same agent can continue to exist.

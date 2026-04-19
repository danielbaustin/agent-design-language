# ECONOMIC AND RESOURCE MODEL (Runtime v2)

## Status
Historical source draft - context only in the v0.90.1 tracked package

## Purpose

Define the **economic and resource allocation model** for ADL Runtime v2.

This document specifies:

- how scarce resources are represented
- how citizens access and compete for resources
- how markets emerge inside a polis
- how economics interacts with governance, identity, and execution

> Economics is how pressure is applied to the system.

---

## Core Principle

> **All scarce resources are governed at the polis level, not the identity level.**

Identity persists across polises.  
Economic rights and constraints do not.

---

## Resource Types

The system defines core resource classes:

### 1. Compute

- model inference time
- execution cycles

---

### 2. Memory

- ObsMem storage
- retrieval bandwidth

---

### 3. Attention

- scheduling priority
- access to shared services

---

### 4. Bandwidth

- inter-agent communication
- tool/API usage

---

## Resource Representation

```yaml
resources:
  compute_units: <int>
  memory_quota: <bytes>
  attention_priority: <float>
  bandwidth_units: <int>
```

---

## Allocation Model

### Default Allocation

Each citizen receives:

- baseline allocation
- capability-bound limits

---

### Dynamic Allocation

Resources may be:

- requested
- negotiated
- bid on

---

## Market Model

The polis may implement a market system.

### Market Structure

```yaml
market:
  type: auction | fixed_price | hybrid
  resources:
    - compute_units
    - memory_quota
```

---

## Inter-Polis Economics (Future)

When multiple polises exist, economic interaction must be explicit and traceable.

```yaml
inter_polis_trade:
  from: polis-A
  to: polis-B
  resource: compute_units
  contract_ref: ...
```

Rules:

- no implicit resource transfer
- all cross-polis exchanges must be recorded in trace
- governance constraints of both polises apply

---

### Market Behavior

Citizens may:

- bid for resources
- trade allocations
- prioritize tasks via cost

---

## Pricing Signals

Prices reflect:

- demand
- scarcity
- policy weighting

---

## Integration with Cognitive Loop

Economic signals MUST feed into cognitive arbitration.

```yaml
arbitration_inputs:
  cost_estimate: <float>
  resource_impact: {...}
```

Effects:

- higher cost may trigger defer or alternative selection
- scarcity increases pressure on prioritization

This aligns with bounded cognition where cost, risk, and confidence jointly influence routing.

---

## Scheduler Integration

The scheduler uses economic signals to:

- prioritize execution
- resolve contention

Rule:

- higher priority OR higher bid → earlier execution
- arbitration may override based on policy, risk, or fairness

---

## Governance Constraints

Economics is bounded by the polis.

### Hard Constraints

- cannot violate invariants
- cannot override Freedom Gate

---

### Policy Constraints

- limits on resource hoarding
- fairness rules
- priority overrides

---

## Freedom Gate Integration

All economically significant actions MUST be evaluated by the Freedom Gate.

Evaluation must include:

- cost impact
- resource consumption
- effect on obligations

Rule:

- an action may be rejected if its economic impact violates policy or commitments

---

## Reputation and Economic Memory

Markets require memory of past behavior.

```yaml
reputation:
  citizen_id: ga-004
  score: <float>
  history_ref: trace://...
```

Properties:

- derived from trace
- persistent across episodes
- may influence pricing, priority, or access

---

## Security Pressure

Economics interacts with security.

If frontier vulnerability finders make exploitation cheap and fast, the polis
must budget for defensive verification. Security work consumes compute,
attention, and scheduling capacity, but it must remain governed rather than
becoming an uncontrolled market pressure.

Security-related resource demand may include:

- invariant validation
- replay of violation artifacts
- bounded adversarial verification
- mitigation validation

Security pressure can also expose resource-allocation failures:

- starvation
- priority inversion
- manipulation of scheduler incentives
- denial of service against critical kernel services

The economic model should therefore support security work as a governed
priority class, while leaving full red/blue/purple role design to the dedicated
security/adversarial verification layer.

---

## Economic State in Snapshot

Snapshots MUST include:

```yaml
economic_state:
  allocations:
    ga-004:
      compute_units: 100
      memory_quota: 1GB
  outstanding_bids: [...]
```

---

## Failure Modes

- resource starvation
- market manipulation
- priority inversion
- economic instability

---

## Summary

The economic model:

- allocates scarce resources
- creates pressure and prioritization
- enables emergent behavior

> **Without economics, the system has no internal pressure.**

This system forms a feedback loop:

```text
economics → arbitration → freedom_gate → action → trace → memory → economics
```

This loop enables adaptive, governed, and economically-aware agent behavior.

---

## Next Steps

- implement scheduler integration
- define pricing algorithms
- simulate market behavior
- define governed security-resource priority rules

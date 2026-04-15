

# Adversarial Execution Runner

## Metadata
- Project: `ADL`
- Status: `Draft`
- Owner: `Daniel Austin`
- Created: `2026-04-12`

---

## Purpose

Define the **Adversarial Execution Runner** as the orchestration surface that wires together:

- red / blue / purple roles
- security posture
- continuous verification loop
- exploit artifacts
- replay manifests
- mitigation and validation
- promotion to regression

The runner is the minimal executable surface that turns the adversarial subsystem into a real, repeatable workflow.

---

## Overview

The Adversarial Execution Runner is responsible for executing a bounded, policy-governed adversarial loop against a declared target.

It provides:

- a single entrypoint for adversarial runs
- consistent sequencing of lifecycle stages
- artifact production and linking
- trace emission and replay hooks
- posture enforcement via the Freedom Gate

---

## CLI Surface (Proposed)

```bash
adl adversarial run \
  --target <target_ref> \
  --posture <profile> \
  --goal <string> \
  --limit <n|time> \
  --output <path>
```

### Flags

- `--target` : target reference (id or path)
- `--posture` : named posture profile (audit|validation|hardening|internal_contest)
- `--goal` : optional focus (e.g., "input validation", "auth boundary")
- `--limit` : cap on iterations (count or duration)
- `--output` : artifact root directory

---

## Execution Flow

The runner executes the canonical loop:

```text
load target
-> declare posture
-> enumerate surfaces
-> generate hypothesis
-> attempt exploit (bounded)
-> record evidence
-> build replay manifest
-> (optional) run replay pre-fix
-> generate mitigation
-> (optional) apply bounded fix
-> run replay post-fix
-> classify and promote
-> emit artifacts + trace
```

Each step must be:

- bounded by posture
- attributable to a role (red/blue/purple)
- recorded in trace
- materialized as artifacts where applicable

---

## Minimal Implementation Slice (v0)

Start with a single-path, single-issue flow:

1. One target (demo fixture)
2. One hypothesis
3. One exploit attempt
4. One replay manifest
5. One mitigation
6. One post-fix replay
7. One promotion artifact

No parallelism, no scheduling-just a clear, linear proof path.

---

## Components

### 1. Target Loader

- resolves `target_ref`
- validates scope vs posture
- prepares environment (demo/sandbox)

### 2. Posture Enforcer

- loads posture profile
- enforces allowed actions
- integrates with Freedom Gate decisions

### 3. Red Engine

- surface enumeration
- hypothesis generation
- bounded exploit attempts

### 4. Blue Engine

- risk evaluation
- mitigation proposal
- (optional) bounded remediation

### 5. Replay Engine

- builds `AdversarialReplayManifest`
- executes replay (pre/post fix)
- evaluates success criteria

### 6. Artifact Manager

- creates and links artifacts
- enforces schema completeness
- writes to output tree

### 7. Trace Emitter

- records posture
- records steps and outcomes
- links artifacts to trace refs

### 8. Promotion Engine (Purple)

- classifies exploit
- decides promotion targets
- emits `ExploitPromotionArtifact`

---

## Artifact Output Layout

```text
<output>/
  target/
  posture.yaml
  hypothesis.yaml
  evidence.yaml
  replay.yaml
  mitigation.yaml
  replay_pre_fix/
  replay_post_fix/
  classification.yaml
  promotion.yaml
  trace/
```

All artifacts must include:
- `artifact_id`
- `artifact_type`
- `schema_version`
- `trace_refs`
- `target_ref`
- `security_posture_ref`

---

## Posture Enforcement Rules

- Block exploit attempts in `audit` posture
- Block mutation in `no_mutation` modes
- Restrict targets by `target_scope`
- Require evidence level by `evidence_requirement`
- Gate high-intensity modes behind approval

All violations must be:
- prevented or
- recorded explicitly as blocked actions

---

## Trace Requirements

Trace must include events for:

- posture declaration
- hypothesis generation
- exploit attempt (with outcome)
- replay execution (pre/post)
- mitigation decision
- promotion decision

Each event should include:
- timestamp
- agent role
- step id
- references to artifacts

---

## Success Criteria

A run is successful when:

- at least one hypothesis is generated
- at least one exploit attempt is recorded
- a replay manifest is produced
- replay executes with defined outcome
- mitigation is linked to evidence
- post-fix replay shows changed behavior (if fix applied)
- promotion artifact is emitted (or explicit defer recorded)

---

## Failure Modes

- No artifacts produced (prose-only run)
- Replay cannot be executed or explained
- Posture violations not enforced
- Mitigation not linked to evidence
- No before/after validation

These should fail the run or mark it `incomplete`.

---

## Demo Hook

Primary demo command:

```bash
adl adversarial run \
  --target demo.simple_api \
  --posture validation \
  --goal "input validation bypass" \
  --limit 1 \
  --output reports/adversarial-demo
```

Expected outcome:
- full artifact chain present
- replay works pre-fix
- replay fails post-fix

---

## Future Extensions

- parallel hypothesis exploration
- scheduling (continuous runs)
- posture escalation workflows
- multi-target campaigns
- provider capability gating
- signed trace integration
- CI integration for regression suites

---

## Conclusion

The Adversarial Execution Runner is the minimal bridge from design to reality.

It takes the adversarial architecture and makes it:

- runnable
- repeatable
- inspectable
- provable

It is the surface where contested cognition becomes engineering practice.

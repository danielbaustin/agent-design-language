# Adversarial Execution Runner

## Metadata
- Project: `ADL`
- Milestone: `v0.89.1`
- Status: `Implemented`
- Owner: `Daniel Austin`
- Updated: `2026-04-15`
- WP: `WP-04`

---

## Purpose

Make the adversarial runner explicit as a bounded orchestration and evidence-capture contract instead of leaving it as a proposed CLI sketch.

The core shift is simple:

> ADL adversarial execution must have a declared target, posture, stage order, role attribution, evidence chain, and defer points.

`WP-04` does not yet implement exploit schemas, strict replay manifests, continuous self-attack scheduling, or the flagship demo. It defines the runner contract those later `v0.89.1` surfaces must preserve.

---

## Owned Runtime Surfaces

`WP-04` is now owned by these concrete repo surfaces:

- `adl::adversarial_execution_runner::AdversarialExecutionRunnerContract`
- `adl::adversarial_execution_runner::AdversarialRunnerStageContract`
- `adl::adversarial_execution_runner::AdversarialRunnerPosturePolicyContract`
- `adl::adversarial_execution_runner::AdversarialEvidenceCaptureContract`
- `adl identity adversarial-runner`

Proof hook:

```bash
adl identity adversarial-runner --out .adl/state/adversarial_execution_runner_v1.json
```

Primary proof artifact:

- `.adl/state/adversarial_execution_runner_v1.json`

---

## Runtime Condition

ADL adversarial execution is only reviewable if the runner shape is explicit before any exploit/replay machinery arrives.

That means:

- the target, posture, goal, and limit are declared before execution
- every stage has a role and a required evidence shape
- blocked stages are preserved as first-class evidence rather than disappearing
- replay may be strict, bounded, or explicitly deferred, but never silent

This is a runner-orchestration claim, not yet a full exploit/replay engine.

---

## Planned Entrypoint Contract

The runner contract reserves this eventual command shape:

```bash
adl adversarial run \
  --target <target_ref> \
  --posture <profile> \
  --goal <string> \
  --limit <n|time> \
  --output <path>
```

This CLI is not authoritative yet. In `WP-04`, the authoritative proof surface is the `adl identity adversarial-runner` contract artifact.

Required inputs:

- `target_ref`
- `posture_profile`
- `goal_or_focus`
- `bounded_limit`
- `artifact_root`

Limit semantics:

- iteration count or duration limit must be declared before execution
- limit exhaustion produces an explicit defer record instead of silent continuation

---

## Canonical Stage Order

The bounded runner contract defines this stage order:

1. `load_target`
2. `declare_posture`
3. `enumerate_surfaces`
4. `generate_hypothesis`
5. `attempt_bounded_exploit`
6. `evaluate_risk`
7. `decide_mitigation`
8. `capture_replay_decision`
9. `emit_runner_packet`

Role attribution:

- purple owns target loading, posture declaration, replay/defer decisions, and packet emission
- red owns bounded enumeration, hypothesis generation, and exploit attempt stages
- blue owns risk evaluation and mitigation decisions

The runner is successful only when it preserves the stage order and records whether each stage executed, blocked, or deferred.

---

## Posture Policy

Supported posture profiles:

- `audit`
- `validation`
- `hardening`
- `internal_contest`

Freedom Gate requirements:

- target scope must be declared before red execution
- posture must be visible before exploit attempts
- mutation-capable stages require explicit posture permission

Enforcement rules:

- `audit` posture blocks exploit attempts
- no-mutation posture blocks remediation application
- limit exhaustion produces an explicit defer record

Prohibited shortcuts:

- unbounded adversarial loops
- exploit attempts without posture declaration
- evidence capture without role attribution
- silent replay omission

---

## Evidence Capture

The runner contract defines the required evidence family names without yet owning the detailed exploit/replay schemas.

Artifact families:

- target scope record
- posture declaration
- attack-surface inventory
- exploit hypothesis draft
- exploit evidence or blocked-action record
- risk evaluation
- mitigation decision
- replay decision
- adversarial runner review packet

Trace requirements:

- every stage records role, `stage_id`, posture, and `target_ref`
- every produced artifact links back to the stage that produced it
- blocked stages are preserved as first-class evidence

Linkage rules:

- mitigation decisions cite exploit evidence or a blocked-action record
- replay decisions cite exploit evidence and mitigation decision when both exist
- the runner packet preserves canonical stage order for review

---

## Review Surface

Reviewers should be able to answer these questions directly from the proof surface:

- what target, posture, goal, and limit governed the run
- which red, blue, and purple stages executed or were blocked
- what evidence was captured and how is it linked across stages
- which replay or schema details are explicitly deferred downstream

Minimum required visibility:

- target scope and posture declaration
- canonical stage order and role attribution
- evidence capture and blocked-action records
- defer boundaries for exploit/replay schemas

---

## Relationship To Existing ADL Concepts

This contract is intentionally downstream of `WP-02` and `WP-03`:

- `WP-02` defines the contested-runtime assumption
- `WP-03` defines persistent red, blue, and purple role architecture
- `WP-04` defines how the runner orders those roles, captures evidence, and records defer points

It also preserves existing ADL boundaries:

- Freedom Gate and posture govern mutation and exploit permission
- trace and artifact review remain the proof surface
- chronosense can later make contested activity temporally legible

---

## Explicit Boundaries

`WP-04` is intentionally smaller than the whole adversarial band.

Still deferred downstream:

- canonical exploit artifact schemas: `WP-05`
- replay manifests and strict replay execution: `WP-05`
- continuous verification and self-attack loops: `WP-06`
- flagship adversarial demo entry points: `WP-07`

This keeps the issue truthful: it resolves the runner-orchestration and evidence-capture contract without pretending exploit/replay execution is already complete.

---

## Acceptance For WP-04

`WP-04` is satisfied when:

- the adversarial runner contract is explicit in code and docs
- reviewers can inspect a concrete proof artifact for stage order, posture policy, evidence capture, and downstream defers
- later `v0.89.1` exploit, replay, verification, and demo work has a stable bounded runner contract to extend

That is the current state of this feature.

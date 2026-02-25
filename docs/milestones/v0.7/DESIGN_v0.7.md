# ADL v0.7 Design

## Metadata
- Milestone: `v0.7`
- Version: `v0.7.x (release train)`
- Date: 2026-02-24
- Owner: Daniel Austin
- Related issues: #412, #413, #414, #415, #429, #430, #369, #370, #371, #383, #386, #472, #336

---

## Purpose

v0.7 establishes ADL as a hardened, policy-driven, replayable agent runtime with a stable foundation for dynamic learning and controlled self-improvement.

This milestone has two distinct layers:

1. **v0.7.0 (Foundation release)**  
   Ship runtime hardening, delegation patterns, resilience, scheduler surfaces, and security envelope stabilization.

2. **v0.7.x minors (Learning train)**  
   Deliver EPIC-A (Dynamic Learning) incrementally: observe → score → suggest → apply → export.

The design explicitly prevents *silent drift*. Any adaptive behavior must be opt-in, artifacted, auditable, and reversible.

---

## Problem Statement

ADL must:

- Compete as a serious multi-agent runtime (determinism + security + replayability).
- Support small-model plug-in cognition as a first-class pattern.
- Enable recursive improvement (Gödel-style policy evolution) without runtime instability.
- Preserve strong guarantees:
  - Deterministic behavior
  - Strict artifact validation
  - Security envelope integrity
  - No hidden state mutation

Without disciplined v0.7 boundaries:

- Learning features could destabilize runtime semantics.
- Security surfaces could drift.
- Self-improvement could bypass audit trails.
- CLI and artifact conventions could fragment.

---

## Goals

### G1 — Hardened Runtime Foundation (v0.7.0)

Deliver:

- Delegation runtime (paper-driven patterns) (#413)
- Security envelope + trust model hardening (#429)
- Runtime resilience + checkpointing surfaces (#430)
- Scheduler policy surface (#369)
- Remote execution signing + trust policies (#370, #371, #386)
- Sandbox hardening (symlink escape prevention) (#472)
- Canonical execution path consolidation (#383)
- Deferred high-churn rename (#336)

### G2 — Stable Learning Surfaces

Before introducing learning mechanics:

- Stabilize `run_id`, `step_id`
- Stabilize artifact directory structure (`.adl/runs/<run_id>/`)
- Stabilize trace event schemas
- Enforce strict JSON schemas (`deny_unknown_fields`)

These surfaces must not depend on ObsMem integration (deferred to v0.8).

### G3 — Controlled Learning Substrate (v0.7.x)

Deliver incremental, reversible learning:

1. `run_summary.json` (observe)
2. Scoring hooks
3. Explainable suggestions
4. Deterministic overlays
5. Dataset export

No base-model mutation.  
No workflow YAML mutation.  
No silent auto-promotion.

---

## Non-Goals

- Runtime self-modifying Rust code.
- Automatic mutation of security/trust policies.
- Mid-run gradient updates (LoRA, etc.).
- Distributed cluster execution (v0.8).
- Durable checkpoint engine final form (v0.8).
- ObsMem integration (retargeted to v0.8).

---

## Scope

### In Scope

- EPIC-B: Delegation runtime
- EPIC-C: Learning surfaces (without ObsMem integration)
- EPIC-D: Cleanup + deferred systems work
- EPIC-E: Security envelope hardening
- EPIC-F: Resilience + checkpointing surfaces
- EPIC-A (v0.7.x): learning substrate
- Gödel Agent pattern specification (artifacted policy evolution)
- Canonical milestone docs under `docs/milestones/v0.7/`

### Out of Scope

- Distributed cluster execution (#339 → v0.8)
- Durable checkpoint engine (#340 → v0.8)
- Adapter registry + fine-tune infra
- Autonomous self-modifying code
- ObsMem integration (v0.8)

---

## Architecture Overview

v0.7 is structured as layered control-plane resolution:

```
workflow spec
    ↓
runtime defaults
    ↓
overlay (opt-in)
    ↓
effective execution
```

Learning and self-improvement operate only at the overlay/profile layer — never by mutating source workflow YAML.

---

## Learning Artifact Model (v0.7.x)

Artifacts under `.adl/`:

```
.adl/
  runs/<run_id>/
    trace.json
    run_summary.json
    feedback.json
    suggestions.json
  learning/
    overlays/<workflow_id>.yaml
    exports/<timestamp>.jsonl
```

Overlay application requires `--learn=apply` and records:

- overlay sha256
- overridden fields
- trace event `OverlayApplied`

---

## Gödel Agent Pattern (Policy Evolution)

Self-improvement is modeled as artifacted policy evolution.

Artifacts:

- `agent_profile.v1.json`
- `mutation.v1.json`
- `eval_report.v1.json`
- `promotion.v1.json`

Hard constraints:

- All mutations are structured patches.
- No direct source code mutation.
- Promotion requires passing gates.
- Security/trust policies require HITL approval to modify.

---

## Execution Semantics

- All step execution routes through canonical helpers.
- Retry + provider switching logic centralized.
- Scheduler policy surface is explicit and configurable.
- Sandbox validation ensures canonicalized paths stay within base_dir.
- Overlay precedence:

  1. Workflow spec
  2. Overlay (if apply mode)
  3. CLI defaults
  4. Engine defaults

Security/trust policies cannot be overridden by overlays.

### Scheduler Policy Surface (WP-05 / #369)

- Surface area is intentionally minimal for determinism:
  - `run.defaults.max_concurrency` (run-level default)
  - `run.workflow.max_concurrency` or `workflows.<id>.max_concurrency` via `run.workflow_ref` (workflow-level override)
- Precedence for concurrent runs is explicit and deterministic:
  1. workflow-level override
  2. run-level default
  3. engine default (`4`)
- Deterministic scheduling invariants:
  - ready-step selection is lexicographic by full step id
  - bounded batching uses the effective concurrency cap above
  - trace/run summary surfaces expose effective scheduler policy for auditability
- Out of scope in v0.7:
  - fairness/priorities/preemption/QoS
  - distributed scheduling and durable checkpoint orchestration

### Signing Trust Policy Profile (WP-03 / #371)

- Trust checks are policy-driven via an explicit verification profile:
  - allowed signature algorithms (allow-list)
  - required `key_id` toggle
  - allowed key sources (`embedded`, `explicit_key`)
- ADL run config keys:
  - `run.remote.require_signed_requests` (default `false`)
  - `run.remote.require_key_id` (default `false`, requires
    `run.remote.require_signed_requests=true`)
  - `run.remote.verify_allowed_algs` (optional allow-list; empty means receiver
    defaults)
  - `run.remote.verify_allowed_key_sources` (optional allow-list; allowed values
    are exactly `embedded`, `explicit_key`)
- Policy failures are distinct from signature integrity failures:
  - policy violations (missing key_id, disallowed alg/source)
  - malformed signature material
  - signature mismatch
- Remote execution uses the centralized envelope gate in
  `swarm/src/remote_exec.rs`; trust-policy checks are performed there before
  remote execution.
- Learning overlays cannot bypass trust checks (#490).

---

## Risks and Mitigations

- Risk: Learning destabilizes runtime semantics.  
  Mitigation: Ship surfaces first; learning isolated to minors.

- Risk: Security regressions via overlays.  
  Mitigation: Overlays cannot override trust envelope; strict schema validation.

- Risk: CLI churn across minors.  
  Mitigation: Introduce `--learn=off` stub in v0.7.0.

- Risk: Unbounded mutation in Gödel pattern.  
  Mitigation: Strict mutation operator whitelist + schema validation.

---

## Validation Plan

Tests must cover:

- Sandbox boundary checks
- Canonical execution path
- Overlay precedence rules
- Artifact schema round-trip
- Deterministic ordering and stable outputs
- Symlink escape prevention
- Promotion gate logic (Gödel pattern)

Success metrics:

- No security regressions
- Stable artifact schemas
- Measurable improvement in controlled workflows (learning minors)
- No undocumented breaking changes between minors

Rollback strategy:

- Disable learning via `--learn=off`
- Delete overlay file to revert behavior
- Re-select prior promoted `agent_profile`

---

## Exit Criteria

- v0.7.0 foundation shipped with hardened security envelope.
- Learning surfaces stable and independent of ObsMem.
- At least observe + score delivered in minors.
- Overlay mechanism implemented and reversible.
- All changes reflected in milestone docs + release notes.
- Major architectural decisions tracked in `DECISIONS_v0.7.md`.

---

# ADL v0.7 Decisions Log

## Metadata
- Milestone: `v0.7`
- Version: `v0.7.x`
- Date: 2026-02-24
- Owner: Daniel Austin

---

## Purpose

Capture significant architectural, scope, and sequencing decisions for v0.7 at the time they are made. These decisions are binding for the milestone unless explicitly superseded.

---

## Decision Log

| ID | Decision | Status | Rationale | Alternatives | Impact | Link |
|----|----------|--------|-----------|--------------|--------|------|
| D-01 | v0.7 ships as a foundation release, with follow-on roadmap split to v0.75 (EPIC-A/EPIC-B) and v0.8 (EPIC-C/EPIC-D). | accepted | Separates runtime hardening from adaptive behavior while keeping the roadmap explicit and auditable. | (1) Ship full learning loop in 0.7.0; (2) Defer everything to v0.8+. | Keeps v0.7 bounded and clarifies next milestones. | #412 |
| D-02 | ObsMem v1 is not part of v0.7 and is tracked in v0.75 planning scope. | accepted | Learning surfaces must stabilize independently of memory infrastructure. Prevents cross-layer instability. | Integrate ObsMem directly in v0.7.x. | Keeps v0.7 focused and reduces coupling between learning and memory layers. | #414 |
| D-03 | Delegation runtime (paper-driven / DeepMind-style patterns) is part of the v0.7.0 foundation. | accepted | Delegation is a core execution primitive and must be deterministic and hardened before learning layers. | Defer delegation to a later milestone and ship minimal planner only. | Establishes ADL as a serious multi-agent runtime in v0.7.0. | #413 |
| D-04 | Learning in v0.7.x is artifact-driven and overlay-based only (no workflow YAML mutation). | accepted | Preserves replayability, auditability, and no-silent-drift guarantees. | Allow direct workflow mutation; allow implicit auto-promotion. | Maintains ADL’s core philosophy and deterministic guarantees. | #412 |
| D-05 | Security envelope and trust model hardening are mandatory before enabling learning features. | accepted | Adaptive overlays must not weaken sandbox or signing constraints. | Introduce learning first and harden security later. | Ensures learning cannot bypass trust boundaries. | #429 |
| D-06 | Runtime identity rename is deferred to late v0.7 (high-churn, do last): crate/package + binaries become `adl`; keep the `swarm/` directory path stable in v0.7; provide a one-release compatibility window. | accepted | Renaming causes widespread changes; safer after runtime surfaces stabilize. | Rename early in v0.7; defer rename; rename the `swarm/` directory path in 0.7. | Minimizes merge churn and avoids destabilizing early v0.7 work. | #336 |
| D-07 | Distributed cluster execution and durable checkpoint engine are deferred beyond v0.8 (target: v0.85/v0.9 planning). | accepted | These are infrastructure-scale features requiring stable execution semantics first. | Attempt partial cluster support in 0.7.x. | Keeps v0.7 scope controlled and reduces integration complexity. | #339, #340 |

---

## Open Questions

- What formal evaluation harness is required before enabling Gödel-style promotion in v0.8? (Owner: Daniel Austin) (Target: v0.8 design pass)
- Should learning overlays support versioned precedence rules beyond workflow/runtime/CLI layering? (Owner: Daniel Austin) (Target: v0.7.x refinement)
- Confirm the canonical remote runner binary name (default target: `adl-remote`) and the exact deprecation/removal messaging for legacy compatibility entrypoints (target removal: post-v0.75 compatibility window).

---

## Exit Criteria

- All v0.7 architectural boundaries (learning, security, delegation, memory deferral) are explicitly recorded.
- No major scope decisions remain implicit.
- Any superseded decisions are updated in this log.

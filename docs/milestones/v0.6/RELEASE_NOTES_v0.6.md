# ADL v0.6.0 Release Notes (Draft)

## Metadata
- Product: ADL (Agent Design Language)
- Version: v0.6.0
- Status: Draft (will be finalized as part of WP-J)
- Related WPs: #401–#411
- Quality gate: #409 (coverage audit >80% per file)

---

## Summary

ADL v0.6 is a **stabilize + formalize** release focused on determinism, explicit runtime semantics, and better observability.
It expands the pattern surface, introduces minimal human-in-the-loop control, adds streaming output, and improves trace tooling — without turning ADL into a learning system.

These notes are intentionally scoped to the v0.6 umbrella WPs. Update the “Shipped” bullets during WP-J to reflect the exact merged PRs and behavior.

---

## Highlights

- Determinism invariants strengthened and more explicitly tested. (WP-F, #406)
- Pattern registry/compiler surface formalized to support consistent multi-agent patterns. (WP-A, #401)
- Minimal HITL pause/resume semantics added (explicit + trace-visible). (WP-B, #402)
- Streaming output defined as an observability feature that does not alter artifact determinism. (WP-C, #403)
- Better tooling for trace export, replay diff, and workflow graph visualization. (WP-G, #407)
- Coverage ratchet introduced (>80% per file or documented exception). (WP-H2, #409)

---

## What’s New (By Work Package)

### WP-A — Pattern registry + compiler expansion (#401)
Planned for v0.6:
- A formal pattern registry boundary (pattern IDs → compile transforms).
- Improved documentation for the pattern surface.
- Regression tests for compile/pattern stability where applicable.

Notes:
- Patterns remain declarative and deterministic (no runtime mutation).

### WP-B — HITL pause/resume (minimal) (#402)
Planned for v0.6:
- Explicit pause state in the execution lifecycle.
- Resume entrypoint with trace-visible transitions.
- Tests validating that pause/resume does not introduce hidden state.

Notes:
- HITL is opt-in and must be auditable through trace artifacts.

### WP-C — Streaming output (trace + runtime) (#403)
Planned for v0.6:
- Clear streaming semantics for step output and trace events.
- Ordering guarantees preserved at step lifecycle boundaries.
- Tests confirming streaming does not affect final artifact bytes.

Notes:
- Streaming is treated as observability, not semantics.

### WP-D — Provider profiles: top models (#404)
Planned for v0.6:
- Documented provider profiles (configuration-level), with clear constraints and intended usage.
- No runtime auto-selection heuristics in v0.6.

Notes:
- The exact profile list is expected to evolve; keep claims conservative.

### WP-E — Delegation metadata (schema + trace logging only) (#405)
Planned for v0.6:
- Schema support for structured delegation metadata per step.
- Trace logging of delegation metadata.
- Validation and regression tests around the schema surface.

Notes:
- v0.6 does not enforce delegation policy at runtime (policy engine is v0.7 scope).

### WP-F — Determinism + scheduler policy hardening (#406)
Planned for v0.6:
- Clarified max-concurrency override semantics.
- Hardened lexicographic batching / ordering guarantees where applicable.
- Expanded determinism regression tests.

Notes:
- Any scheduling policy work that introduces adaptive behavior is deferred.

### WP-G — Instrumentation + replay diff + graph export (#407)
Planned for v0.6:
- Structured trace export suitable for downstream tooling.
- Replay diff capability for comparing runs.
- Graph export (format finalized during WP-G execution).

Notes:
- Tooling concerns remain separated from core scheduling logic.

### WP-H — Demo matrix + integration demos (#408)
Planned for v0.6:
- A demo matrix defining canonical scenarios for v0.6.
- Deterministic demos that run cleanly under CI.

### WP-H2 — Test coverage audit (>80% per file) (#409)
Planned for v0.6:
- Per-file coverage audit.
- Target: >80% per file or documented exception with an owner and linked issue.

### WP-I — Docs + review pass (#410)
Planned for v0.6:
- Documentation updated to match v0.6 behavior.
- Threat-model and determinism invariants clarified and easy to find.

### WP-J — Release ceremony (#411)
Planned for v0.6:
- Final checklist completion, tag creation, and GitHub release publication.

---

## Upgrade Notes

- v0.6 is expected to be backward-compatible at the workflow level, but may refine trace fields and tooling outputs.
- If a schema or CLI flag changes, document it here during WP-J with exact migration guidance.

---

## Known Limitations / Explicit Non-Goals

- No distributed cluster execution (deferred; backlog #339).
- No checkpoint/recovery engine (deferred; backlog #340).
- No adaptive scheduler policies in v0.6 (deferred; backlog #338).
- No ObsMem integration in core runtime (remains separate; backlog #337).
- Delegation policy enforcement is deferred to v0.7 (EPIC-B, #413).
- v0.6 does not claim production-grade remote authn/authz; security envelope work remains tracked separately (#370, #371, #386).

---

## Validation Notes

- CI must be green at tag time: fmt, clippy (deny warnings), and tests.
- Coverage gate is enforced via WP-H2 (#409).
- Determinism must be verified across repeated runs for canonical demos.

---

## What’s Next

v0.7 epics are already defined and intentionally out of scope for v0.6:
- EPIC-A: Dynamic learning (trace/feedback → adaptation) (#412)
- EPIC-B: Delegation runtime + policy engine (#413)
- EPIC-C: ObsMem integration + learning surfaces (#414)
- EPIC-D: Cleanup + deferred hard systems work (#415)

---

## Exit Criteria

- Notes reflect shipped behavior only (convert all "Planned for v0.6" bullets to "Shipped in v0.6" during WP-J).
- Known limitations and future work remain explicitly separated.
- Text is ready to paste into the GitHub Release UI for tag v0.6.0.

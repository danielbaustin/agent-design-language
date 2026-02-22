# Work Breakdown Structure (WBS) — v0.6

## Metadata
- Milestone: v0.6
- Version: v0.6
- Owner: ADL core (Daniel + Codex-assisted implementation)
- Governing issues: #401–#411
- Quality gate: #409 (coverage audit >80% per file)

---

## WBS Summary

v0.6 is organized around eleven umbrella Work Packages (WPs), each mapped
directly to an open GitHub issue (#401–#411).

The milestone structure emphasizes:

1. Determinism preservation.
2. Explicit runtime semantics.
3. Instrumentation + observability.
4. Controlled expansion (no v0.7 scope creep).
5. Measurable quality ratchet (coverage audit).

Each WP below is decomposed into concrete, mergeable deliverables.

---

## Work Packages

| ID | Work Package | Description | Deliverables | Dependencies | Issue |
|----|-------------|------------|-------------|--------------|-------|
| WP-A | Pattern registry + compiler expansion | Formalize registry abstraction and stabilize compile transforms | Registry abstraction, compile transforms updated, byte-stability tests, docs update | None (foundation) | #401 |
| WP-B | HITL pause/resume (minimal) | Add explicit pause state + resume semantics | State machine extension, trace events, resume tests | WP-F (scheduler invariants) | #402 |
| WP-C | Streaming output (trace + runtime) | Define and implement streaming semantics without breaking determinism | Streaming lifecycle events, stdout/stderr policy docs, tests | WP-F | #403 |
| WP-D | Provider profiles (curated set) | Curated documented provider configurations | Profile definitions, config examples, docs section | None | #404 |
| WP-E | Delegation metadata (log-only) | Schema extension + trace logging (no enforcement) | Schema update, trace logging, validation tests | WP-A (compile stability) | #405 |
| WP-F | Determinism + scheduler hardening | Clarify concurrency + ordering invariants | Tests proving invariants, docs clarification | None | #406 |
| WP-G | Instrumentation + replay diff + graph export | Improve trace tooling and visualization | Structured trace export, replay diff utility, graph export (Mermaid/DOT), demo | WP-A | #407 |
| WP-H | Demo matrix + integration demos | Standardized demo coverage for v0.6 features | Demo matrix doc, CI-verifiable demos | WP-A–WP-G | #408 |
| WP-H2 | Coverage audit (>80% per file) | Per-file coverage ratchet + exclusions documented | Coverage report, exclusions documented, follow-up issues | WP-A–WP-G implemented | #409 |
| WP-I | Docs + review pass | Consolidate documentation for release | README updates, milestone doc alignment, threat-model references | WP-H2 | #410 |
| WP-J | Release ceremony | Final validation + tag + release notes | Tag, release notes, checklist signoff | WP-I | #411 |

---

## Detailed Breakdown

### WP-A — Pattern Registry (#401)

Subtasks:
- Define registry abstraction boundary.
- Map pattern IDs → compile transforms.
- Ensure byte-stable compilation.
- Add regression tests for pattern stability.
- Update DESIGN and docs.

Deliverable:
- Deterministic registry-based compile pipeline.

---

### WP-B — HITL Pause/Resume (#402)

Subtasks:
- Extend execution state machine.
- Add explicit pause trace event.
- Implement resume entrypoint.
- Add tests for state integrity.
- Document pause semantics.

Deliverable:
- Deterministic pause/resume feature with full trace visibility.

---

### WP-C — Streaming Output (#403)

Subtasks:
- Define lifecycle streaming boundaries.
- Implement trace event emission during execution.
- Preserve deterministic completion ordering.
- Add streaming-specific tests.
- Update docs.

Deliverable:
- Streaming that does not alter artifact determinism.

---

### WP-D — Provider Profiles (#404)

Subtasks:
- Identify curated provider list.
- Define profile schema conventions.
- Add example configurations.
- Document tradeoffs and constraints.

Deliverable:
- Clear provider configuration surface (documentation-level).

---

### WP-E — Delegation Metadata (#405)

Subtasks:
- Extend schema with delegation block.
- Validate schema changes.
- Log delegation metadata in trace.
- Add regression tests.
- Document log-only scope.

Deliverable:
- Delegation metadata recorded in trace (no enforcement).

---

### WP-F — Determinism + Scheduler Hardening (#406)

Subtasks:
- Clarify max_concurrency override semantics.
- Harden lexicographic batching guarantees.
- Add regression tests.
- Update DESIGN invariants section.

Deliverable:
- Deterministic scheduler guarantees explicitly enforced.

---

### WP-G — Instrumentation + Replay + Graph Export (#407)

Subtasks:
- Define structured trace export format.
- Implement replay diff comparison tool.
- Implement graph export (Mermaid or DOT).
- Add CLI integration.
- Provide demo artifact example.

Deliverable:
- Trace → diff → visualization workflow.

---

### WP-H — Demo Matrix (#408)

Subtasks:
- Define demo matrix coverage.
- Implement demo scenarios covering new features.
- Ensure deterministic CI execution.
- Document demo usage.

Deliverable:
- CI-validated demos covering v0.6 scope.

---

### WP-H2 — Coverage Audit (#409)

Subtasks:
- Run per-file coverage analysis.
- Identify files <80%.
- Add targeted tests.
- Document justified exclusions.
- Commit coverage report artifact.

Deliverable:
- >80% coverage per file (where practical) + documented exceptions.

Gate:
- WP-I cannot complete until coverage audit is complete.

---

### WP-I — Docs + Review Pass (#410)

Subtasks:
- Align README with v0.6 scope.
- Link milestone docs.
- Validate threat-model alignment (#370, #371).
- Final regression review.

Deliverable:
- Documentation consistent with runtime behavior.

---

### WP-J — Release Ceremony (#411)

Subtasks:
- Final checklist verification.
- Tag release.
- Publish release notes.
- Confirm CI stability.

Deliverable:
- v0.6.0 tagged + published.

---

## Sequencing

### Phase 1 — Foundations
WP-A, WP-F

### Phase 2 — Runtime Extensions
WP-B, WP-C, WP-E

### Phase 3 — Tooling + Profiles
WP-D, WP-G

### Phase 4 — Validation
WP-H, WP-H2

### Phase 5 — Ship
WP-I, WP-J

This order minimizes merge conflicts and preserves determinism invariants first.

---

## Acceptance Mapping

- Determinism invariants → WP-F
- Pattern formalization → WP-A
- Streaming semantics → WP-C
- HITL support → WP-B
- Delegation logging → WP-E
- Instrumentation improvements → WP-G
- Demonstrable capability → WP-H
- Quality ratchet → WP-H2
- Documentation coherence → WP-I
- Release readiness → WP-J

---

## Exit Criteria

- Every WP (#401–#411) closed.
- Coverage audit complete (#409).
- Determinism invariants preserved across test suite.
- Demo matrix passes in CI.
- Documentation reflects shipped behavior.
- No placeholder content remains in milestone docs.

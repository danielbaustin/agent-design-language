# Decisions — v0.6

## Metadata
- Milestone: v0.6
- Version: v0.6
- Status: Active
- Owner: ADL core (Daniel + Codex-assisted implementation)
- Related WPs: #401–#411

---

## Purpose

Capture architectural and scope decisions for v0.6 so that:

- v0.6/v0.7 boundaries are explicit.
- Determinism invariants remain non-negotiable.
- Deferred work is clearly tracked.
- Implementation decisions are traceable to rationale.

This document supplements ADRs but is milestone-scoped.

---

# Decision Log

| ID | Decision | Status | Rationale | Alternatives Considered | Impact | Link |
|----|----------|--------|-----------|--------------------------|--------|------|

| D-01 | Determinism remains a hard invariant in v0.6 | Accepted | ADL’s core identity is deterministic execution planning + ordering guarantees. v0.6 extends runtime surface but must not compromise determinism. | Allow adaptive runtime behavior in v0.6 | Preserves trust model and replay guarantees | ADR-0001 + WP-F (#406) |

| D-02 | Delegation metadata is log-only in v0.6 | Accepted | We introduce structured delegation metadata but defer enforcement to avoid destabilizing runtime semantics. | Implement full policy engine in v0.6 | Keeps v0.6 low-risk and sets stage for v0.7 EPIC-B | WP-E (#405) |

| D-03 | No distributed execution in v0.6 | Accepted | Distributed orchestration increases complexity and risk; determinism must be preserved first. | Introduce multi-node execution in v0.6 | Keeps runtime single-node deterministic | Backlog #339 |

| D-04 | No checkpoint/recovery engine in v0.6 | Accepted | Checkpointing interacts deeply with scheduler and state semantics; defer to avoid destabilization. | Add resumable workflows in v0.6 | Maintains scheduler simplicity | Backlog #340 |

| D-05 | Streaming must not alter artifact determinism | Accepted | Streaming is an observability feature, not a semantic one. Artifacts must remain byte-stable. | Allow streaming to influence step completion ordering | Maintains replay guarantees | WP-C (#403) |

| D-06 | Pause/Resume is explicit and trace-visible | Accepted | Human control must be visible, auditable, and deterministic. | Implicit pause via runtime hooks | Maintains transparency + auditability | WP-B (#402) |

| D-07 | Coverage >80% per file becomes milestone gate | Accepted | Establish quality ratchet without requiring perfection. | Global coverage % only | Raises engineering discipline | WP-H2 (#409) |

| D-08 | Provider profiles are documentation-level in v0.6 | Accepted | Avoid runtime heuristics and auto-selection; profiles are configuration contracts. | Dynamic provider auto-selection | Keeps runtime predictable | WP-D (#404) |

| D-09 | Graph export + replay diff are tooling-layer concerns | Accepted | Instrumentation belongs outside core scheduling logic. | Embed visualization logic in scheduler | Maintains clean separation of concerns | WP-G (#407) |

| D-10 | Remote execution security hardening remains bounded in v0.6 | Accepted | v0.5 established threat model; v0.6 clarifies envelope but does not claim production-grade auth. | Full authn/authz implementation | Prevents false security claims | #370, #386 |

| D-11 | v0.6 is a “stabilize + formalize” release, not a learning release | Accepted | Learning/adaptation introduces non-determinism and policy enforcement complexity. | Introduce adaptive behavior in v0.6 | Preserves conceptual clarity | v0.7 EPIC-A (#412) |

| D-12 | ObsMem remains a separate project | Accepted | Memory integration must remain loosely coupled. ADL provides interfaces only. | Integrate RAG directly into runtime | Maintains modularity | v0.7 EPIC-C (#414) |

---

# Open Questions

- Should streaming trace events include fine-grained token-level output, or remain step-bounded?  
  Owner: Daniel  
  Tracking: WP-C (#403)

- What minimal schema shape is required for delegation policy enforcement in v0.7?  
  Owner: Daniel  
  Tracking: EPIC-B (#413)

- What graph export format becomes canonical (Mermaid vs DOT vs JSON)?  
  Owner: Daniel  
  Tracking: WP-G (#407)

- Should coverage exclusions be centralized in a documented file (e.g., COVERAGE_EXCLUSIONS.md)?  
  Owner: Daniel  
  Tracking: WP-H2 (#409)

---

# Exit Criteria

- All milestone-critical architectural decisions recorded.
- Deferred features clearly linked to backlog or v0.7 epics.
- No placeholder template content remains.
- Decisions align with DESIGN_v0.6.md.

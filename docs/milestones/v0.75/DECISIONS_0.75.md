# Decisions — v0.75

## Metadata
- Milestone: v0.75
- Version: 0.75
- Date: 2026-03-02
- Owner: Daniel / Agent Logic team

## Purpose
Capture milestone-critical architectural and scope decisions made during v0.75 (Deterministic Substrate + ObsMem v1).

v0.75 is a contract-freeze milestone. Decisions recorded here define the execution and memory guarantees that future milestones (v0.8+) build upon.

---

## Decision Log

| ID | Decision | Status | Rationale | Alternatives | Impact | Link |
|----|----------|--------|-----------|--------------|--------|------|
| D-01 | v0.75 scope limited to EPIC-A (Deterministic Substrate) + EPIC-B (ObsMem v1) | accepted | Reduce milestone size and freeze contracts before layering Gödel/authoring | Ship Gödel or cluster in v0.75 | Lower risk, cleaner review, stable foundation for v0.8 | WBS_0.75.md |
| D-02 | Freeze Activation Log schema in v0.75 | accepted | Replay determinism requires stable identifiers and append-only semantics | Continue evolving schema during ObsMem build | Guarantees replay invariants for future layers | DESIGN_0.75.md |
| D-03 | Determinism defined as identical outputs/artifacts given identical inputs + captured boundary events (excluding run-id/timestamps) | accepted | Explicit contract needed for replay verification and CI gating | Weaker definition (best-effort equivalence) | Enables strict regression testing and auditability | DESIGN_0.75.md |
| D-04 | Introduce Trace Bundle v2 with versioned manifest and canonical serialization | accepted | v0.7 export insufficient for replay + memory ingestion guarantees | Patch existing format | Stable portable artifact for replay + ObsMem ingest | DESIGN_0.75.md |
| D-05 | Failure taxonomy must use stable machine-readable codes | accepted | Memory, reports, and deterministic retrieval require stable classification | Human-readable strings only | Enables consistent indexing and reporting | DESIGN_0.75.md |
| D-06 | ObsMem ranking must be deterministic (stable scoring + tie-break rules) | accepted | Prevent nondeterministic query results across identical corpora | Rely on embedding ranking only | Reproducible memory queries and CI-testable behavior | DESIGN_0.75.md |
| D-07 | Hybrid (semantic) retrieval optional; must record embedding model + config | accepted | Embeddings introduce drift risk; configuration must be captured | Always-on semantic retrieval | Controlled nondeterminism boundary; audit-friendly | DESIGN_0.75.md |
| D-08 | Cluster / distributed execution deferred to v0.85+ | accepted | Infrastructure nondeterminism increases risk before substrate freeze | Ship cluster in v0.8 | Keeps v0.75 and v0.8 coherent and reviewable | [VISION_v0.85.md](../v0.85/VISION_v0.85.md) |
| D-09 | Gödel layer deferred to v0.8 | accepted | Self-improvement layer requires stable substrate + memory | Implement during v0.75 | Avoids rework and contract churn | VISION_0.80.md |
| D-10 | No absolute host paths or secrets may be persisted in artifacts or bundles | accepted | Portability + security + determinism | Allow host-relative metadata | Enables reproducible bundles and safe sharing | DESIGN_0.75.md |

---

## Open Questions

- OQ-01: Should replay enforce byte-for-byte artifact equality or structure-equivalent equality in all cases?  
  Owner: Daniel  
  Status: Under evaluation  
  Target: Decide before WP-03 completion

- OQ-02: Should ObsMem store raw activation logs or only normalized summaries?  
  Owner: Daniel  
  Status: Under evaluation  
  Target: Decide before WP-07 schema freeze

- OQ-03: Do we introduce a formal “Determinism Test Suite” artifact (separate crate or directory) for long-term contract enforcement?  
  Owner: Daniel  
  Status: Deferred to Sprint 02 discussion

---

## Exit Criteria

- All contract-freezing decisions (activation log, replay semantics, trace bundle v2, failure taxonomy) are explicitly recorded.
- Any deferred items (cluster, Gödel, authoring) are clearly tied to future milestones.
- Open questions have an owner and milestone placement.
- No milestone-critical behavior remains undocumented.

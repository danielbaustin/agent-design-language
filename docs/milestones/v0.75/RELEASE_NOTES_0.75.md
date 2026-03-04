# Release Notes — ADL v0.75

## Metadata
- Product: **ADL (Agent Design Language)**
- Version: **v0.75.0**
- Release date: **TBD**
- Tag: **v0.75.0**

## Summary
v0.75 is the interstitial stabilization milestone between v0.7 and v0.8. It focuses on shipping a **reviewable deterministic substrate** and the **ObsMem v1 integration boundary** (interfaces + trace/bundle contracts) without coupling a full memory/RAG system into the core runtime.

## Highlights
- Deterministic substrate consolidation for reliable replay and auditability.
- ObsMem v1 integration contract clarified (ObsMem remains an external project; core runtime stays decoupled).
- Milestone slicing and documentation tightened to reduce drift and improve reviewability.

## What’s New In Detail

### Deterministic Substrate (EPIC-A)
- Determinism and replay expectations documented and reinforced across runtime surfaces.
- Artifact/trace contracts clarified to support stable export and downstream analysis.
- Release quality gates (tests, linting, coverage floor) treated as first-class release criteria.

### ObsMem Integration Boundary (EPIC-B)
- Defined the integration boundary for observable memory as an external learning substrate.
- Clarified the “trace → memory indexing” handoff so memory systems can consume ADL artifacts without internal coupling.
- Documented retrieval/policy hook concepts at the interface level (implementation remains external).

### Documentation and Planning Hygiene
- Canonical planning docs live under `.adl/docs/v075planning/`.
- v0.8 planning updated to reflect the redistributed roadmap (v0.75 = EPIC-A/B; v0.8 = EPIC-C/D; cluster deferred).
- Reduced “two sources of truth” risk by using milestone READMEs as pointers rather than duplicating planning docs.

## Upgrade Notes
- This release preserves the v0.7 compatibility window for naming and CLI usage.
- If you have internal tooling that assumes older paths or names, consult the v0.75 DESIGN/DECISIONS docs for the updated canonical structure.

## Known Limitations
- ObsMem is not “built in”: this release defines the boundary and contracts; full memory retrieval/ranking remains outside the core runtime.
- Distributed/cluster execution remains deferred (target v0.85+), and is not part of v0.75.

## Validation Notes
- CI required checks must be green on `main`.
- Workspace checks (fmt/clippy/tests) and coverage floor are enforced as release gates.

## What’s Next
- v0.8: Gödel and Authoring surfaces (EPIC-C/D).
- v0.85+: cluster/distributed execution planning becomes implementation work.

## Exit Criteria
- Notes reflect only shipped behavior.
- Known limitations and future work are explicitly separated.
- Final text is ready to paste into GitHub Release UI with minimal edits (date + links).

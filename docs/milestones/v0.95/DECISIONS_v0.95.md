# v0.95 Decisions

## Status

Forward-planning decision log. These are current milestone-shaping decisions,
not final implementation acceptances.

## Decision Log

| ID | Decision | Status | Rationale | Alternatives | Impact |
| --- | --- | --- | --- | --- | --- |
| D-01 | Treat `v0.95` as MVP convergence rather than a new architecture milestone. | accepted | The feature list and roadmap already assign the remaining large new domains to earlier milestones. | Let `v0.95` absorb overflow from any band. | Keeps the MVP boundary explicit. |
| D-02 | Require a web editor baseline even if Zed remains optional. | accepted | Editor/operator capability needs one required MVP floor. | Treat Zed or another host-specific editor as the only path. | Preserves portability and workflow truth. |
| D-03 | Keep distributed execution subordinate to secure-execution and review boundaries. | accepted | Integration is valuable only if it stays deterministic and reviewable. | Recast distributed execution as a separate orchestration architecture. | Protects the platform’s core trust story. |
| D-04 | Treat dashboard/reporting and evaluation-platform surfaces as first-class MVP work. | accepted | These surfaces are now explicit feature rows and cannot remain implied. | Leave them as backlog-like supporting notes. | Aligns milestone package with feature-list truth. |

## Open Questions

- Should Zed ship as an MVP integration, or should `v0.95` close with an
  explicit defer/drop decision?
- Which `v0.94.1` payments/economic surfaces are truly required inputs for MVP
  convergence, and which remain adjacent but non-blocking?

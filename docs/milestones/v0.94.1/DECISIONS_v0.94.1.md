# v0.94.1 Decisions

## Status

Forward-planning decisions for `v0.94.1`.

## Decision Log

| ID | Decision | Status | Rationale | Impact |
| --- | --- | --- | --- | --- |
| D-01 | Payments and settlement belong in `v0.94.1`. | Accepted for planning | The cluster is too large and distinct to remain floating. | Prevents roadmap spill into `v0.95`. |
| D-02 | `x402` and Lightning are adapter-boundary work, not implicit substrate assumptions. | Accepted for planning | The rails need explicit contracts and bounded demos. | Forces a reviewable interface. |
| D-03 | Financial claims remain bounded and non-production. | Accepted for planning | Avoids overclaiming live payment deployment. | Keeps demos and docs honest. |

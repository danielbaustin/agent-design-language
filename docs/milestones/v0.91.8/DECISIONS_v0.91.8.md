# v0.91.8 Decisions

| ID | Decision | Status | Rationale |
| --- | --- | --- | --- |
| D-01 | Treat `v0.91.8` as a bridge prerequisite for `v0.92`, not as the birthday milestone. | planned | Keeps platform acceptance separate from birthday-facing claims. |
| D-02 | Restore `#4641` to `v0.91.7` WP-14 and move overwritten v0.91.8 content to `#5384` WP-14A. | done for issue routing | Prevents loss of original launch/birthday handoff truth. |
| D-03 | Treat `#5383` as the historical setup authority for this package. | done; closed 2026-07-15 | Operator requested a new `v0.91.7` issue for the planning docs; it is no longer current in-progress work. |
| D-04 | Preserve `#5335` as stale setup predecessor, not the active owner. | planned | Avoids silently duplicating setup authority. |
| D-05 | Keep Runtime v3 and C-SDLC v2 externally owned. | planned | Prevents ADL core from reabsorbing runtime/lifecycle surfaces. |
| D-06 | Require exact-revision acceptance before deletion, deployment, or v0.92 handoff claims. | planned | Keeps planning text from becoming proof. |

# Current Behavior

The normal v2 lifecycle can finish an exact reviewed green PR through `csdlc-finish`, which derives immutable terminal truth in the Git common directory without rewriting tracked lifecycle state. Cleanup is independently provided by `csdlc-clean`.

An older parallel authority remains: `csdlc-closeout` exposes post-merge readiness, `merge_ready`, `merged`, and `closed_out` mutations; full terminal receipts duplicating the record, six cards, design, diagram, and authored artifacts; repair and transport commands; historical reconciliation; and closeout-coupled prune operations. The public schema and operator installation inventory still advertise those mutation surfaces.

Legacy tracked records and retained receipts are consumed read-only by the compatibility index added in issue 5779. Lifecycle phase variants and terminal receipt deserialization must therefore remain readable even after their writers are removed.

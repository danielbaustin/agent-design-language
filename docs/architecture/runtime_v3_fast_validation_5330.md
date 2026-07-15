# Runtime v3 independent fast validation

Runtime v3 changes need a bounded CI profile selected from changed paths. The
profile owns only the Runtime v3 crate, its init/guardian configuration, and
the directly coupled Observatory proof. It must never silently broaden to the
legacy repository-wide lane. A mixed diff selects both profiles; an unmapped
Runtime v3-looking path fails closed until the selector is updated.

The implementation stays in the existing path-policy and PR-fast workflow
boundary. It adds an explicit `runtime_v3_fast` profile, a Rust-only command
set, and contract fixtures for v3-only, mixed, and unmapped path sets.

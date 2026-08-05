# ADL v2 Core

ADL v2 owns the six-primitives language core, deterministic compiler, portable
records interface, thin CLI, and generation selector accepted for the v0.91.8
bridge.

It must not own Runtime v3 execution behavior or C-SDLC v2 lifecycle records.

Retained proof comes from `#5336`, `#5337`, `#5338`, `#5339`, `#5340`, `#5342`,
`#5345`, and `#5350`. WP-16 records the ADL v2 locked all-target suite as
passing and keeps Runtime v3 execution and C-SDLC v2 lifecycle authority out of
the ADL v2 core claim.

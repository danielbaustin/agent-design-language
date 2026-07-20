# v0.91.8 Sprint Plan

## Execution Order

1. Setup and baseline: WP-01 through WP-03.
2. Core implementation: WP-04 through WP-09, then fan out WP-10 and WP-10A
   conductor work on disjoint paths before WP-11.
3. Parity, soak, cutover, acceptance-gated deletion: WP-11 through WP-13.
4. Platform acceptance and v0.92 handoff: WP-14A.
5. Demo, quality, documentation, review, remediation, and closeout:
   WP-15 through WP-23.

## Sprint Rules

- Use [PARALLEL_EXECUTION_PLAN_v0.91.8.md](PARALLEL_EXECUTION_PLAN_v0.91.8.md)
  as the planned execution overlay for WIP limits, card-prep waves, review
  shadows, and the single integration/merge queue.
- Do not start deletion before WP-12 produces reviewed rollback and selector
  proof.
- Do not start WP-14A closeout before ADL v2, Runtime v3, and C-SDLC v2
  acceptance/deployment gates are current.
- Do not use planning text as release proof.
- Keep C-SDLC v2 lifecycle work on typed v2 routes.
- Keep Runtime v3 ownership out of ADL compiler issues.

## Parallelism

Implementation merges for WP-04 through WP-07 are serial on the interface
freeze path: `WP-04 -> WP-05 -> WP-06 -> WP-07`. Prep and review shadows may
run one dependency wave ahead, but implementation waits for reviewed upstream
interfaces.

After WP-09 freezes provider and adapter contracts, WP-10 `#5345` and WP-10A
conductor `#5499` may run concurrently on disjoint paths. WP-10A then proceeds
`#5499 -> #5498 -> (#5500 || #5502) -> #5501 -> #5497`. Actual review,
publication, merge, post-merge validation, closeout, and WP-15 through WP-23
release-tail gates remain serialized.

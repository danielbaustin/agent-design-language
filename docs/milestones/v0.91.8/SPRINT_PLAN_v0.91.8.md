# v0.91.8 Sprint Plan

## Execution Order

1. Setup and baseline: WP-01 through WP-03.
2. Core implementation: WP-04 through WP-10.
3. Parity, soak, cutover, and deletion: WP-11 through WP-13.
4. Platform acceptance and v0.92 handoff: WP-14A.
5. Demo, quality, documentation, review, remediation, and closeout:
   WP-15 through WP-23.

## Sprint Rules

- Do not start deletion before WP-12 produces reviewed rollback and selector
  proof.
- Do not start WP-14A closeout before ADL v2, Runtime v3, and C-SDLC v2
  acceptance/deployment gates are current.
- Do not use planning text as release proof.
- Keep C-SDLC v2 lifecycle work on typed v2 routes.
- Keep Runtime v3 ownership out of ADL compiler issues.

## Parallelism

WP-04, WP-05, WP-06, and WP-07 may proceed in parallel only after WP-02 and
WP-03 establish denominator and corpus truth. WP-08 and WP-09 should wait for
enough engine and records contracts to avoid adapter churn. Review and closeout
WPs must remain sequential.


# #5697 Design: Chronosense Trusted-Time Startup Ordering

## Intent

Move the trusted-time Chronosense correction from the terminal #5663 lane into
the live #5697 follow-up lane. Production Chronosense must consume the
qualified `trusted_time` service through `RecorderTrustedTime`, fail closed
until the authority is qualified, and start immediately after `trusted_time`
before Scheduler and time-observing adapters.

## Boundaries

- Runtime v3 local assembly, production call sites, operation factory
  dependency metadata, and focused assembly tests.
- No #5663 lifecycle reuse, no main checkout edits, no WP-12 evidence mutation,
  and no provider/ACIP/A2A/Cloud Bridge scope.

## Validation

Run focused assembly and governed-operation tests, strict all-target Clippy,
and an exact-head review over the #5697 head before ready publication.

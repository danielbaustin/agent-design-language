# C-SDLC Validation And Routing Hardening

## Metadata

- Feature Name: C-SDLC Validation And Routing Hardening
- Milestone Target: `v0.91.4`
- Status: planned
- Planned WP Home: WP-02 through WP-09

## Purpose

Make lifecycle truth mechanically enforceable enough for repeatable C-SDLC
execution.

## Acceptance Criteria

- Validators reject legacy SRP semantics for new issues.
- Doctor reports clear lifecycle stage truth.
- Workflow-conductor routes to the correct lifecycle or editor skill.
- Sprint-conductor cannot advance past stale child closeout.
- Combined-lane validation catches shared-state hazards.
- Validators or readiness checks reject durable C-SDLC evidence that remains
  only in ignored local paths.
- Validators or readiness checks verify new durable C-SDLC records use the
  documented `workflow/c-sdlc/v0.91.4/` namespace.
- Signed trace verification is part of the durable-proof lane.

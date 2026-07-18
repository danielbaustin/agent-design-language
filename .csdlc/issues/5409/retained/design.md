# #5409 WP-07A Runtime Rearchitecture Completion Design

## Scope

Complete and prove the runtime rearchitecture boundary identified by #5121.
The implementation must make the assembled topology real, make readiness
observe required components, and renew runtime API credentials before expiry.

## Approach

- Inspect the supervised component set, production assembly, readiness contract,
  and runtime API credential lifecycle.
- Integrate the smallest runtime-owned changes needed for topology, health, and
  proactive credential renewal.
- Retain an assembled-runtime soak result and negative renewal/health evidence.

## Validation

- Focused Rust tests for supervision, topology, readiness, and credential policy.
- An assembled-runtime soak proof using the production assembly path.
- Diff hygiene before review.

## Deferred Boundary

Milestone packet wording remains outside this issue unless the bound claim
permits a narrow source-grounded update.

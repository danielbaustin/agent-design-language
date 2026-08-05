# Issue #5384 Preparation Design

## Decision

WP-14A is a thin acceptance gate for the platform already built and deployed.
It consumes four direct inputs: C-SDLC v2 `#5358`, Runtime v3 `#5361`, ADL v2
soak/rollback `#5344`, and the reversible ADL v2 default switch `#5343`.

WP-13 deletion `#5346` and `#5347` is intentionally deferred until immediately
before internal review `#5356`. It is not a WP-14A predecessor.

## Execution Shape

1. Refresh the exact merged revisions for the four direct inputs.
2. Confirm those revisions are present in the acceptance baseline.
3. Run focused fresh-consumer checks for ADL v2, Runtime v3, and C-SDLC v2.
4. Index existing operations, rollback, and recovery evidence without rerunning
   expensive soak proof.
5. Publish one compact acceptance ledger after one bounded Gemini review.

No child proof graph or receipt inventory is required to begin. Missing or
conflicting direct-input truth fails closed; unrelated downstream work does not.

## Preparation Boundary

Preparation owns only:

- `.csdlc/issues/5384`
- `.csdlc/prepared/issues/5384`
- `.csdlc/locks/5384.lock`

No implementation or publication is authorized in this preparation step.

## Reuse

- Typed C-SDLC v2 owns lifecycle state.
- Git owns revision and ancestry checks.
- Existing merged proof packets remain the source for soak, rollback,
  deployment, and recovery evidence.
- Gemini performs the single bounded pre-PR review.

No new dependency, service, cloud resource, wrapper, or test framework is
needed.

## Deferred Deletion Gate

The release-tail plan must place WP-13 directly before `#5356`. Internal review
cannot start until both deletion issues merge and focused post-deletion
validation passes. This preserves late deletion without delaying platform
acceptance or the intervening demo, quality, and documentation work.

# #4758 Launch Readiness Preparation Design

Status: preparation-only design for later execution.

## Single Concern

Issue #4758 owns one concern: `launch-readiness`. Later execution must produce a concrete, evidence-backed package that release review can consume before v0.92 work starts. This preparation does not create public launch copy, claim launch readiness, start v0.92 implementation, or absorb activation, capability, Memory Palace, identity, birthday, demo, or release-closeout work.

## Integrated Delivery Artifact

The later execution output is the versioned issue-local bundle rooted at:

`.csdlc/evidence/4758/launch-readiness/`

Its canonical artifact is `launch-readiness.v1.json`. Release review consumes that manifest together with its bounded supporting evidence:

- `inputs.v1.json`: exact revisions, digests, source state, and claim class for each accepted input.
- `launch-readiness.v1.json`: readiness decisions, blockers, non-claims, proof references, and rollback disposition.
- `launch-readiness.v1.md`: human-readable projection of the canonical manifest; it is not independent authority or the consuming release review.
- `consumption.v1.json`: evidence that release review read the canonical artifact by exact path and digest.
- `rollback.v1.json`: rollback trigger, method, pre-state, post-state, and verification.
- `validation.v1.log`: output from the smallest proving validation lanes.
- `review.v1.md`: exact-revision execution review and finding dispositions.

The bundle is integrated when release review consumes `launch-readiness.v1.json` by exact digest and `consumption.v1.json` records that fact. Merely creating files, writing planning prose, linking an issue, or citing a closeout receipt is not integration proof.

## Dependency Boundary

Later execution must re-check all dependency truth against current `origin/main`:

1. WP-14A #5384 is closed and its accepted baseline `11151e0beab02b1667f6505b7f8992bfd47d2f8f` plus accepted merges remain ancestors of current `origin/main`.
2. WP-20 #5363 has completed release-preflight remediation or supplies an operator-approved blocker that release review consumes as a blocker, never as readiness.
3. WP-21 parent #5362 and exact-revision handoff #5352 provide current routing and input truth. Their open state is not itself proof and cannot be silently deferred.
4. `docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md` continues to route public launch docs to #4758/#4763, and `docs/milestones/v0.91.8/features/V092_HANDOFF_v0.91.8.md` continues to route launch work to #4758.

#5335 and typed closeout receipts are audit context only. They do not gate, release, or prove #4758 execution.

## Claim And Path Boundary

Execution-time typed claim acquisition is deferred by operator direction. Before any canonical card update or execution work, the later owner must obtain a live typed v2 claim restricted to:

- `.csdlc/issues/4758`
- `.csdlc/locks/4758.lock`
- `.csdlc/prepared/issues/4758`
- `.csdlc/evidence/4758`

No shared milestone documentation path is part of the intended execution claim. Any need to modify a shared path is a replan trigger requiring explicit operator scope expansion.

## COTS And Side Effects

No new COTS product, SDK, hosted service, connector, provider, credential, package dependency, or runtime service is required. Later execution reuses installed Git, Ruby, and `jq`; typed C-SDLC v2 is repository-owned tooling. Mermaid CLI and local Chrome are used only to render-check this preparation diagram and are not delivery dependencies. Network reads used to confirm GitHub issue state are observations, not proof substitutes.

The package has no runtime, deployment, provider, credential, database, or public-publication side effect.

## Budget

Later execution target budget:

- elapsed time: 195 minutes target, 240 minutes hard stop
- primary manifest: at most 180 nonblank lines
- input inventory: at most 120 nonblank lines
- consumption and rollback records: at most 80 nonblank lines each
- human review projection, non-claims, and review record: at most 120 nonblank lines combined
- total new issue-local evidence: 500 nonblank lines target, 650 hard stop
- validation: 900 seconds and 8,000 tokens
- total execution: 24,000 tokens target, 32,000 hard stop

Crossing a hard stop requires typed replanning before further work. The budget does not authorize reducing mandatory proof.

## PVF, No Deferral, And Rollback

PVF is a deterministic small/medium evidence lane. Required gates are dependency ancestry, schema/content integrity, issue-local path confinement, exact-digest release-review consumption, rollback completeness, and exact-revision review.

Every required gate must pass. A missing input, unavailable dependency, ambiguous claim, absent consumer, failed check, or unresolved finding produces `blocked` or `failed`; it must not be represented as deferred, skipped, or passed. Claim acquisition is deferred only from this preparation session to execution start, not from execution proof.

Rollback is evidence-only and fail-closed: discard uncommitted issue-local evidence or revert the committed execution change before publication, verify the prior release-review state is unchanged, and record the result in `rollback.v1.json`. No rollback claim is valid without before/after revisions and verification output.

## Preparation Stop Line

This lane stops after the issue-local contract, design, diagram, validation ledger, and one preparation review/fix pass are committed and pushed. It does not run implementation validation, create launch content, mutate GitHub, publish a PR, merge, or perform closeout.

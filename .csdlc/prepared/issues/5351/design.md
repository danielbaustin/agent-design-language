# #5351 WP-16 Integrated Platform Quality Gate Design

## Status

Preparation-only design for v0.91.8 WP-16. It grants no authority to run the
quality gate, edit product code, publish, open a PR, or merge while the typed
#5354 closeout blocker remains. A preparation-only branch push is allowed by
the operator boundary after validation.

## Objective

Produce one exact-revision quality-gate packet that evaluates the integrated
ADL v2, Runtime v3, and C-SDLC v2 platform after reviewed WP-14A acceptance and
WP-15 demo convergence are consumable as exact merged truth. The packet must
expose every failed or unavailable gate as a blocker; it must never turn a
failure into a documentation disposition.

## Authority Boundary

Preparation owns only:

- `.csdlc/issues/5351`
- `.csdlc/locks/5351.lock`
- `.csdlc/prepared/issues/5351`
- `.csdlc/evidence/5351`

No product, runtime, deployment, demo, milestone-document, CI-policy, or release
path is claimed during preparation. The written execution plan is
`.csdlc/prepared/issues/5351/WP16_EXECUTION_PLAN.md`; the bounded gap analysis
is `.csdlc/prepared/issues/5351/WP16_GAP_ANALYSIS.md`. Any future path
amendment must be typed, reviewed, collision-free, and occur only after the
#5354 dependency gate opens.

## Current Dependency Truth

WP-14A #5384 is closed out in typed C-SDLC v2 at generation 16. It accepts the
integrated platform baseline from PR #5726 with accepted platform input merges:
C-SDLC v2 `fc75f4fc697262f89f99461679a406be0b4b3775`, Runtime v3
`f7258b07e9da414bfee518f0c89a76071bc03ee8`, ADL v2 soak and rollback
`d4825d4be9ed14ed6060dd33cbdafe5eaa5efcd2`, and ADL v2 reversible default
`e1b6a34e4763a79d1c40c641e64c0c061a0aa96c`. Its retained platform acceptance
ledger is `.csdlc/evidence/5384/platform-acceptance-ledger.v1.json`.

WP-15 #5354 has current GitHub merged truth but incomplete local typed terminal
truth. PR #5731 merged at `97427f324c87d97cb1b36c7804c50bf80c9389d8` and
retains `.csdlc/evidence/5354/convergence-proof.v1.json`. The later demo-matrix
reconciliation PR #5747 merged at `ab4e9e2217c152df47b1754b66b01febb4a59549`
and retains `docs/milestones/v0.91.8/review/wp15_demo_matrix_5733/RECONCILIATION_LEDGER_v1.md`.
The retained `csdlc-v2/closeout/5354.json` receipt exists and records
`closed_out` at generation 23 with no claim, but `.csdlc/issues/5354/index.json`
in this checkout still reports `phase: reviewed` at generation 13 with active
claim `claim-5354-v0918-wp15-reacquired`. The receipt also records observed SHA
`e8c63268429b0162671e7f1bfae5f560171d7099`, the PR head, which is not
ancestral to this squash-merged head; PR #5731's merge commit is
`97427f324c87d97cb1b36c7804c50bf80c9389d8`. Those reconciliation and ancestry
semantics are the remaining execution blocker.

## Dependency Gate

Execution is admitted only when all of the following are true for #5354:

1. The retained terminal record reports an actually merged PR.
2. Typed C-SDLC v2 reports `closed_out`.
3. The typed claim is absent.
4. The shared Git terminal receipt exists and verifies.
5. The receipt records a merged disposition, PR number, and observed merge SHA.
6. The observed merge SHA is an ancestor of the exact #5351 execution revision.

The gate fails closed on absent, stale, malformed, contradictory, or
non-ancestral evidence. WP-14A is consumed through the #5384 retained platform
ledger and through the #5354 convergence packet; #5351 does not reopen or
re-evaluate WP-14A acceptance.

## Quality Packet

The future packet will bind exact revisions and retained evidence for:

- product contracts and characterization;
- stable deployment and accepted revision identities;
- deterministic compiler and portable engine behavior;
- signing, trust, provider, and governed-tool boundaries;
- Runtime v3 ingress, continuity, supervision, observability, and rollback;
- C-SDLC v2 lifecycle and closeout integrity;
- distributed workcell acceptance;
- deployment, demo convergence, and public claim boundaries;
- deletion eligibility and post-deletion proof;
- demo rows and documentation checks;
- exact revision matrix and release-blocker truth.

Each row has one of: `pass`, `fail`, `blocked`, `not_applicable`, or
`explicit_non_claim`. Only `pass` satisfies a required gate. `blocked` and
`fail` stop WP-17. Missing evidence is `blocked`, never inferred green.

## COTS And Simplicity

No new dependency is permitted. The gate composes existing repository tools,
typed C-SDLC v2 binaries, Git, Ruby standard library, existing validators, and
existing product test commands. It does not implement a test runner, workflow
engine, signer, telemetry system, deployment manager, or evidence database.

## Budgets

- new issue-local gate orchestration and fixtures: at most 1,500 nonblank lines;
- each new script/module: below 500 nonblank lines;
- focused quality assertions: fewer than 150;
- preparation validation: 120 seconds;
- focused product-contract gate: 600 seconds;
- integrated platform gate: 1,800 seconds;
- complete pre-publication or post-merge gate: 2,280 seconds;
- new third-party dependencies: zero.

Any variance requires an exact-revision review and explicit recorded
disposition; the 2,280-second ceiling is not automatic authorization. The six
lane ceilings total exactly the `large` profile's 7,200-second automatic
validation budget. COTS remains zero-new-dependency: use existing repository
tools, Ruby standard library, Git, and installed typed C-SDLC v2 binaries only.

## PVF

- `preparation-contract`: deterministic, small, release-planning proof;
- `wp15-terminal-gate`: deterministic, small, execution admission proof;
- `focused-quality`: deterministic, medium, required pre-integration proof;
- `integrated-platform`: deterministic, large, required release-gate proof;
- `complete`: deterministic, large, required pre-publication proof;
- `post-merge-exact`: deterministic, large, required closeout proof.

All future test additions must be classified in the existing tracked PVF
inventory in the same issue before they are credited.

## Failure And Rollback

Any failed, missing, stale, non-ancestral, secret-bearing, host-bound, or
out-of-scope result stops the gate and routes a blocker to its owning issue.
#5351 does not repair unrelated products inside the quality-gate change. Before
publication, rollback is deletion of issue-local generated evidence. After an
authorized merge, rollback follows the accepted platform rollback contract and
requires a new reviewed issue; #5351 never rewrites terminal evidence. WP-17
#5360 is released after #5351 has merged with passing exact-head integrated
proof. Typed closeout follows asynchronously and does not block WP-17.

# Issue #5360 WP-17 Documentation And Release-Truth Alignment Design

## Decision

Execute an evidence-driven documentation alignment pass after WP-16 issue
#5351 merged as `2e9d2dd7c4260dcf6ec6af954b0eea97554212df` with passing
exact-head integrated proof and became ancestral to the #5360 execution
revision. Typed predecessor closeout is asynchronous and nonblocking.
The implementation will reconcile existing source-of-truth documents; it will
not create another release database, runtime, deployment system, or planning
authority.

## Preparation Boundary

Preparation owns exactly these paths:

- `.csdlc/issues/5360`
- `.csdlc/locks/5360.lock`
- `.csdlc/prepared/issues/5360`
- `.csdlc/evidence/5360`

No product source, shared documentation, deployment state, issue, PR, or release
surface may change during preparation. Before implementation, a fresh collision
check and typed claim amendment must protect the exact approved documentation
paths below.

The preparation baseline is exact revision
`fbf96beac1cb61c85bf7889e9c08729916c0796b`. Zero-change proof evaluates both
committed and uncommitted paths relative to that revision, so a clean worktree
cannot hide an out-of-scope preparation commit.

## Implementation Paths

The gpt-5.5 documentation audit narrowed the active implementation set to the
exact collision-free paths in the typed claim. This includes repository
entrypoints, current v0.91.8 status and feature surfaces, the formal-review
handoff, and the two v0.92 bridge documents that require current input truth.

Two audited paths remain intentionally outside #5360: WP-18 #5356 owns
`docs/milestones/v0.91.8/review/README.md`, and #5765 owns
`docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml`. Their owners must consume the
WP-17 result without overlapping this issue. Any other additional path requires
a fresh collision check and typed claim amendment before editing.

## Source And Claim Model

The alignment packet will inventory each material statement as one of:
`proven`, `planned`, `blocked`, `deferred`, `superseded`, or `explicit_non_claim`.
Only exact retained evidence may support `proven`. Live issue state and the
canonical issue wave determine dependency truth. Aggregate prose, screenshots,
old milestone packets, or successful component checks cannot promote a claim.

For every changed statement, the packet records source path, old classification,
new classification, evidence reference, owning product, and disposition. ADL v2,
Runtime v3, and C-SDLC v2 remain separate products with separate ownership even
where deployment and acceptance evidence converges.

## Dependency Gate

`check-dependencies.rb` fails closed unless the exact WP-16 merge commit is
ancestral to the #5360 execution revision and the retained WP-16 quality gate
reports `pass`. Typed closeout remains independent audit work and cannot block
WP-17 execution. WP-18 remains blocked only until #5360 merges.

## COTS And Architecture

Use repository-native Git, Ruby standard library, typed C-SDLC v2 binaries,
existing Markdown/YAML/JSON structures, and existing focused documentation
checks. Add no crate, gem, package, service, parser framework, workflow engine,
database, deployment manager, telemetry system, or signing layer. Structured
formats must use existing structured parsers or owner tools during execution.

## Budgets

- New dependencies: `0`.
- Product source changes during preparation: `0`.
- Preparation orchestration: at most 1,500 nonblank lines total.
- Individual preparation module: fewer than 500 nonblank lines.
- Focused assertions: fewer than 150.
- Documentation implementation delta: at most 2,500 changed lines across the
  exact protected path set unless an exact reviewed variance is recorded.
- Preparation and dependency gates: 120 seconds each.
- Focused alignment lane: 600 seconds.
- Complete and post-merge exact lanes: 900 seconds each.
- PVF token budgets: 3,500 / 2,000 / 6,000 / 8,000 / 8,000 respectively.

## PVF Plan

`preparation-contract` proves current-registry six-card integrity, reviewed
design and diagram, exact preparation scope, COTS, budgets, clean diff, zero
product changes, and typed doctor truth. `wp16-terminal-gate` is a deterministic,
local, network-denied merge-and-quality ancestry proof.
`focused-doc-alignment`, `complete`, and `post-merge-exact` are required release
gates but remain unavailable until their declared lifecycle points.

Immediately after typed design approval and before bind, a typed
`csdlc-validate` request runs the `current-registry-card-integrity` lane. Typed
approval atomically refreshes the reviewed design/diagram digests and generated
projections; the lane then verifies all six card pairs against the active native
registry shape and requires typed doctor to report no finding. Final preparation
reruns typed doctor and the required PVF request in the bound phase.

## Review And Release Truth

The bounded preparation review checks bypasses, unsupported claims, path
collisions, product-owner ambiguity, COTS duplication, budget gaps, PVF mapping,
and zero-product-change truth. Every actionable finding is fixed before typed
design approval and bind. Future implementation requires exact-revision review,
green required checks, authorized merge, and post-merge proof. The #5360 merge
releases WP-18 immediately; typed closeout follows asynchronously.

## Stop Conditions

Stop without implementation or publication if the exact #5351 merge and quality
proof are not ancestral, any
source claim lacks exact evidence, a protected path collides, a required shared
path falls outside the reviewed set, a change would alter product behavior, a
new dependency is required, Runtime v2/AWS/raw `gh`/credentials are requested,
a required lane fails or is deferred, review becomes stale, or release truth
would be represented more strongly than the evidence supports.

# v0.91.8 Internal Review Plan

Date: `2026-07-29`
Owner issue: `#5356` `[v0.91.8][WP-18] Run internal milestone review`
Status: `planned_not_started`

## Purpose

This document prepares the v0.91.8 WP-18 internal milestone review. It follows
the prior v0.91.4 and v0.91.5 review pattern: build one source-grounded review
packet, run bounded specialist lanes over code, tests, docs, evidence, issue
truth, and release-tail readiness, then synthesize findings before external
review.

This plan does not perform the review, approve release readiness, approve
third-party review, remediate findings, or close WP-18. It defines the entry
gate, source packet, lane structure, findings schema, durable outputs, and stop
boundaries for the later review execution.

## Review Owner

- WP-18 owner: `#5356`
- Required predecessor: WP-17 / `#5360` documentation alignment
- Next stages:
  - WP-19 / `#5357` formal external milestone review
  - WP-20 / `#5363` remediation and release preflight
  - WP-21 / `#5362` feature-list and v0.92 planning truth
  - WP-21A / `#5355` next-milestone closeout planning
  - WP-22 / `#5359` next-milestone planning review
  - WP-23 / `#5348` release ceremony and lifecycle closeout

## Entry Gate

Start the internal review only after all of the following are true at a fresh
exact target revision:

- WP-17 / `#5360` is merged, typed `closed_out`, receipt-backed, claim-free,
  and ancestral to the target revision.
- All completed v0.91.8 implementation, acceptance, deployment, docs, and
  release-tail issue records have current typed lifecycle truth.
- Live GitHub issue and PR state is captured for the v0.91.8 wave, including
  open, closed, merged, draft, blocked, and conflicted states.
- The review packet enumerates every source surface used by the review and
  distinguishes landed proof from planned, deferred, blocked, or non-proving
  evidence.
- No reviewer lane is asked to inspect local-only secrets, hidden credentials,
  or unredacted host-private paths.

If any gate fails, WP-18 records a blocked or deferred review status and does
not advance to WP-19.

## Source Packet

The review packet should be built under:

`docs/reviews/v0.91.8/internal-review-5356/`

Required packet inputs:

- milestone control docs:
  - `docs/milestones/v0.91.8/README.md`
  - `docs/milestones/v0.91.8/WBS_v0.91.8.md`
  - `docs/milestones/v0.91.8/SPRINT_v0.91.8.md`
  - `docs/milestones/v0.91.8/SPRINT_PLAN_v0.91.8.md`
  - `docs/milestones/v0.91.8/PARALLEL_EXECUTION_PLAN_v0.91.8.md`
  - `docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml`
  - `docs/milestones/v0.91.8/WP_EXECUTION_READINESS_v0.91.8.md`
  - `docs/milestones/v0.91.8/QUALITY_GATE_v0.91.8.md`
  - `docs/milestones/v0.91.8/MILESTONE_CHECKLIST_v0.91.8.md`
  - `docs/milestones/v0.91.8/CANONICAL_DOC_INVENTORY_v0.91.8.md`
- feature and proof docs:
  - `docs/milestones/v0.91.8/features/`
  - `docs/milestones/v0.91.8/FEATURE_PROOF_COVERAGE_v0.91.8.md`
  - `docs/milestones/v0.91.8/DEMO_MATRIX_v0.91.8.md`
  - `docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md`
  - `docs/milestones/v0.91.8/FEATURE_PRESERVATION_CROSSWALK_v0.91.8.md`
- release and handoff docs:
  - `docs/milestones/v0.91.8/RELEASE_PLAN_v0.91.8.md`
  - `docs/milestones/v0.91.8/RELEASE_NOTES_v0.91.8.md`
  - `docs/milestones/v0.91.8/NEXT_MILESTONE_HANDOFF_v0.91.8.md`
  - `docs/milestones/v0.91.8/review/README.md`
  - `docs/milestones/v0.91.8/review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md`
- typed lifecycle state:
  - `.csdlc/issues/` records for every issue in the v0.91.8 wave
  - terminal closeout receipts for completed work
  - retained review, publication, readiness, and SOR evidence where present
- live repository state:
  - exact target commit
  - target branch and base branch
  - live issue and PR states for the v0.91.8 wave
  - current changed-path inventory
  - current CI and validation status for the target revision

## Durable Outputs

The review execution should produce tracked, publication-safe outputs:

- `docs/reviews/v0.91.8/internal-review-5356/README.md`
- `docs/reviews/v0.91.8/internal-review-5356/PACKET_MANIFEST.md`
- `docs/reviews/v0.91.8/internal-review-5356/LIVE_STATE.md`
- `docs/reviews/v0.91.8/internal-review-5356/SPECIALIST_LANE_RESULTS.md`
- `docs/reviews/v0.91.8/internal-review-5356/FINDINGS_REGISTER.md`
- `docs/reviews/v0.91.8/internal-review-5356/SYNTHESIS.md`
- `docs/reviews/v0.91.8/internal-review-5356/VALIDATION.md`
- `docs/milestones/v0.91.8/review/V0918_INTERNAL_REVIEW_5356.md`

Ignored local control artifacts may live under `.adl/local-artifacts/5356-*`,
but every reviewer-facing claim and finding must be summarized in tracked
publication-safe files.

## Review Lanes

### 1. Issue Graph And Lifecycle Truth

Check:

- WP-01 through WP-23 dependency order and current live state
- WP-10A visibility and dependency truth
- typed C-SDLC phase, claim, review, publication, readiness, terminal, and SOR
  truth for each completed or in-progress issue
- closed issue and merged PR consistency
- stale, duplicate, orphaned, or unclaimed worktrees

Questions:

- Can any downstream WP appear ready while a predecessor is not actually
  closed out?
- Do issue cards and milestone docs agree on status, evidence, and routing?
- Are any claims expired, overlapping, or still protecting paths after terminal
  closeout?

### 2. ADL Core Code And Architecture

Check ADL core implementation surfaces landed in v0.91.8, including parser,
engine, model, data model, public API, migration, compatibility, and deletion
boundaries.

Questions:

- Are replacement surfaces integrated rather than mocked?
- Are compatibility contracts explicit and tested?
- Are deletion candidates backed by acceptance proof and rollback truth?

### 3. Runtime v3 And Deployment Path

Check Runtime v3 adapter, functional parity, transport, deployment, observatory,
proof packets, and v0.92 handoff surfaces.

Questions:

- Does the runtime path work through the integrated route that users will
  exercise?
- Are negative cases, authentication, observability, and failure modes proven?
- Are local, hosted, and planned-only deployment claims separated clearly?

### 4. C-SDLC v2 Tooling And Lifecycle

Check typed bootstrap, bind, edit, review, publish, readiness, merge, closeout,
binary installation, GitHub/token behavior, issue-create reconciliation,
publication boundaries, and terminal receipts.

Questions:

- Do typed tools fail closed without creating false lifecycle truth?
- Are installed owner binaries present, current, and treated as operational
  source of truth rather than disposable target artifacts?
- Are issue/PR creation and readback races handled idempotently?

### 5. Provider, Adapter, And Platform Acceptance

Check provider adapters, account identity controls, artifact write semantics,
platform acceptance, and model/runtime handoff docs.

Questions:

- Are paid or non-idempotent provider calls protected against duplicate retry?
- Are account, credential, and artifact paths redacted and fail-closed?
- Are MLX, llama.cpp, vLLM, Docker Model Runner, and local Metal plans clearly
  marked as planned work unless implemented in v0.91.8?

### 6. Tests, Coverage, CI, And PVF

Check test taxonomy, shard separation, coverage lanes, CI gating, PVF
classification, exact-head validation reuse, and failure visibility.

Questions:

- Are fast and slow tests separated truthfully?
- Do coverage outputs avoid shared mutable state between concurrent runs?
- Are docs-only, skipped, blocked, deferred, and release-gate proofs visible
  instead of hidden behind aggregate success?

### 7. Documentation And Release Truth

Check milestone docs, feature docs, review docs, release notes, release plan,
next-milestone handoff, demo matrix, and canonical inventory.

Questions:

- Does every feature claim distinguish planned, implemented, integrated,
  deployed, reviewed, blocked, deferred, and deleted states?
- Are WP-21, WP-21A, WP-22, and WP-23 strictly downstream of WP-20?
- Are v0.92 and birthday semantics claim-safe and evidence-bound?

### 8. Evidence, Demo, Podcast, And Site Surfaces

Check evidence packets, demo/proof matrices, website and podcast plans,
operator-facing demo expectations, and externally visible handoffs.

Questions:

- Does each new feature have a reviewable proof or explicit gap?
- Are podcast/site launch plans separated from milestone completion claims?
- Are generated assets, audio/RSS, hosted pages, and guest workflows either
  proven or marked as planned?

### 9. Security, Redaction, And Publication Safety

Check secrets, token handling, AWS non-use, provider credentials, host paths,
logs, artifacts, review packets, and public docs.

Questions:

- Do review packets avoid tokens, account IDs, raw credentials, private host
  paths, and hidden local state?
- Are provider and GitHub token sources referenced only by approved policy,
  never printed or copied?
- Are failed external-model or provider-review attempts recorded truthfully?

### 10. Synthesis And Review Quality

Merge lane findings into one findings-first register.

Questions:

- Are duplicate findings deduplicated by surface, invariant, and failure mode?
- Are severities evidence-bound?
- Does every finding have a route, owner recommendation, and disposition state?
- Does the review avoid opening one issue per finding?

## Finding Schema

Each finding must include:

- severity: `P0`, `P1`, `P2`, or `P3`
- lane
- affected surface
- invariant or expected behavior
- evidence path and line when available
- failure mode
- impact
- recommended remediation
- disposition: `open`, `fixed`, `accepted_risk`, `out_of_scope`, or
  `deferred_with_route`
- proposed route, normally WP-20 / `#5363` unless the operator approves another
  route

Agreement between reviewers may increase confidence, but it must not increase
severity without additional impact evidence.

## Execution Flow

1. Refresh exact target revision, live issue/PR state, and typed lifecycle
   state.
2. Build the bounded packet under `docs/reviews/v0.91.8/internal-review-5356/`.
3. Dispatch specialist review lanes with read-only authority.
4. Record lane outputs without letting reviewers mutate source, issues, or PRs.
5. Synthesize findings into one register and deduplicate before routing.
6. Run review-quality and publication-safety checks over the packet.
7. Update typed SRP/SOR truth for #5356 with the exact review revision,
   validation, and remaining blockers.
8. Stop before WP-19 unless the operator explicitly starts the external review
   and all WP-18 outputs are current.

## Minimal Validation For This Plan

Plan preparation should run:

```sh
git diff --check
ruby -e 'require "yaml"; YAML.safe_load(File.read("docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml"), aliases: true)'
CARGO_TARGET_DIR="${ADL_CARGO_TARGET_DIR:-target/wp18-review}" cargo run --locked --manifest-path csdlc-v2/Cargo.toml --bin csdlc-doctor -- --repo . --issue 5356
```

Review execution should additionally run the live-state, packet-manifest,
path-existence, link, redaction, and exact-revision checks recorded in the
review packet itself.

## Non-Goals

WP-18 must not:

- execute v0.91.8 implementation work
- start WP-19 external review
- remediate findings inside the review plan
- close WP-18 before findings and dispositions are durable
- open one issue per finding without synthesis and operator approval
- claim release readiness, external-review readiness, or v0.92 readiness
- use AWS or provider credentials
- mutate root `main`

## Stop Boundary

This plan is complete when the WP-18 review entrypoint exists, is linked from
the v0.91.8 review index, passes focused validation, and accurately records
that the internal review has not yet run.

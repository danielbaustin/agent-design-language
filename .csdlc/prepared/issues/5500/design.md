# WP-10A Read-Only Workcell Dashboard Preparation Design

## Status

Preparation only. Issue #5500 must not implement dashboard code until #5498
and the final WP-09 gate #5349 are live-merged on `origin/main` and their
merged revisions are ancestors of the execution base. Typed closeout receipts
and retained lifecycle records are audit-only evidence for this preparation
packet; they do not block readiness.

## Purpose

Extend `docs/tooling/milestone-dashboard/` into one dense, read-only operator
view for the distributed C-SDLC workcell. The dashboard observes typed retained
state and bounded live status; it never becomes a state owner or command
surface.

## Inputs And Authority

The dashboard consumes two explicitly different classes of input:

- a versioned generated snapshot containing issue/task ownership, claim
  heartbeat, branch/worktree, lifecycle phase, dependencies, protected paths,
  PR/check/review links, declared outputs, blockers, provenance, and freshness;
- bounded authenticated Runtime v3 Observatory observations for agent topology
  and health.

Every field carries source class, observed revision or timestamp, freshness,
and authority. States are rendered as `live`, `retained`, `stale`, `unknown`,
`blocked`, or `non-authoritative`; absence is never converted to green.
C-SDLC v2 records and owner binaries remain lifecycle authority, GitHub remains
PR/check authority, and Runtime v3 remains runtime-observation authority.

## Planned Product Paths

Future implementation is constrained to:

- `docs/tooling/milestone-dashboard/`;
- `adl/tools/test_milestone_dashboard.sh`.

Those paths are not part of the preparation claim. They are disjoint from the
#5502 convergence/replanning component and the other WP-10A child product
paths. Product authority may be added only through a later typed claim
amendment after the dependency gate opens and active-claim collision checks
pass.

The existing dashboard is extended in place. No second dashboard framework,
backend service, lifecycle database, task adapter, or mutation API is allowed.
Runtime composition links to or consumes the existing
`demos/v0.91.7/html-observatory/` contract rather than copying its runtime
logic.

## Security And Non-Authority Boundary

- The page exposes no write controls and sends no mutation request.
- Live Runtime access is authenticated HTTPS only; credentials stay in
  ephemeral browser session state and never enter snapshots, URLs, DOM text,
  logs, or retained evidence.
- Snapshot and live values are untrusted data. Rendering uses text-safe DOM
  operations, bounded lengths/counts, schema/version checks, and URL allowlists;
  no input is inserted as HTML.
- Live endpoints are configuration-driven and constrained to declared origins;
  no hard-coded IP, implicit loopback claim, credential scan, or arbitrary
  cross-origin fetch is permitted.
- Polling uses bounded concurrency, payload limits, timeouts, and backoff. A
  failed or partial observation becomes stale/unknown rather than authoritative.
- Private task transcript content, secrets, tokens, provider payloads, and host
  filesystem paths are excluded from retained snapshots.
- Links to owner actions are explanatory only. Mutation remains in typed owner
  binaries/adapters and requires independent authority.

## COTS Strategy

No new dashboard framework or runtime dependency is planned. Reuse the browser
Fetch, URL, WebSocket/EventSource, AbortController, DOM, and CSS platform APIs,
the existing milestone dashboard renderer, and the existing Runtime v3
Observatory feed. Validation reuses maintained repository tools: Node syntax
checking, Ruby JSON/YAML parsing, and the existing dashboard shell test. A new
crate, server, paid service, database, or cloud dependency requires a reviewed
typed design change.

## Budgets

- Dashboard implementation: at most 2,000 physical HTML/CSS/JavaScript lines.
- Tests and fixtures: at most 2,000 physical lines and fewer than 100 focused
  cases.
- Focused deterministic validation: at most 120 seconds.
- Complete typed validation envelope: at most 3,600 seconds. The focused
  dashboard contract remains capped at 120 seconds; the larger generated
  envelope is orchestration headroom, not permission to add lanes or scope.
- New direct third-party dependencies: zero by default.

Any exception requires exact-revision review and an explicit typed budget
change; broader repository ceilings do not grant growth authority.

## Validation Plan

1. Preparation validation proves all six cards, design, diagram, issue-local
   claim, exact dependency gate, COTS/budgets, product-path disjointness, and no
   product change.
2. The dependency gate verifies #5498 and #5349 live merge on `origin/main`
   plus ancestry to the #5500 execution base. Typed closeout receipts and
   retained lifecycle records remain audit-only evidence.
3. Future fixture tests prove deterministic snapshot normalization, complete
   state labels, freshness downgrade, missing/partial input behavior, XSS-safe
   rendering, URL/origin rejection, payload limits, token non-retention,
   responsive layouts, and zero mutation requests.
4. Future integration tests consume a bounded Runtime v3 Observatory fixture
   and verify that runtime observations cannot overwrite retained lifecycle or
   GitHub authority.
5. Syntax, focused dashboard tests, line/test budgets, diff hygiene, and an
   exact-revision subagent review must pass before publication.

## Non-Goals

- Product implementation during preparation.
- Replacing GitHub, C-SDLC records, Runtime Observatory, or release authority.
- Task creation, messaging, cancellation, replanning, merge, or closeout.
- A new dashboard framework, backend, state store, or sidecar.
- Runtime v2 edits, AWS use, provider calls, or unauthenticated HTTP access.
- Owning #5502 convergence or any other workcell child's product paths.

## Stop Conditions

- #5498 or #5349 is not live-merged on `origin/main` or its merged revision is
  not ancestral to the #5500 execution base.
- A future dashboard path overlaps an active typed claim.
- Any requested feature needs mutation authority or a second source of truth.
- Live access cannot remain authenticated HTTPS and configuration-driven.
- Validation cannot remain deterministic, bounded, offline for fixtures, and
  within the declared budgets.

# WP-10A Live Distributed Workcell Proof Preparation Design

## Status

Preparation only. No live workcell execution, product implementation,
publication, or PR is authorized by this packet. Execution remains fail-closed
until WP-09 #5349 and WP-10A children #5499, #5498, #5500, and #5502 all have
live merged heads that are ancestral to the execution revision. Typed
`closed_out` state and retained receipts are audit-only signals and must not
block readiness when live merge plus ancestry truth is satisfied.

## Purpose

#5501 proves the complete supported distributed C-SDLC workcell with real
Codex tasks. It consumes the reviewed conductor plan, task/context adapter,
read-only dashboard, and convergence contract. It does not replace those
components and does not gain autonomous review, publication, merge, or
closeout authority.

## Required Live Proof

The retained exact-revision packet must demonstrate all of the following in
one bounded run:

- at least two real writable Codex shards, each with a distinct issue, typed
  claim, branch, worktree, and disjoint protected/write paths;
- a conductor-emitted admitted plan and at least one real fail-closed negative
  case such as overlapping paths, stale context, or invalid authority;
- bounded provenance-bearing context transfer with source revision, task,
  issue, claim, path, and artifact identity;
- dashboard observations derived from typed state and live task/dependency
  observations rather than manually asserted green status;
- collected outputs routed through review and #5502 convergence, producing a
  truthful integration, replan, or blocked decision;
- serialized publication, PR/check, review, merge, post-merge validation, and
  closeout evidence;
- timing, coordination overhead, failures, retries, and comparison with one
  bounded single-agent baseline over equivalent declared work.

Fixtures, mocks, screenshots without source observations, prose summaries, or
library-only tests do not count as the live proof.

## Authority And Data Boundary

The issue owns only an issue-local orchestration and evidence packet. It may
invoke already reviewed owner interfaces after dependency admission, but it
must not add a scheduler, task transport, lifecycle store, dashboard, or
convergence engine. C-SDLC v2 claims and protected paths remain ownership
authority. Codex task state and GitHub/check state are observations, not
permission to mutate lifecycle truth.

Retained context must be bounded and redaction-safe. Preserve task prompts,
declared inputs, exact revisions, digests, output references, and review
dispositions; exclude provider credentials, private unrelated transcripts,
machine-local absolute paths, and unbounded session history.

## Planned Evidence Boundary

Execution should remain under `.csdlc/evidence/5501/` and
`.csdlc/prepared/issues/5501/`. If a later exact review proves that a reusable
proof harness belongs elsewhere, the owner must amend the typed claim before
writing that path. This preparation claim grants no product-write authority.

The packet should include:

- admitted plan and negative-case refusal;
- shard identity, claim, branch, worktree, protected-path, and revision table;
- bounded context envelopes and digests;
- timestamped task/dependency/dashboard observations;
- output, validation, review, and convergence records;
- serialized integration/closeout receipts;
- elapsed-time and coordination-overhead measurements;
- equivalent bounded single-agent baseline and comparison.

Before any task is created, a versioned `live-run-manifest.json` must freeze
the actual shard set. Each shard entry must name its issue, typed claim id,
claim generation and owner, branch, worktree, exact source revision, protected
paths, intended write paths, task identity, bounded context-envelope digest,
and expected output/review references. The manifest validator must require two
to four shards, reject duplicate identities, prove pairwise-disjoint protected
and write paths, and prove every claim is live at the admitted generation.

The manifest is selected only after the terminal dependency gate opens because
the real issues and claims must be live, not invented during preparation. It
is retained and independently reviewed before task spawn; any subsequent
identity, revision, claim, path, or context change invalidates admission and
requires a new reviewed manifest rather than silent drift.

Preparation retains `live-run-manifest.template.json` and the executable
`validate-live-run-manifest.rb` contract. The template deliberately contains
zero shards and must fail validation: inventing future issues, claims, tasks,
or revisions during preparation would be false evidence. After dependencies
are terminal, the operator copies the declared shape into the retained
evidence path, fills it only from live observations, and the same validator
must pass before task creation.

## COTS Strategy

Use the installed typed C-SDLC v2 owner binaries, existing Codex task adapter,
existing #5499 conductor, existing #5500 dashboard, existing #5502 convergence
component, Git, and Ruby standard-library `json`, `open3`, `pathname`, and
`digest` for issue-local validation. No new Rust crate, paid service, database,
message bus, cloud resource, or orchestration framework is planned.

## Budgets

- Product implementation: zero lines unless a later exact design review and
  typed claim amendment prove a missing reusable boundary.
- Issue-local proof harness, validators, and retained structured fixtures: at
  most 2,500 physical lines.
- Focused preparation validation: at most 120 seconds on FastWork.
- Live two-shard workcell and single-agent baseline: each at most 1,800
  seconds; complete exact-revision issue validation at most 3,600 seconds.
- At least two and at most four writable shards; fewer than 100 focused
  contract assertions; no hidden retry loop or unbounded task fan-out.
- New direct dependencies: zero. Any change requires exact design review and
  typed lifecycle amendment before execution.

## Validation Plan

1. Preparation proof checks all six cards, design/diagram, exact live-merge
   dependencies, preparation-only paths, COTS, budgets, PVF, and zero product
   changes.
2. The dependency gate proves #5349, #5499, #5498, #5500, and #5502 live
   merged heads are ancestral to the execution revision. Typed closeout,
   retained receipts, and claim-release records are retained as audit-only
   observations.
3. A dry contract gate validates the live-run manifest, disjoint write sets,
   exact issue/claim/branch/worktree/revision identities, bounded context
   schema, negative case, dashboard observation plan, timing plan, and baseline
   equivalence before any real shard is started.
4. The live run captures real task, output, review, convergence, PR/check, and
   closeout evidence. At least two writable shards must complete or the proof
   fails.
5. Exact-revision review independently checks evidence identity, scope,
   redaction, timings, comparison fairness, and claim non-overlap before
   publication and after integration.

## Non-Goals

- Product implementation during preparation.
- Fixture-only, prose-only, screenshot-only, or library-only substitution for
  the live workcell.
- Unbounded autonomous delivery, task fan-out, issue creation, review,
  publication, merge, or closeout.
- Reimplementing conductor, task adapter, dashboard, convergence, Runtime, or
  C-SDLC lifecycle authority.
- Runtime v2 edits, AWS, provider credentials, paid services, or raw `gh`.

## Stop Conditions

- Any dependency lacks a live merged head ancestral to the execution revision.
- Fewer than two real writable shards can be admitted with disjoint claims and
  protected paths.
- Context identity, redaction, task provenance, dashboard observations, or
  output/review/convergence continuity cannot be proven.
- The negative case is simulated only in prose or the baseline is not
  equivalent enough for a truthful comparison.
- Execution would require product writes outside an amended typed claim,
  Runtime v2, AWS, credentials, hidden network authority, or a deferred
  acceptance proof.

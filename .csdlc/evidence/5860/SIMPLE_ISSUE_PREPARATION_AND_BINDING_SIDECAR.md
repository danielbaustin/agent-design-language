# Simple Issue Preparation And Binding Sidecar

## Purpose

Redesign C-SDLC v2 issue creation, design-time preparation, and execution
binding so an operator can create an issue, make it genuinely execution-ready,
and start work without coordinating hidden claim identities or repairing
partially rendered cards.

This is a v0.92 sidecar. It does not block the current readiness repair and it
does not relax overlap protection, worktree isolation, exact-digest design
approval, or truthful lifecycle records.

## Failure Evidence

The WP-01 issue wave exposed five coupled defects:

1. GitHub issues and six card files can be published while the design and plan
   remain generic placeholders.
2. Rendered cards can say `Status: ready` even though typed bind correctly
   rejects the issue as not design/card ready.
3. `csdlc-init` reserves a specific claim, while `csdlc-bind` rejects a
   semantically equivalent replacement unless it is the exact reserved claim.
4. `csdlc-edit` requires `bound` for substantive card replacement, forcing a
   preparation task to masquerade as execution merely to finish design-time
   cards.
5. A live claim on an open preparation issue has no ordinary owner-release
   operation; cleanup requires operator revoke or expiry recovery.

These are product-design failures, not operator-training failures. More
wrappers, retries, and ceremony would preserve the same traps.

## Proposed State Model

Use four states with one unambiguous predicate each:

1. `draft`: GitHub issue and source intent exist; no readiness claim.
2. `prepared`: design, diagram, and all six typed cards exist as an editable,
   schema-readable generation; semantic placeholders may remain and no
   readiness claim exists.
3. `execution_ready`: prepared packet has approved exact digests, dependency
   gates, owned paths, validation lanes, and a machine-checkable bindability
   receipt; no active claim.
4. `bound`: one session has atomically acquired the execution claim and bound
   the issue to one branch and worktree.

`ready` must never be inferred from file presence or a rendered status string.

## Operator Contract

The normal flow should be three commands, with one optional iterative step:

```text
csdlc-issue create --request create.json
csdlc-prepare run --issue 1234 --request prepare.json
csdlc-bind run --issue 1234
```

`csdlc-prepare sync` owns iterative design-time generation:

- ingest the GitHub issue and active template registry;
- create or update design and diagram sources;
- render all six cards from typed values;
- allow card edits before execution binding;
- leave a truthful `prepared` generation even when semantic placeholders remain;
- never emit an execution-readiness receipt.

`csdlc-prepare seal` owns strict readiness proof:

- run structure, schema, semantic-placeholder, dependency, path, and validation
  checks;
- record exact-digest design approval;
- evaluate bind preconditions that are stable at the current revision without
  acquiring a claim;
- atomically publish one `execution_ready` receipt only if every check passes;
- leave the last valid prepared generation available for correction on failure.

`csdlc-prepare run` is the convenience form: perform `sync`, then `seal`. A
failed seal returns the exact predicates while preserving the synced generation.

Preparation crash safety uses immutable generations rather than pretending six
files can be updated atomically. Render the complete packet under a staging
generation, validate its structure, fsync it, rename it to a content-addressed
immutable generation, then atomically replace one `current` manifest. A crash
before the manifest replacement leaves the prior generation authoritative.

`csdlc-bind run` should:

- consume the current execution-ready receipt;
- derive claim id, owner, branch, and worktree from the current session and
  issue rather than requiring the operator to repeat them;
- atomically reject stale receipts, dependency drift, overlap, dirty root, or
  changed source intent;
- be idempotent for the owning session;
- support an ordinary owner release before implementation starts;
- never require operator revoke for normal preparation cleanup.

Binding uses an intent-first recovery record:

1. atomically acquire paths and persist `binding` intent;
2. create or verify the branch and worktree;
3. persist `bound` after Git state matches the intent.

A repeated bind by the owner resumes the recorded intent. Owner release before
implementation removes any worktree/branch artifacts created by that intent,
then releases paths. Operator revoke remains exceptional recovery.

## Batch Preparation

Milestone waves need a bounded batch mode:

```text
csdlc-prepare batch --manifest issue-wave.yaml --jobs 5
```

Each issue remains an independent transaction and receipt. Batch success means
every child is execution-ready; partial success is reported by issue and cannot
promote an umbrella or milestone-wide readiness claim.

An execution-ready receipt proves semantic preparation at its recorded revision;
it does not reserve paths or promise that resources remain available. Bind must
recheck dependency, revision, root cleanliness, and overlap truth. Concurrent
overlapping binds deterministically allow one winner and leave no artifacts for
the loser.

## Required Invariants

- No tracked issue work occurs on `main`.
- Design approval is pinned to exact design and diagram digests.
- Every protected product path is declared before execution binding.
- Preparation performs no product implementation.
- Failure is atomic and leaves the last valid prepared generation intact.
- Claims protect active execution, not inactive planning packets.
- A semantic validator rejects placeholder scope, generic plans, empty
  validation lanes, contradictory dependencies, and pre-filled review/output
  claims.
- Doctor reports the failed predicate and one next operation, not a generic
  phase-inspection instruction.

## Compatibility And Deletion

Implement the new state machine in the Rust v2 binaries. Retain a narrow,
audited import path for existing records, but do not add shell wrappers.
Initialized records with preparation-only claims migrate to `prepared` with no
claim and an explicit migration event. Bound or implemented records preserve
valid execution claims and bindings. Ambiguous records fail closed with one
repair operation. Once parity tests pass, delete or deprecate the coupled
init/reserve/reacquire behavior and document one canonical operator route.

## Acceptance Criteria

1. A newly created issue is visibly `draft`, never execution-ready by file
   presence alone.
2. Typed sync produces an editable prepared generation without binding or a
   claim; placeholders remain visibly non-ready and are preserved for editing.
3. Seal fails on placeholders, invalid schemas, stale dependencies, missing
   owned paths, or non-proving validation plans while preserving the last
   complete prepared generation.
4. A successful prepare operation emits a digest-pinned execution-readiness
   receipt and doctor reports `ready: true` with `next_operation: bind`.
5. Bind requires only issue/session context, creates the branch/worktree and
   execution claim atomically, and is idempotent for the owner.
6. The owner can release an unstarted binding normally; operator revoke remains
   exceptional recovery only.
7. Concurrent bind attempts and overlapping path claims fail deterministically
   without partial worktrees or stale locks.
8. Batch preparation proves every child independently and cannot overstate
   umbrella readiness after partial success.
9. Existing valid records import without losing audit history; invalid legacy
   records remain non-ready with an explicit repair operation.
10. Focused Rust tests cover happy path, every failure above, crash recovery,
    concurrency, idempotence, and exact-digest drift.

## Gemini Review Disposition

Direct-hosted Gemini review identified four actionable gaps, all incorporated:

- split preparation into iterative `sync` and strict `seal` to avoid an
  uneditable atomic-failure loop;
- replace multi-file atomicity claims with immutable staged generations and an
  atomic current-manifest update;
- add an intent-first `binding` to `bound` recovery protocol around Git worktree
  creation;
- define execution readiness as semantic revision proof, with overlap and other
  volatile predicates rechecked at bind, and audit migration of legacy
  preparation claims.

The implementation issue must retain Gemini's adversarial tests: placeholder
sync/seal behavior, mid-generation crash, post-worktree bind crash, legacy
claim import, concurrent overlap, and forged `Status: ready` without a valid
readiness receipt.

## Non-Goals

- Removing issue-bound worktrees or overlap protection.
- Weakening design review, card schemas, or evidence requirements.
- Reintroducing v1 wrappers.
- Combining implementation, review, publication, or closeout into preparation.
- Making the current 41-child readiness repair depend on this sidecar.

## Gemini Review Questions

1. Does the four-state model remove the observed ambiguity without hiding
   important lifecycle truth?
2. Which operations must be atomic together, and where are crash-recovery
   boundaries still underspecified?
3. Can execution readiness be claim-free without weakening overlap safety?
4. What compatibility hazards arise when importing existing initialized,
   bound, or claim-dormant records?
5. What acceptance tests would catch a return of the WP-01 false-readiness
   failure?

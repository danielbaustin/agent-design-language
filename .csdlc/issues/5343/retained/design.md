# #5343 Reversible ADL Default Switch Design

## Status And Boundary

This packet prepares issue #5343 only. It does not execute a selector
transaction, change any default, install a binary, open a pull request, publish,
delete legacy code, use AWS, or edit Runtime v2. Execution remains fail-closed
until parent #5344 and selector/installer parent #5345 have live merged landing
commits ancestral to the exact #5343 execution revision, and #5344 is
accompanied by accepted exact-revision soak and rollback evidence. Typed
closeout and retained receipts are audit-only observations and never substitute
for live merge plus ancestry.

## Objective

Execute one reviewed and reversible default-generation transaction through the
authoritative selector interface delivered by #5345. The transaction must bind
the selected generation to an exact fresh-install receipt and executable digest,
retain the exact prior selector bytes and identity, prove explicit v1 override,
and open a documented compatibility and rollback window. #5343 grants no
deletion authority.

## Ownership

#5343 owns only:

- issue-local typed lifecycle, preparation, review, and validation records;
- issue-local retained execution evidence under `.csdlc/evidence/5343`;
- the normalized cutover packet under
  `docs/milestones/v0.91.8/evidence/wp12/cutover-5343/`.

The selector, installer, CLI, lock, compare-and-swap transaction, and rollback
primitive are read-only dependencies owned by #5345. The accepted soak,
rollback, compatibility-window, and handoff evidence are read-only inputs owned
by #5344. Runtime v2 and the incumbent ADL installation are untouched rollback
targets and may not be edited, imported, linked, or deleted. The proof may make
one byte-identical copy of the installed v1 executable into the isolated
selector root after recording and rechecking its SHA-256 digest. That copy is
test evidence only, never a production installation or an alternate default.

## Exact #5344 Dependency Gate

Before any cutover command, installation, selector mutation, or product-path
edit, the gate must prove all of the following at the exact execution revision:

1. #5344 and #5345 each have a live merged landing commit on `origin/main`.
2. Each landing commit remains ancestral to current `origin/main`.
3. Each landing commit is an ancestor of the #5343 execution revision.
4. Typed state and retained receipts, when present, are recorded only as
   audit-only observations and are never dependency gates.
5. The #5344 handoff binds a reviewed soak manifest, rollback proof, exact prior
   and restored selector digests, fresh-install receipt, accepted residual-risk
   disposition, and a declared rollback-window duration.
6. Every required #5344 soak and rollback scenario is accepted; no failed,
   unclassified, deferred, credential-overclaimed, or restoration-mismatch row
   remains.

Any absent, malformed, contradictory, stale, or non-ancestral fact stops before
selector mutation. Metadata or a green issue label never substitutes for the
retained typed receipt and accepted evidence.

## Selector Transaction

The eventual cutover uses only the authoritative #5345 interface:

1. Resolve a fresh isolated installation root and verify the exact selected
   executable digest and installation receipt.
2. Snapshot the complete prior selector bytes, schema, generation, digest,
   executable identity, and receipt identity.
3. Verify the #5344 handoff and confirm the rollback window is declared and has
   not expired.
4. Perform one locked compare-and-swap default switch from the expected prior
   selector digest to the reviewed ADL v2 installation.
5. Re-read the selector and verify the exact transaction receipt and selected
   executable identity.
6. Execute the fresh-installed v2 binary, execute the byte-identical isolated
   v1 copy, restore and execute v1 after an actual rollback, then select and
   execute v2 as the final default without changing or deleting the source v1
   installation.
7. Retain deterministic, redacted, repo-relative evidence and start the
   compatibility/rollback clock only after every verification passes.

Stale expectation, lock contention, invalid installation receipt, executable
digest mismatch, unsupported schema, interruption, re-read mismatch, or smoke
failure must preserve the prior selector bytes or trigger explicit rollback
through the same #5345 transaction. Silent fallback and direct selector-file
editing are forbidden.

## Rollback Window

The cutover packet records an operator-approved duration and exact start/end
timestamps. Throughout the window, the prior v1 installation and receipt remain
intact and explicit v1 selection must be re-proven at the exact cutover revision
and at every required checkpoint. A rollback request uses the retained prior
selector digest and the same locked compare-and-swap interface. Failure to prove
v1 restoration blocks publication, closeout, WP-13 deletion, and WP-14A.

## COTS And Simplicity

Use the #5345 CLI/selector/installer, typed C-SDLC v2 owner binaries, Git, and
repository-standard JSON/Ruby validation. Add no crate, selector, installer,
lock implementation, signing system, HTTP client, workflow engine, supervisor,
cloud adapter, or telemetry stack. The cutover harness, if one is necessary,
must be a thin offline adapter over the #5345 public command contract.

## Budgets

- preparation and dependency gates: 120 seconds each;
- focused transaction and failure-preservation proof: 300 seconds;
- fresh-install, override, and rollback-window proof: 600 seconds;
- complete exact-revision and post-merge proof: 1,200 seconds;
- cutover orchestration implementation: at most 500 nonblank lines;
- tests and fixtures: at most 800 nonblank lines;
- each new script or module: below 400 lines;
- no new production dependency or crate;
- every variance requires exact evidence and bounded review before publication.

## PVF And No-Deferral Contract

Each acceptance criterion maps to an executable deterministic lane. Lanes that
cannot run before #5344 closes are dependency-gated, not waived. Before
publication, dependency, transaction, failure-preservation, fresh-install,
explicit-v1, rollback-window, evidence-integrity, budget, CI, exact-review, and
post-merge requirements must all pass. No deferred implementation or validation
claim may satisfy acceptance.

## Stop Conditions

Stop without execution, selector mutation, publication, or PR if:

- #5344 lacks live merge, ancestry, or accepted exact evidence;
- the #5345 selector/installer is not merged, ancestral, or exact-install
  verifiable;
- the prior selector bytes, digest, v1 installation, or rollback receipt cannot
  be retained and verified;
- any path requires direct selector storage editing, hidden network,
  credentials, AWS, Runtime v2 edits, or legacy deletion;
- the rollback window is missing, expired, ambiguous, or not operator approved;
- evidence is host-bound, secret-bearing, non-deterministic, or incomplete;
- any acceptance item would be skipped, deferred, or replaced by metadata.

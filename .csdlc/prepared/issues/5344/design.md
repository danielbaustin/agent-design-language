# WP-12 Opt-In Soak And Rollback Design

## Status And Boundary

This is preparation-only design authority for issue #5344. It does not execute
the soak, change a selector, perform cutover, publish a PR, or modify Runtime v2.
Execution remains fail-closed until both #5350 and #5361 have live merged
landing commits ancestral to the exact #5344 execution revision. Typed
closeout and retained receipts are audit-only observations and never substitute
for live merge plus ancestry.

## Objective

Prove that the reviewed ADL v2 path can run representative opt-in workloads for
a bounded soak interval and can always restore the exact prior selector state
after both successful and failed selection attempts. The packet must distinguish
local deterministic proof, CI proof, Runtime v3 proof, provider disposition, and
demo proof without converting mock or credential-free evidence into production
claims.

## Ownership

#5344 owns only:

- issue-local typed lifecycle, preparation, review, and evidence records;
- `adl-v2/tools/run-soak.sh` as the bounded orchestration harness;
- `adl-v2/tools/prove-rollback.sh` as the selector rollback/fault harness;
- `docs/milestones/v0.91.8/evidence/wp12/` as the retained normalized report.

The selector and installer implemented by #5345 are read-only dependencies.
Parity implementation and classification owned by #5350 are read-only inputs.
Runtime v3 implementation and acceptance owned by #5361 are read-only inputs.
Default switching remains child #5343 authority. No incumbent ADL or Runtime v2
source is modified.

## Dependency Gate

Before any soak, selector mutation, or product-path edit, the execution gate
must prove for each dependency `5350` and `5361`:

1. a live merged landing commit exists on `origin/main`;
2. that landing commit remains ancestral to the current `origin/main`;
3. that landing commit is an ancestor of the exact #5344 execution revision;
4. typed `csdlc-doctor` and retained receipts are recorded only as audit-only
   observations.

Any missing, stale, malformed, contradictory, or non-ancestral fact stops the
lane without selector mutation.

## Reversible Selector Transaction

The rollback harness must use only the authoritative installed ADL v2 selector
interface supplied by #5345. It must never edit selector files directly.

1. Resolve an isolated test root and reject production/default selector paths.
2. Read and retain the prior selector bytes, digest, schema, generation,
   executable digest, and installation receipt identity.
3. Verify the requested opt-in generation and installation receipt.
4. Perform the opt-in through the authoritative locked compare-and-swap
   transaction and retain its deterministic receipt.
5. Run the bounded soak and collect normalized evidence.
6. Perform explicit rollback through the same locked compare-and-swap path.
7. Re-read and prove that prior selector bytes and digest are exactly restored.
8. Emit a redacted, repo-relative rollback receipt.

Failure injection covers invalid generation, missing/mismatched receipt, stale
expected digest, lock contention, interrupted pre-commit, interrupted
post-commit/pre-verify, failed soak, and rollback verification mismatch. Every
pre-commit failure preserves prior bytes. Every post-selection failure triggers
the explicit rollback path. A rollback mismatch is a hard stop and never falls
through to a default or alternate generation.

## Soak Matrix

The exact-revision packet must include:

- deterministic local validate/schema/plan/run/inspect/sign/verify scenarios;
- representative fixture and negative-input scenarios from reviewed parity;
- Runtime v3 opt-in scenarios accepted by #5361;
- CI execution of the same manifest without hidden network or credentials;
- provider scenarios classified as live, local/mock, credential-blocked, or
  not applicable, with no production claim from non-live evidence;
- demo scenarios with exact revision, command, result, duration, and artifact
  digest;
- successful-selection rollback and failed-selection rollback proof.

The manifest is immutable for an execution revision, canonically ordered, and
hashed before execution. Evidence records deterministic inputs, commands,
versions, exit status, bounded durations, artifact digests, selector receipts,
and explicit claim class. Secrets, credentials, absolute host paths, and raw
provider payloads are prohibited.

## Budgets And Growth Control

- preparation validation: 120 seconds;
- dependency/receipt/ancestry gate: 120 seconds;
- focused rollback fault matrix: 300 seconds;
- representative local soak: 1,800 seconds;
- complete CI/Runtime/demo evidence lane: 3,600 seconds;
- orchestration implementation: at most 800 nonblank lines;
- tests/fixtures: at most 1,200 nonblank lines;
- each new script/module: below 500 lines;
- no new Rust crate and no new production dependency without reviewed variance;
- every duration, line count, test count, dependency decision, and variance is
  measured at the exact reviewed revision.

## COTS Decision

Use the existing #5345 ADL v2 selector/CLI, typed C-SDLC v2 owner binaries,
Git, `jq`, shell, and repository validation runners. Do not implement a second
selector, process supervisor, metrics stack, signing system, HTTP client, cloud
adapter, or test runner. Any optional timing/reporting utility must already be
present in the repository toolchain, remain offline, and have a pinned version
recorded in the evidence packet.

## PVF And No-Deferral Contract

Each acceptance criterion maps to an executable deterministic lane. Lanes that
cannot run before dependency closure are gated, not waived. Before publication,
all applicable soak, rollback, negative, budget, CI, exact-review, and evidence
integrity lanes must pass. Provider-unavailable cases require an explicit
credential-blocked disposition and cannot satisfy live-provider readiness.

## Cutover And Rollback Invariants

- #5344 never changes the repository or installed default generation.
- Only an isolated opt-in selector root may be mutated during proof.
- Selector mutation always uses the authoritative locked CAS API.
- The prior selector bytes and digest are retained before mutation.
- Successful and failed soak paths both end in verified exact restoration.
- Failure to restore is terminal and blocks #5343 and all deletion work.
- #5343 remains blocked until the exact #5344 packet is reviewed and accepted.
- WP-13 deletion remains blocked through the compatibility/rollback window.
- Runtime v2 is untouched and remains rollback evidence, not an edit target.

## Stop Conditions

Stop without product work, soak, selector mutation, publication, or cutover if:

- a dependency lacks live merge, typed closeout, receipt, or ancestry proof;
- protected-path ownership collides with another active typed claim;
- the selector cannot operate in an isolated root or exact prior bytes cannot be retained;
- a scenario requires AWS, raw credentials, hidden network, or Runtime v2 edits;
- any acceptance item would be deferred, skipped, or satisfied by metadata only;
- budgets are exceeded without an exact evidence-backed reviewed exception;
- the normalized report contains secrets, absolute host paths, or unverifiable claims.

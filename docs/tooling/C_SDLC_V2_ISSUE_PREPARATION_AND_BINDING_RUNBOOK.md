# C-SDLC v2 issue preparation and binding runbook

## Purpose

This runbook covers the v0.92 issue path that separates semantic preparation
from execution ownership. Preparation is claim-free. The operator never
creates, copies, or supplies a claim ID. `csdlc-bind run` derives internal
claim identity from the issue, the current session, and the sealed readiness
receipt.

The user-visible states are:

1. `draft`: source intent exists; readiness is not claimed.
2. `prepared`: one complete immutable generation is current.
3. `execution_ready`: the current generation has a digest-pinned receipt.
4. `bound`: the issue has a matching execution owner, branch, and worktree.

`binding` is a recoverable protocol state. It is not a successful resting
state.

## Preconditions

- Run the installed typed v2 binaries from `.adl/bin/csdlc-v2/`, or use the
  matching binaries built from `csdlc-v2/Cargo.toml` during focused
  development.
- Run create, prepare, and initial bind from a clean primary checkout on the
  declared base branch.
- Use repository-relative design, diagram, and owned paths.
- Obtain the current session ID from the governed session ledger. Do not
  invent a second owner token.
- Keep the issue source, design, diagram, and six-card input in the request
  packet. Human comments and display-only timestamps do not belong in the
  semantic payload.

Examples below use the installed directory:

```bash
CSDLC=.adl/bin/csdlc-v2
```

## 1. Create the draft

Create `issue-create.json`:

```json
{
  "issue": 5861,
  "repository": "agent-logic/agent-design-language",
  "title": "Simplify issue creation, preparation, and binding",
  "slug": "v092-csdlc-preparation",
  "version": "v0.92"
}
```

Run:

```bash
"$CSDLC/csdlc-issue" --root . create --request issue-create.json
```

Expected result: `state` is `draft`. No canonical issue claim, branch, or
worktree is created.

Repeated create with byte-equivalent semantic input is idempotent. Different
source intent for the same issue fails closed.

## 2. Sync a prepared generation

Create `prepare-sync.json` using the public `prepare_sync_request` schema. The
request includes:

- issue and repository identity;
- design and diagram paths plus design approval truth;
- normalized owned paths;
- exact dependency issue/revision entries;
- the expected base revision;
- the complete `InitialCardInput` for all six cards;
- `expected_manifest_digest` when replacing an existing generation.

Run:

```bash
"$CSDLC/csdlc-prepare" --root . sync --request prepare-sync.json
```

Expected result: `state` becomes `prepared`, and the result names an immutable
`generation_id` containing its semantic digest prefix. Sync stages the whole
generation before atomically replacing the current manifest pointer.

When editing a prepared or ready issue, use the current manifest digest as
`expected_manifest_digest`. A successful successor sync demotes
`execution_ready` to `prepared`; historical generations and receipts remain
audit evidence.

## 3. Seal execution readiness

Create `prepare-seal.json` from the exact sync result and current manifest:

```json
{
  "issue": 5861,
  "expected_generation": "2-0123456789abcdef",
  "expected_semantic_digest": "0123456789abcdef...",
  "expected_manifest_digest": "fedcba9876543210...",
  "dependencies": []
}
```

Run:

```bash
"$CSDLC/csdlc-prepare" --root . seal --request prepare-seal.json
```

Seal fails closed for placeholder content, missing design approval, changed
design or diagram bytes, stale dependency vectors, changed generations, or
manifest digest drift. A successful result is an
`execution_readiness_receipt`; the manifest state becomes `execution_ready`.

The convenience command performs sync followed by seal:

```bash
"$CSDLC/csdlc-prepare" --root . run --request prepare-run.json
```

If seal fails, `run` preserves the complete prepared generation, reports the
failure, and returns `csdlc-prepare seal` as the next operation. It does not
roll back to draft or acquire an execution claim.

## 4. Bind from issue and session identity

Create `bind-run.json`:

```json
{
  "issue": 5861,
  "session_id": "current-governed-session-id",
  "base_branch": "main",
  "expected_base_revision": "exact-revision-sealed-above",
  "lease_seconds": 14400
}
```

Run:

```bash
"$CSDLC/csdlc-bind" --root . run --request bind-run.json
```

The command resolves the active issue claim for `session_id` from the shared
primary-checkout `.adl/session-ledger/ledger.json`, then derives the owner,
internal claim ID, stable issue branch, and issue worktree. It validates the
current receipt, writes durable binding
intent, uses the repository-wide binding lock for path authority, and then
routes through the existing typed initialization and bind implementation.

The request contains no actor, claim ID, branch, worktree, or protected-path
copy. Those values come from the governed session claim, issue draft, and
readiness receipt. A missing, expired, duplicate, released, or globally frozen
session claim fails closed before Git mutation. Bind uses trusted host time and
caps its lease to the governed session expiry; callers cannot supply clock
authority.

Expected result: `state` is `bound`, and the result reports the derived branch,
worktree, and owner for operator visibility.

## 5. Release an unstarted binding

Create `bind-release.json`:

```json
{
  "issue": 5861,
  "session_id": "current-governed-session-id"
}
```

Run:

```bash
"$CSDLC/csdlc-bind" --root . release --request bind-release.json
```

Release requires the exact session-derived owner and serializes against bind.
For an unstarted binding it verifies the bound issue projection and rejects
any drift from the exact intent-materialized lifecycle digest, then removes
only worktree and branch artifacts with durable creation evidence before
returning the preparation state to `execution_ready`.

If the original session expires while an intent is interrupted, a new governed
session may repair it by supplying `expected_intent_digest` from doctor output.
That digest-pinned takeover is accepted only after the old intent lease has
expired; ordinary release omits the field.

Do not use ordinary release after implementation starts. Route terminal work
through `csdlc-finish`, then use `csdlc-clean cleanup` for the exact registered
worktree.

## Batch preparation

Use `csdlc-prepare batch` with a typed `prepare_batch_request` containing one
batch ID and complete child run requests:

```bash
"$CSDLC/csdlc-prepare" --root . batch --request prepare-batch.json
```

Each child syncs and seals independently. Successful child receipts remain
current when another child fails. The batch result reports per-child outcome,
dependency-cycle participants, and intra-batch owned-path overlaps. The batch
is ready only when every child is `execution_ready` and no cycle or overlap is
present. Retry only non-ready children.

## Migrate preparation-only legacy claims

Use the exact current legacy record digest:

```bash
"$CSDLC/csdlc-migrate" --root . preparation --request migrate-preparation.json
```

The migration retains active and terminal issues unchanged. For an
initialized or ready issue with no execution, review, publication, or terminal
evidence, it writes an immutable legacy snapshot, imports the exact six typed
cards into a prepared generation, and releases the preparation-only claim.
Ambiguous or stale inputs are quarantined and return `csdlc-migrate repair` as
the only next operation; they are not guessed into a ready state.

Create a repair request from the exact quarantine result:

```json
{
  "issue": 5861,
  "expected_legacy_digest": "<legacy-record-digest>",
  "expected_quarantine_digest": "<quarantine-resulting-digest>",
  "expected_preparation_digest": null,
  "quarantine_path": ".csdlc/preparation/issues/5861/migration/quarantine-<legacy-record-digest>.json",
  "disposition": "retain_legacy_authority",
  "actor": "operator-or-agent-name",
  "reason": "verified the unchanged legacy authority remains valid"
}
```

Then run:

```bash
"$CSDLC/csdlc-migrate" --root . repair --request migrate-repair.json
```

`retain_legacy_authority` requires the canonical record to remain byte-truth
equivalent at its expected digest. `tombstone_stale_preparation` is allowed
only after the canonical issue projection is already absent and the request
pins the exact current preparation manifest digest in
`expected_preparation_digest`; it removes draft, manifest, receipt, and
generation sidecars while retaining migration audit records. Neither
disposition creates, rewrites, or releases an active claim.

## Failure handling

| Result | Meaning | Next operation |
| --- | --- | --- |
| `stale_digest` during sync | Another generation or receipt became current | Reload the manifest and rebuild the request |
| `validation_failed` during seal | Current generation is complete but not ready | Correct semantic input, sync a successor, then seal |
| `stale_digest` during bind | Receipt or base revision is no longer current | Re-sync and re-seal; do not copy old binding data |
| `claim_collision` | Another issue or session won path authority | Inspect the named owner/issue and wait or route release |
| `binding`/intent remains after interruption | Bind did not reach a stable terminal state | Retry with the same issue and session; after expiry, a governed replacement session may release only with the exact intent digest and proven topology |
| ambiguous Git topology | Existing artifacts cannot be proven intent-owned | Stop and use an audited repair path; do not delete manually |

## Compatibility boundary

The legacy `csdlc-init` route is deleted. New v0.92 work uses the claim-free
issue and preparation commands followed by `csdlc-bind run`. Do not recreate a
wrapper, alias, compatibility retry, operator-supplied hidden claim, or manual
canonical-state edit for the deleted route. The caller-supplied init, bind, and
reacquire Rust APIs are also absent from release builds. A debug-only hidden
test-support surface retains historical fixture construction for regression
tests; it is not linked into release binaries and is not operator authority.

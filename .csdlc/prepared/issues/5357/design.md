# Issue #5357 WP-19 External Review Preparation Design

## Decision

Prepare the formal v0.91.8 external-review lane without dispatching it. The canonical handoff remains exactly `docs/milestones/v0.91.8/review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md`. Preparation may reference and validate that file, but must not rename, copy, or rewrite it. Execution begins only after WP-18 issue #5356 is merged, typed `closed_out`, claim-free, backed by its retained receipt, and ancestral to the exact #5357 target revision.

## Authority And Independence Boundary

The reviewer is a read-only evidence producer. It may report findings and residual risk, but cannot mutate repository or GitHub state, accept findings, create remediation scope, approve release, publish, merge, or close lifecycle records. Reviewer identity and process independence are separate facts. The dispatch receipt records the reviewer/provider/model identity and also who selected the corpus, wrote the prompt, selected the provider, funded the call, and controlled retries. A project-dispatched model review must not be described as fully independent merely because a different model generated the output.

## Preparation Scope

Preparation owns exactly `.csdlc/issues/5357`, `.csdlc/locks/5357.lock`, `.csdlc/prepared/issues/5357`, and `.csdlc/evidence/5357`. No product or shared-document path is protected. The canonical handoff is read-only input. Future send-time evidence remains issue-local unless an exact additional path is collision-checked, reviewed, and added through typed claim amendment.

## Dependency Gate

`check-dependencies.rb` fails closed unless the shared-Git receipt `csdlc-v2/closeout/5356.json` exists; its record exactly matches the current #5356 projection; phase is `closed_out`; claim is null; typed doctor passes; terminal state is merged with a valid PR and SHA; and that SHA is ancestral to the #5357 execution revision. Preparation is allowed while the gate fails. Corpus freeze, reviewer selection, dispatch, synthesis, publication, merge, closeout, and WP-20 release are forbidden.

## Immutable Corpus Contract

At execution time, the corpus builder reads only tracked files from the exact target commit. It emits sorted records containing Git mode, object type, object hash, and repository-relative path, plus the normalized canonical handoff record defined by the handoff's digest procedure. The manifest binds repository, base, head, target SHA, entry count, object-record digest, and handoff digest.

The manifest rejects missing paths, duplicate paths, untracked input, unexpected symlinks or submodules, absolute or parent-traversal paths, self-inclusion, mutable working-tree content, changed target identity, and a target not descended from WP-18. A digest is never computed over a record containing its own digest value.

## Dispatch Receipt Contract

The dispatch receipt is separate from the corpus and review output. It binds exact target/base/head and corpus/handoff/prompt digests; distinct corpus-selector, prompt-author, prompt-selector, reviewer, provider-selector, process-owner, funder, and retry-controller identities; provider/model and declared independence/conflict boundary; dispatch/completion times, attempts, provider outcome, degradation state, output path, byte count, and SHA-256; and explicit review, release-approval, and lifecycle-authority booleans. A failed or partial call remains a failed receipt. Retry appends an attempt rather than overwriting history. A superseding receipt names an immutable issue-local history receipt by path and digest; the validator requires the previous attempt array to be an unchanged prefix.

## Findings And Synthesis Contract

The reviewer returns findings first in P0-P3 order. Every finding has stable id, summary, exact repository-relative file/line evidence with a digest of the cited target-revision line bytes, impact, violated invariant, failure mode, bounded remediation, and residual risk. The validator resolves every citation against the frozen corpus and target Git object. `observed_evidence`, `inference`, and `open_author_decision` remain separate fields; unsupported assertions cannot become evidence. Informational notes stay outside P0-P3. Internal synthesis deduplicates by surface, invariant, and failure mode, maps accepted findings through typed `csdlc-review`, and never auto-creates one issue per finding.

## Publication And Redaction Boundary

Publication-safe evidence excludes credentials, private prompts, raw provider payloads, personal data, machine-local paths, local artifact roots, temporary files, and hidden provider metadata. Redaction is deterministic, recorded, and must not silently change a finding's meaning. When safe publication would erase necessary evidence, retain only a private digest/reference and publish a truthful non-claim.

## COTS, Budgets, PVF, And Rollback

Use Git, SHA-256, JSON, Ruby standard library, and installed typed C-SDLC v2. Add no crate, gem, package, service, workflow engine, signing layer, evidence store, or review orchestrator. Preparation changes zero product/shared-document files and stays within 1,800 nonblank authored lines, 500 per module, fewer than 160 focused assertions, and 120/300/900-second PVF budgets.

The VPP template owns lane identity, proof role, acceptance mapping, determinism, resource profile, time/token budget, argv, parallel group, and deferral truth. The typed PVF manifest must agree on every representable VPP field. Its `release_gate=required`, `network=denied`, and empty credentials are explicit fail-closed policy extensions because those fields do not exist in the active VPP template schema; the preparation validator enforces them for every lane rather than pretending they are VPP fields.

Before dispatch, rollback deletes only unpublished issue-local generated artifacts and returns to `prepared_not_sent`; immutable completed receipts are never rewritten. After dispatch, a stale target or invalid result is superseded by a linked new exact attempt, not mutated in place. No required gate may be deferred.

Typed sequencing is explicit: `csdlc-init` renders generation-0 cards with design review pending; bounded review fixes the packet; typed design approval creates generation 1; the card-integrity PVF runs immediately at initialized generation 1; `csdlc-bind` then creates the bound preparation state; and the full preparation validator runs only after bind. None of these steps authorizes corpus freeze or dispatch.

## Stop Conditions

Stop on incomplete WP-18 truth, handoff mutation, path collision, stale target, non-reproducible corpus, self-referential digest, undisclosed reviewer control, ambiguous provider outcome, malformed findings, evidence/inference confusion, unsafe retained data, new dependency, Runtime v2, AWS, raw `gh`, credentials, paid service during preparation, budget breach, or any deferred required proof.

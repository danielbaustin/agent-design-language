# #5341 Design: Thin ADL v2 to Runtime v3 Adapter

Status: execution approved. Product implementation starts when each dependency
merge is ancestral to current `origin/main`; typed closeout may finish in parallel.

## Decision

Implement one small ADL-owned adapter crate at
`adl-v2/crates/adl-runtime-v3-adapter`. The crate converts the portable
`ExecutionPlan` and engine event/record contracts owned by #5340 and #5342 into
the canonical typed Runtime v3 ingress contract accepted and terminally
retained by #5591. Runtime v3 remains the sole execution authority. The adapter
must not supervise processes, schedule work, execute providers or tools, sign
records, mutate C-SDLC state, or reinterpret ADL language semantics.

The accepted contract is the #5591 merged and typed `closed_out` receipt plus
its merged-SHA ancestry in `origin/main`. The currently visible #5591 branch or
PR is preview evidence only and is not implementation authority.

## Dependency Gate

Implementation may start when all three merge and ancestry rows pass. Closeout
truth remains visible but is intentionally non-blocking:

| Gate | GitHub truth | Closeout truth | Ancestry truth |
| --- | --- | --- | --- |
| WP-06 #5340 | issue/PR merged | retained when available; non-blocking | merged commit is an ancestor of current `origin/main` |
| WP-07 #5342 | issue/PR merged | delegated in parallel; non-blocking | merged commit is an ancestor of current `origin/main` |
| Runtime v3 ingress #5591 | issue/PR merged | retained when available; non-blocking | merged commit is an ancestor of current `origin/main` |

`.csdlc/prepared/issues/5341/dependency_gate.rb` implements this fail-closed
check without network mutation, raw `gh`, AWS, or product writes. A missing,
malformed, non-merged, non-`closed_out`, or non-ancestral receipt is `waiting`.
After it passes, live GitHub issue/PR state is refreshed read-only before claim
scope can be amended.

Current execution result: #5340, #5342, and #5591 are merged and ancestral to
`origin/main`; #5342 typed closeout is delegated in parallel and does not block.

## Authority Boundary

| Owner | Owns | #5341 may do | #5341 must not do |
| --- | --- | --- | --- |
| ADL language/compiler | validated source semantics and canonical plan construction | consume the merged typed `ExecutionPlan` contract | add or reinterpret language primitives or compiler rules |
| ADL engine #5340 | bounded scheduling, retry/failure, joins, resume, provider/tool ports | consume plan dispatch and engine event contracts | schedule, retry, join, resume, or execute ports inside the adapter |
| ADL records #5342 | versioned errors/events/traces/results/artifacts, canonical bytes, signing and verification | preserve identifiers, canonical bytes, signatures, and verification outcomes | sign, re-sign, weaken verification, invent trust, or rewrite record history |
| Runtime v3 #5591 | canonical ingress validation, admission, execution, continuity, pressure behavior, and live state | submit a typed request and map a typed result/error without escalation | supervise Runtime v3, bypass ingress, write Runtime state, reopen admission, or call internal components directly |
| C-SDLC v2 | issue/card/claim/review/publication/closeout authority | retain issue-local lifecycle evidence | grant product or merge authority |

## Preparation And Future Protected Paths

The active preparation claim protects exactly these disjoint paths:

- `.csdlc/issues/5341`
- `.csdlc/locks/5341.lock`
- `.csdlc/prepared/issues/5341`
- `.csdlc/evidence/5341`

No product path is protected during dependency watch. After the dependency gate
passes, a typed `csdlc-bind` claim transition may add exactly:

- `adl-v2/crates/adl-runtime-v3-adapter`

That future product path is disjoint from the active ADL core owners
`adl-characterization`, `adl-v2/crates/adl-language`, and
`adl-v2/crates/adl-compiler`; it is also disjoint from the active Runtime v3
owner paths `adl-runtime`, `adl-runtime-kernel`, `infra/runtime-v3`, and
`adl/tools/run_runtime_v3_guardian_soak.sh`. Shared manifests, Runtime v2,
Runtime v3 owner trees, and other ADL v2 crates are not in #5341 scope. If the
merged predecessors require any additional shared write, stop and typed-replan
instead of widening implicitly.

## Adapter Contract

The final contract is derived from the three terminal predecessor revisions,
not guessed during preparation. It must preserve these semantics:

1. Accept only a validated, versioned portable execution plan and a verified
   engine dispatch/event envelope.
2. Derive one stable Runtime v3 work identifier and canonical payload from
   predecessor-owned identifiers and canonical bytes; identical accepted input
   produces identical adapter output.
3. Submit only through #5591's canonical typed ingress. No direct Runtime v3
   component, queue, checkpoint, supervisor, provider, or internal state access
   is permitted.
4. Preserve backpressure and terminal errors exactly. Saturated, closed,
   unsupported, malformed, unauthorized, conflicting, and failed execution
   results cannot be converted to success, retried without engine authority, or
   hidden.
5. Map accepted Runtime v3 results into #5342-owned result/trace artifacts while
   preserving correlation, idempotency, provenance, digest, and trust fields.
6. Expose no network listener, transport selection, credential handling,
   address default, process launch, or deployment policy.

## COTS Inventory

| Class | Dependency | Purpose | Decision |
| --- | --- | --- | --- |
| Production COTS | `serde_json` `=1.0.151` | deterministic canonical payload construction over predecessor-owned typed APIs | reuse the version already pinned by ADL v2 |
| Production COTS | `sha2` `=0.10.9` | stable payload and correlation digests | reuse the version already pinned by ADL v2 |
| In-repo path dependency | #5340 engine crate | portable plan dispatch and engine event/error contracts | required only at the terminal merged revision |
| In-repo path dependency | #5342 records crate | canonical record/result/trust contracts | required only at the terminal merged revision |
| In-repo path dependency | `adl-runtime-kernel` | #5591 canonical ingress request/result/error types | consume public ingress only; no internal Runtime access |
| Dev-only COTS | `tokio` `=1.52.3` | deterministic async test runtime for canonical ingress integration tests | test-only; features limited to `macros`, `rt`, `sync`, and `time` |
| Dev-only COTS | `ed25519-dalek` `=2.2.0` | construct real signed #5342 envelopes in tests | test-only and aligned with `adl-records` |

The exact implementation revision must retain `cargo tree --locked` output and
fail if any unplanned direct production COTS dependency appears. Transitive
dependencies belong to the three merged owner crates and are inventory, not
new #5341 authority.

## Source And Test Budgets

Budgets use sorted tracked `*.rs` files under the future adapter crate and
physical `wc -l` lines. Generated output and Cargo build artifacts are excluded.

| Surface | Hard budget | Additional rule |
| --- | ---: | --- |
| production Rust under `src/` | 500 physical lines | no production file over 250 lines |
| Rust tests under `tests/` | 1,000 physical lines | at least 12 focused tests; inline `#[cfg(test)]` modules are forbidden so production/test measurement stays exact |
| direct production COTS dependencies | 2 | only the declared pinned serialization and digest crates |
| direct dev COTS dependencies | 2 | only the declared Tokio harness and Ed25519 test signer |

A budget breach is not an automatic exception. It stops execution for typed
replanning and exact design review; proof may not be deleted or deferred to
make the count pass.

## Validation-Time Budget

All Rust build output uses `/Volumes/FastWork/adl-5341`. The complete local
validation budget is 2,400 seconds:

| PVF lane | Class | Proof role | Budget |
| --- | --- | --- | ---: |
| dependency gate | deterministic control | terminal receipts and merged ancestry | 30 s |
| focused mapping/unit tests | deterministic behavior | stable mapping, IDs, bytes, and error preservation | 300 s |
| canonical ingress integration | deterministic integration | real #5591 public ingress with bounded success/backpressure/terminal outcomes | 600 s |
| negative authority suite | deterministic security/authority | reject bypass, malformed, unverified, escalation, internal-state, Runtime v2, and direct-owner access | 300 s |
| complete adapter suite | deterministic regression | all crate targets and doctests | 300 s |
| strict format and Clippy | deterministic quality | format plus warning-free all-target/all-feature proof | 420 s |
| COTS/LoC/boundary inventory | deterministic contract | locked dependency tree, budgets, protected paths, and forbidden references | 180 s |
| exact-revision diff and card truth | deterministic lifecycle | clean diff, typed doctor, and exact revision identity | 270 s |

Skipped, ignored, pending, degraded, fixture-only, prose-only, or CI-deferred
lanes do not satisfy acceptance. CI reruns the relevant lanes but is not a
substitute for this complete FastWork local proof.

## No-Deferral Acceptance Matrix

| Acceptance | Positive proof | Required negative proof | Deferral policy |
| --- | --- | --- | --- |
| AC-1 terminal dependency authority | all three executable gate rows pass at current `origin/main` | missing/stale/non-merged/non-ancestral receipt fails | none |
| AC-2 deterministic plan mapping | canonical plan maps to one stable ingress request and work ID | malformed, unknown-version, non-canonical, oversized, and conflicting input fail | none |
| AC-3 engine event fidelity | accepted, saturated, closed, unsupported, conflict, and execution-failed outcomes map exactly | adapter cannot retry, swallow failure, or manufacture success | none |
| AC-4 records and trust fidelity | identifiers, canonical bytes, correlations, digests, signatures, and verification outcomes round-trip | unverified/tampered/re-signed/history-rewritten input fails | none |
| AC-5 Runtime authority isolation | work enters through the public canonical ingress and returns through public result/error contracts | no internal component/queue/state/supervisor/provider access, admission reopen, or checkpoint write | none |
| AC-6 product isolation | adapter is confined to its owned crate and contains no Runtime v2 dependency | Runtime v2, C-SDLC, AWS, listener, credential, hard-coded address, deployment, or cutover references fail the boundary scan | none |
| AC-7 budget and COTS | exact LoC, module, test, and locked-tree budgets pass | undeclared production COTS or scope growth fails | none |
| AC-8 exact-revision integration | FastWork lanes, bounded review, CI, merge, post-merge proof, and typed closeout all agree | stale review, red CI, non-ancestral merge, or missing receipt fails | none |

## Negative Authority Proof

The negative suite must prove that the adapter cannot:

- accept an unvalidated plan or unverified/tampered record;
- construct or submit an ingress kind not allowed by the terminal #5591
  contract;
- bypass canonical ingress to invoke Runtime components, operations, queues,
  checkpoints, control, or supervision;
- retry or reorder work without #5340 engine authority;
- sign, re-sign, downgrade verification, replace provenance, or rewrite history
  owned by #5342;
- reinterpret a Runtime saturated, closed, unsupported, conflict, or failure
  response as success;
- depend on, import, include, or modify any Runtime v2 implementation path;
- open a socket, choose TLS/auth policy, read credentials, launch a process,
  use AWS, hard-code an address, or alter deployment/cutover state;
- import or mutate C-SDLC issue, card, review, publication, or closeout state.

Static forbidden-reference scans complement compile-time visibility and
behavioral tests; they do not replace them.

## Rollback

Before cutover, rollback is deletion of the additive adapter crate from its
issue branch; no runtime or default behavior changes. After merge but before a
later selector owner adopts it, rollback is a focused revert of the #5341 merge
commit. If a later consumer has adopted the crate, that owner must first restore
its prior selector/consumer path and prove the previous generation, then revert
#5341. #5341 never deletes Runtime v2, changes Runtime v3 deployment, or owns a
selector, so rollback must not mutate those surfaces.

## Execution Sequence

1. Keep the preparation claim read-only with respect to product paths while the
   executable dependency gate reports `waiting`.
2. When all gates pass, refresh `origin/main`, verify sole-writer/disjointness,
   integrate current `origin/main` into the issue branch, and use typed
   `csdlc-bind` to transition the claim and add only the adapter crate path.
3. Reconcile the three terminal contracts into an exact adapter API and typed
   replan if any assumption or budget changes.
4. Implement the smallest mapping/authority crate without Runtime v2 or shared
   owner edits.
5. Run every FastWork PVF lane and retain exact-revision evidence.
6. Obtain one bounded exact-revision GPT-5.5 subagent review immediately before
   PR publication, fix every actionable finding, and rerun affected lanes.
7. Use typed review and publication, require green CI, self-merge only under the
   operator's standing authorization and exact-head match, then run post-merge
   validation and typed closeout/receipt.
8. Prune only through the guarded typed closeout path after retained terminal
   authority proves it safe.

## Stop Conditions

- Any dependency merge is not ancestral to current `origin/main`.
- A product-path claim overlaps an active owner or requires any path beyond the
  exact adapter crate.
- The terminal contracts contradict this design or require Runtime v2, shared
  manifest, runtime-owner, signing-owner, scheduler-owner, C-SDLC, AWS,
  listener, credential, deployment, or selector writes.
- Any required acceptance or PVF lane is skipped, pending, degraded,
  fixture-only, prose-only, CI-only, or failing.
- Source/test/COTS/validation budgets cannot be met without weakening proof.
- Review is stale or actionable findings remain.

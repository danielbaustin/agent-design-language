# Issue 5340 portable bounded execution engine design

## Status and exact dependency gate

This design is implemented and in exact-revision remediation review. Before
product work began, a fetch-only refresh of `origin/main` proved issue #5338
GitHub merged, its retained receipt passed the installed typed v2
terminal-receipt validator, the retained record was `closed_out` with terminal
disposition and observed state both `merged`, and its governed merge ancestry
reached both current `origin/main` and the #5340 implementation revision. That
dependency, ancestry, and active sole-writer proof remains a required validation
gate through publication and closeout. Publication remains blocked until this
post-implementation amendment has renewed design approval and the repaired
product receives a clean exact-revision rereview.

#5338 is the sole direct WP-06 execution dependency. #5336 is transitive
architecture, ownership, clean-room, and budget authority only; it is an input
to this design, not a second gate.

The landed `adl_compiler::ExecutionPlan` API and fixture inventory were
reconciled before implementation. Any later material mismatch still requires a
typed SPP/VPP amendment and renewed design review before work continues.

## Ownership and authority

WP-06 owns only `adl-v2/crates/adl-engine` and issue-local C-SDLC records. The
crate consumes the inert, canonical `ExecutionPlan` landed by #5338 and owns a
portable plan-level orchestration state machine: deterministic readiness,
bounded dispatch, joins, retry and failure propagation, quiescent checkpoints,
strict resume, and typed provider/tool port requests and completions.

The engine does not own source parsing, language validation, compilation,
records/signing, provider or tool adapters, network/filesystem/database IO,
process or thread supervision, wall-clock scheduling, credentials, cloud work,
Runtime v2, Runtime v3, C-SDLC, CLI/selector behavior, parity, cutover, deletion,
or release. Runtime v3 remains the operational runtime authority: it may host
the engine and connect its effects to governed services, but WP-06 neither
changes Runtime v3 nor redefines its supervision, recovery, or resource-policy
contracts. Provider and governed-tool adapters remain #5349 scope.

Incumbent ADL implementation and tests are behavioral evidence only. No
incumbent implementation, internal crate, Runtime v2 source, or Runtime v3
source may be copied, adapted, imported, linked, or changed for WP-06. The sole
fixture exception is read-only test access to the canonical six landed #5338
YAML fixtures through test-only `adl-language`; fixture content is neither
copied into nor linked by the product library.

## Portable execution model

The engine is a deterministic state machine driven by typed inputs. It never
creates threads, starts an async runtime, sleeps, reads a clock, or performs an
external side effect. A host repeatedly supplies an `ExecutionPlan`, bounded
configuration, zero or more typed port completions, and an explicit logical
tick. The engine returns a new snapshot plus ordered typed effects.

Plan nodes use explicit states:

1. `pending`: predecessor outcomes are incomplete;
2. `ready`: all required predecessor and join conditions are satisfied;
3. `dispatched`: one typed port request is in flight with a stable idempotency
   key and attempt number;
4. `retry_wait`: a retryable completion consumed an attempt and is waiting for
   an explicit logical-tick threshold;
5. `cancelling`: exactly one typed cancel effect is outstanding for a
   previously dispatched request;
6. `succeeded`, `failed`, or `cancelled`: terminal outcomes.

`EnginePolicy` is WP-06-owned bounded configuration keyed only to canonical
plan node and edge identities. It declares retry classes and attempt/delay
limits, join behavior (`all`, bounded `at_least(n)`, or fail-fast), failure
propagation, and cancellation behavior. #5338 supplies only inert nodes, edges,
ports, identities, and digests; it does not own or supply engine scheduling,
join, retry, or cancellation policy. Policy references to unknown plan
identities or impossible thresholds fail admission, and the canonical policy
digest is bound into checkpoints.

Each transition is total and validated. At every logical turn, newly ready
nodes are ordered by canonical compiler node identity. The engine admits only
the first `max_in_flight` nodes and retains the rest in the bounded ready set.
Port completions are applied in request identity order, independent of arrival
order. An impossible join becomes a stable typed failure rather than waiting
forever.

State-dependency inputs are resolved only from the typed output of their
declared landed edge. `application/json` output is decoded as a JSON value and
`text/*` output as a UTF-8 string; every other media type and invalid encoding
fails closed. Parsed state output is cached once, every repeated materialization
is charged before cloning, and the resolved input digest is bound into both the
request identity and idempotency key.

## Mandatory bounds and saturation behavior

`EngineLimits` rejects zero, contradictory, or plan-incompatible limits before
execution. It contains explicit ceilings for plan nodes, dependency edges,
serialized plan bytes, serialized policy bytes, ready nodes, in-flight
requests, total attempts, per-node attempts, serialized request bytes,
serialized completion bytes, completions per turn, cancellations per turn,
serialized turn-input bytes, retained output bytes, event count, checkpoint
bytes, and logical turns. No default is unbounded. A small in-memory capped
writer delegates JSON encoding to `serde_json` and stops before the byte limit;
it has no filesystem, socket, process, or other external I/O capability.

Reaching a ceiling has one declared outcome:

- admission bounds fail before dispatch;
- ready/in-flight saturation preserves canonical queued order and reports
  backpressure without dropping work;
- attempt exhaustion produces a terminal stable failure;
- output/event/checkpoint/turn exhaustion fails closed with a typed resource
  error;
- no retry, join, cancellation, or resume path may reset or bypass a consumed
  budget.

Tests must cover every limit at `limit - 1`, `limit`, and `limit + 1`, plus
permuted completion arrival and repeated fresh-process execution.

## Retry and failure semantics

Retry policy is engine-owned bounded data in `EnginePolicy`, keyed to canonical
plan node identity. It has an explicit maximum attempt count, retryable failure
classes, and logical-tick delay schedule. It is validated against, but is not
supplied by, the inert #5338 plan. The engine does not implement wall-clock
backoff or jitter.
Permanent, invalid-request, policy-denied, cancelled, dependency, saturation,
protocol, timeout, and retry-exhausted failures are distinct stable types.

Failure propagation is explicit per edge/join policy. A required predecessor
failure prevents dispatch and yields a stable dependency failure. Optional or
threshold joins may continue only when their declared condition remains
satisfiable. Unknown completion IDs, duplicate non-identical completions,
attempt mismatches, and completion-kind mismatches fail closed. Byte-identical
duplicate completions are idempotent and do not consume another attempt.

Cancellation is an explicit deterministic input, never a host timing signal.
Within one logical turn the engine first validates and applies completions in
request-identity order, then applies cancellation intents in canonical node
order; therefore a valid completion presented in the same turn wins. Cancelling
a `pending`, `ready`, or `retry_wait` node moves it directly to terminal
`cancelled` without emitting a port request. Cancelling a `dispatched` node
emits exactly one typed cancel effect for its stable request identity and moves
the node to `cancelling`; the node remains non-quiescent until a matching cancel
acknowledgement or matching terminal completion arrives. A matching terminal
completion received while `cancelling` wins and is processed normally. After a
node is terminal, a byte-identical replay is ignored idempotently and any
different or mismatched late completion is a stable protocol error. Cancellation
never refunds attempts, output, events, requests, or logical-turn budgets, and
dependency/join propagation follows the declared engine policy.

## Typed provider and tool ports

The public boundary contains separate typed provider and tool request variants,
a shared stable request identity, canonical node identity, attempt number,
idempotency key, bounded input/output envelopes, declared timeout budget, and
typed success/failure completion variants. The engine decides *when* a plan
node is eligible and emits a request; it does not decide *how* a provider or
tool is reached.

Production HTTP, model, tool, credential, sandbox, policy, and compatibility
adapters are excluded. Tests use deterministic in-memory fake ports. Runtime v3
and #5349 consumers may translate the typed effects, but may not mutate engine
snapshots or fabricate completions that violate request identity.

## Checkpoint and resume contract

Checkpoints are canonical, serializable engine snapshots, not persistence
implementations. A checkpoint may be emitted only at a quiescent boundary with
no `dispatched` or `cancelling` request. It binds the engine contract version,
compiler plan
contract version, plan source digest, canonical node/edge identity set,
limits, canonical engine-policy digest, logical turn, consumed attempt budgets,
terminal outputs, retry waits, and next event/request sequences.

The checkpoint retains a bounded canonical journal of every normalized
`TurnInput` plus each typed completion, its logical completion tick, input
digest, sequence, and completion digest. Resume deterministically replays that
turn journal from the exact initial engine and requires byte-for-byte snapshot
equality, binding logical ticks, turn count, event count, intermediate retry
outcomes, cancellations, dispatch chronology, and final state. It also requires
contiguous attempts `1..=attempts` per node, exact global sequence coverage,
and recomputed request and completion identities. A zero-turn checkpoint must
be the exact initial snapshot; later quiescent checkpoints cannot retain
`ready` work, and `pending` work must still be waiting on its dependency
decision.

Resume accepts only an exact compatible plan and limits contract. Unknown or
missing nodes, changed edges/ports, changed limits, truncated state, invalid
state transitions, attempt rollback, sequence rollback, oversized data, or a
non-canonical encoding are terminal compatibility errors. A successful resume
must produce the same ordered effects and final result as uninterrupted
execution from the same quiescent boundary. The engine never guesses how to
recover an in-flight side effect.

## COTS decisions

| Concern | Decision | Boundary |
| --- | --- | --- |
| Plan input | path dependency on the landed `adl-compiler` crate from #5338 | Consume only the reviewed inert `ExecutionPlan` public contract. |
| Landed fixture parsing in tests | test-only path dependency on canonical `adl-language` | Parse and validate every applicable landed YAML fixture before compiler and engine execution; never link language parsing into the product library. |
| Serialization | exact `serde` 1.0.229 and `serde_json` 1.0.151 pins reconciled with the landed #5338 lock | Typed snapshots, effects, canonical fixture encoding, and capped in-memory serialization; any later mismatch requires typed replanning. |
| Stable request/checkpoint identity | `sha2` 0.10.9 and `hex` 0.4.3, matching #5338 | Domain-separated SHA-256 identities over length-delimited canonical fields only. |
| Scheduling, joins, queues, state, and bounds | Rust standard-library enums, ordered collections, and stable sorts | No graph, scheduler, workflow, retry, actor, async-runtime, or persistence framework. |
| Validation-only source-authority proof | `syn` `2.0.118` in the issue-local validator | Proven Rust AST parsing replaces custom lexical/parser logic. Because macro bodies are opaque token streams, product-source macro invocations/definitions fail closed; derive/serde attributes remain available. The validator is not linked into `adl-engine`, is excluded from product LoC/dependency budgets, and builds only under `/Volumes/FastWork`. |
| Test generation | deterministic issue-local tables and checked fixtures | No RNG, property-test, fuzz, timer, network, or process dependency is required for release proof. |

The engine manifest must use exact-version requirements for all four registry
COTS, and its lockfile must preserve those exact resolved versions and registry
checksums from the approved crates.io registry source. These pins have been
reconciled with landed #5338; any later source, version, or checksum mismatch is
a mandatory typed replan, never an implicit substitution.
`adl-compiler` must resolve only as the canonical repository path
`adl-v2/crates/adl-compiler`. The landed `adl-v2/crates/adl-language` path is
permitted transitively and as a test-only direct dependency solely so AC-6 can
parse the six actual YAML fixtures before compiling and engine-executing them;
it is not a normal product dependency. Any source, path, version, checksum, or
dependency set drift requires a typed, evidence-backed COTS amendment before
review.
Forbidden direct
or transitive dependency families include incumbent ADL crates other than the
new compiler and its language input, Runtime v2/v3, C-SDLC, Tokio, async-std,
smol, HTTP/TLS,
cloud/provider/database SDKs, graph/execution/workflow engines, scheduler or
retry frameworks, filesystem persistence, and nondeterministic RNGs.

## Source, test, and validation budgets

The milestone ceilings remain 30,000 implementation LoC and 15,000 test LoC
for the complete ADL v2 product. WP-06 has a strict allocation of at most 4,000
Rust implementation LoC and 3,500 test/fixture LoC. Generated build output,
vendored source, incumbent code, code movement, build scripts, examples, or
logic moved into unmeasured scripts do not bypass the budget. Any increase
requires a typed exact-revision design amendment and independent review.

Focused warm engine validation and strict quality validation must each complete
within 120 seconds. Deterministic ordering/saturation/failure/resume proof must
complete within 300 seconds. The complete WP-06 suite, including dependency,
scope, LoC, and test measurement, must complete within 600 seconds. These are
typed PVF process-group deadlines, not elapsed-time observations after a command
returns. Cargo target, registry/git home, sccache, and temporary paths are
canonicalized beneath `/Volumes/FastWork`; symlink escape fails closed.
A typed, controlled-external `preparation-tool-cache` setup sublane has a
60-second deadline and fetches only the pinned issue-local `syn` validator
closure into FastWork before the network-denied `preparation-contract` proof.
It is setup, not acceptance evidence, and removes any undeclared cold-host
cache prerequisite from preparation.
After the dependency gate opens, a separate typed, 120-second cache-warm lane
may fetch the exact lockfile closure into the FastWork Cargo home. It is
controlled-external setup and is not validation evidence. All required engine
and post-merge validation lanes then set Cargo offline mode and pass `--offline`,
making their declared `network: denied` posture executable.

## PVF and test classification

| Lane | PVF role | Determinism | Resource profile | Release gate | Hard budget |
| --- | --- | --- | --- | --- | ---: |
| `preparation-contract` | cards/design/dependency/scope/budget contract | deterministic | small, no network | preparation gate | 120 s |
| `engine-cache-warm` | exact-lock cache setup, not proof | controlled external | small, crates registry only | optional setup | 120 s |
| `engine-focused` | state transitions, dispatch, joins, retry, failures, ports | deterministic | medium, local | required | 120 s |
| `engine-quality` | format/lint and forbidden-authority surface | deterministic | small, local | required | 120 s |
| `ordering-resume` | completion permutations, saturation edges, checkpoint/resume equivalence | deterministic | medium, local | required | 300 s |
| `engine-budgets` | exact COTS, forbidden graph, LoC, scope, full suite | deterministic | medium, local | required | 600 s |
| `post-merge-exact` | detached exact-merge full validation before closeout | deterministic | medium, FastWork clone | required | 600 s |

All implementation tests are issue-local contract, negative, boundary, or
deterministic replay tests. The `fresh_process_driver` test target is
`harness = false`; the typed ordering/resume lane builds it once under
FastWork, starts two clean OS processes with isolated output files, and requires
byte-identical artifacts. None is a flaky, remote, soak, AWS, provider, or
manual lane. Deferred lane execution during preparation is a dependency state,
not a pass and not permission to omit the lane after #5338 closes.

Before closeout, `csdlc-publish reconcile-merged` must first record a typed
merged publication for the exact governed PR head. The post-merge orchestrator
then derives that reviewed head and PR identity from canonical publication and
readiness evidence rather than accepting a caller SHA, fetches and captures
current `origin/main` as the integration revision, and proves the governed head
is an ancestor of that captured integration revision. It creates an isolated
shared clone under FastWork, checks out the integration revision detached, and
runs the offline `post-merge-exact` PVF lane there. The lane rechecks formatting,
lint, all engine tests, exact COTS, and LoC at the merged main tree without
product edits. Its typed report records reviewed-head and integration SHAs
separately and is copied with the log into issue-local evidence before closeout;
the scratch clone is removed. If the merge strategy does not preserve governed
head ancestry, closeout blocks for a typed proof-design amendment rather than
mislabeling the PR head as the integrated revision.

## No-deferral acceptance matrix

| Acceptance | Required exact-revision proof | Deferral rule |
| --- | --- | --- |
| AC-1 | Landed #5338 `ExecutionPlan` consumed without parser/compiler/runtime authority | No deferral; mismatch requires replan before code. |
| AC-2 | Canonical bounded scheduling and completion-order permutation tests | No deferral. |
| AC-3 | Engine-owned join/retry/failure policy, deterministic cancellation/late-completion transitions, saturation, and budget-edge negative tests | No deferral. |
| AC-4 | Typed provider/tool request-completion and idempotency protocol tests | No deferral. |
| AC-5 | Quiescent checkpoint compatibility and uninterrupted-versus-resumed byte-equivalence proof | No deferral. |
| AC-6 | Every applicable landed #5338 plan/port fixture mapped, with reviewed non-engine classifications | No silent skip or normalization. |
| AC-7 | Exact direct COTS source/path/version/checksum set, structural forbidden dependency scan, exact scope allowlist, non-overlapping typed claims, and zero Runtime v2/v3/C-SDLC/adaptor source changes | No deferral. |
| AC-8 | 4,000 implementation LoC, 3,500 test/fixture LoC, and 120/120/300/600-second ceilings | No deferral; variance requires replan and review. |
| AC-9 | Fetch-refreshed #5338 merge, scratch-validated typed `closed_out` receipt, PR/observed-state truth, merge-SHA and current-main ancestry, and sole-writer proof before product code | No substitute signal. |
| AC-10 | Exact-revision subagent review, typed publication, green checks, authorized merge, detached exact-merge typed post-merge proof, retained closeout receipt | No completion claim until all pass. |

## Failure and rollback

Implementation and publication fail closed on a stale dependency signal,
unreviewed #5338 contract drift, nondeterministic ordering, unbounded work,
impossible join wait, retry-budget bypass, port protocol ambiguity, duplicate
side effect, incompatible resume, forbidden dependency, Runtime source change,
budget variance, stale review, red CI, absent merge authorization, or missing
terminal evidence.

Before any consumer or selector integrates this crate, rollback is removal of
the isolated `adl-engine` crate and issue-local integration reference only.
Shared workspace membership, Runtime adapters, CLI/selector changes, incumbent
deletion, and downstream consumer rollback are separately owned and are not in
WP-06 protected scope. No Runtime v2 or Runtime v3 rollback action is needed
because this issue may not modify either runtime.

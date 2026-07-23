# Issue 5338 pure deterministic compiler design

## Status and dependency gate

The dependency gate passed: issue #5339 is merged, its retained typed lifecycle
is `closed_out`, its claim is released, and the squash merge is an ancestor of
this worktree. Implementation and local validation are complete; exact-revision
review remediation is in progress. The landed `adl-language` API and its
reviewed fixtures remain read-only compiler inputs.

## Ownership and authority

WP-05 owns only `adl-v2/crates/adl-compiler` and issue-local C-SDLC records.
It accepts an already parsed and validated `adl_language::AdlDocument` and
returns a canonical, serializable `ExecutionPlan` or stable typed compile
diagnostics. It performs no parsing, validation policy, execution, provider or
tool IO, filesystem mutation, clock or random access, environment lookup,
network access, persistence, signing, lifecycle governance, or cloud work.

Incumbent ADL compiler source and tests are behavioral evidence only. No
incumbent implementation, fixture, or internal crate may be copied, adapted,
linked, or imported into this clean-room crate.

## Compiler pipeline

The public compiler is one total, side-effect-free transformation:

1. Resolve declared language identities and references against the validated
   document without consulting external registries.
2. Expand validated workflow composition in an explicitly specified order:
   sequential edges, concurrent fan-out, saved-state dependencies, and joins.
3. Lower the expanded graph into typed plan nodes and edges.
4. Derive each node identity from a domain-separated canonical semantic path,
   never traversal order, address, process state, clock, or randomness.
5. Canonically order nodes, edges, inputs, outputs, and metadata, preserving
   only language-declared ordering.
6. Serialize to byte-stable canonical JSON for fixtures and replay comparison.

Resolution and expansion use ordered standard-library collections and explicit
stable sorts. Graph traversal uses a deterministic Kahn-style topological pass
with a sorted ready set. Diagnostics are sorted by stable code and semantic
path before return. Hash collisions fail closed rather than silently merging
nodes.

## Stable node identity contract

The identity preimage is a versioned tuple containing a fixed domain tag,
compiler contract version, root document identity, declaration identity,
composition/pattern expansion path, node role, and any language-declared
instance key. Each component is length-delimited before SHA-256 hashing. The
public ID is lowercase hexadecimal with an explicit version prefix. Incidental
map order, YAML versus JSON syntax, source location, traversal order, and
process state are excluded.

Golden vectors pin preimages and IDs. Permutation, repeated-run, clean-process,
and equivalent-source tests require identical plan bytes. Semantic changes
that affect execution identity must change the relevant node ID; unrelated
changes must not churn stable identities.

## Landed language boundary

The merged #5339 API represents composition with `WorkflowKind::Sequential`,
`WorkflowKind::Concurrent`, ordered `WorkflowStep` values, and `@state:` input
references. It deliberately rejects the incumbent top-level `patterns` and
`run.pattern_ref` syntax. The compiler therefore expands only constructs that
survive typed language validation. The characterization fixtures `branch-a`,
`branch-b`, and `fork-join` remain explicit legacy-pattern evidence and are
classified as non-input cases until a separately reviewed language version
adds an equivalent typed construct. WP-05 must not acquire YAML parsing or
silently accept rejected legacy syntax to manufacture pattern coverage.

## ExecutionPlan boundary

`ExecutionPlan` is data, not an executor. It contains a contract version,
source digest, sorted typed nodes, sorted dependency edges, declared input and
output port contracts, and bounded provenance needed to explain lowering.
There are no callbacks, trait objects, handles, futures, retry loops, provider
clients, runtime policies, timestamps, or host paths. WP-06 may consume this
plan but owns scheduling, joins, retries, resume, failure propagation, ports at
runtime, and side effects.

## COTS decisions

| Concern | Decision | Boundary |
| --- | --- | --- |
| Language input | path dependency on the landed `adl-language` crate from #5339 | Typed validated documents only; no parser duplication or incumbent dependency. |
| Serialization | `serde` 1.x plus `serde_json` 1.x, using the versions landed by #5339 | Derives and canonical fixture encoding only; no arbitrary-value escape hatch in plan semantics. |
| Stable digest | `sha2` 0.10.9 and `hex` 0.4.3 | Domain-separated SHA-256 identity and digest encoding only. |
| Ordering and graph lowering | Rust standard library ordered collections and stable sorts | No `petgraph`, parser generator, async runtime, or canonicalization framework. |
| Property/permutation proof | deterministic issue-local test generators | No random or fuzz dependency is required for the release proof; every permutation seed/case is checked in. |

The implementation lockfile must preserve reviewed versions or record a typed,
evidence-backed COTS amendment before review. Forbidden dependency families
include incumbent ADL crates, Runtime v2 or v3, C-SDLC, async runtimes, HTTP,
cloud/provider/database SDKs, execution engines, and nondeterministic RNGs.

## Budgets

The milestone ceilings remain 30,000 implementation LoC and 15,000 test LoC
for the complete ADL v2 product. WP-05 has a strict allocation of at most 3,500
Rust implementation LoC and 3,500 test/fixture LoC. Generated build output,
vendored source, copied incumbent code, and code movement do not satisfy the
budget. Any increase requires an exact-revision design amendment and review;
the limit may not be bypassed by moving logic into tests, scripts, build files,
generated source, or another crate.

Focused warm compiler validation and strict quality validation must each
complete within 120 seconds; deterministic replay must complete within 300
seconds. The full deterministic WP-05 suite, including replay/permutation and
budget proof, must complete within 600 seconds. All Cargo output uses
`/Volumes/FastWork`.

## Validation and fixtures

- Golden fixtures cover resolution, sequential/concurrent composition,
  saved-state dependency expansion, joins, ports,
  edges, stable node identity, and canonical JSON.
- Negative fixtures cover unresolved or ambiguous references, expansion
  conflicts, cycles discovered during lowering, duplicate node preimages,
  hash/identity collision handling, and resource-bound violations.
- Equivalent-document permutations and repeated fresh-process runs must emit
  byte-identical plans and diagnostics.
- The compiler must map every applicable #5339 fixture and explicitly classify
  language-rejected legacy-pattern and other non-compiler cases; no fixture is
  silently skipped.
- Contract tests prove `ExecutionPlan` contains no execution authority and is
  consumable as deterministic data by the future WP-06 boundary.
- Dependency, source/test LoC, strict Clippy, diff hygiene, focused latency,
  full latency, and typed doctor evidence are captured at the exact review
  revision.

The issue-local `validate-compiler.sh` is the executable adapter for declared
lanes. Before the dependency and crate exist it exits with a typed blocked
message; this keeps the VPP executable without representing deferred proof as
passed.

## Failure and rollback

Compilation fails closed on missing or ambiguous references, unspecified
ordering, identity collisions, nondeterministic output, unclassified language
fixtures, forbidden dependencies, or budget variance. Before any consumer is
selected, rollback is removal of the isolated compiler crate. Shared workspace
or selector membership is outside this issue's protected scope and requires
separately owned integration. This issue does not mutate incumbent behavior or
select ADL v2.

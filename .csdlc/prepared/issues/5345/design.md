# Issue 5345 thin CLI and selector design

## Status and dependency gate

This packet prepares WP-10 only. Product implementation is prohibited until
WP-04 through WP-09 are each merged, typed `closed_out`, represented by a
retained shared-Git closeout receipt, and ancestral to the implementation
base. The executable dependency gate checks issues `#5339`, `#5338`, `#5340`,
`#5342`, `#5341`, and `#5349`. For each dependency it requires the current typed
projection to equal the retained shared-Git terminal record, a merged GitHub PR
observation, green required-check evidence bound to the merged SHA, released
claim, and ancestry to the exact implementation revision. A merged PR is an
immutable GitHub fact; the typed closeout receipt is the repo-authoritative
observation of that fact. The gate fails closed on any mismatch.

## Ownership

WP-10 owns a thin `adl-v2` owner CLI, its generation-selector library, and the
tracked installer that places accepted binaries outside Cargo output. It does
not own language, compiler, engine, records/signing implementation, Runtime v3,
provider or governed-tool adapters, C-SDLC lifecycle commands, release cutover,
or deletion.

The commands `validate`, `schema`, `plan`, `run`, `inspect`, `sign`, and
`verify` are adapters over the reviewed WP-04 through WP-09 library contracts.
They must not fork duplicate parsers, graph logic, execution engines, signing,
provider transport, or Runtime behavior. Machine-readable success and failure
payloads use stdout; diagnostics and observability use stderr.

## Command and component contract

The CLI uses one `clap` command tree with typed arguments and stable JSON
envelopes. Each command constructs one request for an upstream component and
renders its typed response. Exit codes are stable and documented. No command
silently selects a generation, performs network access, obtains credentials,
or changes selector state.

The selector is a small library used by the CLI and installer. A selection
record identifies a generation, exact executable digest, installation receipt,
and previous selection. Mutation takes an exclusive process-safe lock, validates
the target binary and receipt, writes a complete temporary record, atomically
persists it, re-reads it, and emits a deterministic selector receipt. A failed
validation or write leaves the prior selection byte-for-byte intact.

The default location is resolved from explicit configuration or the stable ADL
data root. Tests always use an isolated temporary root. No repository path,
username, host path, IP address, or credential location is hard-coded.

## Selector, cutover, and rollback boundary

WP-10 implements explicit generation selection and the reversible transaction.
It does not authorize or perform the milestone default-generation switch. Issue
`#5343` owns the reviewed cutover transaction, and `#5344` owns soak and rollback
proof. WP-10 must expose the primitives those issues consume:

- inspect current and previous generation;
- verify an installed generation and exact receipt before selection;
- select explicitly with compare-and-swap protection;
- restore the prior verified generation with the same transaction;
- emit an exact, non-secret receipt for every successful mutation;
- fail closed without changing state on stale expectation, invalid digest,
  missing receipt, unsupported schema, interrupted write, or lock failure.

There is no implicit fallback. A caller must request rollback explicitly or use
the later reviewed cutover controller. Runtime v2 and incumbent ADL surfaces are
read-only behavioral evidence and are never imported into the new crate.

## COTS decisions

| Concern | Decision | Boundary |
| --- | --- | --- |
| CLI parsing | `clap` 4.6.1 with derive | Command grammar and help only; no command implementation generation. |
| Serialization | `serde` 1.0.228 and `serde_json` 1.0.150 | Stable request, response, selector, and receipt envelopes. |
| Atomic persistence | `tempfile` 3.27.0 | Same-directory temporary file and persist operation; no custom temporary-name scheme. |
| Process-safe locking | `fs2` 0.4.3 | Exclusive selector lock only; lock acquisition has a bounded failure path. |
| Digests | `sha2` 0.10.9 | Installed executable and receipt identity only; signing delegates to WP-07. |
| Errors | narrow issue-local enums | No general CLI framework or hidden retry layer. |

These are reviewed candidate pins, grounded by exact lockfile paths and SHA-256
digests in `cots-lock-baseline.json`; they are not a claim that the future
`adl-v2/Cargo.lock` closure already exists. Implementation must produce that
lockfile, resolve these exact direct pins, retain the closure, and repeat
exact-revision review, or record a typed COTS amendment before proceeding. No HTTP, cloud, provider,
database, async-runtime, terminal-UI, plugin, incumbent ADL, Runtime, or C-SDLC
dependency is allowed in the default CLI/selector graph except the explicit
WP-08/WP-09 adapter interfaces required by `run`.

## Budgets

WP-10 has hard preparation targets of at most 2,500 Rust implementation lines
and 2,500 test/fixture lines. The command adapter should remain visibly thin;
selector state and transaction code should remain under 800 implementation
lines unless exact-revision review approves evidence-backed variance. New
modules should remain under 500 lines where practical and any module above
1,000 lines is a stop condition.

Focused warm tests and strict Clippy each have a 120-second budget. The complete
offline WP-10 suite, install smoke, selector interruption/rollback matrix, LoC,
and dependency checks have a 600-second ceiling. Cargo output must be under
`/Volumes/FastWork`; installed operational binaries must be outside Cargo target
directories.

## Validation design

- Every command has success, malformed-input, upstream-error, and JSON stdout /
  stderr-separation tests.
- CLI tests prove argument order, default absence, stable exit codes, and that
  command adapters delegate rather than reimplement upstream semantics.
- Selector tests cover initial selection, compare-and-swap, stale writer,
  concurrent lock contention, invalid digest, missing receipt, unsupported
  schema, interrupted temporary write, atomic persistence, re-read verification,
  explicit rollback, and preservation of prior bytes on every failure.
- Installer tests use an isolated root and prove exact binary digest, stable
  receipt, executable permissions, reinstall idempotence, and no Cargo-target
  dependency.
- Dependency and source scans reject forbidden crates and authority expansion.
- The fresh-install selector smoke owned jointly with #5344 is planned but does
  not claim release readiness by itself.

## Failure and rollback

Any incomplete dependency gate, command-level authority duplication, selector
mutation without exact target verification, non-atomic state change, secret or
host-path disclosure, hard-coded address, forbidden dependency, budget breach,
or validation nondeterminism stops review and publication. Before #5343 selects
ADL v2 by default, rollback is removal of the isolated installation and
selector record. After selection, rollback must use the verified previous
generation transaction and evidence owned by #5343/#5344.

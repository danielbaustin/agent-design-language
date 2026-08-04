# Issue #5350: exact-revision normalized shadow parity design

## Status

This packet is preparation-only. It defines the complete WP-11 comparison
contract, but it does not run parity, change product code, publish a PR, switch
the default generation, authorize cutover or deletion, or modify Runtime v2.

Product execution is fail-closed until all dependency evidence named below is
merged, typed `closed_out`, retained in shared-Git receipts, and ancestral to
the exact comparison revision. A green check, open PR, local fixture, or
`csdlc-doctor` pass without terminal receipt truth is insufficient.

## Ownership and dependency gates

WP-11 owns the normalized comparison and disposition record, not the systems
being compared.

- WP-03 #5337 owns the approved v1 characterization corpus, normalization
  rules, retained observations, and pinned incumbent identity.
- WP-10 #5345 owns the complete ADL v2 CLI and selector surface. Its terminal
  receipt is the direct ADL v2 execution gate and transitively incorporates
  WP-04 through WP-09.
- WP-10A #5497 owns distributed-workcell convergence; child #5501 must supply
  the completed live proof. Both terminal receipts are required.
- Runtime parity issues #5591, #5592, #5589, and #5590 own the four Runtime v3
  lanes. Adapter issues #5341 and #5349 own the ADL-to-Runtime and provider/tool
  boundaries. WP-11 consumes their exact evidence; it does not reimplement or
  relabel it.
- Runtime acceptance #5361 is downstream of WP-11 and therefore is not a
  prerequisite for running WP-11. WP-11 instead produces one of #5361's
  acceptance inputs.

No execution starts if an active typed claim collides with the eventual
parity-runner or evidence paths. The preparation claim remains issue-local and
does not reserve product paths prematurely.

## Exact comparison identities

The comparison manifest binds two immutable subjects.

### ADL v1 baseline

The canonical source is the merged #5337 corpus:

- `adl-characterization/corpus/v1/corpus.yaml`;
- incumbent source revision
  `19c2b6e2ad18bddc75db9231643a54b2a446ce72`;
- incumbent executable SHA-256
  `f558fa2111474e2fab540f8d0244be82cdb727ebbaa15aee758d8a7d57d0969c`;
- 25 cases, 75 observations, 23 required behaviors, two equivalence groups,
  and one difference group;
- retained raw and normalized observations plus
  `observations/v1/verification.json`.

The execution lane must verify the corpus bundle, executable digest, evidence
envelopes, portable stream hashes, repetitions, and normalization derivation
before comparing any v2 result.

### ADL v2 candidate

The candidate identity is captured only after WP-10 closes. It includes:

- exact integrated Git revision ancestral to current `origin/main`;
- exact installed ADL v2 executable SHA-256 and generation/selector identity;
- exact Cargo lock digest and command-contract manifest digest;
- exact corpus bundle digest inherited from #5337;
- terminal receipt digests and merged revisions for every direct dependency;
- a cleared environment, isolated home/temp/output roots, denied network, no
  credentials, and the same portable root/work tokenization used by #5337.

The runner rejects moving branches, mutable paths, unpinned binaries, altered
corpus data, undeclared environment, unknown command shapes, or a comparison
where either subject cannot be reproduced from retained identity.

## Approved ADL corpus

Every #5337 case is mandatory. The coverage set is:

1. CLI help and version;
2. six-primitives planning;
3. graph JSON and prompt projection;
4. fork/join ordering;
5. map and branch declaration-order equivalence;
6. sequential-order difference;
7. invalid argument, malformed YAML, and schema rejection;
8. unknown provider, agent, task, tool, and workflow references;
9. unsupported run field, missing state, and dependency cycle;
10. repeated portable-byte stability;
11. credential-free local mock execution;
12. Ed25519 sign, verify, and tamper rejection.

The machine-authoritative case and behavior list remains the exact merged
`corpus.yaml`; this prose cannot add, remove, rename, or excuse a case.

## Normalization and comparison

The runner first verifies each subject independently, then derives comparison
records. It may only apply normalization rules already declared and proved by
#5337. Object-key ordering may normalize where declared; arrays, exits,
diagnostics, semantic identifiers, signature verdicts, and sequential order do
not. A no-op or unknown normalization rule fails closed.

Each case compares exact command shape, portable arguments, exit status,
portable stdout/stderr digests, normalized semantic payload, required output
fragments, repetition stability, and equivalence/difference-group behavior.
The report recomputes totals from case rows and binds every row to both subject
identities and the corpus digest.

## Mismatch disposition contract

Every case ends in exactly one disposition:

- `exact_match`: exact portable observations agree;
- `normalized_match`: only an approved #5337 normalization accounts for the
  difference;
- `approved_intentional_difference`: behavior differs by an explicit reviewed
  design decision, linked owner issue, rationale, risk, replacement proof,
  reviewer identity, and rollback impact;
- `regression_blocker`: v2 violates required characterized behavior;
- `unsupported_blocker`: v2 cannot execute or prove the required case;
- `evidence_invalid`: either subject, corpus, identity, or evidence envelope
  fails verification.

There is no `unknown`, `later`, `fixture_only`, `close_enough`, or silently
ignored state. Only exact/normalized matches and reviewed intentional
differences are nonblocking. A blocker or invalid-evidence row prevents WP-11
completion, #5361 acceptance, soak, cutover, and deletion.

## Runtime v3 and WP-10A evidence overlay

ADL v1/v2 corpus comparison and Runtime v3 parity are separate sections of one
WP-11 acceptance packet. Runtime evidence receives credit only when the exact
retained proof satisfies the canonical ten-group plan: initialized process,
secure canonical ingress, production component execution, positive and
negative evidence, exact revision, and graceful shutdown/recovery where
stateful. Fixtures, metadata, library-only calls, fixed bootstrap behavior,
degraded executors, and Runtime v2 source reuse receive no live credit.

The Runtime overlay binds terminal evidence from #5591/#5592/#5589/#5590,
#5341, and #5349 and records each canonical proof group as `pass` or `blocker`.
The WP-10A overlay independently binds terminal #5497 and live #5501 evidence.
Neither overlay may be inferred from issue closure alone, and neither can hide
an ADL corpus mismatch.

## Outputs

Future execution produces one canonical packet beneath an issue-owned evidence
directory containing:

- immutable subject and dependency identity manifest;
- one row for each corpus case and every required behavior;
- equivalence and difference-group results;
- mismatch register with complete reviewed dispositions;
- Runtime v3 ten-group evidence overlay;
- WP-10A live-proof binding;
- deterministic summary with recomputed counts and zero unclassified rows;
- exact command/log references, validation report, review, and rollback gate.

No output changes a selector or default. Rollback for this issue means discard
the candidate comparison run, preserve immutable evidence, and keep ADL v1 and
Runtime v2 available while the owning defect is repaired.

## COTS and reuse decisions

WP-11 should extend or compose the merged independent `adl-characterization`
harness rather than create another parser, process supervisor, hasher, schema
validator, signature implementation, or timeout mechanism. It consumes the
exact dependency closure and lockfile reviewed with #5337, including Serde,
serde_json/serde_yaml, jsonschema, SHA-2, wait-timeout, walkdir, Clap,
Ed25519-dalek, tempfile, assert_cmd, and predicates. Any version or dependency
change requires a typed design amendment and new review. No network, provider,
cloud, database, workflow-engine, async-runtime, or Runtime crate dependency is
permitted.

## Budgets and PVF

- Preparation contract: 120 seconds, deterministic, no network.
- Corpus identity and evidence verification: 120 seconds.
- Exact v1/v2 shadow comparison: 300 seconds.
- Runtime/WP-10A evidence overlay and mismatch audit: 120 seconds.
- Complete validation including strict lint, scope, dependency, LoC, test,
  deterministic rerun, and exact-review proof: 600 seconds.
- The per-lane execution caps total 1,260 seconds. The card-level 7,200-second
  validation allowance is an outer lifecycle ceiling for cold setup, retained
  evidence assembly, and two independent reviews; it does not widen any lane
  cap.
- Any new or changed parity implementation is capped at 1,500 Rust source
  lines and 2,000 Rust test/fixture lines; the preferred result is smaller by
  reusing the #5337 harness. Generated output and scripts cannot hide code.
- The parity implementation may add at most 120 tests/fixture cases. The 25
  corpus cases remain mandatory and are not multiplied to inflate this count;
  deterministic repetitions are runtime observations, not separate tests.
- All Cargo target, temporary, and evidence-staging paths remain beneath
  `/Volumes/FastWork`; retained committed paths are repo-relative.

Execution acceptance has no deferrals. Every corpus case, behavior, comparison
group, Runtime proof group, WP-10A binding, budget check, and exact-revision
review must be present at the same accepted revision.

The future parity lanes are specifications at preparation time, not executable
proof. `validate-parity.sh` must remain fail-closed until the dependencies are
terminal and a later typed implementation step replaces its explicit stub with
the bounded COTS-based runner. WP-11 receives no readiness or parity credit
from the stub, and publication remains prohibited while any planned lane is
unimplemented.

## Stop conditions

Stop before execution or publication if any dependency lacks merged typed
terminal receipt and ancestry, either subject identity is mutable or
unverifiable, the #5337 corpus changes without renewed review, a command could
use network or credentials, a mismatch is unclassified, Runtime proof is
fixture/metadata/library-only, WP-10A live proof is absent, paths collide, a
budget is exceeded without reviewed amendment, or Runtime v2/product code
would need modification.

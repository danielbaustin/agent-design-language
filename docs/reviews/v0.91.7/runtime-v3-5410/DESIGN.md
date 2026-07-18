# Runtime v3 Live Kernel Assembly And Authenticity (#5410)

## Decision

The `serve` command must construct the operational Runtime v3 kernel through
the component and contract registries. The proof topology remains available
only to explicitly proof-oriented commands. Live startup must fail closed when
required factories, runtime bindings, signed continuity identity, or time
qualification are unavailable.

## Required Service Set

The live composition root freezes the service inventory in code and tests:

- operational: agent runtime, shepherd, provider, scheduler, Chronosense,
  ACIP, A2A, cloud bridge, checkpoint store, and lifelog;
- reasoning: graph, loop, evaluation/feedback, adaptation state, and mutation
  gate;
- governance: ingress, Freedom Gate, AEE, and governance audit;
- cognition: moral/affect/wellbeing, curiosity/intelligence/theory-of-mind,
  and cognition review record; and
- kernel infrastructure: observability, system weather, and signed continuity.

External provider, cloud, and actuation executors remain injected runtime
bindings. Construction refuses absent required bindings rather than installing
test doubles silently.

## Continuity

Live restore and checkpoint use the existing Ed25519 `CheckpointCoordinator`.
The signed manifest binds generation, topology hash, canonical configuration
hash, signer identity, service schemas, and snapshot digests. Unknown signers,
forgery, substitution, rollback, or identity drift refuse API readiness.
Signing material is supplied outside canonical runtime configuration.

## Time Authority

Chronosense begins degraded. A bounded adapter uses the maintained `rsntp`
crate; only a successful response within configured offset and round-trip
bounds may publish authoritative time. Timeout, DNS/transport failure,
excessive uncertainty, and cancellation remain explicitly degraded.

## Inventory Truth

The #5175 counts are labeled historical. A deterministic generator derives the
current Runtime v3 Rust LoC, direct dependencies, tests, and parity-module count
from tracked source and emits a current JSON artifact checked by tests.

## Scope And Budget

- New production modules: `assembly.rs`, `time.rs`, and a narrow live
  continuity adapter only when the existing coordinator cannot be called
  directly.
- Integrate only `adl-runtime-kernel`; do not modify Runtime v2 or `adl-runtime`.
- Add no custom networking, signing, retry, graph, or serialization framework.
- Keep Runtime v3 implementation below 12,000 Rust source lines and below
  1,000 tests.
- Target no more than 1,200 net new production Rust lines for this issue.

## Validation

Focused tests must prove exact live membership, dependency-first readiness,
reverse shutdown, missing-binding refusal, signed restore and forgery refusal,
degraded-until-qualified time, binary-level refusal before API readiness, and
deterministic inventory regeneration. The full Runtime v3 test and strict
Clippy lanes remain the integration proof.

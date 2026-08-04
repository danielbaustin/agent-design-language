# #4760 Memory Palace Implementation/Proof Design

## Status And Boundary

This is a preparation artifact. It defines the smallest real implementation and
proof scope for later execution of issue #4760. It does not implement Memory
Palace, acquire execution authority, accept ADR 0051, publish a PR, or claim
v0.92 readiness.

The existing C-SDLC record remains in `initialized` with an expired preparation
claim. Per operator direction, that state is truthful and non-blocking for this
packet. The execution owner must acquire a fresh typed v2 issue-local claim
before changing lifecycle projections or product code.

## Source Evidence

- GitHub issue #4760: implemented, integrated, evidence-backed MVP handoff.
- `docs/adr/0051-chronosense-and-memory-palace-adr-disposition.md`: ADR 0051 is
  deferred until implementation proof, continuity semantics, storage/retrieval
  boundaries, and runtime handoff evidence exist.
- `docs/milestones/v0.91.8/ADR_PLAN_v0.91.8.md`: #5007 / WP-21 owns Memory
  Palace acceptance; disposition remains deferred pending implementation.
- `docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md`: #4760 and
  #5007 jointly supply the Memory Palace activation row.
- `docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml`: WP-21/#5362 follows
  WP-20/#5363 in milestone execution ordering.
- `docs/milestones/v0.92/features/MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md`:
  topology, working set, provenance, stale-context detection, and redaction are
  planned expectations, not current runtime truth.
- `docs/adr/0007-obsmem-external-boundary.md`: ObsMem remains an external
  deterministic contract boundary rather than a hard runtime backend.
- `docs/adr/0010-chronosense-substrate.md`: temporal anchors and inspectable
  continuity remain the accepted substrate.
- `adl/src/obsmem_contract/models.rs`: existing `MemoryRecord`,
  `MemoryCitation`, `MemoryTemporalAnchor`, and temporal query validation.
- `adl/src/long_lived_agent/types.rs`: `AgentSpec.memory` is an existing
  declared configuration surface.
- `adl/src/long_lived_agent.rs`: cycle `decision_request.json` currently emits
  an empty `memory_refs` list, which is the narrow integration point.

## One Concern

Implement one deterministic Memory Palace handoff from declared ObsMem-shaped
records into the long-lived-agent cycle decision request. The implementation
must materialize a bounded, reviewable context packet and make that packet an
actual runtime consumer input. It must not become a new memory backend, search
engine, scheduler, identity system, or v0.92 runtime.

## Minimal Contract

The implementation should add one module that accepts declared input records
compatible with the existing ObsMem contract and emits:

1. `MemoryPalaceTopologyPacket`
   - named room and anchor identifiers;
   - repository- or run-relative source citations with stable hashes;
   - Chronosense-compatible temporal anchors and continuity identifiers;
   - deterministic traversal order.
2. `MemoryPalaceWorkingSetPacket`
   - a bounded subset selected from declared inputs;
   - explicit inclusion/exclusion reasons;
   - provenance for every selected item;
   - no raw chat transcript authority.
3. `MemoryPalaceStaleContextReport`
   - stale, missing, malformed, and provenance-mismatch dispositions;
   - fail-closed result when a required anchor is unsafe.

The same normalized input, bounds, and temporal reference must serialize to the
same packet bytes. Ordering must be canonical before hashing or writing.

## Exact Runtime Handoff

The existing `AgentSpec.memory` value should accept one optional, explicit
configuration object:

```yaml
memory:
  memory_palace:
    input_ref: memory_palace_input.json
    max_working_set_items: 16
    stale_after_ms: 86400000
```

`input_ref` must be relative to the agent state/spec boundary and pass the same
host-path/privacy posture used by ObsMem. When configured, one cycle must:

1. read and validate the declared input;
2. build the topology, working set, and stale-context report;
3. write `cycles/<cycle-id>/memory_palace_context.json`;
4. place `memory_palace_context.json` in `decision_request.memory_refs`;
5. refuse the handoff when required provenance, redaction, bounds, or temporal
   invariants fail.

When not configured, existing behavior remains `memory_refs: []`. This is
backward compatibility, not proof of the feature.

## Intended Execution Paths

Product paths are intentionally bounded to:

- `adl/src/memory_palace.rs` - contract, normalization, validation, packet build.
- `adl/src/lib.rs` - module export only.
- `adl/src/long_lived_agent.rs` - declared-input load, packet write, and
  `decision_request.memory_refs` consumer hook.
- `adl/tests/memory_palace_tests.rs` - integrated packet and runtime consumer
  tests.
- `adl/tests/fixtures/memory_palace/long_running_context.json` - deterministic
  declared input fixture.
- `.csdlc/evidence/4760/` - execution commands and retained proof only.

No shared milestone, product, ADR, release, or feature document is an intended
implementation path. #5007 owns the later ADR acceptance action after it
consumes exact #4760 evidence.

## ObsMem And Chronosense Boundaries

- ObsMem owns durable storage, retrieval ranking, and backend interaction.
- Memory Palace consumes validated records/citations and owns topology plus the
  bounded working-set handoff; it does not write or rank the ObsMem store.
- Chronosense owns temporal meaning and continuity anchors. Memory Palace
  preserves `MemoryTemporalAnchor` fields and uses explicit staleness inputs; it
  does not create another clock or redefine ADR 0010.
- The long-lived agent owns the runtime cycle and consumes only the emitted
  packet reference.

Chronosense dependencies #4765, #4768, and #4771 are compatibility inputs. The
execution owner must verify their accepted field/behavior contracts at the
execution revision. Missing compatibility evidence blocks completion; it does
not justify a parallel temporal model.

WP-20/#5363 is the milestone ordering predecessor for WP-21/#5362. The
execution owner must verify its current disposition before starting product
work. GitHub merge/ancestry may provide that ordering evidence; unrelated typed
closeout receipts do not gate #4760 preparation or substitute for product proof.

## COTS Decision

No new COTS dependency is justified for the MVP. Reuse already-declared
`serde`, `serde_json`, `chrono`, and `sha2`, plus existing ObsMem contract
types. Do not add a vector database, graph database, cache service, embedding
provider, network service, or new Rust crate. A new dependency requires a typed
SPP replan with license, supply-chain, determinism, and rollback analysis.

## Budgets

Execution budget:

- production Rust: at most 500 net new lines;
- tests and fixtures: at most 500 net new lines;
- issue-local evidence: at most 200 lines excluding generated command logs;
- total reviewable source/fixture delta: at most 1,200 net new lines;
- implementation and focused local proof: 6 to 10 engineer hours;
- focused local validation: at most 45 minutes on a warm same-host cache;
- bounded review and fixes: at most 90 minutes.

Exceeding 1,200 lines, 10 hours, or the five intended product paths is a replan
trigger, not permission to hide adjacent runtime work in #4760.

## Required Proof And No-Deferral Bar

The issue cannot close, and #5007 cannot accept ADR 0051, unless all of these
are present at one exact implementation revision:

- deterministic contract/unit proof;
- negative proof for stale context, missing citation/hash, host-path/private
  content, temporal mismatch, and working-set overflow;
- integrated long-lived-agent proof that `decision_request.memory_refs`
  contains the emitted packet;
- replay proof that identical declared inputs produce identical packet bytes;
- retained runtime artifact packet and bounded review with all findings fixed
  or explicitly blocking.

Required local lanes may not be deferred to CI. CI may repeat them, but a remote
or paid lane is not required unless execution introduces a platform-specific
surface. Planning text, mocks alone, isolated serialization, and an unconsumed
artifact are non-proving.

## Rollback And Stop Criteria

Stop and rollback before merge if any of these occur:

- existing unconfigured long-lived-agent behavior changes;
- packet generation is nondeterministic;
- private/absolute host paths or unproven summaries enter the packet;
- stale or temporally incompatible context is silently accepted;
- the runtime consumer can reference a packet that failed validation;
- ObsMem storage/ranking or Chronosense clock semantics must be rewritten;
- a new external service or crate becomes necessary without replan.

Rollback is additive and path-bounded: remove the consumer hook, module export,
new module, test, and fixture, then verify unconfigured cycles again emit the
prior empty `memory_refs`. Persisted proof packets are evidence only and must
not be treated as runtime state after rollback.

## ADR Gate

#4760 supplies evidence; it does not accept the architecture decision. #5007
must remain deferred until it can cite the exact implementation revision,
contract semantics, ObsMem/Chronosense boundary, runtime consumer proof,
negative/replay proof, and review disposition produced by #4760.

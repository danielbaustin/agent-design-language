# ADR 0058: Memory Palace Context Handoff Architecture

- Status: Accepted
- Date: 2026-07-31
- Accepted in: v0.91.8
- Related issues: #5007, #4760, #4765, #4768, #4771, #5362
- Related PRs: #5740
- Related ADRs: ADR 0007, ADR 0010, ADR 0011, ADR 0013, ADR 0051
- Source evidence:
  - PR #5740, merged Memory Palace proof head `9719252262913351144a20adf0affb7ed4b5480d`
    with merge commit `d3dbfb31ba4bd53f4166ee5e09da2a8b9f89968e`
  - GitHub issue #4760 observed closed after PR #5740 merged
  - #4760 SOR at merged PR #5740 head: `.csdlc/issues/4760/cards/sor.md`
  - #4760 review at merged PR #5740 head: `.csdlc/issues/4760/cards/srp.md`
  - #4760 focused proof at merged PR #5740 head: `.csdlc/evidence/4760/memory-palace-focused-runtime.log`
  - #4760 diff hygiene at merged PR #5740 head: `.csdlc/evidence/4760/diff-hygiene.log`
  - #4760 implementation at merged PR #5740 head: `adl/src/memory_palace.rs`
  - #4760 runtime consumer hook at merged PR #5740 head: `adl/src/long_lived_agent.rs`
  - #4760 tests and fixture at merged PR #5740 head: `adl/tests/memory_palace_tests.rs` and `adl/tests/fixtures/memory_palace/long_running_context.json`
  - `docs/adr/0051-chronosense-and-memory-palace-adr-disposition.md`

## Context

ADR 0051 deliberately deferred an accepted Memory Palace decision until
implementation evidence existed. The missing evidence was not planning text; it
needed a concrete Memory Palace handoff path, continuity semantics, storage and
retrieval boundaries, negative behavior, and long-lived-agent runtime
consumption.

PR #5740 supplied that proof surface for #4760 at final head
`9719252262913351144a20adf0affb7ed4b5480d` and merged through
`d3dbfb31ba4bd53f4166ee5e09da2a8b9f89968e`, closing #4760. This ADR therefore
accepts the architecture direction from the final merged #4760 proof surface
while keeping #5007 limited to the ADR decision and evidence-mapping outcome.

## Decision

Memory Palace is the deterministic long-running context handoff packet layer
between ObsMem-shaped memory records and the long-lived-agent runtime decision
cycle.

Memory Palace owns:

- validating bounded memory input and agent memory configuration
- canonicalizing records into replay-stable context packet bytes
- building an inspectable topology of rooms and anchors from workflow, run,
  continuity, citation, and temporal-anchor data
- selecting a bounded working set and recording overflow exclusions
- rejecting missing citations, private or host-local references, stale context,
  future temporal anchors, and continuity mismatches before runtime consumption
- emitting `memory_palace_context.json` for the long-lived-agent cycle path and
  adding that reference to `decision_request.memory_refs` and the cycle manifest

ObsMem owns the underlying memory-record shape and citation/temporal-anchor
fields consumed by Memory Palace. Memory Palace does not replace ObsMem storage,
general retrieval, or source-of-truth record retention.

Chronosense owns temporal semantics and anchor validation. Memory Palace consumes
Chronosense-compatible effective times and continuity identifiers to reject stale
or future context and to preserve the continuity boundary in the handoff packet;
it does not become a clock, scheduler, or distributed temporal truth authority.

The long-lived-agent runtime owns cycle execution and artifact retention. Its
#4760 hook writes the Memory Palace context packet into the cycle directory,
sanitizes it with public artifacts, exposes it in `decision_request.memory_refs`,
and retains it in `cycle_manifest.artifacts`.

## Proof To Claim Table

| Claim | Evidence | Decision boundary |
| --- | --- | --- |
| Memory Palace has a concrete implementation surface. | PR #5740 final head `9719252262913351144a20adf0affb7ed4b5480d`, merge `d3dbfb31ba4bd53f4166ee5e09da2a8b9f89968e`, `adl/src/memory_palace.rs`, #4760 SOR. | Accepted as the merged #4760 proof surface; #5007 adds no runtime implementation. |
| Packets are deterministic and replay-stable. | `memory_palace_packet_is_deterministic_after_canonical_ordering`, `memory_palace_fixture_builds_deterministic_obs_mem_handoff`, and `context_packet_bytes` proof in #4760 focused runtime log. | Deterministic packet generation is accepted for the bounded MVP input schema. |
| Memory Palace consumes ObsMem-shaped records rather than replacing ObsMem. | `MemoryPalaceInput.records: Vec<MemoryRecord>`, citation validation, and fixture-driven tests. | ObsMem remains the record/citation substrate. |
| Chronosense-compatible continuity and temporal boundaries are enforced. | `MemoryTemporalAnchor` validation, `required_continuity_id`, stale/future temporal rejection, and focused negative tests. | Memory Palace consumes temporal anchors; it does not become the time authority. |
| Runtime handoff is consumed by the long-lived-agent path. | `long_lived_agent_cycle_consumes_memory_palace_context_ref`, `decision_request.memory_refs`, and `cycle_manifest.artifacts.memory_palace_context`. | The accepted path is one long-lived-agent cycle consumer hook, not a universal runtime migration. |
| Boundary failures are fail-closed. | Tests for missing citation hash, stale context, private paths, temporal mismatch, working-set overflow, and continuity mismatch. | Negative behavior is proven for the MVP validation surface. |

The issue-local expanded table is retained at
`.csdlc/evidence/5007/proof-to-claim-table.md`.

## Consequences

- ADR 0051's deferred Memory Palace obligation is satisfied by merged #4760 proof
  rather than by planning intent.
- v0.92 handoff planning may refer to Memory Palace as the accepted bounded
  context-handoff architecture while preserving the bounded #4760 proof scope.
- Future Memory Palace work should extend the packet, topology, retrieval, or
  runtime-consumer contract through new evidence-backed issues rather than
  silently widening this ADR.
- Future changes after the #5740 merge require a new issue or ADR amendment
  rather than retroactive scope expansion in #5007.

## Alternatives Considered

### Keep Memory Palace Deferred

Rejected. PR #5740 now provides the implementation, runtime-consumer, validation,
and review evidence ADR 0051 required for an accepted bounded architecture
decision.

### Treat Memory Palace As ObsMem Storage

Rejected. The proof consumes ObsMem-shaped records and citations but does not
replace ObsMem as the durable memory substrate.

### Treat Memory Palace As Chronosense

Rejected. The proof consumes temporal anchors and continuity identifiers but
does not own clocks, temporal indexing, or distributed time truth.

## Validation Notes

Local #5007 validation should prove documentation hygiene, source-reference
grounding, and lifecycle truth. Runtime proof remains #4760's proof surface:
`git diff --check` passed, and
`.csdlc/prepared/issues/4760/validate_memory_palace.sh` passed at PR #5740 final
head, including five Memory Palace contract tests and two integration tests.

## Non-Claims

- This ADR does not add new runtime code beyond the merged #4760 implementation.
- This ADR does not perform or claim typed C-SDLC closeout for #5007.
- This ADR does not claim broad v0.92 readiness, release approval, or a universal
  runtime memory migration.
- This ADR does not claim long-running context is solved beyond the bounded MVP
  handoff packet and long-lived-agent consumer path proven by #4760.
- This ADR does not authorize new runtime code, providers, AWS, databases, or
  broad v0.92 implementation.

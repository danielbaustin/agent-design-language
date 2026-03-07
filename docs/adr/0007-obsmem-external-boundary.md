# ADR 0007: ObsMem External Boundary for v0.75

## Status

Accepted (v0.75).

## Context

ADL v0.75 introduces ObsMem indexing/retrieval surfaces while preserving core
runtime determinism and replay guarantees. Review feedback highlighted missing
architectural documentation for why ObsMem is kept outside core runtime logic
and how versioned contracts should be treated.

Current runtime integration points are:
- `swarm::obsmem_contract` (`ObsMemClient` trait and request/response types)
- `swarm::obsmem_adapter` (runtime-facing adapter to contract client)
- `swarm::obsmem_indexing` and `swarm::obsmem_retrieval_policy` (deterministic
  indexing/query policy behavior)

## Decision

For v0.75, ObsMem remains an external subsystem behind a stable contract
boundary and is not embedded as a hard runtime dependency.

The canonical integration model is:
1. ADL runtime emits deterministic artifacts/traces.
2. Adapter builds validated contract requests from those artifacts.
3. An `ObsMemClient` implementation handles write/query behavior.
4. Retrieval remains optional and must not change replay determinism.

Versioning and contract expectations:
- Contract payloads are validated before client usage.
- Validation failures return explicit contract errors.
- Request/path surfaces are relative-path constrained and privacy-safe.
- Future backends may evolve independently as long as they honor contract
  semantics and deterministic ordering guarantees.

## Consequences

Positive:
- Preserves runtime modularity and reduces coupling risk.
- Keeps replay-sufficient artifacts independent of memory backend availability.
- Makes deterministic validation behavior testable at the contract boundary.

Trade-offs:
- No single built-in memory backend in core runtime for v0.75.
- Users must treat memory operations as boundary-driven integration surfaces.

## Related References

- `docs/OBSMEM.md`
- `docs/milestones/v0.75/OBSMEM_INTEGRATION_CONTRACT_0.75.md`
- `docs/milestones/v0.75/DESIGN_0.75.md`

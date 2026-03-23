# ObsMem User Guide (v0.75)

This guide explains the user-facing ObsMem surface in ADL v0.75.

## What ObsMem Is in v0.75

ObsMem in v0.75 is an operational memory boundary built on deterministic ADL
run artifacts and traces.

Important scope notes:
- ObsMem is external to the core runtime implementation.
- ADL integrates through a contract/adapter boundary.
- Deterministic indexing/retrieval behavior is required for identical inputs.

## Why ObsMem Is External

ADL keeps memory decoupled so core execution/replay remains stable and
backend-agnostic. Runtime code depends on `adl::obsmem_contract` and adapter
surfaces, not a concrete memory service implementation.

## Contract and Versioning Expectations

Contract surface:
- `ObsMemClient::write_entry(...)`
- `ObsMemClient::query(...)`

Validation expectations:
- Requests are validated for required fields and path safety.
- Relative artifact paths are required.
- Validation failures are surfaced as explicit contract errors.

Versioning expectations:
- Contract behavior must remain deterministic for equivalent validated inputs.
- Future backend evolution must preserve contract semantics and ordering rules.

## Deterministic Indexing and Retrieval

v0.75 indexing/retrieval behavior is deterministic:
- Indexing derives ordered context from activation-log event order.
- Query policy normalizes tags/limits/filters deterministically.
- Retrieval ordering uses deterministic tie-break rules.

For identical normalized request inputs and identical index state, result-set
membership and ordering should be identical.

## Current User-Facing Usage Surface

v0.75 does not expose a dedicated top-level `adl obsmem ...` CLI.

ObsMem behavior is demonstrated through existing workflows and artifacts:
- Run with `ADL_OBSMEM_DEMO=1` in the demo matrix.
- Inspect emitted artifacts under:
  - `.adl/runs/<run_id>/learning/obs_mem_index_summary.json`
  - `.adl/runs/<run_id>/learning/obs_mem_query_result.json`

See:
- `docs/milestones/v0.75/DEMO_MATRIX.md`
- `docs/milestones/v0.75/OBSMEM_INTEGRATION_CONTRACT_0.75.md`
- `docs/adr/0007-obsmem-external-boundary.md`

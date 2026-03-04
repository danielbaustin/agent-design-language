# ObsMem Integration Contract v0.75

## Purpose
Define the stable interface boundary between ADL runtime surfaces and an external ObsMem implementation.

This contract intentionally keeps ObsMem decoupled from the core runtime and establishes deterministic, privacy-safe request/response structures for indexing and retrieval.

## Versioning
- Contract constant: `OBSMEM_CONTRACT_VERSION = 1`
- Consumers must reject unsupported versions deterministically with code:
  - `OBSMEM_CONTRACT_VERSION_MISMATCH`

## Rust Surface
Implemented in `swarm/src/obsmem_contract.rs`:
- `ObsMemClient` trait:
  - `write_entry(&MemoryWriteRequest) -> Result<MemoryWriteAck, ObsMemContractError>`
  - `query(&MemoryQuery) -> Result<MemoryQueryResult, ObsMemContractError>`

## Write Interface (Indexing)
`MemoryWriteRequest` fields:
- `contract_version`
- `run_id`, `workflow_id`
- `trace_bundle_rel_path`
- `activation_log_rel_path`
- `failure_code` (optional stable code)
- `summary` (privacy-safe summary only)
- `tags` (sorted/deduped during normalization)
- `citations` (relative paths + stable content hash)

Validation and safety rules:
- reject empty required fields
- reject absolute paths, traversal (`..`), Windows drive prefixes, backslash paths
- reject token-like content and host-path leakage markers
- normalize `tags`/`citations` for deterministic ordering before persistence

## Retrieval Interface
`MemoryQuery` fields:
- `contract_version`
- optional filters: `workflow_id`, `failure_code`
- `tags`
- `limit`

Validation rules:
- `limit >= 1`
- deterministic upper bound (`limit <= 1000`)
- normalize/sort query tags before evaluation

`MemoryQueryResult` returns ordered `hits` with:
- stable `entry_id`
- `run_id`, `workflow_id`
- deterministic score representation
- privacy-safe `summary`
- deterministic citations

## Identifier Mapping Rules
- `run_id` and `workflow_id` map directly to ADL run artifacts.
- `entry_id` is implementation-defined but must be stable for the same indexed record.
- citation paths must remain repository/bundle-relative and replay-friendly.

## Error/Failure Handling
Stable error codes:
- `OBSMEM_CONTRACT_VERSION_MISMATCH`
- `OBSMEM_INVALID_REQUEST`
- `OBSMEM_INVALID_QUERY`
- `OBSMEM_PRIVACY_VIOLATION`
- `OBSMEM_BACKEND_UNAVAILABLE`

Requirement: callers and tests assert on these stable codes, not free-form strings.

## Determinism and Replay Guarantees
- The contract is side-effect free from ADL scheduler semantics.
- ObsMem retrieval is optional and must not affect replay determinism.
- Given identical normalized request/query payloads and identical index state, result ordering must be deterministic.
- Trace bundles remain replay-sufficient regardless of ObsMem availability.

## Security and Privacy
- No secrets, raw prompts, raw tool arguments, or host absolute paths in contract payloads.
- Contract payloads are privacy-safe summaries + citations to deterministic artifacts.
- Implementations may enrich internally, but the interface boundary must remain sanitized.


## WP-09 ObsMem indexing pipeline (deterministic)

The v0.75 indexing pipeline is implemented in `swarm::obsmem_indexing` and consumes
run artifacts plus normalized activation-log events to produce deterministic
memory-entry payloads.

Deterministic guarantees:
- Input surface: `run_summary.json`, `run_status.json`, and `logs/activation_log.json`.
- Step contexts are indexed with explicit sequence numbers from activation-log order.
- Output tags are lexicographically sorted and deduplicated.
- Re-indexing the same run artifacts yields byte-identical serialized indexing output.

Privacy boundary:
- Raw prompts and tool arguments are not persisted by the indexing summary surface.
- Host absolute paths and token-like strings are rejected by deterministic local checks.

## WP-10 ObsMem retrieval policy v1 (deterministic)

The retrieval policy layer is implemented in `swarm::obsmem_retrieval_policy`.
It defines a deterministic runtime interface for selecting and ordering memory
records retrieved through `ObsMemClient`.

Policy v1 behavior:
- Query parameters are resolved from request inputs plus policy defaults.
- Required tags and failure-code constraints are merged deterministically.
- Result ordering is deterministic (default score-desc with lexical tie-breaks,
  or explicit lexical-id ordering).
- Identical policy + request + backend state yields identical result set and order.

Configuration boundary:
- Retrieval parameters are represented as explicit runtime policy/request values.
- WP-11 demo integration can map workflow-level configuration onto this policy
  without coupling core runtime to a specific memory backend.

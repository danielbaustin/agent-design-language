
## WP-09 ObsMem indexing pipeline (deterministic)

The v0.75 indexing pipeline is implemented in `swarm::obsmem_indexing` and consumes
run artifacts plus normalized activation-log events to produce deterministic
memory-entry payloads.

Canonical indexed types:
- `IndexedMemoryEntry`
  - `run_id`: run identifier for the indexed artifact set
  - `workflow_id`: resolved workflow identifier from `run_summary.json`
  - `status`: overall run status from `run_status.json`
  - `failure_code`: optional stable failure code from `run_status.json`
  - `summary`: deterministic, sanitized run-level summary string
  - `tags`: sorted/deduplicated deterministic tag set
  - `steps`: ordered list of `IndexedStepContext`
- `IndexedStepContext`
  - `sequence`: activation-log sequence index (canonical event order)
  - `step_id`: resolved step identifier
  - `event_kind`: normalized event class (`step_started`, `prompt_assembled`, `step_output_chunk`, `step_finished`)
  - `context`: sanitized deterministic context fragment (no raw prompt/tool args)

Deterministic guarantees:
- Mandatory input surface: `run_summary.json`, `run_status.json`, and `logs/activation_log.json`.
- Missing/invalid mandatory inputs fail indexing deterministically with a stable `OBSMEM_INVALID_REQUEST` contract error.
- `sequence` ordering is derived from canonical activation-log append order, not wall-clock timestamps.
- Output tags are lexicographically sorted and deduplicated.
- Re-indexing the same run artifacts yields byte-identical serialized indexing output.

Privacy boundary:
- Raw prompts and tool arguments are not persisted by the indexing summary surface.
- Host absolute paths and token-like strings are rejected by deterministic local checks.

# Multi-Agent Workcell State Contract

## Purpose

This contract defines the tracked, reviewable state packet for a bounded
multi-agent C-SDLC workcell run.

The packet exists to make assignment and routing truth inspectable without
creating a hidden orchestration database. It complements, and does not replace:

- GitHub issue and PR state
- issue-local cards (`SIP -> STP -> SPP -> SRP -> SOR`)
- retained sprint state under `.adl/reviews/`
- conductor-selected lifecycle skills

## Non-goals

This packet is not:

- canonical issue lifecycle state
- a replacement for cards or GitHub truth
- an autonomous scheduler state store
- permission to bypass worktrees, review, janitor work, or closeout

## Schema

- `schema_version`: must be `adl.multi_agent_workcell_state.v1`
- `workcell_id`: bounded run identifier for the workcell packet
- `sprint_issue_number`: sprint umbrella that owns the workcell run
- `planner_manifest_path`: repo-relative path to the shard-assignment input
- `conductor_hooks`: explicit hook map showing which lifecycle skill owns each
  routing boundary
- `shard_assignments`: assignment records for worker, reviewer, janitor, and
  closeout lanes

## Conductor hook points

These hooks keep conductor authority explicit instead of implicit:

- `worker_admission` -> `pr-run`
- `review_publication` -> `pr-finish`
- `janitor_remediation` -> `pr-janitor`
- `closeout_reconciliation` -> `pr-closeout`

Each hook must also name the canonical truth surfaces it depends on. At minimum,
the contract requires:

- `worker_admission`: `github_issue_state`, `sip`, `stp`, `spp`, `sprint_state`
- `review_publication`: `srp`, `sor`, `pr_state`, `published_artifacts`
- `janitor_remediation`: `pr_state`, `check_status`, `review_findings`
- `closeout_reconciliation`: `github_issue_state`, `sor`, `sprint_state`,
  `closeout_artifacts`

## Assignment record contract

Each `shard_assignments[]` record must include:

- `shard_id`
- `issue_number`
- `role`
- `branch`
- `worktree_path`
- `write_paths`
- `dependencies`
- `admission_status`
- `validation_gate`
- `review_lane`
- `closeout_status`
- `cards`
- `github_issue_state`
- `pr_state`

Worker lanes must also declare:

- `execution_backend`
- `model_hint`

This lets the later proof issue record heterogeneous worker truth, including
local Ollama and hosted Codex shards, without inventing a second ad hoc packet.

## Required truth surfaces per assignment

### Worker lanes

Worker assignments are admitted only when:

- `SIP`, `STP`, and `SPP` are `ready` or `approved`
- the issue is still open on GitHub
- the worktree and branch are declared explicitly
- the declared write set is present

Parallel-safe workers use:

- `admission_status: parallel_admitted`
- `validation_gate: parallel_pvf_lane`
- `review_lane: bounded_subagent_review`

Serialized workers use:

- `admission_status: serial_only`
- `validation_gate: serialized_gate`

### Reviewer lanes

Reviewer lanes represent a published shard that is ready for bounded review.
They use:

- `admission_status: review_ready`
- `validation_gate: published_artifact_gate`
- `review_lane: bounded_subagent_review`

### Janitor lanes

Janitor lanes represent a real PR blocker. They use:

- `admission_status: janitor_ready`
- `validation_gate: pr_blocker_gate`
- `review_lane: pr_janitor`
- `closeout_status: janitor_pending`

### Closeout lanes

Closeout lanes represent merged execution that is ready for reconciliation.
They use:

- `admission_status: closeout_ready`
- `validation_gate: closeout_truth_gate`
- `review_lane: pr_closeout`
- `closeout_status: ready_to_close` or `closed_out`

## Fail-closed rules

The validator must reject packets when any of the following are true:

- a parallel-admitted worker is already closed on GitHub
- required worker cards are not ready
- a closeout lane claims `closed_out` while GitHub issue truth is still open
- non-worker lanes declare provider/model fields
- absolute host paths or `..` traversal segments appear in repo artifacts
- hook points drift away from the required lifecycle skills

## Recommended proof-path output

A later live proof should emit its runtime packet at a repo-relative path such
as:

- `artifacts/v0914/multi_agent_workcell/<run_id>/workcell_state.json`

That keeps runtime evidence separate from tracked contract fixtures while still
preserving a stable replay location.

## Included fixtures

This issue ships:

- `workcell_state_packet_example.json`
- `workcell_state_packet_invalid.json`
- `validate_multi_agent_workcell_state.py`
- `test_validate_multi_agent_workcell_state.sh`

The proof issue should consume the example packet shape and extend it with live
run evidence rather than inventing a new packet family.

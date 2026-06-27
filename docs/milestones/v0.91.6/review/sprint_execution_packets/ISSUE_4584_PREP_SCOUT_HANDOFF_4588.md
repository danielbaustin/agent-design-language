# Issue 4584 Prep-Scout Handoff For Rescue Sprint 4588

## Supersession Notice

Status: superseded by execution, watcher, doctor, and closeout truth.

This packet is retained as historical prep-scout evidence only. It must not be
used as current routing guidance for `#4584`.

Current truth:

- `#4584` is closed.
- `ISSUE_4584_WATCH_PACKET_4588.json` conservatively classifies the closed
  issue as `closeout_needed`.
- `ISSUE_4584_DOCTOR_PACKET_4588.json` records lifecycle `closed`, ready status
  `PASS`, and doctor status `PASS`.
- The stale execution claim was released during `#4588` sprint closeout
  hygiene.

The validation commands below were a pre-execution scout suggestion from before
the binary-first rescue rule landed. They are not current instructions. Any
future rerun should use the current ADL binary-first command contract rather
than hidden Cargo control flow.

## Scope

Read-only prep scout for `#4588`.

The scout did not edit files, claim the issue, create a PR, mutate GitHub, or
start execution. The purpose was to remove wait time before the ordered queue
reaches `#4584`.

## Current State Reported By Scout

Historical scout state, superseded by the notice above:

- Issue: `#4584`
- State: ready for normal execution after session claim
- Lifecycle: `pre_run`
- Open PR count: `0`
- Linked PR: none
- Bound worktree: none
- Collision: none reported
- Next skill: `pr-run`

## Likely Code Surfaces

- `adl/src/csdlc_prompt_editor.rs`
- Existing legacy SOR import tests near the prompt-template editor test module
- Repro input:
  `.adl/v0.91.6/tasks/issue-3976__v0-91-6-wp-11-demo-demo-matrix-and-proof-convergence/sor.md`
- Active SOR template:
  `docs/templates/prompts/1.0.3/sor.md`

## Focused Validation Suggested

```bash
cargo run --manifest-path adl/Cargo.toml --bin adl-csdlc -- \
  tooling prompt-template import-values \
  --kind sor \
  --input .adl/v0.91.6/tasks/issue-3976__v0-91-6-wp-11-demo-demo-matrix-and-proof-convergence/sor.md \
  --out /tmp/4584-sor.values.yaml \
  --normalized-out /tmp/4584-sor.normalized.md

cargo run --manifest-path adl/Cargo.toml --bin adl-csdlc -- \
  tooling prompt-template validate-values \
  --kind sor \
  --values /tmp/4584-sor.values.yaml

cargo run --manifest-path adl/Cargo.toml --bin adl-csdlc -- \
  tooling prompt-template validate-structure \
  --kind sor \
  --input /tmp/4584-sor.normalized.md

bash adl/tools/test_prompt_template_workflow_integration.sh
```

## Handoff Note

The executing session still must take a fresh session claim and run the normal
issue-bound `pr run` lifecycle. This packet is advisory prep only.

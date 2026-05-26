## Card Repair Examples

This directory contains durable, validator-clean examples of the card shapes we
expect after bounded editor-skill repair.

These examples are intentionally small, but they are not placeholders. Each one
matches the current `docs/templates/prompts/current.json` family closely enough
to pass the structured-prompt validator. Where the validator currently consumes
an explicit `--phase` flag, the proof lane passes it; otherwise the examples
are validated without claiming extra phase-aware enforcement.

Included repaired examples:

- `sip_repaired_pre_run.md`
- `stp_repaired_issue_ready.md`
- `spp_repaired_issue_plan.md`
- `srp_repaired_review_truth.md`
- `sor_repaired_pr_open.md`

These examples are for:

- operator orientation
- editor-skill boundary clarification
- focused proof in `adl/tools/test_card_editor_repair_examples.sh`

They are not authoritative lifecycle records for a real issue. They are
repaired shape examples that show what truthful cards should look like. Real
issue work must still be grounded in the bound issue bundle and normalized
through the matching editor skill.

Boundary notes:

- `sip-editor` owns truthful pre-run input-card normalization.
- `stp-editor` owns design-time task-shape repair.
- `spp-editor` owns issue-local planning truth and `codex_plan` hygiene.
- `srp-editor` owns Structured Review Prompt semantics and review-result truth.
- `sor-editor` owns execution and integration truth, but not PR publication or
  merge.

The browser prompt editor remains a human drafting/recovery surface. It does
not replace the editor skills required by `AGENTS.md`.

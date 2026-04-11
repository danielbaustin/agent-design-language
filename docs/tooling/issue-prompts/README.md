# Issue Prompt Templates

This directory is the tracked home for reusable issue-prompt templates and authoring guidance.

The intended model is:

- issue prompts: structured design prompts used to create or edit GitHub issues
- input cards: structured implementation prompts used after an issue enters run phase
- output cards: structured implementation records produced after execution

Issue prompts are an important software/GitHub integration surface, but they are not the universal ontology for ADL. The broader system should remain task-centric so it can support software engineering, writing, research, and other human-creation workflows.

## Why These Templates Live Here

The repo ignores `.adl/`, which is a good place for generated or work-in-progress bodies, reconciliation manifests, and local authoring experiments.

But the reusable templates themselves should be tracked so they can:

- evolve under review
- support future editors and validation tooling
- serve as canonical references for issue-prompt generation

So the split is:

- tracked templates and guidance: `docs/tooling/issue-prompts/`
- historical tracked public task-record examples: versioned directories under `docs/records/`
- live local/generated prompt drafts and canonical task execution records: `.adl/`

The important distinction is:

- `docs/tooling/issue-prompts/`
  - reusable template and authoring-contract layer for GitHub-facing issue prompts
- `docs/records/`
  - historical tracked task-bundle examples, organized by milestone/version where applicable
- `.adl/`
  - local draft workspace, generated intermediate state, and the current canonical task-bundle / issue-record surface

## Template Set

- `issue-prompt-template.md`
  - full structured issue-prompt template
- `issue-prompt-stub-template.md`
  - transitional issue-prompt stub template for reconciliation passes

## Authoring Guidance

- Keep one canonical prompt file per target issue/WP.
- Pull existing GitHub issue content into issue prompts selectively, not mechanically.
- State the required outcome explicitly so an agent can tell whether code, tests, docs, or a demo must ship.
- Prefer exact repo paths in `Repo Inputs` when a prompt depends on specific files or modules.
- Include demo expectations when the WP requires runnable proof surfaces.
- Use `Issue-Graph Notes` to record duplicate, supersede, split, merge, or renumber intent explicitly.
- Treat `.adl/` as the live execution surface for current workflow state; use `docs/records/` only when intentionally preserving a tracked historical/public example.

## Transitional Stub Guidance

Use the stub template when the tracker must be reorganized now but the final issue prompt is not ready yet.

A stub should:

- keep the title, labels, sprint, and slug correct
- state the WP purpose and required outcome briefly
- name key dependencies and known repo inputs
- explicitly say that the full structured issue prompt is still pending

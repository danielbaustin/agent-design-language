# HITL Editor / Review Demo

This bounded demo is the milestone-facing proof surface for ADL's current
human-in-the-loop authoring and review workflow.

It demonstrates the real editor slice that now exists in the repo:

1. one linked STP / SIP / SOR task-bundle workspace
2. visible validation and rendered preview while editing
3. bounded `pr start` command generation through the thin adapter
4. review-flow guidance tied to proof surfaces rather than side conversation

It does not claim a finished browser-only workflow platform.

## One-command demo

From repository root:

```bash
adl/tools/demo_hitl_editor_review.sh
```

## Primary proof artifacts

- `docs/records/v0.85/tasks/task-v085-wp05-demo/stp.md`
- `docs/records/v0.85/tasks/task-v085-wp05-demo/sip.md`
- `docs/records/v0.85/tasks/task-v085-wp05-demo/sor.md`
- `.adl/reports/demo-hitl-editor-review/editor_review_demo_manifest.v1.json`

## What the command verifies

- the static editor surface can be served locally
- the tracked task-bundle proof bundle exists and stays linked
- the thin editor adapter emits a bounded `pr start` command in dry-run mode
- the canonical walkthrough doc still matches the actual tracked editor files

## Optional interactive review

After running the one-command demo, open the URL recorded in
`editor_review_demo_manifest.v1.json` in a browser and then follow:

- `docs/tooling/editor/demo.md`

Look for:
- the linked three-card workspace
- validation and preview behavior
- bounded review-flow output
- public task-bundle destination visibility

## Why this matters

This is the clearest bounded proof that ADL has moved beyond raw prompt text
editing and into explicit human-in-the-loop artifact authoring and review.

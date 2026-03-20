# Editor Workflow Demo

This bounded demo is the proof surface for the first WP-05 task-bundle workspace slice.

## Steps

1. Open `docs/tooling/editor/index.html` in a browser.
2. Confirm the workspace shows three linked cards:
   - `Structured Task Prompt`
   - `Structured Implementation Prompt`
   - `Structured Output Record`
3. Leave the default `Structured Task Prompt` card selected.
4. Edit the task ID, title, and required sections.
5. Observe the validation panel:
   - required fields show warnings when blank
   - unchanged placeholder content is flagged
   - valid task IDs, run IDs, versions, branch values, and enum values show passing checks
6. Review the rendered artifact preview.
   - confirm the H1 uses the task title rather than a generic artifact-class heading
7. Switch to `Structured Implementation Prompt` and confirm the active bundle target changes to:
   - `docs/records/v0.85/tasks/<task-id>/sip.md`
8. Switch to `Structured Output Record` and confirm the SOR card remains visibly linked in the same bundle workspace.
9. Edit the SOR review fields and confirm the surface now supports:
   - integration state
   - verification scope
   - primary proof surface
   - artifact verification and deferred follow-ups
10. Compare the preview output with the tracked example bundle:
   - `docs/records/v0.85/tasks/task-v085-wp05-demo/stp.md`
   - `docs/records/v0.85/tasks/task-v085-wp05-demo/sip.md`
   - `docs/records/v0.85/tasks/task-v085-wp05-demo/sor.md`
11. Return to the `Structured Task Prompt` card, set a numeric GitHub issue number that matches the branch prefix, and confirm the workflow action panel shows a ready `pr start` command.
12. Copy the command from the editor and run it from the repo root:
    - `swarm/tools/editor_action.sh start --issue <issue-number> --branch codex/<issue>-<slug>`

## Demo Claims

- the editor is a real tracked repo surface, not a design sketch
- the editor presents a linked task-bundle workspace rather than isolated artifact editing only
- the editor supports STP and SIP authoring while keeping SOR review visible and editable in the same workspace
- the editor preview and validation are materially closer to the current STP/SIP contract expectations
- the editor exposes one bounded validated control-plane action without duplicating workflow logic in browser code
- the editor keeps the public task-bundle destination visible
- the editor reduces structural editing fragility by guiding required fields and rendering the final markdown artifact live

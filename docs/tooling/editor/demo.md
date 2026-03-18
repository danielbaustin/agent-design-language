# Editor Workflow Demo

This bounded demo is the proof surface for the first WP-05 editor slice.

## Steps

1. Open `docs/tooling/editor/index.html` in a browser.
2. Leave the default `Structured Task Prompt` selection in place.
3. Edit the task ID, title, and required sections.
4. Observe the validation panel:
   - required fields show warnings when blank
   - valid task IDs and branch values show passing checks
5. Review the rendered artifact preview.
6. Switch to `Structured Implementation Prompt`.
7. Confirm the bundle target changes to:
   - `docs/records/v0.85/tasks/<task-id>/sip.md`
8. Compare the preview output with the tracked example bundle:
   - `docs/records/v0.85/tasks/task-v085-wp05-demo/stp.md`
   - `docs/records/v0.85/tasks/task-v085-wp05-demo/sip.md`

## Demo Claims

- the editor is a real tracked repo surface, not a design sketch
- the editor supports both STP and SIP flows
- the editor keeps the public task-bundle destination visible
- the editor reduces structural editing fragility by guiding required fields and rendering the final markdown artifact live

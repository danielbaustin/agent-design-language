# Editor Workflow Demo

This bounded demo is the canonical proof surface for the current editor slice.

It demonstrates the real workflow that now exists in the repo:

- open one tracked task bundle
- edit STP, SIP, and SOR in one workspace
- see validation and rendered preview update live
- prepare a bounded current lifecycle command from the editor
- use the integrated SOR review flow to prepare a reviewer handoff
- keep the browser-visible language contract aligned with the canonical ADL six-primitives model

The canonical near-term adapter contract for this slice is:

- `docs/tooling/editor/command_adapter.md`

It does not claim a finished browser-only workflow platform.

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
7. Switch to `Structured Implementation Prompt` and confirm the current local bundle target changes to:
   - `.adl/<version>/tasks/<task-id>__<slug>/sip.md`
8. Switch to `Structured Output Record` and confirm the SOR card remains visibly linked in the same bundle workspace.
9. Edit the SOR review fields and confirm the surface now supports:
   - integration state
   - verification scope
   - primary proof surface
   - artifact verification and deferred follow-ups
10. Confirm the new review-flow panel now shows:
    - a bounded recommendation
    - a reviewer checklist tied to the SOR proof surface
    - a copyable review note that summarizes review focus and follow-ups
11. Confirm the new ADL Language Contract panel shows:
    - the six primitives: `providers`, `tools`, `agents`, `tasks`, `workflows`, `run`
    - singular `run` as the top-level execution primitive
    - `patterns` and `signature` as top-level language features rather than extra primitives
12. Compare the preview output with a current local issue bundle:
    - `.adl/<version>/tasks/<task-id>__<slug>/stp.md`
    - `.adl/<version>/tasks/<task-id>__<slug>/sip.md`
    - `.adl/<version>/tasks/<task-id>__<slug>/sor.md`
13. Return to the `Structured Task Prompt` card, set a numeric GitHub issue number that matches the branch prefix, and confirm the workflow action panel shows a ready `pr run` command.
14. Copy the command from the editor and run it from the repo root:
    - `adl/tools/editor_action.sh prepare --phase run --issue <issue-number> --slug <slug> --version <version>`
15. Copy the review note from the Review Flow panel and confirm it summarizes:
    - the bounded recommendation
    - the current proof surface
    - the follow-ups that still remain manual

## Current Supported Loop

- task bundle opens as a linked STP/SIP/SOR workspace
- structured field editing and preview happen in the browser
- validation feedback is visible while editing
- bounded copy-only current lifecycle command generation is available from the editor
- SOR review and reviewer handoff are visible in the same workspace

## Still Manual / Out Of Scope

- `pr create` is not launched from the browser in this slice
- `pr init` is not launched from the browser in this slice
- `pr doctor` / `pr ready` is not launched from the browser in this slice
- `pr run` is not launched from the browser in this slice
- `pr finish` is not launched from the browser in this slice
- `pr janitor` and `pr closeout` remain skill-owned, not browser-owned
- final review judgment is still human-made
- the browser does not write tracked files directly
- no claim is made that this is already a full HTA platform

## Demo Claims

- the editor is a real tracked repo surface, not a design sketch
- the editor presents a linked task-bundle workspace rather than isolated artifact editing only
- the editor supports STP and SIP authoring while keeping SOR review visible and editable in the same workspace
- the editor turns SOR proof/evidence fields into a bounded review loop rather than leaving review as a side conversation
- the editor surfaces the canonical six-primitives language contract instead of relying on stale or implied language-shape assumptions
- the editor preview and validation are materially closer to the current STP/SIP contract expectations
- the editor exposes bounded validated lifecycle command preparation without duplicating workflow logic in browser code
- the demo makes the remaining manual steps explicit instead of hiding them
- the editor keeps the public task-bundle destination visible
- the editor reduces structural editing fragility by guiding required fields and rendering the final markdown artifact live

# Current Skill Wiring Demo

This proof surface shows how the static task-bundle editor aligns with the current ADL workflow skill family without claiming direct browser execution.

It also confirms that the HTML editor stack now points at the corrected ADL
language contract: six primitives with singular `run`, plus top-level
`patterns` and `signature` features.

## Scenario

Use the default editor state:

- task id: `issue-2053`
- title: `[v0.90][tools] Refresh web task editor for current ADL skills`
- branch: `codex/2053-backlog-tools-refresh-web-task-editor-current-skills`
- version: `v0.90`

The editor should display:

- current local bundle root: `.adl/v0.90/tasks/issue-2053__backlog-tools-refresh-web-task-editor-current-skills/`
- current local card target for the active STP, SIP, or SOR card
- a copy-only workflow action for the `pr-run` handoff
- an ADL language contract panel listing `providers`, `tools`, `agents`, `tasks`, `workflows`, and `run`

## Command Proof

The adapter command used by the editor is:

- `adl/tools/editor_action.sh prepare --phase run --issue 2053 --slug backlog-tools-refresh-web-task-editor-current-skills --version v0.90`

Expected output:

- `./adl/tools/pr.sh run 2053 --slug backlog-tools-refresh-web-task-editor-current-skills --version v0.90`

Additional supported handoffs:

- `adl/tools/editor_action.sh prepare --phase init --issue 2053 --slug backlog-tools-refresh-web-task-editor-current-skills --version v0.90`
- `adl/tools/editor_action.sh prepare --phase doctor-ready --issue 2053 --slug backlog-tools-refresh-web-task-editor-current-skills --version v0.90`
- `adl/tools/editor_action.sh prepare --phase finish --issue 2053 --slug backlog-tools-refresh-web-task-editor-current-skills --version v0.90 --title "[v0.90][tools] Refresh web task editor for current ADL skills" --paths "docs/tooling/editor/README.md,docs/tooling/editor/index.html,docs/tooling/editor/task_bundle_editor.js,adl/tools/editor_action.sh"`

## Skill Alignment

The browser editor is a preparation surface only.

It can help prepare commands for:

- `pr-init`
- `pr-ready` / doctor readiness
- `pr-run`
- `pr-finish`

It must not take over:

- `pr-janitor`
- `pr-closeout`
- STP, SIP, or SOR card-editor judgment
- worktree binding or tracked file writes

## Truth Boundary

The editor may produce useful markdown previews, validation hints, review notes, and copyable commands. The repo-owned lifecycle still runs through the ADL control plane and associated skills.

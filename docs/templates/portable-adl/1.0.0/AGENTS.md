# <project_name> Agent Guidelines

This repository follows ADL C-SDLC workflow discipline.

Before doing tracked issue work, read `adl_project.json` at the repository root.
That file declares the project profile, ADL tooling discovery rules, issue
authority, card/worktree policy, validation profile, and artifact boundaries.

## Required Workflow

- Do not do tracked issue work on `main`.
- Do not hand-roll lifecycle cards.
- Use the resolved ADL checkout from `adl_project.json`.
- Route issue execution through ADL conductor-style workflow.
- Preserve the canonical lifecycle: `SIP -> STP -> SPP -> SRP -> SOR`.
- Keep public artifacts free of host-local absolute paths, credentials, private
  notes, and scratch state.
- Record what validation did and did not run.

## Tooling Discovery

Resolve ADL tooling in this order:

1. `ADL_HOME`, if set.
2. The repo-relative path declared in `adl_project.json`, if present.
3. The sibling checkout declared in `adl_project.json`, if present.
4. Fail closed with setup instructions.

Do not search the whole filesystem or use vendored ADL tooling.

## Project-Specific Notes

- Project profile: `<profile>`
- Issue authority: `<issue_authority>`
- Card location: `<cards_location>`
- Worktree location: `<worktree_location>`
- Validation profile: `<validation_profile>`

Use repo-specific validation commands only when they are declared in
`adl_project.json` or in a tracked issue card.

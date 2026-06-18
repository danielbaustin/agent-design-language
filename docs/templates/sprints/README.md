# Versioned Sprint Templates

This directory is the canonical tracked home for ADL sprint-level execution templates.

The active sprint-template set is declared in [`current.json`](current.json).
The first active template is the Sprint Execution Packet (SEP), which records
sprint execution mode, dependency order, safe parallel lanes, watcher policy,
PVF notes, sprint activity logging, sprint-level review, closeout bars, and
residual routing.

## Current Set

- Template set: `1.0.0`
- Template root: `docs/templates/sprints/1.0.0/`
- Registry: `docs/templates/sprints/current.json`
- Active SEP: `docs/templates/sprints/1.0.0/sprint_execution_packet.md`

## Contract

SEP is a sprint-level orchestration surface. It does not replace issue-local
`SIP -> STP -> SPP -> SRP -> SOR` cards.

Parallel and hybrid execution modes are sprint intent and coordination evidence
until the relevant automation proves multi-active lane execution. Each child
issue still owns its own lifecycle, review, closeout, and worktree pruning.

## Versioning Policy

- Template-set versions use SemVer.
- `1.0.0/` is immutable after adoption except for obvious typo fixes.
- Future semantic changes create a new SemVer directory, such as `1.1.0/` or
  `2.0.0/`, then update `current.json`.
- Tools and skills should resolve active sprint templates from `current.json`
  rather than hard-coding a template version.

## AST / Markdown Editing Policy

The SEP is currently a Markdown bootstrap template. Follow-on work should move
SEP rendering and updates onto the same AST-backed `markdown.rs` path as other
C-SDLC templates. Until then, direct edits must preserve the versioned registry
and section contract.

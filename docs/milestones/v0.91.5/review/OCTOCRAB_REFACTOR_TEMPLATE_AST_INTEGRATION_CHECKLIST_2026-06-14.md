# Octocrab, Refactor, Prompt Template, and Markdown AST Integration Checklist

Date: 2026-06-14
Milestone: v0.91.5
Primary issue: #3697
Primary PR: #3702
Status: coordination checklist for open PR review

This document is not proof that the integrated system is complete. Checked
items mean the current #3697 branch or existing milestone evidence reports that
the item is covered for the named slice. They remain subject to code review,
CI, PR merge truth, and later end-to-end proof.

## Purpose

ADL now has four related acceleration tracks that must converge instead of
creating parallel half-systems:

- refactoring the command surface into the Rust `adl-csdlc` control plane
- replacing GitHub shell transport with typed octocrab-backed GitHub paths
- keeping prompt cards generated through the versioned template renderer
- moving Markdown edits toward AST-backed editing rather than brittle text edits

The intended outcome is faster, safer C-SDLC execution: fewer hand-written
commands, fewer malformed cards, fewer GitHub workflow mistakes, and fewer
review cycles caused by tooling drift.

## Non-negotiable integration rules

- `gh` is not an operational dependency for covered issue or PR workflow paths.
- GitHub issue and PR state that ADL owns must route through Rust/octocrab or a
  clearly documented fail-closed gap.
- Prompt cards remain template-owned state, not ad hoc Markdown.
- Markdown AST editing is an editing substrate, not a new lifecycle authority.
- The workflow conductor remains the lifecycle router.
- Subagent review remains required before PR publication.
- Closeout truth must still reconcile cards, issue state, PR state, and evidence.

## Checklist

### 1. GitHub transport and octocrab

- [x] Rust GitHub client boundary exists and supports octocrab token-based API use
  in the current #3697 branch.
- [x] Issue metadata reads have octocrab-backed paths for covered workflow cases
  in the current #3697 branch.
- [x] PR creation/edit/ready/merge paths have octocrab-backed implementations for
  covered workflow cases.
- [x] PR closing-linkage check uses a GitHub API path rather than shelling out to
  `gh`.
- [x] Legacy shell helpers that cannot truthfully use octocrab fail closed instead
  of silently calling `gh`.
- [x] Documentation records that `GITHUB_TOKEN` or `GH_TOKEN` is required for
  Rust/octocrab GitHub operations.
- [ ] Remaining GitHub release or watcher gaps are routed to explicit follow-up
  work rather than hidden inside successful workflow claims.

### 2. Refactored command surface

- [x] `adl-csdlc` is the target owner for typed C-SDLC workflow operations.
- [x] Shell wrappers are treated as compatibility delegates, not the long-term
  source of workflow truth.
- [ ] Every surviving shell wrapper has one of these statuses: delegated to
  `adl-csdlc`, local-only utility, explicit fail-closed gap, or scheduled for
  removal.
- [ ] Operator-facing docs describe the Rust command path first and shell wrappers
  second.
- [ ] Validation lanes distinguish command-surface behavior from wrapper behavior.

### 3. Prompt templates and card lifecycle

- [x] `SIP -> STP -> SPP -> SRP -> SOR` remains the canonical card lifecycle.
- [x] New or fully regenerated cards must use the active prompt-template registry.
- [x] Card updates should edit values first, then render and validate structure.
- [ ] `pr.sh` / `adl-csdlc` paths that create or repair cards consistently route
  through prompt-template renderer commands instead of direct Markdown assembly.
- [ ] Card editor skills and renderer tooling have a documented boundary:
  renderer/schema for structure, editor skills for issue-local lifecycle truth.
- [ ] Prompt-template validation is available as a focused PVF lane for card and
  template changes.
- [ ] Integrated workflow proof includes `current.json` registry resolution.
- [ ] Integrated workflow proof includes values import or values editing when
  updating existing rendered cards.
- [ ] Integrated workflow proof includes render, structure validation, and schema
  validation for generated or regenerated cards.
- [ ] Integrated workflow proof includes schema parity validation when prompt
  schema artifacts change.

### 4. markdown.rs / AST editing

- [ ] Markdown AST editing has an issue-bound implementation plan and does not
  mutate lifecycle cards outside the renderer/editor policy.
- [ ] The implementation plan names the concrete Rust Markdown parser/editor
  dependency and the supported Markdown node set.
- [ ] The implementation plan defines unsupported-node behavior and requires
  fail-closed repair notes when safe mutation is impossible.
- [ ] AST editing targets are classified by document type: prompt card, planning
  doc, review packet, README/runbook, feature doc, or generated evidence.
- [ ] For prompt cards, AST edits are limited to safe inspection or import paths
  unless the active template system cannot represent the required change.
- [ ] For planning and review docs, AST edits preserve headings, code fences,
  tables, links, and front matter without text-regex drift.
- [ ] AST editor validation includes a fixture corpus covering prompt cards,
  planning docs, review packets, tables, links, code fences, and front matter.
- [ ] AST editor validation includes round-trip stability checks with explicit
  diff criteria on representative docs.
- [ ] AST editor failures fail closed and produce a human-readable repair note.

### 5. Cross-system proof before broader rollout

- [ ] One issue is executed end-to-end using the refactored command path,
  octocrab GitHub transport, prompt-template card handling, and documented
  Markdown editing policy.
- [ ] The proof names the exact issue, branch, PR, commit, and command path.
- [ ] The proof records elapsed time from issue start to PR open.
- [ ] The proof records which steps were automated, delegated, manual, or blocked.
- [ ] The proof records whether any fallback path was used.
- [ ] The proof includes subagent review before PR publication.
- [ ] The proof includes closeout after merge or intentional closure.

### 6. Follow-up routing

- [ ] Create or confirm a follow-up for octocrab-native release/watcher support if
  those commands remain required for v0.91.5 or v0.92.
- [ ] Create or confirm a follow-up for wrapper inventory and retirement status.
- [ ] Create or confirm a follow-up for prompt-template renderer integration in
  every card-producing workflow path.
- [ ] Create or confirm a follow-up for markdown.rs AST-backed editing.
- [ ] Create or confirm a follow-up for a combined end-to-end timing proof.

## Review questions

- Does any user-facing workflow still require `gh` without saying so?
- Can an operator run the issue-to-PR path with only the Rust tooling and a valid
  GitHub token?
- Are prompt cards still generated from templates, or did any path recreate
  direct Markdown card assembly?
- Are AST edits a safe substrate beneath the lifecycle, or did they become an
  ungoverned parallel editor?
- Does the current proof show speed, or only architecture?
- Are all known gaps explicit and scheduled?

## Current residual risk

This checklist is a coordination surface, not final proof. The current #3697
work substantially advances the octocrab transport path, but full process-speed
proof requires at least one clean end-to-end issue run after the refactor,
octocrab, prompt-template, and Markdown AST editing tracks are integrated.

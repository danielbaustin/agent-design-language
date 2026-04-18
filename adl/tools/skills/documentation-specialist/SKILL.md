---
name: documentation-specialist
description: Plan, write, audit, repair, or polish bounded repository documentation from explicit source evidence. Use when Codex needs to work on README, milestone, feature, ADR, demo, review, architecture, onboarding, runbook, or skill documentation while separating source-backed facts from recommendations and assumptions, flagging stale commands, overclaims, missing validation evidence, and stopping before publication or broad unbounded rewrites.
---

# Documentation Specialist

Plan, write, audit, repair, or polish bounded repository documentation without
turning docs work into unsupported storytelling.

This skill is a documentation authoring and truth-boundary skill. It may edit
bounded documentation targets when asked to write or repair docs. It may also
produce a handoff packet when the requested output is planning, audit, or review
only.

## Quick Start

1. Confirm the bounded documentation target:
   - README or onboarding page
   - milestone or feature document
   - ADR or candidate ADR
   - demo/runbook document
   - review packet or report
   - architecture packet
   - skill documentation or input schema
2. Confirm the source packet or evidence paths.
3. Classify each claim as source-backed fact, assumption, recommendation, or
   gap.
4. Check for stale commands, broken paths, overclaims, missing validation
   evidence, unclear audience, and unsafe publication language.
5. Write or repair only the bounded target, or emit a documentation handoff
   packet if edits are not requested.
6. Record validation performed, validation not run, residual risk, and follow-up
   gaps.

## Required Inputs

At minimum, gather:

- `mode`
- `target`
- `source_packet`
- `audience`
- `policy`

Supported modes:

- `write_doc`
- `repair_doc`
- `audit_doc`
- `polish_doc`
- `plan_doc`
- `refresh_doc`
- `handoff_packet`

Useful policy fields:

- `bounded_target_required`
- `source_evidence_required`
- `allow_repo_edits`
- `write_handoff_artifact`
- `check_commands`
- `stop_before_publication`
- `stop_before_broad_rewrite`

If there is no bounded target or source evidence, stop with `blocked` rather
than inventing repository behavior.

## Workflow

### 1. Establish Documentation Boundary

Record:

- target path or requested artifact
- source evidence paths
- intended audience
- doc type
- allowed edit scope
- validation expectations
- publication intent

Do not infer a repo-wide rewrite from a broad docs request. Ask for or derive a
bounded slice, then keep edits there.

### 2. Build A Claim Ledger

For the target doc, track:

- source-backed facts
- assumptions that must be labeled
- recommendations that must not be presented as completed work
- gaps where source evidence is missing
- commands or paths that need validation
- claims that depend on another issue, PR, demo, or reviewer decision

Prefer deleting or qualifying unsupported claims over making them sound nicer.

### 3. Write Or Repair The Doc

Write for the declared audience:

- operator docs should be direct, procedural, and command-truthful
- reviewer docs should foreground scope, evidence, validation, and residual risk
- architecture docs should separate current implementation from planned design
- demo docs should state what the demo proves and does not prove
- ADRs should separate proposed, accepted, superseded, and rejected decisions
- skill docs should include entry conditions, required inputs, workflow, stop
  boundary, outputs, blocked states, and handoffs

Keep structure durable. Prefer short sections, explicit validation commands, and
repo-relative paths.

### 4. Validate Truthfully

Use the smallest useful validation set:

- check referenced files exist when possible
- run documented commands only when safe and bounded
- run Markdown or contract tests when the repo provides them
- check generated docs for absolute host paths and secret markers when
  publication is possible
- verify diagram or demo links only when they are part of the bounded target

If validation is not run, say why.

### 5. Stop Boundary

Stop after the bounded documentation edit, audit, or handoff packet.

Do not:

- publish externally
- claim release approval
- claim review approval
- accept ADRs without human decision
- create issues or PRs unless another lifecycle skill is explicitly invoked
- rewrite broad doc sets without a bounded target
- treat planned work as implemented work
- hide stale commands, missing proof, or source gaps

## Output

When editing docs, report:

- target paths changed
- source evidence used
- claims clarified
- assumptions or gaps surfaced
- commands checked
- validation run or not run
- residual risk

When writing a handoff packet, use `references/output-contract.md`.

## Handoffs

- Use `repo-review-docs` when the task is review-only and findings-first.
- Use `repo-architecture-review` when the problem is primarily architecture
  structure, coupling, or lifecycle drift.
- Use `diagram-author` when source-grounded diagrams are required.
- Use `gap-analysis` when comparing expected baseline to observed evidence.
- Use `product-report-writer` for customer-grade CodeBuddy report packaging.
- Use `medium-article-writer` or `arxiv-paper-writer` for platform-specific
  publication drafts.

## Blocked States

Return `blocked` when:

- no bounded target is provided
- no source evidence is available for factual claims
- the user asks for publication, approval, compliance, or release readiness
  claims without proof
- the request would require broad repo mutation without an explicit scope
- validation is required but unsafe or impossible in the current environment

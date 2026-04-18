---
name: adr-curator
description: Draft source-grounded Architecture Decision Record candidates from CodeBuddy review packets, architecture reviews, findings, migration notes, docs, PR notes, or repo evidence with status, context, decision, consequences, supersession links, validation notes, and explicit proposed/accepted boundaries while stopping before accepting decisions or mutating repositories.
---

# ADR Curator

Draft reviewable Architecture Decision Record candidates from CodeBuddy packets,
architecture reviews, findings, migration notes, docs, PR notes, and repo
evidence. This skill is a curation lane, not a decision authority.

It may identify candidate decisions, draft proposed ADR text, preserve evidence,
and connect supersession links. It must not accept an ADR, change architecture,
create tracker items, open PRs, or mutate customer repositories unless a
separate approved implementation workflow explicitly authorizes that work.

## Quick Start

1. Confirm the bounded source:
   - CodeBuddy review packet
   - architecture review artifact
   - synthesis artifact
   - findings file
   - migration notes or docs path
   - repo path with existing ADR evidence
2. Prefer packet artifacts when available:
   - `evidence_index.json`
   - `repo_inventory.json`
   - `run_manifest.json`
   - specialist architecture or dependency review artifacts
3. Run the deterministic curation scaffold when local access is available:
   - `scripts/curate_adrs.py <source-root> --out <artifact-root>`
4. Review the emitted ADR candidates and tighten the status, context, decision,
   consequences, alternatives, and supersession links.
5. Stop before accepting, publishing, committing, or opening tracker/PR work.

## Focus

Prioritize:

- candidate decisions already implied by code, docs, migrations, reviews, or PRs
- decisions that need durable explanation before follow-up implementation
- status clarity: proposed, accepted-existing, superseded-existing, or deferred
- context, decision, consequences, alternatives, validation notes, and evidence
- supersession and replacement relationships between decisions
- clear distinction between observed repo truth and proposed new ADR language

Defer primary ownership of these areas:

- finding original architecture defects: `repo-architecture-review`
- planning executable architecture checks: `architecture-fitness-function-author`
- creating follow-up issues: `finding-to-issue-planner`
- final report synthesis: `repo-review-synthesis` or report writer skills
- editing ADR files in a repository: implementation issue workflow

## Required Inputs

At minimum, gather:

- `repo_root`
- one concrete target:
  - `target.review_packet_path`
  - `target.architecture_review_artifact`
  - `target.findings_file`
  - `target.migration_notes_path`
  - `target.target_path`

Useful additional inputs:

- `artifact_root`
- `adr_status_policy`
- `supersession_policy`
- `validation_mode`
- `existing_adr_dir`

If there is no bounded decision source or evidence, stop and report `blocked`.

## Workflow

### 1. Establish Scope

Record:

- source artifacts reviewed
- repo/path scope
- existing ADR directory, if known
- whether accepted ADRs already exist
- whether proposed ADR drafts are allowed

Do not widen one candidate decision into a whole architecture history project.

### 2. Extract Candidate Decisions

Look for:

- explicit ADR files or headings
- `Status`, `Context`, `Decision`, `Consequences`, and `Supersedes` fields
- architecture-review candidate ADR sections
- migration notes that imply durable decisions
- repeated review findings that need a policy decision before automation
- PR notes that changed boundaries, providers, persistence, runtime, or
  lifecycle behavior

For each candidate, decide whether it is:

- `proposed`: a draft decision needs human acceptance
- `accepted_existing`: an existing source already records acceptance
- `superseded_existing`: an existing source records supersession
- `deferred`: evidence is too weak or the decision boundary is unclear

Default to `proposed` when status is not explicit.

### 3. Draft ADR Candidates

Each ADR candidate should include:

- ADR id
- title
- status
- source evidence
- context
- decision
- consequences
- alternatives considered
- supersession links
- validation notes
- approval boundary

Keep proposed ADRs visibly proposed. Do not convert a proposed draft into an
accepted decision without explicit source evidence.

### 4. Emit Handoffs

Recommended handoffs:

- `architecture-fitness-function-author` for executable checks after a decision
  is approved or stable enough to plan
- `finding-to-issue-planner` for follow-up issue candidates
- implementation issue workflow for committing accepted ADR files
- `repo-architecture-review` when evidence is too weak to draft safely

Do not invoke downstream skills automatically unless the operator explicitly
asks for that follow-on execution.

## Output Expectations

Default output should include:

- ADR candidate catalog
- proposed ADR drafts
- accepted or superseded existing decisions, if any
- deferred decision candidates
- supersession map
- validation notes
- approval boundary
- residual decision risk

Use `references/output-contract.md` and the shared suite contract in
`adl/tools/skills/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md`.

## Stop Boundary

Stop after producing the ADR candidate packet.

Do not:

- accept, reject, supersede, or publish architecture decisions
- edit ADR files, docs, tests, CI, code, policy files, issues, or PRs
- mutate customer repositories
- create issues or PRs
- claim a decision has been accepted when only a proposed draft exists
- use network or paid services

## CodeBuddy Integration Notes

This skill consumes CodeBuddy packet artifacts, review outputs, and repo
evidence. It produces ADR candidate packets that can feed architecture review,
fitness-function planning, issue planning, implementation work, or final
customer reports after human review.

Deferred automation:

- language-specific architecture decision inference
- automatic ADR file placement and numbering
- repo-specific ADR template adoption
- cross-run ADR history and supersession graph maintenance

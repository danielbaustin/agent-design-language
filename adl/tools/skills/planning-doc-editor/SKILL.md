---
name: planning-doc-editor
description: Normalize and correct ADL milestone or project planning documents that are generated from, intended for, or being aligned with versioned planning templates without editing C-SDLC issue cards or inventing review, approval, release, PR, or closeout truth. Use when README, WBS, sprint, vision, design, decision, demo matrix, checklist, release plan, release notes, or handoff docs have placeholder residue, stale planning claims, missing required sections, or drift from the available planning-template contract.
---

# Planning Doc Editor

This skill owns bounded editing of ADL planning documents.

Its job is to:
- normalize milestone and project planning docs
- remove unresolved placeholder residue
- repair required-section drift against the planning-template contract
- preserve the difference between generated, reviewed, and approved documents
- keep planning documents aligned with source-backed milestone truth
- stop before editing C-SDLC issue cards or claiming implementation results

This is a planning-document editor, not a lifecycle-card editor.

It must not replace:
- `sip-editor`
- `stp-editor`
- `spp-editor`
- `srp-editor`
- `sor-editor`
- `documentation-specialist`
- `workflow-conductor`

## Entry Conditions

Use this skill when all of the following are true:
- there is one bounded planning-document target or packet
- the requested defect is local to planning documentation
- the source evidence or intended milestone context is available
- the work does not require changing SIP/STP/SPP/SRP/SOR cards

Examples:
- a generated milestone README still contains placeholders
- a WBS document is missing required sections
- a sprint plan says it is approved when only generation occurred
- a demo matrix has stale milestone labels
- a release plan includes absolute host paths or temp paths
- a planning packet needs status wording normalized before review

Do not use this skill for:
- card-local defects
- PR publication
- release approval
- issue closeout
- repo-wide docs rewrites
- inventing missing milestone decisions

## Required Inputs

At minimum, gather:
- `repo_root`
- `target_path`
- `planning_doc_type`
- `editing_mode`
- `source_evidence`

Useful additional inputs:
- `template_version`
- `template_registry_path`
- `issue_number`
- `milestone`
- `source_issue_prompt`
- `review_findings`
- `status_truth`
- `validation_command`

Supported editing modes:
- `placeholder_cleanup`
- `required_section_repair`
- `status_truth_normalization`
- `planning_packet_review_cleanup`
- `template_contract_alignment`
- `portable_path_cleanup`

If no bounded target or source evidence exists, stop with `blocked`.

## Planning Document Types

The target planning-template family is expected to include:
- `README`
- `WBS`
- `SPRINT`
- `VISION`
- `FEATURE_DOC`
- `DESIGN`
- `DECISIONS`
- `DEMO_MATRIX`
- `MILESTONE_CHECKLIST`
- `RELEASE_PLAN`
- `RELEASE_NOTES`
- `HANDOFF`

When the versioned planning registry is available, use it as the template
contract. When it is unavailable, treat these as target document families
rather than proof that a template contract already exists for every type.

When a document type is unknown, classify it as `planning_doc_unknown` and
preserve extra caution. Do not infer approval or milestone truth from shape
alone.

## Workflow

### 1. Establish The Boundary

Record:
- target document path
- document type
- template version if known
- whether the document is generated, reviewed, approved, historical, or unknown
- source evidence used for milestone claims
- allowed edit scope

Planning docs may summarize issue, PR, and card truth, but they do not replace
those sources.

### 2. Build A Defect Ledger

Classify defects before editing:
- unresolved placeholders
- missing required sections
- stale milestone or sprint claims
- generated/reviewed/approved status drift
- unsupported release or PR claims
- stale commands
- stale paths
- absolute host paths
- duplicated card truth
- ambiguous owner or next-step wording

Prefer narrowing or qualifying unsupported claims over making them sound more
confident.

### 3. Apply Bounded Edits

Allowed edits:
- replace placeholders with source-backed values
- mark unknown values as explicit gaps instead of guessing
- add missing required sections when source evidence supports them
- demote unsupported approval or publication claims
- normalize status labels such as `generated`, `draft`, `reviewed`, or
  `approved`
- repair repo-relative links and portable paths
- clarify validation that was run, not run, or deferred
- add a short residual-risk or follow-up section when needed

Disallowed edits:
- editing SIP/STP/SPP/SRP/SOR cards
- claiming an issue, PR, demo, release, or review passed without evidence
- accepting ADRs or release decisions without human approval
- rewriting broad doc sets outside the bounded target
- hiding new scope inside a planning-doc cleanup

### 4. Validate Truthfully

Use the smallest validation that proves the edit:
- placeholder scan for generated docs
- required-section check when a template contract exists
- portable-path scan when docs may be published
- markdown link/path check when links are part of the defect

If validation is not run, record why.

### 5. Stop Boundary

Stop after the bounded planning-document edit and result report.

Do not:
- publish a PR
- close an issue
- update lifecycle cards unless routed separately to card editors
- run broad repo validation by reflex
- turn the planning doc into canonical workflow truth

## Handoffs

Use:
- `workflow-conductor` when routing state is unclear
- `documentation-specialist` for broad docs authoring or prose polish
- `stp-editor`, `sip-editor`, `spp-editor`, `srp-editor`, or `sor-editor` for
  card-local defects
- `records-hygiene` for lifecycle-record truth drift
- `portable-contract-normalizer` for broader path/host portability audits
- `review-readiness-cleanup` for review packet structural readiness

## Output

Return a concise structured result including:
- target path
- planning document type
- template version if known
- editing mode
- defects corrected
- claims demoted or qualified
- validation run
- validation not run
- residual risks
- recommended handoff

## Blocked States

Return `blocked` when:
- the target is a lifecycle card rather than a planning document
- source evidence is missing for requested factual claims
- the requested edit would approve, publish, or close work
- the requested edit requires repo-wide migration without explicit scope
- the template contract is required but unavailable

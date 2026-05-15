# CodeFriend Evidence Requirements

## Purpose

Define the minimum evidence and uncertainty rules for the bounded
`CodeFriend` review-packet product surface.

## Required Upstream Artifacts

### Review packet minimum

From `repo-packet-builder`:

- `run_manifest.json`
- `repo_scope.md`
- `repo_inventory.json`
- `evidence_index.json`
- `specialist_assignments.json`

Why required:

- without these, CodeFriend cannot truthfully state scope, exclusions, or
  evidence routing

### Review artifact minimum

At least one of:

- specialist findings
- synthesis artifact
- explicit no-findings record

Why required:

- CodeFriend must not generate a polished report from missing review evidence

### Product report minimum

From `product-report-writer`:

- legacy compatibility filenames retained by the current product-report skill
  contract:
- `codebuddy_product_report.md`
- `codebuddy_product_report.json`

Why required:

- the report must exist in both human-readable and machine-readable form

## Evidence Rules

Every customer-facing or reviewer-facing claim should be traceable to one of:

- packet scope artifact
- evidence index entry
- specialist finding
- synthesis finding
- diagram packet
- test recommendation artifact

Claims without explicit evidence must be labeled as:

- assumption
- recommendation
- or gap

They must not be presented as established fact.

## Uncertainty Rules

CodeFriend product surfaces must preserve:

- missing specialist lanes
- non-reviewed surfaces
- confidence or uncertainty when the source artifact provides it
- disagreement between specialist reviewers
- residual risk after remediation recommendations

## Product-Language Rules

Allowed:

- bounded review workflow
- packet-to-report system
- evidence-backed repository review surface
- customer-grade report packaging from completed review artifacts

Not allowed:

- autonomous correctness guarantees
- automatic security certification
- automatic merge approval
- silent remediation claims

## Publication Boundary

Before any external use, CodeFriend should require:

- redaction/evidence audit
- review-quality evaluation

Reason:

- a complete internal review packet is not automatically publishable

## Demo And Fixture Boundary

If a demo or fixture path is used:

- fixture mode must be explicit
- skipped-provider paths must be explicit
- no live tool or publication authority should be implied unless another issue
  explicitly proves that boundary

## Non-Claims

- These requirements do not prove review quality by themselves.
- These requirements do not replace human judgment.
- These requirements do not make Google Workspace or any external surface
  canonical repo truth.
